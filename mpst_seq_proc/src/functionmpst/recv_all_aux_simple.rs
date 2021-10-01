//! Generates code for allowing a role, among three roles, to receive
//! their new MeshedChannels from an active role in a binary choice.
//!
//! # Arguments
//!
//! * The current session representing the passive role
//! * The index of the binary channel among the two of the passive role
//!
//! # Example
//!
//! ```ignore
//! // Among A, B and C, if B is the active role
//! // then the head of the stack of B is
//! // *RoleBtoAll* and its dual is *RoleAlltoB*.
//! // If A is the current receiving role,
//! // then its binary channel with B is the first
//! // one.
//! mpst_seq::recv_all_aux_simple!(s, 1)()
//! ```

use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct RecvAllAuxSimple {
    session: syn::Expr,
    exclusion: u64,
}

impl Parse for RecvAllAuxSimple {
    fn parse(input: ParseStream) -> Result<Self> {
        let session = syn::Expr::parse(input)?;
        <Token![,]>::parse(input)?;

        let exclusion = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap(); // Retrive the index

        Ok(RecvAllAuxSimple { session, exclusion })
    }
}

impl From<RecvAllAuxSimple> for proc_macro2::TokenStream {
    fn from(input: RecvAllAuxSimple) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl RecvAllAuxSimple {
    fn expand(&self) -> proc_macro2::TokenStream {
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

                let new_stack_left = #session.stack.continuation_left();

                Ok((
                    v,
                    crate::meshedchannels::MeshedChannels {
                        #( #all_sessions : #new_sessions , )*
                        stack: new_stack_left,
                        name: #session.name,
                    }
                ))
            }
        }
    }
}
