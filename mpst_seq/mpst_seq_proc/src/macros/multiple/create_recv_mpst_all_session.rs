use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct CreateRecvMPSTAllSessionMacroInput {
    func_name: syn::Ident,
    sender: syn::Ident,
    receiver: syn::Ident,
    meshedchannels_name: syn::Ident,
    nsessions: u64,
    exclusion: u64,
}

impl Parse for CreateRecvMPSTAllSessionMacroInput {
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

        Ok(CreateRecvMPSTAllSessionMacroInput {
            func_name,
            sender,
            receiver,
            meshedchannels_name,
            nsessions,
            exclusion,
        })
    }
}

impl From<CreateRecvMPSTAllSessionMacroInput> for proc_macro2::TokenStream {
    fn from(input: CreateRecvMPSTAllSessionMacroInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl CreateRecvMPSTAllSessionMacroInput {
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

        let recv_types: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
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
                        #recv_types
                    )*
                    #sender<R, R>,
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

                let (new_stack, _) = {
                    fn temp<R1, R2>(r: #sender<R1, R2>) -> (R1, R2)
                    where
                        R1: mpstthree::role::Role,
                        R2: mpstthree::role::Role,
                    {
                        let (here1, there1) = <R1 as mpstthree::role::Role>::new();
                        let (here2, there2) = <R2 as mpstthree::role::Role>::new();
                        r.sender1.send(there1).unwrap_or(());
                        r.sender2.send(there2).unwrap_or(());
                        (here1, here2)
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
