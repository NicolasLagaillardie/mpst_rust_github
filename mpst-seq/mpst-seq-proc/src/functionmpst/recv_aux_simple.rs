use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct RecvAuxSimpleMacroInput {
    session: proc_macro2::TokenStream,
    role: syn::Ident,
    exclusion: syn::LitInt,
}

impl Parse for RecvAuxSimpleMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let role = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let exclusion = syn::LitInt::parse(input)?;
        <Token![,]>::parse(input)?;

        let content;
        let _parentheses = syn::parenthesized!(content in input);
        let session = proc_macro2::TokenStream::parse(&content)?;

        Ok(RecvAuxSimpleMacroInput {
            session,
            role,
            exclusion,
        })
    }
}

impl From<RecvAuxSimpleMacroInput> for proc_macro2::TokenStream {
    fn from(input: RecvAuxSimpleMacroInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl RecvAuxSimpleMacroInput {
    fn expand(&self) -> proc_macro2::TokenStream {
        let session = self.session.clone();
        let role = self.role.clone();
        let recv_session =
            format_ident!("session{}", (self.exclusion).base10_parse::<u64>().unwrap());

        let mut new_sessions = Vec::new();
        let mut all_sessions = Vec::new();
        for i in 1..3 {
            all_sessions.push(format_ident!("session{}", i));
            if i == (self.exclusion).base10_parse::<u64>().unwrap() {
                new_sessions.push(quote! { new_session });
            } else {
                let temp = format_ident!("session{}", i);
                new_sessions.push(quote! { #session.#temp });
            }
        }

        quote! {
            || -> Result<_, Box<dyn std::error::Error>> {

                let (v, new_session) = crate::binary::recv::recv( #session.#recv_session )?;

                let new_stack = {
                    fn temp<R>(r: #role<R>) -> R
                    where
                        R: crate::role::Role,
                    {
                        let (here, there) = <R as crate::role::Role>::new();
                        r.sender.send(there).unwrap_or(());
                        here
                    }
                    temp(#session.stack)
                };

                Ok((
                    v,
                    crate::sessionmpst::SessionMpst {
                        #( #all_sessions : #new_sessions , )*
                        stack: new_stack,
                        name: #session.name,
                    }
                ))
            }
        }
    }
}
