use crate::common_functions::expand::parenthesised::parenthesised;
use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::Result;

pub(crate) fn token_stream(input: ParseStream) -> Result<Vec<TokenStream>> {
    Ok(parenthesised(TokenStream::parse(input)?))
}
