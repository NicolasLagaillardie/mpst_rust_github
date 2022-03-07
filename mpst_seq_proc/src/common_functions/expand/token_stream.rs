use proc_macro2::{TokenStream, TokenTree};
use syn::parse::{Parse, ParseStream};
use syn::Result;

pub(crate) fn token_stream(input: ParseStream) -> Result<Vec<TokenStream>> {
    let content;
    let _parentheses = syn::parenthesized!(content in input);
    let token_stream = TokenStream::parse(&content)?;

    let mut result: Vec<TokenStream> = Vec::new();
    for tt in token_stream.into_iter() {
        let elt = match tt {
            TokenTree::Group(g) => Some(g.stream()),
            _ => None,
        };
        if let Some(elt_tt) = elt {
            result.push(elt_tt)
        }
    }

    Ok(result)
}
