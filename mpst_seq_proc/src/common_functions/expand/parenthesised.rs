use proc_macro2::{TokenStream, TokenTree};
use quote::quote;

/// Expand parenthesised blocks
pub(crate) fn parenthesised(stream: TokenStream) -> Vec<TokenStream> {
    let mut result: Vec<TokenStream> = Vec::new();

    for tt in stream.into_iter() {
        match tt {
            TokenTree::Group(group) => {
                for elt in group.stream().into_iter() {
                    match elt {
                        TokenTree::Ident(i) => result.push(quote! {#i}),
                        _ => {}
                    };
                }
            }
            _ => {}
        }
    }

    result
}

/// Expand parenthesised blocks for labels
pub(crate) fn parenthesised_groups(stream: TokenStream) -> Vec<TokenStream> {
    let mut result: Vec<TokenStream> = Vec::new();

    for tt in stream.into_iter() {
        match tt {
            TokenTree::Group(group) => {
                result.push(group.stream());
            }
            _ => {}
        }
    }

    result
}
