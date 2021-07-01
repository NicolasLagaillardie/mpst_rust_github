use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct SendAuxSimpleMacroInput {
    session: syn::Expr,
    payload: syn::Expr,
    role: syn::Ident,
    exclusion: u64,
}

impl Parse for SendAuxSimpleMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let role = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let exclusion = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();
        <Token![,]>::parse(input)?;

        let session = syn::Expr::parse(input)?;
        <Token![,]>::parse(input)?;

        let payload = syn::Expr::parse(input)?;

        Ok(SendAuxSimpleMacroInput {
            session,
            payload,
            role,
            exclusion,
        })
    }
}

impl From<SendAuxSimpleMacroInput> for proc_macro2::TokenStream {
    fn from(input: SendAuxSimpleMacroInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl SendAuxSimpleMacroInput {
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

                crate::sessionmpst::SessionMpst {
                    #( #all_sessions : #new_sessions , )*
                    stack: new_stack,
                    name: #session.name,
                }
            }
        }
    }
}
