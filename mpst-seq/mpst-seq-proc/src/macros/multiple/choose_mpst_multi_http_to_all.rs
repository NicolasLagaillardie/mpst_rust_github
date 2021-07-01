use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

type VecOfTuple = Vec<(u64, u64, u64)>;

#[derive(Debug)]
pub struct ChooseTypeMultiHttpToAllMacroInput {
    session: syn::Expr,
    labels: Vec<proc_macro2::TokenStream>,
    receivers: Vec<proc_macro2::TokenStream>,
    sender: syn::Ident,
    sessionmpst_name: syn::Ident,
    nsessions: u64,
    exclusion: u64,
}

fn expand_parenthesized(stream: &proc_macro2::TokenStream) -> Vec<proc_macro2::TokenStream> {
    let mut out: Vec<proc_macro2::TokenStream> = Vec::new();
    for tt in stream.clone().into_iter() {
        let elt = match tt {
            proc_macro2::TokenTree::Group(g) => Some(g.stream()),
            _ => None,
        };
        if let Some(elt_tt) = elt {
            out.push(elt_tt)
        }
    }
    out
}

impl Parse for ChooseTypeMultiHttpToAllMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // The session
        let session = syn::Expr::parse(input)?;
        <Token![,]>::parse(input)?;

        // The labels
        let content_labels;
        let _parentheses = syn::parenthesized!(content_labels in input);
        let labels = proc_macro2::TokenStream::parse(&content_labels)?;

        let all_labels: Vec<proc_macro2::TokenStream> = expand_parenthesized(&labels);

        <Token![,]>::parse(input)?;

        // The receivers
        let content_receivers;
        let _parentheses = syn::parenthesized!(content_receivers in input);
        let receivers = proc_macro2::TokenStream::parse(&content_receivers)?;

        let all_receivers: Vec<proc_macro2::TokenStream> = expand_parenthesized(&receivers);

        <Token![,]>::parse(input)?;

        // The sender
        let sender = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        // The sessionmpst_name
        let sessionmpst_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        // The index of the sender
        let exclusion = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        // The number of receivers
        let nsessions = all_receivers.len().to_string().parse::<u64>().unwrap() + 1;

        if all_receivers.len().to_string().parse::<u64>().unwrap()
            != all_labels.len().to_string().parse::<u64>().unwrap()
        {
            panic!("The number of receivers and labels is not the same")
        };

        Ok(ChooseTypeMultiHttpToAllMacroInput {
            session,
            labels: all_labels,
            receivers: all_receivers,
            sender,
            sessionmpst_name,
            nsessions,
            exclusion,
        })
    }
}

impl From<ChooseTypeMultiHttpToAllMacroInput> for proc_macro2::TokenStream {
    fn from(input: ChooseTypeMultiHttpToAllMacroInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl ChooseTypeMultiHttpToAllMacroInput {
    /// Create the whole matrix of index according to line and column
    fn diag(&self) -> VecOfTuple {
        let diff = self.nsessions - 1;

        let mut column = 0;
        let mut line = 0;

        // Create the upper diag
        (0..(diff * (diff + 1) / 2))
            .map(|i| {
                if line == column {
                    column += 1;
                } else if column >= (self.nsessions - 1) {
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

    fn expand(&self) -> proc_macro2::TokenStream {
        let session = self.session.clone();
        let all_labels = self.labels.clone();
        let all_receivers = self.receivers.clone();
        let sender = self.sender.clone();
        let sessionmpst_name = self.sessionmpst_name.clone();
        let diff = self.nsessions - 1;
        let diag = self.diag();

        let new_channels: Vec<proc_macro2::TokenStream> = (1..=(diff * (diff + 1) / 2))
            .map(|i| {
                let (line, column, _) = self.get_tuple_diag(&diag, i);
                let channel_left = syn::Ident::new(
                    &format!("channel_{}_{}", line, column),
                    proc_macro2::Span::call_site(),
                );
                let channel_right = syn::Ident::new(
                    &format!("channel_{}_{}", column, line),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    let ( #channel_left , #channel_right ) =
                        <_ as mpstthree::binary::struct_trait::Session>::new();
                }
            })
            .collect();

        let new_roles: Vec<proc_macro2::TokenStream> = (1..=self.nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("stack_{}", i), proc_macro2::Span::call_site());
                quote! {
                    let ( #temp_ident , _) = <_ as mpstthree::role::Role>::new();
                }
            })
            .collect();

        let new_names: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_name =
                    syn::Ident::new(&format!("name_{}", i), proc_macro2::Span::call_site());
                let temp_role = if let Some(elt) = all_receivers.get(usize::try_from(i - 1).unwrap()) {
                    elt
                } else {
                    panic!("Not enough receivers")
                };
                quote! {
                    let ( #temp_name , _) =
                        <#temp_role::<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
                }
            })
            .collect();

        let new_name_sender = syn::Ident::new(
            &format!("name_{}", self.nsessions),
            proc_macro2::Span::call_site(),
        );

        let new_stack_sender = syn::Ident::new(
            &format!("stack_{}", self.nsessions),
            proc_macro2::Span::call_site(),
        );

        let all_send: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let new_sessions: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
                    .map(|j| {
                        let temp = if i >= self.exclusion { i + 1 } else { i };

                        let temp_ident = syn::Ident::new(
                            &format!("session{}", j),
                            proc_macro2::Span::call_site(),
                        );

                        let temp_channel = if j < temp {
                            syn::Ident::new(
                                &format!("channel_{}_{}", temp, j),
                                proc_macro2::Span::call_site(),
                            )
                        } else {
                            syn::Ident::new(
                                &format!("channel_{}_{}", temp, j + 1),
                                proc_macro2::Span::call_site(),
                            )
                        };

                        quote! {
                            #temp_ident : #temp_channel ,
                        }
                    })
                    .collect();

                let temp_name =
                    syn::Ident::new(&format!("name_{}", i), proc_macro2::Span::call_site());

                let temp_stack =
                    syn::Ident::new(&format!("stack_{}", i), proc_macro2::Span::call_site());

                let temp_session =
                    syn::Ident::new(&format!("session{}", i), proc_macro2::Span::call_site());

                let temp_label = if let Some(elt) = all_labels.get(usize::try_from(i - 1).unwrap())
                {
                    elt
                } else {
                    panic!("Not enough labels")
                };

                quote! {
                    let _ = mpstthree::binary::send::send_http(
                        #temp_label(
                            #sessionmpst_name {
                                #(
                                    #new_sessions
                                )*
                                stack: #temp_stack ,
                                name: #temp_name ,
                            }
                        ),
                        s.#temp_session ,
                        false ,
                        Request::default()
                    );
                }
            })
            .collect();

        let new_sessionmpst: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_session =
                    syn::Ident::new(&format!("session{}", i), proc_macro2::Span::call_site());
                let temp_channel = if i < self.exclusion {
                    syn::Ident::new(
                        &format!("channel_{}_{}", self.exclusion, i),
                        proc_macro2::Span::call_site(),
                    )
                } else {
                    syn::Ident::new(
                        &format!("channel_{}_{}", self.exclusion, i + 1),
                        proc_macro2::Span::call_site(),
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

                let ( #new_name_sender , _) =
                    <#sender<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

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

                #sessionmpst_name {
                    #(
                        #new_sessionmpst
                    )*
                    stack: #new_stack_sender ,
                    name: #new_name_sender ,
                }
            }
        }
    }
}
