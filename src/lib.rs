#![allow(warnings)]
#![recursion_limit="128"]
#[macro_use]
extern crate quote;
extern crate syn;
extern crate proc_macro;

use quote::Tokens;
use proc_macro::TokenStream;
use syn::{Ident,Body,Generics};

#[proc_macro_derive(Arbitrary)]
pub fn arbitrary(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let mut input = syn::parse_derive_input(&input).unwrap();

    add_type_bounds(&mut input.generics, "::quickcheck::Arbitrary");
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let name = input.ident;
    let arbitrary_body = arbitrary_body(&name, &input.body);
    let shrink_body = shrink_body(&name, &input.body);

    let output = quote! {
        impl #impl_generics ::quickcheck::Arbitrary for #name #ty_generics #where_clause {
            fn arbitrary<G: ::quickcheck::Gen>(gen: &mut G) -> Self {
                #arbitrary_body
            }

            fn shrink(&self) -> Box<Iterator<Item=Self>> {
                #shrink_body
            }
        }
    };

    output.parse().unwrap()
}

fn arbitrary_body(name: &Ident, body: &Body) -> Tokens {
    use syn::VariantData::*;
    match *body {
        Body::Enum(..) => panic!("derive(Arbitrary) only supports structs"),
        Body::Struct(Struct(ref fields)) => {
            let field = fields.iter().map(|field| &field.ident);
            quote! {
                #name {
                    #(#field: ::quickcheck::Arbitrary::arbitrary(gen)),*
                }
            }
        },
        Body::Struct(Tuple(ref fields)) => {
            let field = fields.iter().map(|field| &field.ident);
            quote! {
                #name (
                    #(#field ::quickcheck::Arbitrary::arbitrary(gen)),*
                )
            }
        },
        Body::Struct(Unit) => quote! {
            drop(gen);
            #name
        }
    }
}

fn shrink_body(name: &Ident, body: &Body) -> Tokens {
    use syn::VariantData::*;
    match *body {
        Body::Enum(..) => panic!("derive(Arbitrary) only supports structs"),
        Body::Struct(Struct(ref fields)) => {
            let field = &fields.iter().map(|field| &field.ident).collect::<Vec<_>>();
            quote! {
                let val = (#(self.#field.clone()),*);

                Box::new(val.shrink().map(|(#(#field),*)| #name { #(#field),* }))
            }
        },
        Body::Struct(Tuple(ref fields)) => {
            let field = &(0..fields.len()).map(|i| format!("val_{}", i)).map(quote::Ident::new).collect::<Vec<_>>();
            quote! {
                let #name(#(#field),*) = self.clone();
                let val = (#(#field),*);

                Box::new(val.shrink().map(|(#(#field),*)| #name(#(#field),*)))
            }
        },
        Body::Struct(Unit) => quote! {
            ::quickcheck::empty_shrinker()
        }
    }
}

fn add_type_bounds(generics: &mut Generics, bound: &str) {
    let bound = syn::parse_ty_param_bound(bound).unwrap();

    for param in &mut generics.ty_params {
        param.bounds.push(bound.clone());
    }
}
