use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Token};

#[derive(Debug)]
pub(crate) struct ChooseMultiCreateToAll {
    name_macro: Ident,
    sender: Ident,
    meshedchannels_name: Ident,
    exclusion: u64,
}

impl Parse for ChooseMultiCreateToAll {
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

        Ok(ChooseMultiCreateToAll {
            name_macro,
            sender,
            meshedchannels_name,
            exclusion,
        })
    }
}

impl From<ChooseMultiCreateToAll> for TokenStream {
    fn from(input: ChooseMultiCreateToAll) -> TokenStream {
        input.expand()
    }
}

impl ChooseMultiCreateToAll {
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
                    $( $label: path ),+ $(,)?
                ) => {
                    mpstthree::choose_mpst_multi_to_all!(
                        $session ,
                        $( $label , )+ =>
                        #sender ,
                        #meshedchannels_name ,
                        #exclusion
                    )
                }
            }
        }
    }
}
