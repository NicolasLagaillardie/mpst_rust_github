use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Token};

#[derive(Debug)]
pub(crate) struct CreateSendMPSTSession {
    func_name: Ident,
    receiver: Ident,
    sender: Ident,
    meshedchannels_name: Ident,
    n_sessions: u64,
    exclusion: u64,
}

impl Parse for CreateSendMPSTSession {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let receiver = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let sender = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let meshedchannels_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let n_sessions = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();
        <Token![,]>::parse(input)?;

        let exclusion = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(CreateSendMPSTSession {
            func_name,
            receiver,
            sender,
            meshedchannels_name,
            n_sessions,
            exclusion,
        })
    }
}

impl From<CreateSendMPSTSession> for TokenStream {
    fn from(input: CreateSendMPSTSession) -> TokenStream {
        input.expand()
    }
}

impl CreateSendMPSTSession {
    fn expand(&self) -> TokenStream {
        let func_name = &self.func_name;
        let receiver = &self.receiver;
        let sender = &self.sender;
        let meshedchannels_name = &self.meshedchannels_name;

        let session_types: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let session_types_struct: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                quote! {
                    #temp_ident : mpstthree::binary::struct_trait::session::Session ,
                }
            })
            .collect();

        let all_send: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                if i != self.exclusion {
                    quote! {}
                } else {
                    let temp_ident = Ident::new(&format!("session{}", i), Span::call_site());
                    quote! {
                        let new_session = mpstthree::binary::send::send(x, s.#temp_ident);
                    }
                }
            })
            .collect();

        let send_types: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                if i == self.exclusion {
                    quote! {
                        mpstthree::binary::struct_trait::send::Send<T, #temp_ident >,
                    }
                } else {
                    quote! {
                        #temp_ident ,
                    }
                }
            })
            .collect();

        let new_sessions: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("session{}", i), Span::call_site());
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
                    #sender,
                >,
            ) -> #meshedchannels_name<
                    #(
                        #session_types
                    )*
                    R,
                    #sender,
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

                let new_stack = s.stack.continuation();

                #meshedchannels_name {
                    #(
                        #new_sessions
                    )*
                    stack: new_stack,
                    name: s.name,
                }
            }
        }
    }
}
