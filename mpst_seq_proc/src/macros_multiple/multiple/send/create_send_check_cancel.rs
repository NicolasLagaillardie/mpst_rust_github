use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct CreateSendCheckCancel {
    func_name: syn::Ident,
    receiver: syn::Ident,
    sender: syn::Ident,
    meshedchannels_name: syn::Ident,
    n_sessions: u64,
    exclusion: u64,
}

impl Parse for CreateSendCheckCancel {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let receiver = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let sender = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let meshedchannels_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let n_sessions = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();
        <Token![,]>::parse(input)?;

        let exclusion = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(CreateSendCheckCancel {
            func_name,
            receiver,
            sender,
            meshedchannels_name,
            n_sessions,
            exclusion,
        })
    }
}

impl From<CreateSendCheckCancel> for proc_macro2::TokenStream {
    fn from(input: CreateSendCheckCancel) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl CreateSendCheckCancel {
    fn expand(&self) -> proc_macro2::TokenStream {
        let func_name = self.func_name.clone();
        let receiver = self.receiver.clone();
        let sender = self.sender.clone();
        let meshedchannels_name = self.meshedchannels_name.clone();

        let session_types: Vec<proc_macro2::TokenStream> = (2..self.n_sessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let session_types_struct: Vec<proc_macro2::TokenStream> = (2..self.n_sessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident : mpstthree::binary::struct_trait::session::Session ,
                }
            })
            .collect();

        let all_send: Vec<proc_macro2::TokenStream> = (1..self.n_sessions)
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

        let send_types: Vec<proc_macro2::TokenStream> = (2..self.n_sessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
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

        let new_sessions: Vec<proc_macro2::TokenStream> = (1..self.n_sessions)
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
                    mpstthree::binary::struct_trait::end::End,
                    #(
                        #send_types
                    )*
                    #receiver<R>,
                    #sender<mpstthree::role::end::RoleEnd>,
                >,
            ) -> Result<
                    #meshedchannels_name<
                        mpstthree::binary::struct_trait::end::End,
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
                match s.session1.receiver.try_recv() {
                    Ok(mpstthree::binary::struct_trait::end::Signal::Cancel) => {
                        mpstthree::binary::cancel::cancel(s);
                        panic!("Error")
                    },
                    _ => {}
                };

                #(
                    #all_send
                )*

                let new_stack = s.stack.continuation();

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
