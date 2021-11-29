use proc_macro2::{Span, TokenStream, TokenTree};
use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Token};

type VecOfTuple = Vec<(u64, u64, u64)>;

#[derive(Debug)]
pub struct ChooseTypeMultiToAllBundle {
    labels: Vec<TokenStream>,
    receivers: Vec<TokenStream>,
    fn_names: Vec<TokenStream>,
    branches: Vec<TokenStream>,
    new_types: Vec<TokenStream>,
    sender: Ident,
    meshedchannels_name: Ident,
    n_sessions: u64,
    n_branches: u64,
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

impl Parse for ChooseTypeMultiToAllBundle {
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

        // The meshedchannels_name
        let meshedchannels_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        // The index of the sender
        let exclusion = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        // The number of receivers
        let n_sessions = u64::try_from(all_receivers.len()).unwrap() + 1;

        // The number of labels
        let n_branches = u64::try_from(all_branches.len()).unwrap() + 1;

        // The number of functions
        let n_fn_names = u64::try_from(all_fn_names.len()).unwrap() + 1;

        // The number of functions
        let n_new_type = u64::try_from(all_new_types.len()).unwrap() + 1;

        if n_branches != n_fn_names || n_branches != n_new_type || n_new_type != n_fn_names {
            panic!("The number of new types, functions and branches are not the same")
        };

        Ok(ChooseTypeMultiToAllBundle {
            labels: all_labels,
            receivers: all_receivers,
            fn_names: all_fn_names,
            branches: all_branches,
            new_types: all_new_types,
            sender,
            meshedchannels_name,
            n_sessions,
            n_branches,
            exclusion,
        })
    }
}

impl From<ChooseTypeMultiToAllBundle> for TokenStream {
    fn from(input: ChooseTypeMultiToAllBundle) -> TokenStream {
        input.expand()
    }
}

impl ChooseTypeMultiToAllBundle {
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
                let all_new_types = self.new_types.clone();
                let meshedchannels_name = self.meshedchannels_name.clone();
                let diff = self.n_sessions - 1;
                let diag = self.diag();

                // Build Send<*enum*, mpstthree::binary::struct_trait::end::End>,
                let send_types: Vec<TokenStream> = (1..self.n_sessions)
                    .map(|j| {
                        let temp_label =
                            if let Some(elt) = all_labels.get(usize::try_from(j - 1).unwrap()) {
                                elt
                            } else {
                                panic!("Not enough labels")
                            };

                        quote! {
                            Send< #temp_label , mpstthree::binary::struct_trait::end::End> ,
                        }
                    })
                    .collect();

                // Build let ( channel_n_m , channel_m_n ) =
                //  <_ as mpstthree::binary::struct_trait::session::Session>::new();
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
                        quote! {
                            let ( #channel_left , #channel_right ) =
                                <_ as mpstthree::binary::struct_trait::session::Session>::new();
                        }
                    })
                    .collect();

                // Build let ( stack_n , _) = <_ as mpstthree::role::Role>::new();
                let new_roles: Vec<TokenStream> = (1..=self.n_sessions)
                    .map(|j| {
                        let temp_ident = Ident::new(
                            &format!("stack_{}", j),
                            Span::call_site(),
                        );
                        quote! {
                            let ( #temp_ident , _) = <_ as mpstthree::role::Role>::new();
                        }
                    })
                    .collect();

                // Build let ( name_n , _) =
                //      <RoleN::<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
                let new_names: Vec<TokenStream> = (1..self.n_sessions)
                    .map(|j| {
                        let temp_name =
                            Ident::new(&format!("name_{}", j), Span::call_site());
                        let temp_role =
                            if let Some(elt) = all_receivers.get(usize::try_from(j - 1).unwrap()) {
                                elt
                            } else {
                                panic!("Not enough receivers")
                            };
                        quote! {
                            let ( #temp_name , _) =
                                <#temp_role::<mpstthree::role::end::RoleEnd>
                                    as mpstthree::role::Role>::new();
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

                let new_meshedchannels: Vec<TokenStream> = (1..self.n_sessions)
                    .map(|j| {
                        let temp_session = Ident::new(
                            &format!("session{}", j),
                            Span::call_site(),
                        );
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

                let temp_fn_name =
                    if let Some(elt) = all_fn_names.get(usize::try_from(i - 1).unwrap()) {
                        elt
                    } else {
                        panic!("Not enough fn_names")
                    };

                let temp_new_type =
                    if let Some(elt) = all_new_types.get(usize::try_from(i - 1).unwrap()) {
                        elt
                    } else {
                        panic!("Not enough new_type")
                    };

                let temp_branches =
                    if let Some(elt) = all_branches.get(usize::try_from(i - 1).unwrap()) {
                        elt
                    } else {
                        panic!("Not enough branches")
                    };

                let all_send: Vec<TokenStream> = (1..self.n_sessions)
                    .map(|j| {
                        let sessions_sent: Vec<TokenStream> = (1..self.n_sessions)
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

                        let sessions_returned: Vec<TokenStream> = (1..self.n_sessions)
                            .map(|k| {
                                let temp_ident = Ident::new(
                                    &format!("session{}", k),
                                    Span::call_site(),
                                );
                                if j == k {
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

                        let temp_name =
                            Ident::new(&format!("name_{}", j), Span::call_site());
                        let temp_stack = Ident::new(
                            &format!("stack_{}", j),
                            Span::call_site(),
                        );
                        let temp_session = Ident::new(
                            &format!("session{}", j),
                            Span::call_site(),
                        );
                        let temp_label =
                            if let Some(elt) = all_labels.get(usize::try_from(j - 1).unwrap()) {
                                elt
                            } else {
                                panic!("Not enough labels")
                            };

                        quote! {
                            let s = {
                                let new_session = mpstthree::binary::send::send(
                                    #temp_label::#temp_branches(
                                        #meshedchannels_name {
                                            #(
                                                #sessions_sent
                                            )*
                                            stack: #temp_stack ,
                                            name: #temp_name ,
                                        }
                                    ),
                                    s.#temp_session ,
                                );

                                #meshedchannels_name {
                                    #(
                                        #sessions_returned
                                    )*
                                    stack: s.stack,
                                    name: s.name,
                                }
                            };
                        }
                    })
                    .collect();

                quote! {
                    fn #temp_fn_name(
                        s: #meshedchannels_name<
                            #(
                                #send_types
                            )*
                            mpstthree::role::broadcast::RoleBroadcast,
                            #sender<mpstthree::role::end::RoleEnd>,
                        >
                    ) -> #temp_new_type
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

                        let ( #new_name_sender , _) =
                            <#sender<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

                        #(
                            #all_send
                        )*

                        mpstthree::binary::cancel::cancel(s);

                        #meshedchannels_name {
                            #(
                                #new_meshedchannels
                            )*
                            stack: #new_stack_sender ,
                            name: #new_name_sender ,
                        }
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
