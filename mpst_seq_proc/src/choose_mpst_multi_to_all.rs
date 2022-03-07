use proc_macro2::{Span, TokenStream, TokenTree};
use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, LitInt, Result, Token};

type VecOfTuple = Vec<(u64, u64, u64)>;

#[derive(Debug)]
pub(crate) struct  ChooseMultiToAll {
    session: Expr,
    labels: Vec<TokenStream>,
    receivers: Vec<TokenStream>,
    sender: Ident,
    meshedchannels_name: Ident,
    n_sessions: u64,
    exclusion: u64,
}

fn expand_parenthesized(stream: &TokenStream) -> Vec<TokenStream> {
    let mut out: Vec<TokenStream> = Vec::new();
    for tt in stream.clone().into_iter() {
        let elt = match tt {
            TokenTree::Group(g) => Some(g.stream()),
            _ => None,
        };
        if let Some(elt_tt) = elt {
            out.push(elt_tt)
        }
    }
    out
}

impl Parse for ChooseMultiToAll {
    fn parse(input: ParseStream) -> Result<Self> {
        // The session
        let session = Expr::parse(input)?;
        <Token![,]>::parse(input)?;

        // The labels
        let content_labels;
        let _parentheses = syn::parenthesized!(content_labels in input);
        let labels = TokenStream::parse(&content_labels)?;

        let all_labels: Vec<TokenStream> = expand_parenthesized(&labels);

        <Token![,]>::parse(input)?;

        // The receivers
        let content_receivers;
        let _parentheses = syn::parenthesized!(content_receivers in input);
        let receivers = TokenStream::parse(&content_receivers)?;

        let all_receivers: Vec<TokenStream> = expand_parenthesized(&receivers);

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

        assert_eq!(
            all_receivers.len(),
            all_labels.len(),
            "We are comparing number of receivers and labels in choose_mpst_multi_to_all"
        );

        Ok(ChooseMultiToAll {
            session,
            labels: all_labels,
            receivers: all_receivers,
            sender,
            meshedchannels_name,
            n_sessions,
            exclusion,
        })
    }
}

impl From<ChooseMultiToAll> for TokenStream {
    fn from(input: ChooseMultiToAll) -> TokenStream {
        input.expand()
    }
}

impl ChooseMultiToAll {
    /// Create the whole matrix of index according to line and column
    fn diag(&self) -> VecOfTuple {
        let diff = self.n_sessions - 1;

        let mut column = 0;
        let mut line = 0;

        // Create the upper diag
        (0..(diff * (diff + 1) / 2))
            .map(|i| {
                if line == column {
                    column += 1;
                } else if column >= (self.n_sessions - 1) {
                    line += 1;
                    column = line + 1;
                } else {
                    column += 1;
                }
                (line + 1, column + 1, i + 1)
            })
            .collect()
    }

    /// Return (line, column, index) of diag
    fn get_tuple_diag(&self, diag: &[(u64, u64, u64)], i: u64) -> (u64, u64, u64) {
        if let Some((line, column, index)) = diag.get(usize::try_from(i - 1).unwrap()) {
            (*line, *column, *index)
        } else {
            panic!(
                "Error at get_tuple_diag for i = {:?} / diag = {:?}",
                i, diag
            )
        }
    }

    fn expand(&self) -> TokenStream {
        let session = self.session.clone();
        let all_labels = self.labels.clone();
        let all_receivers = self.receivers.clone();
        let sender = self.sender.clone();
        let meshedchannels_name = self.meshedchannels_name.clone();
        let diff = self.n_sessions - 1;
        let diag = self.diag();

        let new_channels: Vec<TokenStream> = (1..=(diff * (diff + 1) / 2))
            .map(|i| {
                let (line, column, _) = self.get_tuple_diag(&diag, i);
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
                let temp_name =
                    Ident::new(&format!("name_{}", i), Span::call_site());
                let temp_role = if let Some(elt) = all_receivers.get(usize::try_from(i - 1).unwrap()) {
                    elt
                } else {
                    panic!("Not enough receivers")
                };
                quote! {
                    let ( #temp_name , _) = #temp_role::new();
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

                let temp_label = if let Some(elt) = all_labels.get(usize::try_from(i - 1).unwrap())
                {
                    elt
                } else {
                    panic!("Not enough labels")
                };

                quote! {
                    let _ = mpstthree::binary::send::send(
                        #temp_label(
                            #meshedchannels_name {
                                #(
                                    #new_sessions
                                )*
                                stack: #temp_stack ,
                                name: #temp_name ,
                            }
                        ),
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

                let ( #new_name_sender , _) = #sender::new();

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
