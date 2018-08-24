extern crate quickcheck;
extern crate syn;
#[macro_use]
extern crate synstructure;
extern crate proc_macro2;

use proc_macro2::TokenStream;

decl_derive!([Arbitrary] => arbitrary_derive);

fn arbitrary_derive(s: synstructure::Structure) -> TokenStream {
    if s.variants().len() == 1 { // struct
        let con = s.variants()[0].construct(|_, _| quote! { Arbitrary::arbitrary(g) });
        s.gen_impl(quote! {
            extern crate quickcheck;

            use quickcheck::{Arbitrary, Gen};

            gen impl Arbitrary for @Self {
                fn arbitrary<G: Gen>(g: &mut G) -> Self {
                    #con
                }
            }
        })
    } else { // enum
        let mut variant_tokens = TokenStream::new();

        for (count, variant) in s.variants().iter().enumerate() {
            let constructor = variant.construct(|_, _| quote! { Arbitrary::arbitrary(g) });
            variant_tokens.extend(quote! { #count => #constructor, });
        }
        let count = s.variants().len();
        s.gen_impl(quote! {
            extern crate quickcheck;

            use quickcheck::{Arbitrary, Gen};

            gen impl Arbitrary for @Self {
                fn arbitrary<G: Gen>(g: &mut G) -> Self {
                    match g.gen_range(0, #count) {
                        #variant_tokens
                        _ => unreachable!()
                    }
                }
            }
        })
    }
}

#[test]
fn test_arbitrary_struct() {
    test_derive!{
        arbitrary_derive {
            #[derive(Clone)]
            struct ArbitraryTest(u8, bool);
        }
        expands to {
            #[allow(non_upper_case_globals)]
            const _DERIVE_Arbitrary_FOR_ArbitraryTest : () = {
                extern crate quickcheck;

                use quickcheck::{Arbitrary, Gen};

                impl Arbitrary for ArbitraryTest {
                    fn arbitrary<G: Gen>(g: & mut G) -> Self {
                        ArbitraryTest(Arbitrary::arbitrary(g),
                                      Arbitrary::arbitrary(g), )
                    }
                }
            };
        }
    }
}

#[test]
fn test_arbitrary_enum() {
    test_derive!{
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
            const _DERIVE_Arbitrary_FOR_ArbitraryTest : () = {
                extern crate quickcheck;

                use quickcheck::{Arbitrary, Gen};

                impl Arbitrary for ArbitraryTest {
                    fn arbitrary<G: Gen>(g: & mut G) -> Self {
                        match g.gen_range(0, 3usize) {
                            0usize => ArbitraryTest::A,
                            1usize => ArbitraryTest::B(Arbitrary::arbitrary(g),
                                                       Arbitrary::arbitrary(g),
                                                      ),
                            2usize => ArbitraryTest::C {
                                    b : Arbitrary::arbitrary(g),
                                    d : Arbitrary::arbitrary(g),
                                },
                            _ => unreachable!()
                        }
                    }
                }
            };
        }
    }
}
