//! Generates code for allowing a role, among three roles, to
//! receive a payload from an active role.
//!
//! # Arguments
//!
//! * The current session representing the receiver
//! * The index of the binary channel among the two of the receiver
//!
//! # Example
//!
//! ```ignore
//! // Let's assume that, among A, B and C, B is the sender
//! // and A is the receiver.
//! // Then the binary channel of A with B is the first
//! // one.
//! mpst_seq::recv_aux_simple!(s, 1)()
//! ```

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, LitInt, Result, Token};

#[derive(Debug)]
pub(crate) struct RecvAuxSimple {
    session: Expr,
    exclusion: u64,
}

impl Parse for RecvAuxSimple {
    fn parse(input: ParseStream) -> Result<Self> {
        let session = Expr::parse(input)?; // Retrive the session
        <Token![,]>::parse(input)?;

        let exclusion = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(RecvAuxSimple { session, exclusion })
    }
}

impl From<RecvAuxSimple> for TokenStream {
    fn from(input: RecvAuxSimple) -> TokenStream {
        input.expand()
    }
}

impl RecvAuxSimple {
    fn expand(&self) -> TokenStream {
        let session = self.session.clone();
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

                let new_stack = #session.stack.continuation();

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
