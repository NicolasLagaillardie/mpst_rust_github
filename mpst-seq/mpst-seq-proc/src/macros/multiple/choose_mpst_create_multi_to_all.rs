use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct ChooseMultiCreateToAllMacroInput {
    name_macro: syn::Ident,
    receivers: Vec<proc_macro2::TokenStream>,
    sender: syn::Ident,
    meshedchannels_name: syn::Ident,
    exclusion: u64,
}

fn expand_parenthesized(stream: &proc_macro2::TokenStream) -> Vec<proc_macro2::TokenStream> {
    let mut out: Vec<proc_macro2::TokenStream> = Vec::new();
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

impl Parse for ChooseMultiCreateToAllMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // The name of the macro
        let name_macro = syn::Ident::parse(input)?;

        <Token![,]>::parse(input)?;

        // The receivers
        let content_receivers;
        let _parentheses = syn::parenthesized!(content_receivers in input);
        let receivers = proc_macro2::TokenStream::parse(&content_receivers)?;

        let all_receivers: Vec<proc_macro2::TokenStream> = expand_parenthesized(&receivers);

        <Token![,]>::parse(input)?;

        // The sender
        let sender = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        // The meshedchannels_name
        let meshedchannels_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        // The index of the sender
        let exclusion = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(ChooseMultiCreateToAllMacroInput {
            name_macro,
            receivers: all_receivers,
            sender,
            meshedchannels_name,
            exclusion,
        })
    }
}

impl From<ChooseMultiCreateToAllMacroInput> for proc_macro2::TokenStream {
    fn from(input: ChooseMultiCreateToAllMacroInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl ChooseMultiCreateToAllMacroInput {
    fn expand(&self) -> proc_macro2::TokenStream {
        let name_macro = self.name_macro.clone();
        let all_receivers = self.receivers.clone();
        let sender = self.sender.clone();
        let meshedchannels_name = self.meshedchannels_name.clone();
        let exclusion = self.exclusion;

        quote! {
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
                    );
                }
            }
        }
    }
}
