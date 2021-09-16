use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct CloseMpstInterleavedMacroInput {
    func_name: syn::Ident,
    meshedchannels_name: syn::Ident,
    nsessions: u64,
}

impl Parse for CloseMpstInterleavedMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let meshedchannels_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let nsessions = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(CloseMpstInterleavedMacroInput {
            func_name,
            meshedchannels_name,
            nsessions,
        })
    }
}

impl From<CloseMpstInterleavedMacroInput> for proc_macro2::TokenStream {
    fn from(input: CloseMpstInterleavedMacroInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl CloseMpstInterleavedMacroInput {
    fn expand(&self) -> proc_macro2::TokenStream {
        let func_name = self.func_name.clone();
        let meshedchannels_name = self.meshedchannels_name.clone();

        let role_names: Vec<proc_macro2::TokenStream> = (1..=self.nsessions)
            .map(|i| {
                let temp_name = syn::Ident::new(&format!("R{}", i), proc_macro2::Span::call_site());

                quote! {
                    #temp_name ,
                }
            })
            .collect();

            let role_struct: Vec<proc_macro2::TokenStream> = (1..=self.nsessions)
                .map(|i| {
                    let temp_name = syn::Ident::new(&format!("R{}", i), proc_macro2::Span::call_site());
    
                    quote! {
                        #temp_name : mpstthree::role::Role ,
                    }
                })
                .collect();

        let session_types: Vec<proc_macro2::TokenStream> = (1..=self.nsessions)
            .map(|i| {
                let temp_end_types: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
                    .map(|_| {
                        quote! { mpstthree::binary::struct_trait::end::End , }
                    })
                    .collect();

                let temp_session =
                    syn::Ident::new(&format!("s_{}", i), proc_macro2::Span::call_site());

                let temp_name = syn::Ident::new(&format!("R{}", i), proc_macro2::Span::call_site());

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

        let session_send: Vec<proc_macro2::TokenStream> = (1..=self.nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("s_{}", i), proc_macro2::Span::call_site());
                
                let temp_session_send: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
                    .map(|j| {
                        let temp_session =
                            syn::Ident::new(&format!("session{}", j), proc_macro2::Span::call_site());
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

        let session_recv: Vec<proc_macro2::TokenStream> = (1..=self.nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("s_{}", i), proc_macro2::Span::call_site());
                
                let temp_session_recv: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
                    .map(|j| {
                        let temp_session =
                            syn::Ident::new(&format!("session{}", j), proc_macro2::Span::call_site());
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
