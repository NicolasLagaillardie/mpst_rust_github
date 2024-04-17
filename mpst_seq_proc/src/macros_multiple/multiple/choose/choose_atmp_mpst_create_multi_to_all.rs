use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Token};

#[derive(Debug)]
pub(crate) struct ChooseTimedMultiCreateToAll {
    name_macro: Ident,
    sender: Ident,
    meshedchannels_name: Ident,
    exclusion: u64,
}

impl Parse for ChooseTimedMultiCreateToAll {
    fn parse(input: ParseStream) -> Result<Self> {
        // The name of the macro
        let name_macro = Ident::parse(input)?;

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
                    mpstthree::choose_atmp_mpst_multi_to_all!(
                        $session ,
                        $all_clocks ,
                        $( $label , )* =>
                        #sender ,
                        #meshedchannels_name ,
                        #exclusion
                    )
                }
            }
        }
    }
}
