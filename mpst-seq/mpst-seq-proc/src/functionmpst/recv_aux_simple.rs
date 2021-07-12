//! Generates code for allowing a role, among three roles, to
//! receive a payload from an active role.
//!
//! # Arguments
//!
//! * The current session representing the receiver
//! * The name of the sender
//! * The index of the binary channel among the two of the receiver
//!
//! # Example
//!
//! ```ignore
//! // Let's assume that, among A, B and C, B is the sender
//! // and A is the receiver.
//! // Then the binary channel of A with B is the first
//! // one.
//! mpst_seq::recv_aux_simple!(s, RoleB, 1)()
//! ```

use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct RecvAuxSimpleMacroInput {
    session: syn::Expr,
    role: syn::Ident,
    exclusion: u64,
}

impl Parse for RecvAuxSimpleMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let session = syn::Expr::parse(input)?; // Retrive the session
        <Token![,]>::parse(input)?;

        let role = syn::Ident::parse(input)?; // Retrive the role
        <Token![,]>::parse(input)?;

        let exclusion = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();

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
        let recv_session = format_ident!("session{}", self.exclusion);

        let mut new_sessions = Vec::new();
        let mut all_sessions = Vec::new();
        for i in 1..3 {
            all_sessions.push(format_ident!("session{}", i));
            if i == self.exclusion {
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
                    crate::meshedchannels::MeshedChannels {
                        #( #all_sessions : #new_sessions , )*
                        stack: new_stack,
                        name: #session.name,
                    }
                ))
            }
        }
    }
}
