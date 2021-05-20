use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct RecvAllAuxSimpleMacroInput {
    session: proc_macro2::TokenStream,
    role: syn::Ident,
    exclusion: syn::LitInt,
}

impl Parse for RecvAllAuxSimpleMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let role = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let exclusion = syn::LitInt::parse(input)?;
        <Token![,]>::parse(input)?;

        let content_session;
        let _parentheses = syn::parenthesized!(content_session in input);
        let session = proc_macro2::TokenStream::parse(&content_session)?;

        Ok(RecvAllAuxSimpleMacroInput {
            session,
            role,
            exclusion,
        })
    }
}

impl From<RecvAllAuxSimpleMacroInput> for proc_macro2::TokenStream {
    fn from(input: RecvAllAuxSimpleMacroInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl RecvAllAuxSimpleMacroInput {
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

                let (new_stack_left, _new_stack_right) = { // new_stack_right = new_stack_left
                    fn temp(r: #role<crate::role::end::RoleEnd, crate::role::end::RoleEnd>)
                    -> (crate::role::end::RoleEnd, crate::role::end::RoleEnd)
                    {
                        let (here1, there1) = <crate::role::end::RoleEnd as crate::role::Role>::new();
                        let (here2, there2) = <crate::role::end::RoleEnd as crate::role::Role>::new();
                        r.sender1.send(there1).unwrap_or(());
                        r.sender2.send(there2).unwrap_or(());
                        (here1, here2)
                    }
                    temp(#session.stack)
                };

                Ok((
                    v,
                    crate::sessionmpst::SessionMpst {
                        #( #all_sessions : #new_sessions , )*
                        stack: new_stack_left,
                        name: #session.name,
                    }
                ))
            }
        }
    }
}
