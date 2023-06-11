//! Implementation for baker!("timed_interleaved", ...)

use proc_macro2::TokenStream;
use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Token};

use crate::common_functions::expand::aux_baking::{
    create_name_structs, create_session_type_structs, create_session_types,
    create_timed_role_structs,
};
use crate::common_functions::expand::cancel::cancel;
use crate::common_functions::expand::choose::{
    choose_timed, choose_timed_mpst_create_multi_to_all,
};
use crate::common_functions::expand::close_timed::close_timed;
use crate::common_functions::expand::fork::fork_timed_interleaved_mpst;
use crate::common_functions::expand::meshedchannels::meshedchannels;
use crate::common_functions::expand::offer::offer_timed;
use crate::common_functions::expand::parenthesised::parenthesised_groups;
use crate::common_functions::expand::recv::{recv_from_all_timed, recv_timed};
use crate::common_functions::expand::send::send_timed_canceled;

#[derive(Debug)]
pub(crate) struct BakingTimedInterleavedWithEnumAndCancel {
    // For the first session
    meshedchannels_name_one: Ident,
    all_roles_one: Vec<TokenStream>,
    number_roles_one: u64,
    index_tuple_one: u64,
    // For the second session
    meshedchannels_name_two: Ident,
    all_roles_two: Vec<TokenStream>,
    number_roles_two: u64,
    index_tuple_two: u64,
    // For the fork function
    func_name: Ident,
}

impl Parse for BakingTimedInterleavedWithEnumAndCancel {
    fn parse(input: ParseStream) -> Result<Self> {
        // Get name of the first MeshedChannels
        let meshedchannels_name_one = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        // Get name of the first Roles
        let content_all_roles_one;
        let _parentheses = syn::parenthesized!(content_all_roles_one in input);
        let all_roles_one = parenthesised_groups(TokenStream::parse(&content_all_roles_one)?);
        <Token![,]>::parse(input)?;

        // Compute number of the first Roles
        let number_roles_one = u64::try_from(all_roles_one.len()).unwrap();

        // Get index of first central role
        let index_tuple_one = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();
        <Token![,]>::parse(input)?;

        // Get name of the second MeshedChannels
        let meshedchannels_name_two = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        // Get name of the second Roles
        let content_all_roles_two;
        let _parentheses = syn::parenthesized!(content_all_roles_two in input);
        let all_roles_two = parenthesised_groups(TokenStream::parse(&content_all_roles_two)?);
        <Token![,]>::parse(input)?;

        // Compute number of the second Roles
        let number_roles_two = u64::try_from(all_roles_two.len()).unwrap();

        // Get index of second central role
        let index_tuple_two = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();
        <Token![,]>::parse(input)?;

        let func_name = Ident::parse(input)?;

        Ok(BakingTimedInterleavedWithEnumAndCancel {
            meshedchannels_name_one,
            all_roles_one,
            number_roles_one,
            index_tuple_one,
            meshedchannels_name_two,
            all_roles_two,
            number_roles_two,
            index_tuple_two,
            func_name,
        })
    }
}

impl From<BakingTimedInterleavedWithEnumAndCancel> for TokenStream {
    fn from(input: BakingTimedInterleavedWithEnumAndCancel) -> TokenStream {
        input.expand()
    }
}

impl BakingTimedInterleavedWithEnumAndCancel {
    fn expand(&self) -> TokenStream {
        // Create the first MeshedChannel structure and related methods and macros
        let meshedchannels_struct_one =
            meshedchannels(&self.meshedchannels_name_one, self.number_roles_one);

        let session_types_one = create_session_types(1, self.number_roles_one);

        let session_types_struct_one = create_session_type_structs(1, self.number_roles_one);

        let roles_struct_one = create_timed_role_structs(&self.all_roles_one);

        let names_struct_one = create_name_structs(&self.all_roles_one);

        let send_methods_one: Vec<TokenStream> = (1..=self.number_roles_one)
            .map(|sender| {
                (1..=self.number_roles_one)
                    .filter_map(|receiver| {
                        if sender != receiver {
                            Some(send_timed_canceled(
                                &self.all_roles_one,
                                sender,
                                receiver,
                                &session_types_one,
                                &session_types_struct_one,
                                &self.meshedchannels_name_one,
                                self.number_roles_one,
                            ))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let recv_methods_one: Vec<TokenStream> = (1..=self.number_roles_one)
            .map(|receiver| {
                (1..=self.number_roles_one)
                    .filter_map(|sender| {
                        if receiver != sender {
                            Some(recv_timed(
                                &self.all_roles_one,
                                receiver,
                                sender,
                                &session_types_one,
                                &session_types_struct_one,
                                &self.meshedchannels_name_one,
                                self.number_roles_one,
                            ))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let recv_from_all_methods_one: Vec<TokenStream> = (1..=self.number_roles_one)
            .map(|receiver| {
                (1..=self.number_roles_one)
                    .filter_map(|sender| {
                        if receiver != sender {
                            Some(recv_from_all_timed(
                                &self.all_roles_one,
                                receiver,
                                sender,
                                &session_types_one,
                                &session_types_struct_one,
                                &self.meshedchannels_name_one,
                                self.number_roles_one,
                            ))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let offer_methods_one: Vec<TokenStream> = (1..=self.number_roles_one)
            .map(|receiver| {
                (1..=self.number_roles_one)
                    .filter_map(|sender| {
                        if receiver != sender {
                            Some(offer_timed(
                                &self.all_roles_one,
                                sender,
                                receiver,
                                &self.meshedchannels_name_one,
                                self.number_roles_one,
                            ))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let choose_methods_one: Vec<TokenStream> = (1..=self.number_roles_one)
            .map(|sender| {
                choose_timed(
                    &self.all_roles_one,
                    sender,
                    &self.meshedchannels_name_one,
                    self.number_roles_one,
                )
            })
            .collect();

        let close_methods_one: TokenStream =
            close_timed(&self.meshedchannels_name_one, self.number_roles_one);

        let choose_mpst_create_multi_to_all_one = choose_timed_mpst_create_multi_to_all(
            &self.meshedchannels_name_one,
            &self.all_roles_one,
            self.number_roles_one,
        );

        let cancel_method_one: TokenStream =
            cancel(&self.meshedchannels_name_one, self.number_roles_one);

        // Create the second MeshedChannel structure and related methods and macros
        let meshedchannels_struct_two =
            meshedchannels(&self.meshedchannels_name_two, self.number_roles_two);

        let session_types_two = create_session_types(1, self.number_roles_two);

        let session_types_struct_two = create_session_type_structs(1, self.number_roles_two);

        let roles_struct_two = create_timed_role_structs(&self.all_roles_two);

        let names_struct_two = create_name_structs(&self.all_roles_two);

        let send_methods_two: Vec<TokenStream> = (1..=self.number_roles_two)
            .map(|sender| {
                (1..=self.number_roles_two)
                    .filter_map(|receiver| {
                        if sender != receiver {
                            Some(send_timed_canceled(
                                &self.all_roles_two,
                                sender,
                                receiver,
                                &session_types_two,
                                &session_types_struct_two,
                                &self.meshedchannels_name_two,
                                self.number_roles_two,
                            ))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let recv_methods_two: Vec<TokenStream> = (1..=self.number_roles_two)
            .map(|receiver| {
                (1..=self.number_roles_two)
                    .filter_map(|sender| {
                        if receiver != sender {
                            Some(recv_timed(
                                &self.all_roles_two,
                                receiver,
                                sender,
                                &session_types_two,
                                &session_types_struct_two,
                                &self.meshedchannels_name_two,
                                self.number_roles_two,
                            ))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let recv_from_all_methods_two: Vec<TokenStream> = (1..=self.number_roles_two)
            .map(|receiver| {
                (1..=self.number_roles_two)
                    .filter_map(|sender| {
                        if receiver != sender {
                            Some(recv_from_all_timed(
                                &self.all_roles_two,
                                receiver,
                                sender,
                                &session_types_two,
                                &session_types_struct_two,
                                &self.meshedchannels_name_two,
                                self.number_roles_two,
                            ))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let offer_methods_two: Vec<TokenStream> = (1..=self.number_roles_two)
            .map(|receiver| {
                (1..=self.number_roles_two)
                    .filter_map(|sender| {
                        if receiver != sender {
                            Some(offer_timed(
                                &self.all_roles_two,
                                sender,
                                receiver,
                                &self.meshedchannels_name_two,
                                self.number_roles_two,
                            ))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let choose_methods_two: Vec<TokenStream> = (1..=self.number_roles_two)
            .map(|sender| {
                choose_timed(
                    &self.all_roles_two,
                    sender,
                    &self.meshedchannels_name_two,
                    self.number_roles_two,
                )
            })
            .collect();

        let close_methods_two: TokenStream =
            close_timed(&self.meshedchannels_name_two, self.number_roles_two);

        let choose_mpst_create_multi_to_all_two = choose_timed_mpst_create_multi_to_all(
            &self.meshedchannels_name_two,
            &self.all_roles_two,
            self.number_roles_two,
        );

        let cancel_method_two: TokenStream =
            cancel(&self.meshedchannels_name_two, self.number_roles_two);

        // Create the fork function elemental blocks
        let quote_fork_mpst = fork_timed_interleaved_mpst(
            &self.func_name,
            &self.meshedchannels_name_one,
            self.number_roles_one,
            self.index_tuple_one,
            &self.meshedchannels_name_two,
            self.number_roles_two,
            self.index_tuple_two,
        );

        quote! {
            // Append the first MeshedChannel structure and related methods and macros
            #meshedchannels_struct_one

            #( #roles_struct_one )*

            #( #names_struct_one )*

            #( #send_methods_one )*

            #( #recv_methods_one )*

            #( #recv_from_all_methods_one )*

            #( #offer_methods_one )*

            #( #choose_methods_one )*

            #close_methods_one

            #cancel_method_one

            #choose_mpst_create_multi_to_all_one

            // Create the second MeshedChannel structure and related methods and macros
            #meshedchannels_struct_two

            #( #roles_struct_two )*

            #( #names_struct_two )*

            #( #send_methods_two )*

            #( #recv_methods_two )*

            #( #recv_from_all_methods_two )*

            #( #offer_methods_two )*

            #( #choose_methods_two )*

            #close_methods_two

            #cancel_method_two

            #choose_mpst_create_multi_to_all_two

            // Create the fork function
            #quote_fork_mpst

        }
    }
}
