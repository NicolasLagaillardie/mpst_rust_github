use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Token};

#[derive(Debug)]
pub(crate) struct CreateRecvHttpSession {
    func_name: Ident,
    sender: Ident,
    receiver: Ident,
    meshedchannels_name: Ident,
    n_sessions: u64,
    exclusion: u64,
}

impl Parse for CreateRecvHttpSession {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let sender = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let receiver = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let meshedchannels_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let n_sessions = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();
        <Token![,]>::parse(input)?;

        let exclusion = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(CreateRecvHttpSession {
            func_name,
            sender,
            receiver,
            meshedchannels_name,
            n_sessions,
            exclusion,
        })
    }
}

impl From<CreateRecvHttpSession> for TokenStream {
    fn from(input: CreateRecvHttpSession) -> TokenStream {
        input.expand()
    }
}

impl CreateRecvHttpSession {
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

        let all_recv: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                if i != self.exclusion {
                    quote! {}
                } else {
                    let temp_ident = Ident::new(&format!("session{}", i), Span::call_site());
                    quote! {
                        let (v, new_session) = mpstthree::binary::recv::recv(s.#temp_ident)?;
                    }
                }
            })
            .collect();

        let recv_types: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
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
                s: #meshedchannels_name<
                    #(
                        #recv_types
                    )*
                    #sender<R>,
                    #receiver,
                >,
                http: bool,
                mut resp_future: Vec::<hyper::client::ResponseFuture>,
            ) -> Result<
                (
                    T,
                    #meshedchannels_name<
                        #(
                            #session_types
                        )*
                        R,
                        #receiver,
                    >,
                    hyper::Response<hyper::Body>
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
                if ( resp_future.len() != 1 && http ) || ( !http && resp_future.len() != 0 ) {
                    panic!("Too many futures: {:?}", resp_future.len())
                }

                let resp = match http {
                    true => {
                        let rt = tokio::runtime::Runtime::new()?;
                        rt.block_on(async move {
                            resp_future.remove(0).await
                        })?
                    },
                    false => hyper::Response::default(),
                };

                #(
                    #all_recv
                )*

                let new_stack = s.stack.continuation();

                Ok((
                    v,
                    #meshedchannels_name {
                        #(
                            #new_sessions
                        )*
                        stack: new_stack,
                        name: s.name,
                    },
                    resp
                ))
            }
        }
    }
}
