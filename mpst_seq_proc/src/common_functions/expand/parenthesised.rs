use proc_macro2::{TokenStream, TokenTree};
use quote::quote;

/// Expand parenthesised blocks
pub(crate) fn parenthesised(stream: TokenStream) -> Vec<TokenStream> {
    let mut result: Vec<TokenStream> = Vec::new();

    if let Some(TokenTree::Group(group)) = stream.into_iter().next() {
        for elt in group.stream().into_iter() {
            if let TokenTree::Ident(i) = elt {
                result.push(quote! {#i});
            }
        }
    }

    result
}

/// Expand parenthesised blocks
pub(crate) fn get_all_roles(stream: TokenStream) -> Vec<TokenStream> {
    let mut result: Vec<TokenStream> = Vec::new();

    for elt_stream in stream.into_iter() {
        if let TokenTree::Group(group) = elt_stream {
            for elt_group in group.stream().into_iter() {
                if let TokenTree::Ident(i) = elt_group {
                    result.push(quote! {#i});
                }
            }
        }
    }

    result
}

/// Expand parenthesised blocks for labels
pub(crate) fn parenthesised_groups(stream: TokenStream) -> Vec<TokenStream> {
    let mut result: Vec<TokenStream> = Vec::new();

    for tt in stream.into_iter() {
        if let TokenTree::Group(group) = tt {
            for elt in group.stream().into_iter() {
                if let TokenTree::Group(i) = elt {
                    result.push(quote! {#i});
                }
            }
        }
    }

    result
}
