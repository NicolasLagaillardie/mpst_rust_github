use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct CloseMpst {
    func_name: syn::Ident,
    meshedchannels_name: syn::Ident,
    n_sessions: u64,
}

impl Parse for CloseMpst {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let meshedchannels_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let n_sessions = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(CloseMpst {
            func_name,
            meshedchannels_name,
            n_sessions,
        })
    }
}

impl From<CloseMpst> for proc_macro2::TokenStream {
    fn from(input: CloseMpst) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl CloseMpst {
    fn expand(&self) -> proc_macro2::TokenStream {
        let func_name = self.func_name.clone();
        let meshedchannels_name = self.meshedchannels_name.clone();

        let session_types: Vec<proc_macro2::TokenStream> = (1..self.n_sessions)
            .map(|_| {
                quote! { mpstthree::binary::struct_trait::end::End , }
            })
            .collect();

        let session_send: Vec<proc_macro2::TokenStream> = (1..self.n_sessions)
                .map(|i| {
                    let temp_ident =
                        syn::Ident::new(&format!("session{}", i), proc_macro2::Span::call_site());
                    quote! {
                        s.#temp_ident.sender.send(mpstthree::binary::struct_trait::end::Signal::Stop).unwrap_or(());
                    }
                })
                .collect();

        let session_recv: Vec<proc_macro2::TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("session{}", i), proc_macro2::Span::call_site());
                quote! {
                    s.#temp_ident.receiver.recv()?;
                }
            })
            .collect();

        quote! {
            fn #func_name<R>(s: #meshedchannels_name<
                #(
                    #session_types
                )*
                mpstthree::role::end::RoleEnd,
                R
            >
            ) -> Result<(), Box<dyn std::error::Error>>
            where
                R: mpstthree::role::Role,
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
