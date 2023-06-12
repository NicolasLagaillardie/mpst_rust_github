//! Implementation for baker_timed!(...)

use proc_macro2::TokenStream;
use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result, Token};

use crate::common_functions::expand::aux_baking::{
    create_name_structs, create_session_type_structs, create_session_types,
    create_timed_role_structs,
};
use crate::common_functions::expand::cancel::cancel;
use crate::common_functions::expand::choose::{
    choose_timed, choose_timed_mpst_create_multi_to_all,
};
use crate::common_functions::expand::close_timed::close_timed;
use crate::common_functions::expand::fork::fork_timed_mpst;
use crate::common_functions::expand::meshedchannels::meshedchannels;
use crate::common_functions::expand::offer::offer_timed;
use crate::common_functions::expand::parenthesised::get_all_roles;
use crate::common_functions::expand::recv::{recv_from_all_timed, recv_timed};
use crate::common_functions::expand::send::send_timed_canceled;

#[derive(Debug)]
pub(crate) struct BakingTimedWithEnumAndCancel {
    meshedchannels_name: Ident,
    all_roles: Vec<TokenStream>,
    number_roles: u64,
}

impl Parse for BakingTimedWithEnumAndCancel {
    fn parse(input: ParseStream) -> Result<Self> {
        // Get name of the MeshedChannels
        let meshedchannels_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        // Get name of the Roles
        let all_roles = get_all_roles(TokenStream::parse(input)?);

        // Compute number of Roles
        let number_roles = u64::try_from(all_roles.len()).unwrap();

        Ok(BakingTimedWithEnumAndCancel {
            meshedchannels_name,
            all_roles,
            number_roles,
        })
    }
}

impl From<BakingTimedWithEnumAndCancel> for TokenStream {
    fn from(input: BakingTimedWithEnumAndCancel) -> TokenStream {
        input.expand()
    }
}

impl BakingTimedWithEnumAndCancel {
    fn expand(&self) -> TokenStream {
        // Get the meshedchannels structure
        let meshedchannels_struct = meshedchannels(&self.meshedchannels_name, self.number_roles);

        let session_types = create_session_types(1, self.number_roles);

        let session_types_struct = create_session_type_structs(1, self.number_roles);

        let role_structs = create_timed_role_structs(&self.all_roles);

        let name_structs = create_name_structs(&self.all_roles);

        let send_methods: Vec<TokenStream> = (1..=self.number_roles)
            .map(|sender| {
                (1..=self.number_roles)
                    .filter_map(|receiver| {
                        if sender != receiver {
                            Some(send_timed_canceled(
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
                            Some(recv_timed(
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
                            Some(recv_from_all_timed(
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
                            Some(offer_timed(
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
                choose_timed(
                    &self.all_roles,
                    sender,
                    &self.meshedchannels_name,
                    self.number_roles,
                )
            })
            .collect();

        let choose_mpst_create_multi_to_all = choose_timed_mpst_create_multi_to_all(
            &self.meshedchannels_name,
            &self.all_roles,
            self.number_roles,
        );

        let cancel_method: TokenStream = cancel(&self.meshedchannels_name, self.number_roles);

        let close_methods: TokenStream = close_timed(&self.meshedchannels_name, self.number_roles);

        let quote_fork_mpst = fork_timed_mpst(&self.meshedchannels_name, self.number_roles);

        quote! {
            #meshedchannels_struct

            #( #role_structs )*

            #( #name_structs )*

            #( #send_methods )*

            #( #recv_methods )*

            #( #recv_from_all_methods )*

            #( #offer_methods )*

            #( #choose_methods )*

            #close_methods

            #cancel_method

            #choose_mpst_create_multi_to_all

            #quote_fork_mpst

            #[allow(unused_macros)]
            macro_rules! offer_mpst {
                ($session: expr, $all_clocks:expr, { $( $pat: pat => $result: expr, )+ }) => {
                    (move || -> Result<_, _> {
                        let (l, s) = $session.recv($all_clocks)?;
                        s.cancel();
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
