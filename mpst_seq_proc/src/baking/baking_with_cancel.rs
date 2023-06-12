//! Implementation for baker!("cancel", ...)

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result};

use crate::common_functions::expand::aux_baking::{
    create_name_structs, create_role_structs, create_session_type_structs, create_session_types,
};
use crate::common_functions::expand::cancel::cancel;
use crate::common_functions::expand::choose::choose;
use crate::common_functions::expand::close::close;
use crate::common_functions::expand::fork::fork_mpst;
use crate::common_functions::expand::meshedchannels::meshedchannels;
use crate::common_functions::expand::offer::offer;
use crate::common_functions::expand::recv::{recv, recv_from_all};
use crate::common_functions::expand::send::send_canceled;
use crate::common_functions::parsing::parse_stream_roles;

#[derive(Debug)]
pub(crate) struct BakingWithCancel {
    meshedchannels_name: Ident,
    all_roles: Vec<TokenStream>,
    number_roles: u64,
}

impl Parse for BakingWithCancel {
    fn parse(input: ParseStream) -> Result<Self> {
        let (meshedchannels_name, all_roles, number_roles) = parse_stream_roles(input)?;

        Ok(BakingWithCancel {
            meshedchannels_name,
            all_roles,
            number_roles,
        })
    }
}

impl From<BakingWithCancel> for TokenStream {
    fn from(input: BakingWithCancel) -> TokenStream {
        input.expand()
    }
}

impl BakingWithCancel {
    fn expand(&self) -> TokenStream {
        // Get the meshedchannels structure
        let meshedchannels_struct = meshedchannels(&self.meshedchannels_name, self.number_roles);

        let quote_fork_mpst = fork_mpst(&self.meshedchannels_name, self.number_roles);

        let session_types = create_session_types(1, self.number_roles);

        let session_types_struct = create_session_type_structs(1, self.number_roles);

        let role_structs = create_role_structs(&self.all_roles);

        let name_structs = create_name_structs(&self.all_roles);

        let send_methods: Vec<TokenStream> = (1..=self.number_roles)
            .map(|sender| {
                (1..=self.number_roles)
                    .filter_map(|receiver| {
                        if sender != receiver {
                            Some(send_canceled(
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

            #( #role_structs )*

            #( #name_structs )*

            #( #send_methods )*

            #( #recv_methods )*

            #( #recv_from_all_methods )*

            #( #offer_methods )*

            #( #choose_methods )*

            #close_methods

            #cancel_method

            #[allow(unused_macros)]
            macro_rules! offer_cancel_mpst {
                ($session: expr, { $( $pat: pat => $result: expr, )+ }) => {
                    (move || -> Result<_, _> {
                        let ((session1, cont), s) = $session.recv()?;
                        let s = s.session1.sender.send(mpstthree::binary::struct_trait::end::Signal::Offer(session1)).unwrap();
                        mpstthree::binary::cancel::cancel(s);
                        match cont {
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
