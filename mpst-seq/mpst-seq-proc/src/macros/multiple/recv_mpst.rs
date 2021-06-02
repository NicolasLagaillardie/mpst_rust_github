use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct RecvMPSTMacroInput {
    sessions: Vec<proc_macro2::TokenStream>,
    sender: syn::Ident,
    receiver: syn::Ident,
    sessionmpst_name: syn::Ident,
    nsessions: u64,
    exclusion: u64,
}

impl Parse for RecvMPSTMacroInput {
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

        let sender = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let receiver = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let sessionmpst_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let nsessions = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();
        <Token![,]>::parse(input)?;

        let exclusion = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(RecvMPSTMacroInput {
            sessions: all_sessions,
            sender,
            receiver,
            sessionmpst_name,
            nsessions,
            exclusion,
        })
    }
}

impl From<RecvMPSTMacroInput> for proc_macro2::TokenStream {
    fn from(input: RecvMPSTMacroInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl RecvMPSTMacroInput {
    fn expand(&self) -> proc_macro2::TokenStream {
        let sender = self.sender.clone();
        let receiver = self.receiver.clone();
        let sessionmpst_name = self.sessionmpst_name.clone();
        let session = if let Some(elt) = self.sessions.get(usize::try_from(0).unwrap()) {
            elt
        } else {
            panic!("Not enough sessions provided")
        };

        let all_recv: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                if i != self.exclusion {
                    quote! {}
                } else {
                    let temp_ident =
                        syn::Ident::new(&format!("session{}", i), proc_macro2::Span::call_site());
                    quote! {
                        let (v, new_session) = mpstthree::binary::recv::recv(s.#temp_ident)?;
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
            || -> Result<_, Box<dyn std::error::Error>> {
                let s = #session;
                
                #(
                    #all_recv
                )*

                let new_stack = {
                    fn temp<R>(r: #sender<R>) -> R
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
                    fn temp(_s: &#receiver<mpstthree::role::end::RoleEnd>) -> Result<(), Box<dyn std::error::Error>> {
                        Ok(())
                    }
                    temp(&s.name)
                }.unwrap();

                Ok((
                    v,
                    #sessionmpst_name {
                        #(
                            #new_sessions
                        )*
                        stack: new_stack,
                        name: s.name,
                    }
                ))
            }
        }
    }
}
