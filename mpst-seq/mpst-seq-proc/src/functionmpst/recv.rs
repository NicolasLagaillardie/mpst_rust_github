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

impl Into<proc_macro2::TokenStream> for RecvAuxSimpleMacroInput {
    fn into(self) -> proc_macro2::TokenStream {
        self.expand()
    }
}

impl RecvAuxSimpleMacroInput {
    fn expand(&self) -> proc_macro2::TokenStream {
        let session = self.session.clone();
        let role = format_ident!("{}", self.role);

        let mut recv_vec = Vec::new();
        for i in 1..3 {
            if i == (self.exclusion).base10_parse::<u64>().unwrap() {
                recv_vec.push(format_ident!("session{}", i));
            }
        }

        let mut new_sessions = Vec::new();
        let mut all_sessions = Vec::new();
        for i in 1..3 {
            if i == (self.exclusion).base10_parse::<u64>().unwrap() {
                all_sessions.push(format_ident!("session{}", i));
                new_sessions.push(format_ident!("new_session"));
            } else {
                all_sessions.push(format_ident!("session{}", i));
                new_sessions.push(format_ident!("temp_session.session{}", i));
            }
        }

        quote! {
            || -> Result<_, Box<dyn std::error::Error>> {

                let temp_session = #session;

                let (v, new_session) = crate::binary::recv::recv( temp_session.#( #recv_vec )* )?;

                let new_stack = {
                    fn temp<R>(r: #role<R>) -> R
                    where
                        R: crate::role::Role,
                    {
                        let (here, there) = <R as crate::role::Role>::new();
                        r.sender.send(there).unwrap_or(());
                        here
                    }
                    temp(temp_session.stack)
                };

                Ok((
                    v,
                    crate::sessionmpst::SessionMpst {
                        #( #all_sessions : #new_sessions , )*
                        stack: new_stack,
                        name: temp_session.name,
                    }
                ))
            }
        }
    }
}
