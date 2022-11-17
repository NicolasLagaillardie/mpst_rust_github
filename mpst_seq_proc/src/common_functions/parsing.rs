use proc_macro2::TokenStream;
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Token};

use super::expand::parenthesised::get_all_roles;

pub(crate) fn parse_stream_roles(input: ParseStream) -> Result<(Ident, Vec<TokenStream>, u64)> {
    // Get name of the MeshedChannels
    let meshedchannels_name = Ident::parse(input)?;
    <Token![,]>::parse(input)?;

    // Get name of the Roles
    let all_roles = get_all_roles(TokenStream::parse(input)?);

    // Compute number of Roles
    let number_roles = u64::try_from(all_roles.len()).unwrap();

    Ok((meshedchannels_name, all_roles, number_roles))
}

pub(crate) fn parse_stream_sessions(input: ParseStream) -> Result<(Ident, Ident, u64)> {
    // Get name of the function
    let func_name = Ident::parse(input)?;
    <Token![,]>::parse(input)?;

    // Get name of the MeshedChannels
    let meshedchannels_name = Ident::parse(input)?;
    <Token![,]>::parse(input)?;

    // Get number of sessions
    let n_sessions = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();

    Ok((func_name, meshedchannels_name, n_sessions))
}
