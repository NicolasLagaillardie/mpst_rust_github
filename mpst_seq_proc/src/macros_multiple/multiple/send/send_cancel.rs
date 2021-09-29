use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct SendCancel {
    func_name: syn::Ident,
    name: syn::Ident,
    meshedchannels_name: syn::Ident,
    nsessions: u64,
    msg: syn::Expr,
}

impl Parse for SendCancel {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let meshedchannels_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let nsessions = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();
        <Token![,]>::parse(input)?;

        let msg = syn::Expr::parse(input)?;

        Ok(SendCancel {
            func_name,
            name,
            meshedchannels_name,
            nsessions,
            msg,
        })
    }
}

impl From<SendCancel> for proc_macro2::TokenStream {
    fn from(input: SendCancel) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl SendCancel {
    fn expand(&self) -> proc_macro2::TokenStream {
        // Get the basic elements
        let func_name = self.func_name.clone();
        let name = self.name.clone();
        let meshedchannels_name = self.meshedchannels_name.clone();
        let msg = self.msg.clone();

        // Build the vec with all the types S1,..,SN
        let session_types: Vec<syn::Ident> = (1..(self.nsessions - 1))
            .map(|i| format_ident!("S{}", i))
            .collect();

        quote! {
            fn #func_name<#( #session_types , )* R>(
                s: #meshedchannels_name<
                    mpstthree::binary::struct_trait::end::End,
                    #( #session_types , )*
                    R,
                    #name<mpstthree::role::end::RoleEnd>,
                >,
            ) -> std::result::Result<(), Box<dyn std::error::Error>>
            where
                #( #session_types : mpstthree::binary::struct_trait::session::Session , )*
                R: mpstthree::role::Role,
            {
                s.session1.sender.send(mpstthree::binary::struct_trait::end::Signal::Cancel).unwrap();;
                mpstthree::binary::cancel::cancel(s);
                panic!("{:?}", #msg);
            }
        }
    }
}
