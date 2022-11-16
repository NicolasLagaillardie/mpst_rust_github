/// Implementation for the baker!("basic", ...)

use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result, Token};

use crate::common_functions::expand::cancel::cancel;
use crate::common_functions::expand::choose::choose;
use crate::common_functions::expand::close::close;
use crate::common_functions::expand::fork::fork_mpst;
use crate::common_functions::expand::meshedchannels::meshedchannels;
use crate::common_functions::expand::name::name;
use crate::common_functions::expand::offer::offer;
use crate::common_functions::expand::parenthesised::get_all_roles;
use crate::common_functions::expand::recv::{recv, recv_from_all};
use crate::common_functions::expand::role::role;
use crate::common_functions::expand::send::send_basic;

#[derive(Debug)]
pub(crate) struct Baking {
    meshedchannels_name: Ident,
    all_roles: Vec<TokenStream>,
    number_roles: u64,
}

impl Parse for Baking {
    fn parse(input: ParseStream) -> Result<Self> {
        // Get name of the MeshedChannels
        let meshedchannels_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        // Get name of the Roles
        let all_roles = get_all_roles(TokenStream::parse(input)?);

        // Compute number of Roles
        let number_roles = u64::try_from(all_roles.len()).unwrap();

        Ok(Baking {
            meshedchannels_name,
            all_roles,
            number_roles,
        })
    }
}

impl From<Baking> for TokenStream {
    fn from(input: Baking) -> TokenStream {
        input.expand()
    }
}

impl Baking {
    fn expand(&self) -> TokenStream {
        // Get the meshedchannels structure
        let meshedchannels_struct = meshedchannels(&self.meshedchannels_name, self.number_roles);

        let quote_fork_mpst = fork_mpst(&self.meshedchannels_name, self.number_roles);

        let session_types: Vec<Ident> = (1..self.number_roles)
            .map(|i| Ident::new(&format!("S{i}"), Span::call_site()))
            .collect();

        let session_types_struct: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{i}"), Span::call_site());
                quote! { #temp_ident : mpstthree::binary::struct_trait::session::Session , }
            })
            .collect();

        let roles_struct: Vec<TokenStream> = self
            .all_roles
            .iter()
            .map(|i| role(format!("{i}")))
            .collect();

        let names_struct: Vec<TokenStream> = self
            .all_roles
            .iter()
            .map(|i| name(format!("{i}")))
            .collect();

        let send_methods: Vec<TokenStream> = (1..=self.number_roles)
            .map(|sender| {
                (1..=self.number_roles)
                    .filter_map(|receiver| {
                        if sender != receiver {
                            Some(send_basic(
                                &self.all_roles,
                                sender,
                                receiver,
                                &session_types,
                                &session_types_struct,
                                &self.meshedchannels_name,
                                self.number_roles,
                            ))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let recv_methods: Vec<TokenStream> = (1..=self.number_roles)
            .map(|receiver| {
                (1..=self.number_roles)
                    .filter_map(|sender| {
                        if receiver != sender {
                            Some(recv(
                                &self.all_roles,
                                receiver,
                                sender,
                                &session_types,
                                &session_types_struct,
                                &self.meshedchannels_name,
                                self.number_roles,
                            ))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let recv_from_all_methods: Vec<TokenStream> = (1..=self.number_roles)
            .map(|receiver| {
                (1..=self.number_roles)
                    .filter_map(|sender| {
                        if receiver != sender {
                            Some(recv_from_all(
                                &self.all_roles,
                                receiver,
                                sender,
                                &session_types,
                                &session_types_struct,
                                &self.meshedchannels_name,
                                self.number_roles,
                            ))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let offer_methods: Vec<TokenStream> = (1..=self.number_roles)
            .map(|receiver| {
                (1..=self.number_roles)
                    .filter_map(|sender| {
                        if receiver != sender {
                            Some(offer(
                                &self.all_roles,
                                sender,
                                receiver,
                                &self.meshedchannels_name,
                                self.number_roles,
                            ))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let choose_methods: Vec<TokenStream> = (1..=self.number_roles)
            .map(|sender| {
                choose(
                    &self.all_roles,
                    sender,
                    &self.meshedchannels_name,
                    self.number_roles,
                )
            })
            .collect();

        let close_methods: TokenStream = close(&self.meshedchannels_name, self.number_roles);

        let cancel_method: TokenStream = cancel(&self.meshedchannels_name, self.number_roles);

        quote! {
            #meshedchannels_struct

            #( #roles_struct )*

            #( #names_struct )*

            #( #send_methods )*

            #( #recv_methods )*

            #( #recv_from_all_methods )*

            #( #offer_methods )*

            #( #choose_methods )*

            #close_methods

            #cancel_method

            #[allow(unused_macros)]
            macro_rules! offer_mpst {
                ($session: expr, { $( $pat: pat => $result: expr, )+ }) => {
                    (move || -> Result<_, _> {
                        let (l, s) = $session.recv()?;
                        mpstthree::binary::cancel::cancel(s);
                        match l {
                            $(
                                $pat => $result,
                            )+
                            _ => panic!("Unexpected payload") ,
                        }
                    })()
                };
            }

            #quote_fork_mpst

        }
    }
}
