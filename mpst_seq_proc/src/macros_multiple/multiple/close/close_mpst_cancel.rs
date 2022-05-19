use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Token};

#[derive(Debug)]
pub(crate) struct CloseMpstCancel {
    func_name: Ident,
    meshedchannels_name: Ident,
    n_sessions: u64,
}

impl Parse for CloseMpstCancel {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let meshedchannels_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let n_sessions = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(CloseMpstCancel {
            func_name,
            meshedchannels_name,
            n_sessions,
        })
    }
}

impl From<CloseMpstCancel> for TokenStream {
    fn from(input: CloseMpstCancel) -> TokenStream {
        input.expand()
    }
}

impl CloseMpstCancel {
    fn expand(&self) -> TokenStream {
        let func_name = &self.func_name;
        let meshedchannels_name = &self.meshedchannels_name;

        let session_types: Vec<TokenStream> = (1..self.n_sessions)
            .map(|_| {
                quote! { mpstthree::binary::struct_trait::end::End , }
            })
            .collect();

        let session_send: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("session{}", i), Span::call_site());
                quote! {
                    s.#temp_ident.sender.send(mpstthree::binary::struct_trait::end::Signal::Stop)?;
                }
            })
            .collect();

        let session_recv: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("session{}", i), Span::call_site());
                quote! {
                    s.#temp_ident.receiver.recv()?;
                }
            })
            .collect();

        quote! {
            fn #func_name<N>(s: #meshedchannels_name<
                #(
                    #session_types
                )*
                mpstthree::role::end::RoleEnd,
                N
            >
            ) -> Result<(), Box<dyn std::error::Error>>
            where
                N: mpstthree::name::Name,
            {
                #(
                    #session_send
                )*

                #(
                    #session_recv
                )*

                Ok(())
            }
        }
    }
}
