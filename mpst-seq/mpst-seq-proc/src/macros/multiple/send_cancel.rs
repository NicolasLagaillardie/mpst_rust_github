use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct SendCancelMacroInput {
    func_name: syn::Ident,
    name: syn::Ident,
    sessionmpst_name: syn::Ident,
    nsessions: u64,
    msg: proc_macro2::TokenStream,
}

impl Parse for SendCancelMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let sessionmpst_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let nsessions = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();
        <Token![,]>::parse(input)?;

        let content_msg;
        let _parentheses = syn::parenthesized!(content_msg in input);
        let msg = proc_macro2::TokenStream::parse(&content_msg)?;

        Ok(SendCancelMacroInput {
            func_name,
            name,
            sessionmpst_name,
            nsessions,
            msg,
        })
    }
}

impl From<SendCancelMacroInput> for proc_macro2::TokenStream {
    fn from(input: SendCancelMacroInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl SendCancelMacroInput {
    fn expand(&self) -> proc_macro2::TokenStream {
        // Get the basic elements
        let func_name = self.func_name.clone();
        let name = self.name.clone();
        let sessionmpst_name = self.sessionmpst_name.clone();
        let msg = self.msg.clone();

        // Build the vec with all the types S1,..,SN
        let session_types: Vec<syn::Ident> =
            (1..(self.nsessions - 1)).map(|i| format_ident!("S{}", i)).collect();

        quote! {
            fn #func_name<#( #session_types , )* R>(
                s: #sessionmpst_name<
                    mpstthree::binary::struct_trait::End,
                    #( #session_types , )*
                    R,
                    #name<mpstthree::role::end::RoleEnd>,
                >,
            ) -> std::result::Result<(), Box<dyn std::error::Error>>
            where
                #( #session_types : mpstthree::binary::struct_trait::Session , )*
                R: mpstthree::role::Role,
            {
                s.session1.sender.send(mpstthree::binary::struct_trait::Signal::Cancel).unwrap();;
                mpstthree::binary::cancel::cancel(s);
                panic!("{:?}", #msg);
            }
        }
    }
}
