use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result, Token};

use crate::common_functions::expand::cancel::cancel;
use crate::common_functions::expand::choose::{choose, choose_mpst_create_multi_to_all};
use crate::common_functions::expand::close::close;
use crate::common_functions::expand::fork::fork_mpst;
use crate::common_functions::expand::meshedchannels::meshedchannels;
use crate::common_functions::expand::name::name;
use crate::common_functions::expand::offer::offer;
use crate::common_functions::expand::recv::{recv, recv_from_all};
use crate::common_functions::expand::role::role;
use crate::common_functions::expand::send::send_canceled;
use crate::common_functions::expand::token_stream::token_stream;

#[derive(Debug)]
pub(crate) struct BakingWithEnumAndCancel {
    meshedchannels_name: Ident,
    all_roles: Vec<TokenStream>,
    number_roles: u64,
}

impl Parse for BakingWithEnumAndCancel {
    fn parse(input: ParseStream) -> Result<Self> {
        let meshedchannels_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;
        let all_roles = token_stream(<&syn::parse::ParseBuffer>::clone(&input))?;

        let number_roles = u64::try_from(all_roles.len()).unwrap();

        Ok(BakingWithEnumAndCancel {
            meshedchannels_name,
            all_roles,
            number_roles,
        })
    }
}

impl From<BakingWithEnumAndCancel> for TokenStream {
    fn from(input: BakingWithEnumAndCancel) -> TokenStream {
        input.expand()
    }
}

impl BakingWithEnumAndCancel {
    fn expand(&self) -> TokenStream {
        let meshedchannels_name = self.meshedchannels_name.clone();

        // Get all the roles provided into a Vec
        let all_roles = self.all_roles.clone();

        // Get the meshedchannels structure
        let meshedchannels_struct = meshedchannels(meshedchannels_name, self.number_roles);

        let quote_fork_mpst = fork_mpst(self.meshedchannels_name.clone(), self.number_roles);

        let session_types: Vec<Ident> = (1..self.number_roles)
            .map(|i| Ident::new(&format!("S{}", i), Span::call_site()))
            .collect();

        let session_types_struct: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                quote! { #temp_ident : mpstthree::binary::struct_trait::session::Session , }
            })
            .collect();

        let roles_struct: Vec<TokenStream> =
            all_roles.iter().map(|i| role(format!("{}", i))).collect();

        let names_struct: Vec<TokenStream> =
            all_roles.iter().map(|i| name(format!("{}", i))).collect();

        let send_methods: Vec<TokenStream> = (1..=self.number_roles)
            .map(|sender| {
                (1..=self.number_roles)
                    .filter_map(|receiver| {
                        if sender != receiver {
                            Some(send_canceled(
                                self.all_roles.clone(),
                                sender,
                                receiver,
                                session_types.clone(),
                                session_types_struct.clone(),
                                self.meshedchannels_name.clone(),
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
                                self.all_roles.clone(),
                                receiver,
                                sender,
                                session_types.clone(),
                                session_types_struct.clone(),
                                self.meshedchannels_name.clone(),
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
                                self.all_roles.clone(),
                                receiver,
                                sender,
                                session_types.clone(),
                                session_types_struct.clone(),
                                self.meshedchannels_name.clone(),
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
                                self.all_roles.clone(),
                                sender,
                                receiver,
                                self.meshedchannels_name.clone(),
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
                    self.all_roles.clone(),
                    sender,
                    self.meshedchannels_name.clone(),
                    self.number_roles,
                )
            })
            .collect();

        let close_methods: TokenStream = close(self.meshedchannels_name.clone(), self.number_roles);

        let choose_mpst_create_multi_to_all = choose_mpst_create_multi_to_all(
            self.meshedchannels_name.clone(),
            self.all_roles.clone(),
            self.number_roles,
        );

        let cancel_method: TokenStream =
            cancel(self.meshedchannels_name.clone(), self.number_roles);

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

            #quote_fork_mpst

            #choose_mpst_create_multi_to_all

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

        }
    }
}
