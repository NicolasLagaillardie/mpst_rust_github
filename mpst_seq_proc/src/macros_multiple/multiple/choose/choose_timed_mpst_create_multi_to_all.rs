use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Token};

use crate::common_functions::expand::parenthesised::parenthesised;

#[derive(Debug)]
pub(crate) struct ChooseTimedMultiCreateToAll {
    name_macro: Ident,
    receivers: Vec<TokenStream>,
    sender: Ident,
    meshedchannels_name: Ident,
    exclusion: u64,
}

impl Parse for ChooseTimedMultiCreateToAll {
    fn parse(input: ParseStream) -> Result<Self> {
        // The name of the macro
        let name_macro = Ident::parse(input)?;

        <Token![,]>::parse(input)?;

        // The receivers
        let content_receivers;
        let _parentheses = syn::parenthesized!(content_receivers in input);
        let receivers = TokenStream::parse(&content_receivers)?;

        let all_receivers: Vec<TokenStream> = parenthesised(receivers);

        <Token![,]>::parse(input)?;

        // The sender
        let sender = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        // The meshedchannels_name
        let meshedchannels_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        // The index of the sender
        let exclusion = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(ChooseTimedMultiCreateToAll {
            name_macro,
            receivers: all_receivers,
            sender,
            meshedchannels_name,
            exclusion,
        })
    }
}

impl From<ChooseTimedMultiCreateToAll> for TokenStream {
    fn from(input: ChooseTimedMultiCreateToAll) -> TokenStream {
        input.expand()
    }
}

impl ChooseTimedMultiCreateToAll {
    fn expand(&self) -> TokenStream {
        let name_macro = &self.name_macro;
        let all_receivers = &self.receivers;
        let sender = &self.sender;
        let meshedchannels_name = &self.meshedchannels_name;
        let exclusion = self.exclusion;

        quote! {
            #[allow(unused_macros)]
            macro_rules! #name_macro {
                (
                    $session: expr,
                    $all_clocks:expr,
                    $( $label: path ),+ $(,)?
                ) => {
                    mpstthree::choose_timed_mpst_multi_to_all!(
                        $session ,
                        $all_clocks ,
                        $( $label , )* =>
                        #( #all_receivers , )* =>
                        #sender ,
                        #meshedchannels_name ,
                        #exclusion
                    )
                }
            }
        }
    }
}
