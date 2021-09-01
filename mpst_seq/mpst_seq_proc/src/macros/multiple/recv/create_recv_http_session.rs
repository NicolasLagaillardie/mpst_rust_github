use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct CreateRecvHttpSessionMacroInput {
    func_name: syn::Ident,
    sender: syn::Ident,
    receiver: syn::Ident,
    meshedchannels_name: syn::Ident,
    nsessions: u64,
    exclusion: u64,
}

impl Parse for CreateRecvHttpSessionMacroInput {
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

        Ok(CreateRecvHttpSessionMacroInput {
            func_name,
            sender,
            receiver,
            meshedchannels_name,
            nsessions,
            exclusion,
        })
    }
}

impl From<CreateRecvHttpSessionMacroInput> for proc_macro2::TokenStream {
    fn from(input: CreateRecvHttpSessionMacroInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl CreateRecvHttpSessionMacroInput {
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
                    #sender<R>,
                    #receiver<mpstthree::role::end::RoleEnd>,
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
                        #receiver<mpstthree::role::end::RoleEnd>,
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
                    },
                    resp
                ))
            }
        }
    }
}
