//! Generates code for allowing a role, among three roles, to
//! send a payload to a receiver role.
//!
//! # Arguments
//!
//! * The current session representing the sender
//! * The payload to be sent
//! * The name of the receiver
//! * The index of the binary channel among the two of the receiver
//!
//! # Example
//!
//! ```ignore
//! // Let's assume that, among A, B and C, B is the receiver,
//! // A is the sender and x is the payload.
//! // Then the binary channel of A with B is its first
//! // channel.
//! mpst_seq::send_aux_simple!(s, x, RoleB, 1)()
//! ```

use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct SendAuxSimple {
    session: syn::Expr,
    payload: syn::Expr,
    role: syn::Ident,
    exclusion: u64,
}

impl Parse for SendAuxSimple {
    fn parse(input: ParseStream) -> Result<Self> {
        let session = syn::Expr::parse(input)?;
        <Token![,]>::parse(input)?;

        let payload = syn::Expr::parse(input)?;
        <Token![,]>::parse(input)?;

        let role = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let exclusion = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(SendAuxSimple {
            session,
            payload,
            role,
            exclusion,
        })
    }
}

impl From<SendAuxSimple> for proc_macro2::TokenStream {
    fn from(input: SendAuxSimple) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl SendAuxSimple {
    fn expand(&self) -> proc_macro2::TokenStream {
        let session = self.session.clone();
        let payload = self.payload.clone();
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
            {
                let new_session = crate::binary::send::send(#payload,  #session.#recv_session );

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

                crate::meshedchannels::MeshedChannels {
                    #( #all_sessions : #new_sessions , )*
                    stack: new_stack,
                    name: #session.name,
                }
            }
        }
    }
}
