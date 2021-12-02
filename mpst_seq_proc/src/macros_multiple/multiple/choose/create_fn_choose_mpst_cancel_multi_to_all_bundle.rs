use proc_macro2::{Span, TokenStream, TokenTree};
use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Token};

type VecOfTuple = Vec<(u64, u64, u64)>;

#[derive(Debug)]
pub struct ChooseTypeCancelMultiToAllBundle {
    labels: Vec<TokenStream>,
    receivers: Vec<TokenStream>,
    fn_names: Vec<TokenStream>,
    branches: Vec<TokenStream>,
    new_types: Vec<TokenStream>,
    sender: Ident,
    broadcaster: Ident,
    meshedchannels_name: Ident,
    n_sessions: u64,
    n_branches: u64,
    n_labels: u64,
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

impl Parse for ChooseTypeCancelMultiToAllBundle {
    fn parse(input: ParseStream) -> Result<Self> {
        // The names of the functions
        let content_fn_names;
        let _parentheses = syn::parenthesized!(content_fn_names in input);
        let fn_names = TokenStream::parse(&content_fn_names)?;

        let all_fn_names: Vec<TokenStream> = expand_parenthesized(&fn_names);
        <Token![,]>::parse(input)?;

        // The names of the functions
        let content_branches;
        let _parentheses = syn::parenthesized!(content_branches in input);
        let branches = TokenStream::parse(&content_branches)?;

        let all_branches: Vec<TokenStream> = expand_parenthesized(&branches);
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

        // The new_types
        let content_new_type;
        let _parentheses = syn::parenthesized!(content_new_type in input);
        let new_types = TokenStream::parse(&content_new_type)?;

        let all_new_types: Vec<TokenStream> = expand_parenthesized(&new_types);
        <Token![,]>::parse(input)?;

        // The sender
        let sender = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        // The broadcaster
        let broadcaster = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        // The meshedchannels_name
        let meshedchannels_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        // The index of the sender
        let exclusion = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        // The number of receivers
        let n_sessions = u64::try_from(all_receivers.len()).unwrap() + 2;

        // The number of labels
        let n_branches = u64::try_from(all_branches.len()).unwrap() + 1;

        // The number of functions
        let n_fn_names = u64::try_from(all_fn_names.len()).unwrap() + 1;

        // The number of functions
        let n_new_type = u64::try_from(all_new_types.len()).unwrap() + 1;

        // The number of functions
        let n_labels = u64::try_from(all_labels.len()).unwrap() + 2;

        if n_branches != n_fn_names || n_branches != n_new_type || n_new_type != n_fn_names {
            panic!("The number of new types, functions and branches are not the same")
        };

        Ok(ChooseTypeCancelMultiToAllBundle {
            labels: all_labels,
            receivers: all_receivers,
            fn_names: all_fn_names,
            branches: all_branches,
            new_types: all_new_types,
            sender,
            broadcaster,
            meshedchannels_name,
            n_sessions,
            n_branches,
            n_labels,
            exclusion,
        })
    }
}

impl From<ChooseTypeCancelMultiToAllBundle> for TokenStream {
    fn from(input: ChooseTypeCancelMultiToAllBundle) -> TokenStream {
        input.expand()
    }
}

impl ChooseTypeCancelMultiToAllBundle {
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
        let all_functions: Vec<TokenStream> = (1..self.n_branches)
            .map(|i| {
                let all_labels = self.labels.clone();
                let all_receivers = self.receivers.clone();
                let all_branches = self.branches.clone();
                let all_fn_names = self.fn_names.clone();
                let sender = self.sender.clone();
                let broadcaster = self.broadcaster.clone();
                let all_new_types = self.new_types.clone();
                let meshedchannels_name = self.meshedchannels_name.clone();
                let diff = self.n_sessions - 1;
                let diag = self.diag();

                let send_types: Vec<TokenStream> = (2..self.n_labels)
                    .map(|j| {
                        let temp_label =
                            if let Some(elt) = all_labels.get(usize::try_from(j - 2).unwrap()) {
                                elt
                            } else {
                                panic!("Not enough labels for send_types")
                            };

                        quote! {
                            Send<
                                (
                                    mpstthree::binary::struct_trait::end::End,
                                    #temp_label
                                ),
                                mpstthree::binary::struct_trait::end::End
                            > ,
                        }
                    })
                    .collect();

                let new_channels: Vec<TokenStream> = (1..=(diff * (diff + 1) / 2))
                    .map(|j| {
                        let (line, column, _) = self.get_tuple_diag(&diag, j);
                        let channel_left = Ident::new(
                            &format!("channel_{}_{}", line, column),
                            Span::call_site(),
                        );
                        let channel_right = Ident::new(
                            &format!("channel_{}_{}", column, line),
                            Span::call_site(),
                        );
                        if j < self.n_sessions {
                            quote! {
                                let ( #channel_left , #channel_right ) =
                                    <mpstthree::binary::struct_trait::end::End
                                        as mpstthree::binary::struct_trait::session::Session>::new();
                                temp.push( #channel_left );
                            }
                        } else {
                            quote! {
                                let ( #channel_left , #channel_right ) =
                                    <_ as mpstthree::binary::struct_trait::session::Session>::new();
                            }
                        }
                    })
                    .collect();
                let new_roles: Vec<TokenStream> = (2..=self.n_sessions)
                    .map(|j| {
                        let temp_ident =
                            Ident::new(&format!("stack_{}", j), Span::call_site());
                        quote! {
                            let ( #temp_ident , _) = <_ as mpstthree::role::Role>::new();
                        }
                    })
                    .collect();
                let new_names: Vec<TokenStream> = (2..self.n_sessions)
                    .map(|j| {
                        let temp_name =
                            Ident::new(&format!("name_{}", j), Span::call_site());
                        let temp_role = if let Some(elt) = all_receivers.get(usize::try_from(j - 2).unwrap()) {
                            elt
                        } else {
                            panic!("Not enough receivers for new_names")
                        };
                        quote! {
                            let ( #temp_name , _) =
                                <#temp_role::<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
                        }
                    })
                    .collect();
                let new_name_sender = Ident::new(
                    &format!("name_{}", self.n_sessions),
                    Span::call_site(),
                );
                let new_stack_sender = Ident::new(
                    &format!("stack_{}", self.n_sessions),
                    Span::call_site(),
                );

                let temp_fn_name =
                    if let Some(elt) = all_fn_names.get(usize::try_from(i - 1).unwrap()) {
                        elt
                    } else {
                        panic!("Not enough fn_names for all_functions")
                    };

                let temp_new_type =
                    if let Some(elt) = all_new_types.get(usize::try_from(i - 1).unwrap()) {
                        elt
                    } else {
                        panic!("Not enough new_type for all_functions")
                    };

                let temp_branches =
                    if let Some(elt) = all_branches.get(usize::try_from(i - 1).unwrap()) {
                        elt
                    } else {
                        panic!("Not enough branches for all_functions")
                    };
                let all_send: Vec<TokenStream> = (2..self.n_sessions)
                    .map(|j| {
                        let new_sessions: Vec<TokenStream> = (1..self.n_sessions)
                            .map(|k| {
                                let temp = if j >= self.exclusion { j + 1 } else { j };
                                let temp_ident = Ident::new(
                                    &format!("session{}", k),
                                    Span::call_site(),
                                );
                                let temp_channel = if k < temp {
                                    Ident::new(
                                        &format!("channel_{}_{}", temp, k),
                                        Span::call_site(),
                                    )
                                } else {
                                    Ident::new(
                                        &format!("channel_{}_{}", temp, k + 1),
                                        Span::call_site(),
                                    )
                                };
                                quote! {
                                    #temp_ident : #temp_channel ,
                                }
                            })
                            .collect();
                        let temp_name =
                            Ident::new(&format!("name_{}", j), Span::call_site());
                        let temp_stack =
                            Ident::new(&format!("stack_{}", j), Span::call_site());
                        let temp_session =
                            Ident::new(&format!("session{}", j), Span::call_site());
                        let temp_label = if let Some(elt) = all_labels.get(usize::try_from(j - 2).unwrap())
                        {
                            elt
                        } else {
                            panic!("Not enough labels for all_send")
                        };
                        quote! {
                            let elt = match temp.pop() {
                                Some(e) => e,
                                _ => panic!("Error type"),
                            };
                            let _  = mpstthree::binary::send::send_canceled(
                                (
                                    elt,
                                    #temp_label::#temp_branches(
                                        #meshedchannels_name {
                                            #(
                                                #new_sessions
                                            )*
                                            stack: #temp_stack ,
                                            name: #temp_name ,
                                        }
                                    )
                                ),
                                s.#temp_session,
                            )?;
                        }
                    })
                    .collect();
                let new_meshedchannels: Vec<TokenStream> = (1..self.n_sessions)
                    .map(|j| {
                        let temp_session =
                            Ident::new(&format!("session{}", j), Span::call_site());
                        let temp_channel = if j < self.exclusion {
                            Ident::new(
                                &format!("channel_{}_{}", self.exclusion, j),
                                Span::call_site(),
                            )
                        } else {
                            Ident::new(
                                &format!("channel_{}_{}", self.exclusion, j + 1),
                                Span::call_site(),
                            )
                        };
                        quote! {
                            #temp_session : #temp_channel ,
                        }
                    })
                    .collect();

                quote! {
                    fn #temp_fn_name(
                        s: #meshedchannels_name<
                            mpstthree::binary::struct_trait::end::End,
                            #(
                                #send_types
                            )*
                            mpstthree::role::broadcast::RoleBroadcast,
                            #sender<mpstthree::role::end::RoleEnd>,
                        >
                    ) -> Result< #temp_new_type , Box<dyn std::error::Error>>
                    {
                        let mut temp = Vec::<mpstthree::binary::struct_trait::end::End>::new();

                        #(
                            #new_channels
                        )*

                        let (stack_1, _) =
                            <mpstthree::binary::struct_trait::end::End
                                as mpstthree::binary::struct_trait::session::Session>::new();

                        #(
                            #new_roles
                        )*

                        let (name_1, _) =
                            <#broadcaster<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

                        #(
                            #new_names
                        )*

                        let ( #new_name_sender , _) =
                            <#sender<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

                        #(
                            #all_send
                        )*

                        let elt = match temp.pop() {
                            Some(e) => e,
                            _ => panic!("Error type"),
                        };
                        let s =
                            s.session1.sender.send(
                                mpstthree::binary::struct_trait::end::Signal::Offer(elt)
                            ).unwrap();

                        Ok(
                            #meshedchannels_name {
                                #(
                                    #new_meshedchannels
                                )*
                                stack: #new_stack_sender ,
                                name: #new_name_sender ,
                            }
                        )
                    }
                }
            })
            .collect();

        quote! {
            #(
                #all_functions
            )*
        }
    }
}
