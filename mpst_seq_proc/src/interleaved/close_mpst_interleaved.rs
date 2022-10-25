use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Token};

#[derive(Debug)]
pub(crate) struct CloseMpstInterleaved {
    func_name: Ident,
    meshedchannels_name: Ident,
    n_sessions: u64,
}

impl Parse for CloseMpstInterleaved {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let meshedchannels_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let n_sessions = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(CloseMpstInterleaved {
            func_name,
            meshedchannels_name,
            n_sessions,
        })
    }
}

impl From<CloseMpstInterleaved> for TokenStream {
    fn from(input: CloseMpstInterleaved) -> TokenStream {
        input.expand()
    }
}

impl CloseMpstInterleaved {
    fn expand(&self) -> TokenStream {
        let func_name = &self.func_name;
        let meshedchannels_name = &self.meshedchannels_name;

        let role_names: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_name = Ident::new(&format!("R{i}"), Span::call_site());

                quote! {
                    #temp_name ,
                }
            })
            .collect();

        let role_struct: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_name = Ident::new(&format!("R{i}"), Span::call_site());

                quote! {
                    #temp_name : mpstthree::name::Name ,
                }
            })
            .collect();

        let session_types: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_end_types: Vec<TokenStream> = (1..self.n_sessions)
                    .map(|_| {
                        quote! { mpstthree::binary::struct_trait::end::End , }
                    })
                    .collect();

                let temp_session = Ident::new(&format!("s_{i}"), Span::call_site());

                let temp_name = Ident::new(&format!("R{i}"), Span::call_site());

                quote! {
                    #temp_session:
                        #meshedchannels_name<
                            #(
                                #temp_end_types
                            )*
                            mpstthree::role::end::RoleEnd,
                            #temp_name
                        > ,
                }
            })
            .collect();

        let session_send: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_ident =
                    Ident::new(&format!("s_{i}"), Span::call_site());

                let temp_session_send: Vec<TokenStream> = (1..self.n_sessions)
                    .map(|j| {
                        let temp_session =
                            Ident::new(&format!("session{j}"), Span::call_site());
                        quote! {
                            #temp_ident.#temp_session.sender.send(mpstthree::binary::struct_trait::end::Signal::Stop).unwrap_or(());
                        }
                    })
                    .collect();

                quote! {
                    #(
                        #temp_session_send
                    )*
                }
            })
            .collect();

        let session_recv: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("s_{i}"), Span::call_site());

                let temp_session_recv: Vec<TokenStream> = (1..self.n_sessions)
                    .map(|j| {
                        let temp_session = Ident::new(&format!("session{j}"), Span::call_site());
                        quote! {
                            #temp_ident.#temp_session.receiver.recv()?;
                        }
                    })
                    .collect();

                quote! {
                    #(
                        #temp_session_recv
                    )*
                }
            })
            .collect();

        quote! {
            fn #func_name<
                #(
                    #role_names
                )*
            >(
                #(
                    #session_types
                )*
            ) -> Result<(), Box<dyn std::error::Error>>
            where
                #(
                    #role_struct
                )*
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
