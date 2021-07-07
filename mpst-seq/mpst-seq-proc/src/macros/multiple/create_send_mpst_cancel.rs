use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct CreateSendMPSTCancelMacroInput {
    func_name: syn::Ident,
    receiver: syn::Ident,
    sender: syn::Ident,
    meshedchannels_name: syn::Ident,
    nsessions: u64,
    exclusion: u64,
}

impl Parse for CreateSendMPSTCancelMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let receiver = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let sender = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let meshedchannels_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let nsessions = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();
        <Token![,]>::parse(input)?;

        let exclusion = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(CreateSendMPSTCancelMacroInput {
            func_name,
            receiver,
            sender,
            meshedchannels_name,
            nsessions,
            exclusion,
        })
    }
}

impl From<CreateSendMPSTCancelMacroInput> for proc_macro2::TokenStream {
    fn from(input: CreateSendMPSTCancelMacroInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl CreateSendMPSTCancelMacroInput {
    fn expand(&self) -> proc_macro2::TokenStream {
        let func_name = self.func_name.clone();
        let receiver = self.receiver.clone();
        let sender = self.sender.clone();
        let meshedchannels_name = self.meshedchannels_name.clone();

        let session_types: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let session_types_struct: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident : mpstthree::binary::struct_trait::Session ,
                }
            })
            .collect();

        let all_send: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                if i != self.exclusion {
                    quote! {}
                } else {
                    let temp_ident =
                        syn::Ident::new(&format!("session{}", i), proc_macro2::Span::call_site());
                    quote! {
                        let new_session = mpstthree::binary::send::send_canceled(x, s.#temp_ident)?;
                    }
                }
            })
            .collect();

        let send_types: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                if i == self.exclusion {
                    quote! {
                        mpstthree::binary::struct_trait::Send<T, #temp_ident >,
                    }
                } else {
                    quote! {
                        #temp_ident ,
                    }
                }
            })
            .collect();

        let new_sessions: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("session{}", i), proc_macro2::Span::call_site());
                if i == self.exclusion {
                    quote! {
                        #temp_ident : new_session ,
                    }
                } else {
                    quote! {
                        #temp_ident : s.#temp_ident ,
                    }
                }
            })
            .collect();

        quote! {
            fn #func_name<
                T,
                #(
                    #session_types
                )*
                R
            >(
                x: T,
                s: #meshedchannels_name<
                    #(
                        #send_types
                    )*
                    #receiver<R>,
                    #sender<mpstthree::role::end::RoleEnd>,
                >,
            ) -> Result<
                    #meshedchannels_name<
                        #(
                            #session_types
                        )*
                        R,
                        #sender<mpstthree::role::end::RoleEnd>,
                    >,
                    std::boxed::Box<dyn std::error::Error>
                >
            where
                T: std::marker::Send,
                #(
                    #session_types_struct
                )*
                R: mpstthree::role::Role,
            {
                #(
                    #all_send
                )*

                let new_stack = {
                    fn temp<R>(r: #receiver<R>) -> R
                    where
                        R: mpstthree::role::Role,
                    {
                        let (here, there) = <R as mpstthree::role::Role>::new();
                        r.sender.send(there).unwrap_or(());
                        here
                    }
                    temp(s.stack)
                };

                Ok(
                    #meshedchannels_name {
                        #(
                            #new_sessions
                        )*
                        stack: new_stack,
                        name: s.name,
                    }
                )
            }
        }
    }
}
