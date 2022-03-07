//! Generates code for allowing a role, among three roles, to
//! send a payload to a receiver role.
//!
//! # Arguments
//!
//! * The current session representing the sender
//! * The payload to be sent
//! * The index of the binary channel among the two of the receiver
//!
//! # Example
//!
//! ```ignore
//! // Let's assume that, among A, B and C, B is the receiver,
//! // A is the sender and x is the payload.
//! // Then the binary channel of A with B is its first
//! // channel.
//! mpst_seq::send_aux_simple!(s, x, 1)()
//! ```

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, LitInt, Result, Token};

#[derive(Debug)]
pub(crate) struct SendAuxSimple {
    session: Expr,
    payload: Expr,
    exclusion: u64,
}

impl Parse for SendAuxSimple {
    fn parse(input: ParseStream) -> Result<Self> {
        let session = Expr::parse(input)?;
        <Token![,]>::parse(input)?;

        let payload = Expr::parse(input)?;
        <Token![,]>::parse(input)?;

        let exclusion = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(SendAuxSimple {
            session,
            payload,
            exclusion,
        })
    }
}

impl From<SendAuxSimple> for TokenStream {
    fn from(input: SendAuxSimple) -> TokenStream {
        input.expand()
    }
}

impl SendAuxSimple {
    fn expand(&self) -> TokenStream {
        let session = self.session.clone();
        let payload = self.payload.clone();
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

                let new_stack = #session.stack.continuation();

                crate::meshedchannels::MeshedChannels {
                    #( #all_sessions : #new_sessions , )*
                    stack: new_stack,
                    name: #session.name,
                }
            }
        }
    }
}
