use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, LitInt, Result, Token};

use crate::common_functions::expand::parenthesised::parenthesised;
use crate::common_functions::maths::{diag, get_tuple_diag};

#[derive(Debug)]
pub(crate) struct ChooseTimedMultiToAll {
    session: Expr,
    labels: Vec<TokenStream>,
    sender: Ident,
    meshedchannels_name: Ident,
    n_sessions: u64,
    exclusion: u64,
}

impl Parse for ChooseTimedMultiToAll {
    fn parse(input: ParseStream) -> Result<Self> {
        // The session
        let session = Expr::parse(input)?;
        <Token![,]>::parse(input)?;

        // The labels
        let content_labels;
        let _parentheses = syn::parenthesized!(content_labels in input);
        let labels = TokenStream::parse(&content_labels)?;

        let all_labels: Vec<TokenStream> = parenthesised(labels);

        <Token![,]>::parse(input)?;

        // The receivers
        let content_receivers;
        let _parentheses = syn::parenthesized!(content_receivers in input);
        let receivers = TokenStream::parse(&content_receivers)?;

        let all_receivers: Vec<TokenStream> = parenthesised(receivers);

        assert_eq!(
            all_receivers.len(),
            all_labels.len(),
            "We are comparing number of receivers and labels in choose_mpst_multi_to_all"
        );

        <Token![,]>::parse(input)?;

        // The sender
        let sender = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        // The meshedchannels_name
        let meshedchannels_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        // The index of the sender
        let exclusion = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        // The number of receivers
        let n_sessions = u64::try_from(all_receivers.len()).unwrap() + 1;

        Ok(ChooseTimedMultiToAll {
            session,
            labels: all_labels,
            sender,
            meshedchannels_name,
            n_sessions,
            exclusion,
        })
    }
}

impl From<ChooseTimedMultiToAll> for TokenStream {
    fn from(input: ChooseTimedMultiToAll) -> TokenStream {
        input.expand()
    }
}

impl ChooseTimedMultiToAll {
    fn expand(&self) -> TokenStream {
        let session = &self.session;
        let sender = &self.sender;
        let meshedchannels_name = &self.meshedchannels_name;
        let diff = self.n_sessions - 1;
        let diag = diag(self.n_sessions);

        let new_channels: Vec<TokenStream> = (1..=(diff * (diff + 1) / 2))
            .map(|i| {
                let (line, column, _) = get_tuple_diag(&diag, i);
                let channel_left =
                    Ident::new(&format!("channel_{}_{}", line, column), Span::call_site());
                let channel_right =
                    Ident::new(&format!("channel_{}_{}", column, line), Span::call_site());
                quote! {
                    let ( #channel_left , #channel_right ) =
                        <_ as mpstthree::binary::struct_trait::session::Session>::new();
                }
            })
            .collect();

        let new_roles: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("stack_{}", i), Span::call_site());
                quote! {
                    let ( #temp_ident , _) = <_ as mpstthree::role::Role>::new();
                }
            })
            .collect();

        let new_names: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let temp_name = Ident::new(&format!("name_{}", i), Span::call_site());
                quote! {
                    let ( #temp_name , _) = < _ as mpstthree::name::Name >::new();
                }
            })
            .collect();

        let new_name_sender = Ident::new(&format!("name_{}", self.n_sessions), Span::call_site());

        let new_stack_sender = Ident::new(&format!("stack_{}", self.n_sessions), Span::call_site());

        let all_send: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let new_sessions: Vec<TokenStream> = (1..self.n_sessions)
                    .map(|j| {
                        let temp = if i >= self.exclusion { i + 1 } else { i };

                        let temp_ident = Ident::new(&format!("session{}", j), Span::call_site());

                        let temp_channel = if j < temp {
                            Ident::new(&format!("channel_{}_{}", temp, j), Span::call_site())
                        } else {
                            Ident::new(&format!("channel_{}_{}", temp, j + 1), Span::call_site())
                        };

                        quote! {
                            #temp_ident : #temp_channel ,
                        }
                    })
                    .collect();

                let temp_name = Ident::new(&format!("name_{}", i), Span::call_site());

                let temp_stack = Ident::new(&format!("stack_{}", i), Span::call_site());

                let temp_session = Ident::new(&format!("session{}", i), Span::call_site());

                let temp_label = if let Some(elt) = self.labels.get(usize::try_from(i - 1).unwrap())
                {
                    elt
                } else {
                    panic!("Not enough labels")
                };

                quote! {
                    let _ = mpstthree::binary_timed::send::send(
                        #temp_label(
                            #meshedchannels_name {
                                #(
                                    #new_sessions
                                )*
                                stack: #temp_stack ,
                                name: #temp_name ,
                            }
                        ),
                        $all_clocks ,
                        s.#temp_session ,
                    );
                }
            })
            .collect();

        let new_meshedchannels: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let temp_session = Ident::new(&format!("session{}", i), Span::call_site());
                let temp_channel = if i < self.exclusion {
                    Ident::new(
                        &format!("channel_{}_{}", self.exclusion, i),
                        Span::call_site(),
                    )
                } else {
                    Ident::new(
                        &format!("channel_{}_{}", self.exclusion, i + 1),
                        Span::call_site(),
                    )
                };
                quote! {
                    #temp_session : #temp_channel ,
                }
            })
            .collect();

        quote! {
            {
                #(
                    #new_channels
                )*

                #(
                    #new_roles
                )*

                #(
                    #new_names
                )*

                let ( #new_name_sender , _) = < #sender as mpstthree::name::Name >::new();

                let s = #session;

                let _ = {
                    fn temp(r: &mpstthree::role::broadcast::RoleBroadcast)
                        -> Result<(), Box<dyn std::error::Error>>
                    {
                        Ok(())
                    }
                    temp(&s.stack)
                };

                #(
                    #all_send
                )*

                #meshedchannels_name {
                    #(
                        #new_meshedchannels
                    )*
                    stack: #new_stack_sender ,
                    name: #new_name_sender ,
                }
            }
        }
    }
}
