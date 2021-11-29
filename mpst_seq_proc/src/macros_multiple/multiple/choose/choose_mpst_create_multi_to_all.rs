use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token, Ident, LitInt};
use proc_macro2::TokenStream;

#[derive(Debug)]
pub struct ChooseMultiCreateToAll {
    name_macro: Ident,
    receivers: Vec<TokenStream>,
    sender: Ident,
    meshedchannels_name: Ident,
    exclusion: u64,
}

fn expand_parenthesized(stream: &TokenStream) -> Vec<TokenStream> {
    let mut out: Vec<TokenStream> = Vec::new();
    for tt in stream.clone().into_iter() {
        let elt = match tt {
            proc_macro2::TokenTree::Group(g) => Some(g.stream()),
            _ => None,
        };
        if let Some(elt_tt) = elt {
            out.push(elt_tt)
        }
    }
    out
}

impl Parse for ChooseMultiCreateToAll {
    fn parse(input: ParseStream) -> Result<Self> {
        // The name of the macro
        let name_macro = Ident::parse(input)?;

        <Token![,]>::parse(input)?;

        // The receivers
        let content_receivers;
        let _parentheses = syn::parenthesized!(content_receivers in input);
        let receivers = TokenStream::parse(&content_receivers)?;

        let all_receivers: Vec<TokenStream> = expand_parenthesized(&receivers);

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
            receivers: all_receivers,
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
        let name_macro = self.name_macro.clone();
        let all_receivers = self.receivers.clone();
        let sender = self.sender.clone();
        let meshedchannels_name = self.meshedchannels_name.clone();
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
