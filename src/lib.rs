extern crate proc_macro2;
#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
extern crate rand;
#[cfg_attr(test, macro_use)]
extern crate syn;
#[macro_use]
extern crate synstructure;

use proc_macro2::TokenStream;

decl_derive!([Arbitrary] => arbitrary_derive);

fn arbitrary_derive(s: synstructure::Structure) -> TokenStream {
    let (g, body) = match s.variants().len() {
        // zero-variant enum
        0 => panic!("Cannot derive `Arbitrary` for an enum with no variants."),

        // struct or single-variant enum
        1 => {
            let body = s.variants()[0].construct(|_, _| quote! { ::quickcheck::Arbitrary::arbitrary(g) });
            let g = if let syn::Fields::Unit = s.variants()[0].ast().fields {
                quote!(_g)
            } else {
                quote!(g)
            };

            (g, body)
        },

        // multiple-variant enum
        _ => {
            let mut variant_tokens = TokenStream::new();

            for (count, variant) in s.variants().iter().enumerate() {
                let constructor = variant.construct(|_, _| quote! { ::quickcheck::Arbitrary::arbitrary(g) });
                variant_tokens.extend(quote! { #count => #constructor, });
            }

            let count = s.variants().len();

            let body = quote! {
                match ::rand::Rng::gen_range(g, 0, #count) {
                    #variant_tokens
                    _ => unreachable!()
                }
            };

            (quote!(g), body)
        },
    };

    s.gen_impl(quote! {
        gen impl ::quickcheck::Arbitrary for @Self {
            fn arbitrary<G: ::quickcheck::Gen>(#g: &mut G) -> Self {
                #body
            }
        }
    })
}

#[test]
fn test_arbitrary_unit_struct() {
    test_derive! {
        arbitrary_derive {
            #[derive(Clone)]
            struct ArbitraryTest;
        }
        expands to {
            #[allow(non_upper_case_globals)]
            const _DERIVE_quickcheck_Arbitrary_FOR_ArbitraryTest: () = {
                impl ::quickcheck::Arbitrary for ArbitraryTest {
                    fn arbitrary<G: ::quickcheck::Gen>(_g: &mut G) -> Self {
                        ArbitraryTest
                    }
                }
            };
        }
    }
}

#[test]
fn test_arbitrary_struct() {
    test_derive! {
        arbitrary_derive {
            #[derive(Clone)]
            struct ArbitraryTest(u8, bool);
        }
        expands to {
            #[allow(non_upper_case_globals)]
            const _DERIVE_quickcheck_Arbitrary_FOR_ArbitraryTest: () = {
                impl ::quickcheck::Arbitrary for ArbitraryTest {
                    fn arbitrary<G: ::quickcheck::Gen>(g: &mut G) -> Self {
                        ArbitraryTest(::quickcheck::Arbitrary::arbitrary(g),
                                      ::quickcheck::Arbitrary::arbitrary(g), )
                    }
                }
            };
        }
    }
}

#[test]
#[should_panic(expected = "Cannot derive `Arbitrary` for an enum with no variants.")]
fn test_arbitrary_zero_variant_enum() {
    let input = parse_quote! {
        #[derive(Clone)]
        enum ArbitraryTest {}
    };

    arbitrary_derive(synstructure::Structure::new(&input));
}

#[test]
fn test_arbitrary_enum() {
    test_derive! {
        arbitrary_derive {
            #[derive(Clone)]
            enum ArbitraryTest {
                A,
                B(usize, u32),
                C{ b: bool, d: (u16, u16) }
            }
        }
        expands to {
            #[allow(non_upper_case_globals)]
            const _DERIVE_quickcheck_Arbitrary_FOR_ArbitraryTest: () = {
                impl ::quickcheck::Arbitrary for ArbitraryTest {
                    fn arbitrary<G: ::quickcheck::Gen>(g: &mut G) -> Self {
                        match ::rand::Rng::gen_range(g, 0, 3usize) {
                            0usize => ArbitraryTest::A,
                            1usize => ArbitraryTest::B(::quickcheck::Arbitrary::arbitrary(g),
                                                       ::quickcheck::Arbitrary::arbitrary(g),
                                                      ),
                            2usize => ArbitraryTest::C {
                                    b : ::quickcheck::Arbitrary::arbitrary(g),
                                    d : ::quickcheck::Arbitrary::arbitrary(g),
                                },
                            _ => unreachable!()
                        }
                    }
                }
            };
        }
    }
}
