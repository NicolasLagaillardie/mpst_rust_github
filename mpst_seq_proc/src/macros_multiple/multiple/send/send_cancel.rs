use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, LitInt, Result, Token};

#[derive(Debug)]
pub(crate) struct SendCancel {
    func_name: Ident,
    name: Ident,
    meshedchannels_name: Ident,
    n_sessions: u64,
    msg: Expr,
}

impl Parse for SendCancel {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let meshedchannels_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let n_sessions = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();
        <Token![,]>::parse(input)?;

        let msg = Expr::parse(input)?;

        Ok(SendCancel {
            func_name,
            name,
            meshedchannels_name,
            n_sessions,
            msg,
        })
    }
}

impl From<SendCancel> for TokenStream {
    fn from(input: SendCancel) -> TokenStream {
        input.expand()
    }
}

impl SendCancel {
    fn expand(&self) -> TokenStream {
        // Get the basic elements
        let func_name = self.func_name.clone();
        let name = self.name.clone();
        let meshedchannels_name = self.meshedchannels_name.clone();
        let msg = self.msg.clone();

        // Build the vec with all the types S1,..,SN
        let session_types: Vec<Ident> = (1..(self.n_sessions - 1))
            .map(|i| format_ident!("S{}", i))
            .collect();

        quote! {
            fn #func_name<#( #session_types , )* R>(
                s: #meshedchannels_name<
                    mpstthree::binary::struct_trait::end::End,
                    #( #session_types , )*
                    R,
                    #name,
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
