use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Token};

#[derive(Debug)]
pub(crate) struct CreateSendHttpSession {
    func_name: Ident,
    sender: Ident,
    receiver: Ident,
    meshedchannels_name: Ident,
    n_sessions: u64,
    exclusion: u64,
}

impl Parse for CreateSendHttpSession {
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

        Ok(CreateSendHttpSession {
            func_name,
            sender,
            receiver,
            meshedchannels_name,
            n_sessions,
            exclusion,
        })
    }
}

impl From<CreateSendHttpSession> for TokenStream {
    fn from(input: CreateSendHttpSession) -> TokenStream {
        input.expand()
    }
}

impl CreateSendHttpSession {
    fn expand(&self) -> TokenStream {
        let func_name = &self.func_name;
        let sender = &self.sender;
        let receiver = &self.receiver;
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
                http: bool,
                req: hyper::Request<hyper::Body>
            ) -> Result<
                (
                    #meshedchannels_name<
                        #(
                            #session_types
                        )*
                        R,
                        #sender,
                    >,
                    hyper::client::ResponseFuture
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
                let https = hyper_tls::HttpsConnector::new();
                let client = hyper::Client::builder().build::<_, hyper::Body>(https);

                let new_req = match http {
                    true => req,
                    false => hyper::Request::default(),
                };

                #(
                    #all_send
                )*

                let new_stack = s.stack.continuation();

                Ok((
                    #meshedchannels_name {
                        #(
                            #new_sessions
                        )*
                        stack: new_stack,
                        name: s.name,
                    },
                    client.request(new_req)
                ))
            }
        }
    }
}
