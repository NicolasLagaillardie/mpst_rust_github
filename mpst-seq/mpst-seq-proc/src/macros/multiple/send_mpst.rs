use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct SendMPSTMacroInput {
    session: syn::Expr,
    receiver: syn::Ident,
    payload: syn::Expr,
    sender: syn::Ident,
    meshedchannels_name: syn::Ident,
    nsessions: u64,
    exclusion: u64,
}

impl Parse for SendMPSTMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let session = syn::Expr::parse(input)?;
        <Token![,]>::parse(input)?;

        let payload = syn::Expr::parse(input)?;
        <Token![,]>::parse(input)?;

        let receiver = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let sender = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let meshedchannels_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let nsessions = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();
        <Token![,]>::parse(input)?;

        let exclusion = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(SendMPSTMacroInput {
            session,
            receiver,
            payload,
            sender,
            meshedchannels_name,
            nsessions,
            exclusion,
        })
    }
}

impl From<SendMPSTMacroInput> for proc_macro2::TokenStream {
    fn from(input: SendMPSTMacroInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl SendMPSTMacroInput {
    fn expand(&self) -> proc_macro2::TokenStream {
        let receiver = self.receiver.clone();
        let sender = self.sender.clone();
        let meshedchannels_name = self.meshedchannels_name.clone();
        let session = self.session.clone();
        let payload = self.payload.clone();

        let all_send: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                if i != self.exclusion {
                    quote! {}
                } else {
                    let temp_ident =
                        syn::Ident::new(&format!("session{}", i), proc_macro2::Span::call_site());
                    quote! {
                        let new_session = mpstthree::binary::send::send(#payload, s.#temp_ident);
                    }
                }
            })
            .collect();

        let new_sessions: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
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
            {
                let s = #session;

                #(
                    #all_send
                )*

                let new_stack = {
                    fn temp<R>(r: #receiver<R>) -> R
                    where
                        R: mpstthree::role::Role,
                    {
                        let (here, there) = <R as mpstthree::role::Role>::new();
                        r.sender.send(there).unwrap_or(());
                        here
                    }
                    temp(s.stack)
                };

                {
                    fn temp(_s: &#sender<mpstthree::role::end::RoleEnd>) -> Result<(), Box<dyn std::error::Error>> {
                        Ok(())
                    }
                    temp(&s.name)
                }.unwrap();

                #meshedchannels_name {
                    #(
                        #new_sessions
                    )*
                    stack: new_stack,
                    name: s.name,
                }
            }
        }
    }
}
