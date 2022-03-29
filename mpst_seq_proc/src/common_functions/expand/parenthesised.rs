use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use proc_macro2::{TokenStream, TokenTree};
use quote::quote;

/// Expand parenthesised blocks
pub(crate) fn parenthesised(stream: TokenStream) -> Vec<TokenStream> {
    let mut enums: Vec<TokenStream> = Vec::new();
    let mut variants: Vec<TokenStream> = Vec::new();
    let mut which_enums_variants = true;

    let mut result: Vec<TokenStream> = Vec::new();

    for tt in stream.into_iter() {
        match tt {
            TokenTree::Group(group) => {
                for elt in group.stream().into_iter() {
                    match elt {
                        TokenTree::Ident(i) => {
                            if which_enums_variants {
                                enums.push(quote! {#i})
                            } else {
                                variants.push(quote! {#i})
                            }
                            which_enums_variants = !which_enums_variants;
                        }
                        _ => {}
                    };
                }
            }
            _ => {}
        }
    }

    for it in enums.iter().zip_longest(variants.iter()) {
        match it {
            Both(elt_enum, elt_variant) => result.push(quote! {#elt_enum::#elt_variant}),
            Left(_) => panic!("Too many enums"),
            Right(_) => panic!("Too many variants"),
        }
    }

    result
}
