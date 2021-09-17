use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct CreateRecvMPSTSession {
    func_name: syn::Ident,
    sender: syn::Ident,
    receiver: syn::Ident,
    meshedchannels_name: syn::Ident,
    nsessions: u64,
    exclusion: u64,
}

impl Parse for CreateRecvMPSTSession {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let sender = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let receiver = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let meshedchannels_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let nsessions = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();
        <Token![,]>::parse(input)?;

        let exclusion = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(CreateRecvMPSTSession {
            func_name,
            sender,
            receiver,
            meshedchannels_name,
            nsessions,
            exclusion,
        })
    }
}

impl From<CreateRecvMPSTSession> for proc_macro2::TokenStream {
    fn from(input: CreateRecvMPSTSession) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl CreateRecvMPSTSession {
    fn expand(&self) -> proc_macro2::TokenStream {
        let func_name = self.func_name.clone();
        let sender = self.sender.clone();
        let receiver = self.receiver.clone();
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
                    #temp_ident : mpstthree::binary::struct_trait::session::Session ,
                }
            })
            .collect();

        let all_recv: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                if i != self.exclusion {
                    quote! {}
                } else {
                    let temp_ident =
                        syn::Ident::new(&format!("session{}", i), proc_macro2::Span::call_site());
                    quote! {
                        let (v, new_session) = mpstthree::binary::recv::recv(s.#temp_ident)?;
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
                        mpstthree::binary::struct_trait::recv::Recv<T, #temp_ident >,
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
                s: #meshedchannels_name<
                    #(
                        #send_types
                    )*
                    #sender<R>,
                    #receiver<mpstthree::role::end::RoleEnd>,
                >,
            ) -> Result<
                (
                    T,
                    #meshedchannels_name<
                        #(
                            #session_types
                        )*
                        R,
                        #receiver<mpstthree::role::end::RoleEnd>,
                    >,
                ),
                Box<dyn std::error::Error>,
            >
            where
                T: std::marker::Send,
                #(
                    #session_types_struct
                )*
                R: mpstthree::role::Role,
            {
                #(
                    #all_recv
                )*

                let new_stack = {
                    fn temp<R>(r: #sender<R>) -> R
                    where
                        R: mpstthree::role::Role,
                    {
                        let (here, there) = <R as mpstthree::role::Role>::new();
                        r.sender.send(there).unwrap_or(());
                        here
                    }
                    temp(s.stack)
                };

                Ok((
                    v,
                    #meshedchannels_name {
                        #(
                            #new_sessions
                        )*
                        stack: new_stack,
                        name: s.name,
                    }
                ))
            }
        }
    }
}
