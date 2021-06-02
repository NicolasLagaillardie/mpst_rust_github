use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct SendMPSTMacroInput {
    sessions: Vec<proc_macro2::TokenStream>,
    receiver: syn::Ident,
    payloads: Vec<proc_macro2::TokenStream>,
    sender: syn::Ident,
    sessionmpst_name: syn::Ident,
    nsessions: u64,
    exclusion: u64,
}

impl Parse for SendMPSTMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let content_sessions;
        let _parentheses = syn::parenthesized!(content_sessions in input);
        let sessions = proc_macro2::TokenStream::parse(&content_sessions)?;

        /////////////////////////
        let mut all_sessions: Vec<proc_macro2::TokenStream> = Vec::new();
        for tt in sessions.clone().into_iter() {
            let elt = match tt {
                proc_macro2::TokenTree::Group(g) => Some(g.stream()),
                _ => None,
            };
            if let Some(elt_tt) = elt {
                all_sessions.push(elt_tt)
            }
        }
        <Token![,]>::parse(input)?;

        let content_payload;
        let _parentheses = syn::parenthesized!(content_payload in input);
        let payloads = proc_macro2::TokenStream::parse(&content_payload)?;

        /////////////////////////
        let mut all_payloads: Vec<proc_macro2::TokenStream> = Vec::new();
        for tt in payloads.clone().into_iter() {
            let elt = match tt {
                proc_macro2::TokenTree::Group(g) => Some(g.stream()),
                _ => None,
            };
            if let Some(elt_tt) = elt {
                all_payloads.push(elt_tt)
            }
        }
        <Token![,]>::parse(input)?;

        let receiver = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let sender = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let sessionmpst_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let nsessions = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();
        <Token![,]>::parse(input)?;

        let exclusion = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(SendMPSTMacroInput {
            sessions: all_sessions,
            receiver,
            payloads: all_payloads,
            sender,
            sessionmpst_name,
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
        let sessionmpst_name = self.sessionmpst_name.clone();
        let session = if let Some(elt) = self.sessions.get(usize::try_from(0).unwrap()) {
            elt
        } else {
            panic!("Not enough sessions provided")
        };
        let payload = if let Some(elt) = self.payloads.get(usize::try_from(0).unwrap()) {
            elt
        } else {
            panic!("Not enough sessions provided")
        };

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

                #sessionmpst_name {
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
