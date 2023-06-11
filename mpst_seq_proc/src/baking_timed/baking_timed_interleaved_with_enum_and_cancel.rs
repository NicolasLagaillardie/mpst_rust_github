//! Implementation for baker!("interleaved", ...)

use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Token};

use crate::common_functions::expand::cancel::cancel;
use crate::common_functions::expand::choose::{choose, choose_mpst_create_multi_to_all};
use crate::common_functions::expand::close::close;
use crate::common_functions::expand::meshedchannels::meshedchannels;
use crate::common_functions::expand::name::name;
use crate::common_functions::expand::offer::offer;
use crate::common_functions::expand::parenthesised::parenthesised_groups;
use crate::common_functions::expand::recv::{recv, recv_from_all};
use crate::common_functions::expand::role::role;
use crate::common_functions::expand::send::send_canceled;
use crate::common_functions::maths::{
    diag_and_matrix, diag_and_matrix_w_offset, get_tuple_diag, get_tuple_matrix,
};

// use crate::common_functions::expand::choose::{
//     choose_timed, choose_timed_mpst_create_multi_to_all,
// };
// use crate::common_functions::expand::close_timed::close_timed;
// use crate::common_functions::expand::fork::fork_timed_mpst;
// use crate::common_functions::expand::meshedchannels::meshedchannels;
// use crate::common_functions::expand::name::name;
// use crate::common_functions::expand::offer::offer_timed;
// use crate::common_functions::expand::parenthesised::get_all_roles;
// use crate::common_functions::expand::recv::{recv_from_all_timed, recv_timed};
// use crate::common_functions::expand::role_timed::role_timed;
// use crate::common_functions::expand::send::send_timed_canceled;

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

        let names_struct_one: Vec<TokenStream> = self
            .all_roles_one
            .iter()
            .map(|i| name(format!("{i}")))
            .collect();

        let session_types_one: Vec<Ident> = (1..self.number_roles_one)
            .map(|i| Ident::new(&format!("S{i}"), Span::call_site()))
            .collect();

        let session_types_struct_one: Vec<TokenStream> = (1..self.number_roles_one)
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{i}"), Span::call_site());
                quote! { #temp_ident : mpstthree::binary::struct_trait::session::Session , }
            })
            .collect();

        let roles_struct_one: Vec<TokenStream> = self
            .all_roles_one
            .iter()
            .map(|i| role(format!("{i}")))
            .collect();

        let send_methods_one: Vec<TokenStream> = (1..=self.number_roles_one)
            .map(|sender| {
                (1..=self.number_roles_one)
                    .filter_map(|receiver| {
                        if sender != receiver {
                            Some(send_canceled(
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
                            Some(recv(
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
                            Some(recv_from_all(
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
                            Some(offer(
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
                choose(
                    &self.all_roles_one,
                    sender,
                    &self.meshedchannels_name_one,
                    self.number_roles_one,
                )
            })
            .collect();

        let close_methods_one: TokenStream =
            close(&self.meshedchannels_name_one, self.number_roles_one);

        let choose_mpst_create_multi_to_all_one = choose_mpst_create_multi_to_all(
            &self.meshedchannels_name_one,
            &self.all_roles_one,
            self.number_roles_one,
        );

        let cancel_method_one: TokenStream =
            cancel(&self.meshedchannels_name_one, self.number_roles_one);

        // Create the second MeshedChannel structure and related methods and macros
        let meshedchannels_struct_two =
            meshedchannels(&self.meshedchannels_name_two, self.number_roles_two);

        let names_struct_two: Vec<TokenStream> = self
            .all_roles_two
            .iter()
            .map(|i| name(format!("{i}")))
            .collect();

        let session_types_two: Vec<Ident> = (1..self.number_roles_two)
            .map(|i| Ident::new(&format!("S{i}"), Span::call_site()))
            .collect();

        let session_types_struct_two: Vec<TokenStream> = (1..self.number_roles_two)
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{i}"), Span::call_site());
                quote! { #temp_ident : mpstthree::binary::struct_trait::session::Session , }
            })
            .collect();

        let roles_struct_two: Vec<TokenStream> = self
            .all_roles_two
            .iter()
            .map(|i| role(format!("{i}")))
            .collect();

        let send_methods_two: Vec<TokenStream> = (1..=self.number_roles_two)
            .map(|sender| {
                (1..=self.number_roles_two)
                    .filter_map(|receiver| {
                        if sender != receiver {
                            Some(send_canceled(
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
                            Some(recv(
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
                            Some(recv_from_all(
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
                            Some(offer(
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
                choose(
                    &self.all_roles_two,
                    sender,
                    &self.meshedchannels_name_two,
                    self.number_roles_two,
                )
            })
            .collect();

        let close_methods_two: TokenStream =
            close(&self.meshedchannels_name_two, self.number_roles_two);

        let choose_mpst_create_multi_to_all_two = choose_mpst_create_multi_to_all(
            &self.meshedchannels_name_two,
            &self.all_roles_two,
            self.number_roles_two,
        );

        let cancel_method_two: TokenStream =
            cancel(&self.meshedchannels_name_two, self.number_roles_two);

        // Create the fork function elemental blocks
        let copy_func_name = &self.func_name;
        let copy_meshedchannels_name_one = &self.meshedchannels_name_one;
        let copy_meshedchannels_name_two = &self.meshedchannels_name_two;
        let sum_nsessions = self.number_roles_one + self.number_roles_two;
        let (matrix_one, _diag_one) = diag_and_matrix(self.number_roles_one);
        let (matrix_two, _diag_two) = diag_and_matrix(self.number_roles_two);
        let (matrix_w_offset_one, diag_w_offset_one) =
            diag_and_matrix_w_offset(self.number_roles_one);
        let (matrix_w_offset_two, diag_w_offset_two) =
            diag_and_matrix_w_offset(self.number_roles_two);

        // Generate
        // S0, S1, S2, ...
        let sessions: Vec<TokenStream> =
            (1..=((self.number_roles_one - 1) * (self.number_roles_one) / 2
                + (self.number_roles_two - 1) * (self.number_roles_two) / 2))
                .map(|i| {
                    let temp_ident = Ident::new(&format!("S{i}"), Span::call_site());
                    quote! {
                        #temp_ident ,
                    }
                })
                .collect();

        // Generate
        // S0: Session + `static ,
        // S1: Session + `static ,
        // ... ,
        let sessions_struct: Vec<TokenStream> =
            (1..=((self.number_roles_one - 1) * (self.number_roles_one) / 2
                + (self.number_roles_two - 1) * (self.number_roles_two) / 2))
                .map(|i| {
                    let temp_ident = Ident::new(&format!("S{i}"), Span::call_site());
                    quote! {
                        #temp_ident : mpstthree::binary::struct_trait::session::Session + 'static ,
                    }
                })
                .collect();

        let roles: Vec<TokenStream> = (1..=sum_nsessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("R{i}"), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let roles_struct: Vec<TokenStream> = (1..=sum_nsessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("R{i}"), Span::call_site());
                quote! {
                    #temp_ident : mpstthree::role::Role + 'static ,
                }
            })
            .collect();

        let new_roles: Vec<TokenStream> = (1..=sum_nsessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("R{i}"), Span::call_site());
                let temp_role = Ident::new(&format!("role_{i}"), Span::call_site());
                quote! {
                    let ( #temp_role , _) = < #temp_ident as mpstthree::role::Role >::new() ;
                }
            })
            .collect();

        let names: Vec<TokenStream> = (1..=sum_nsessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("N{i}"), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let names_struct: Vec<TokenStream> = (1..=sum_nsessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("N{i}"), Span::call_site());
                quote! {
                    #temp_ident : mpstthree::name::Name + 'static ,
                }
            })
            .collect();

        let new_names: Vec<TokenStream> = (1..=sum_nsessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("N{i}"), Span::call_site());
                let temp_name = Ident::new(&format!("name_{i}"), Span::call_site());
                quote! {
                    let ( #temp_name , _) = < #temp_ident as mpstthree::name::Name >::new() ;
                }
            })
            .collect();

        let functions: Vec<TokenStream> = (1..=(sum_nsessions - 2))
            .map(|i| {
                let temp_ident = Ident::new(&format!("F{i}"), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let functions_input_one: Vec<TokenStream> = (1..self.number_roles_one)
            .map(|i| {
                let temp_ident = Ident::new(&format!("F{i}"), Span::call_site());
                let temp_expr = Ident::new(&format!("f{i}"), Span::call_site());
                quote! {
                    #temp_expr : #temp_ident ,
                }
            })
            .collect();

        let functions_input_two: Vec<TokenStream> = (1..self.number_roles_two)
            .map(|i| {
                let temp_ident = Ident::new(
                    &format!("F{}", self.number_roles_one - 1 + i),
                    Span::call_site(),
                );
                let temp_expr = Ident::new(
                    &format!("f{}", self.number_roles_one - 1 + i),
                    Span::call_site(),
                );
                quote! {
                    #temp_expr : #temp_ident ,
                }
            })
            .collect();

        let mut functions_struct_one: Vec<TokenStream> = (1..=self.number_roles_one)
            .map(|i| {
                let temp_sessions: Vec<TokenStream> = (1..self.number_roles_one)
                    .map(|j| {
                        let (k, _, m) = get_tuple_matrix(&matrix_w_offset_one, i, j);
                        let temp_ident =
                            Ident::new(&format!("S{m}"), Span::call_site());
                        if k == i {
                            quote! {
                                #temp_ident ,
                            }
                        } else {
                            quote! {
                                < #temp_ident as mpstthree::binary::struct_trait::session::Session>::Dual ,
                            }
                        }
                    })
                    .collect();

                    let temp_role = Ident::new(&format!("R{i}"), Span::call_site());
                    let temp_name = Ident::new(&format!("N{i}"), Span::call_site());

                if i != self.index_tuple_one {
                    let offset = if i < self.index_tuple_one { i } else { i - 1 };
                    let temp_function =
                        Ident::new(&format!("F{offset}"), Span::call_site());
                    quote! {
                        #temp_function : FnOnce(
                            #copy_meshedchannels_name_one<
                                #(
                                    #temp_sessions
                                )*
                                #temp_role ,
                                #temp_name
                            >
                        ) -> Result<(), Box<dyn std::error::Error>>
                        + std::marker::Send
                        + 'static,
                    }
                } else {
                    quote! {
                        #copy_meshedchannels_name_one<
                            #(
                                #temp_sessions
                            )*
                            #temp_role ,
                            #temp_name
                        >
                    }
                }
            })
            .collect();

        let mut functions_struct_two: Vec<TokenStream> = (1..=self.number_roles_two)
            .map(|i| {
                let temp_sessions: Vec<TokenStream> = (1..self.number_roles_two)
                    .map(|j| {
                        let (k, _, m) = get_tuple_matrix(&matrix_w_offset_two, i, j);
                        let temp_ident =
                            Ident::new(&format!("S{}", ((self.number_roles_one - 1) * (self.number_roles_one) / 2) + m), Span::call_site());
                        if k == i {
                            quote! {
                                #temp_ident ,
                            }
                        } else {
                            quote! {
                                < #temp_ident as mpstthree::binary::struct_trait::session::Session>::Dual ,
                            }
                        }
                    })
                    .collect();

                    let temp_role = Ident::new(&format!("R{}", self.number_roles_one + i), Span::call_site());
                    let temp_name = Ident::new(&format!("N{}", self.number_roles_one + i), Span::call_site());

                if i != self.index_tuple_two {
                    let offset = if i < self.index_tuple_two { i - 1} else { i - 2 };
                    let temp_function =
                        Ident::new(&format!("F{}", self.number_roles_one + offset), Span::call_site());
                   quote! {
                        #temp_function : FnOnce(
                            #copy_meshedchannels_name_two<
                                #(
                                    #temp_sessions
                                )*
                                #temp_role ,
                                #temp_name
                            >
                        ) -> Result<(), Box<dyn std::error::Error>>
                        + std::marker::Send
                        + 'static,
                    }
                } else {
                    quote! {
                        #copy_meshedchannels_name_two<
                            #(
                                #temp_sessions
                            )*
                            #temp_role ,
                            #temp_name
                        >
                    }
                }
            })
            .collect();

        let function_interleaved_struct = {
            let temp_meshedchannels_name_one =
                functions_struct_one.remove(usize::try_from(self.index_tuple_one - 1).unwrap());
            let temp_meshedchannels_name_two =
                functions_struct_two.remove(usize::try_from(self.index_tuple_two - 1).unwrap());

            quote! {
                FInterleaved : FnOnce(
                    #temp_meshedchannels_name_one ,
                    #temp_meshedchannels_name_two ,
                ) -> Result<(), Box<dyn std::error::Error>>
                + std::marker::Send
                + 'static,
            }
        };

        let join_handle: Vec<TokenStream> = (1..=(sum_nsessions - 1))
            .map(|_| {
                quote! {
                    std::thread::JoinHandle<()> ,
                }
            })
            .collect();

        let new_channels_one: Vec<TokenStream> = (1..=((self.number_roles_one - 1)
            * (self.number_roles_one)
            / 2))
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{i}"), Span::call_site());
                let (line, column, _) = get_tuple_diag(&diag_w_offset_one, i);
                let temp_channel_left =
                    Ident::new(&format!("channel_{line}_{column}"), Span::call_site());
                let temp_channel_right =
                    Ident::new(&format!("channel_{column}_{line}"), Span::call_site());
                quote! {
                    let ( #temp_channel_left , #temp_channel_right ) =
                        < #temp_ident as mpstthree::binary::struct_trait::session::Session>::new();
                }
            })
            .collect();

        let new_channels_two: Vec<TokenStream> = (1..=((self.number_roles_two - 1)
            * (self.number_roles_two)
            / 2))
            .map(|i| {
                let temp_ident = Ident::new(
                    &format!(
                        "S{}",
                        ((self.number_roles_one - 1) * (self.number_roles_one) / 2) + i
                    ),
                    Span::call_site(),
                );
                let (line, column, _) = get_tuple_diag(&diag_w_offset_two, i);
                let temp_channel_left = Ident::new(
                    &format!(
                        "channel_{}_{}",
                        self.number_roles_one + line,
                        self.number_roles_one + column
                    ),
                    Span::call_site(),
                );
                let temp_channel_right = Ident::new(
                    &format!(
                        "channel_{}_{}",
                        self.number_roles_one + column,
                        self.number_roles_one + line
                    ),
                    Span::call_site(),
                );
                quote! {
                    let ( #temp_channel_left , #temp_channel_right ) =
                        < #temp_ident as mpstthree::binary::struct_trait::session::Session>::new();
                }
            })
            .collect();

        let new_meshedchannels_one: Vec<TokenStream> = (1..=(self.number_roles_one))
            .map(|i| {
                let temp_sessions: Vec<TokenStream> = (1..self.number_roles_one)
                    .map(|j| {
                        let (line, column, _) = get_tuple_matrix(&matrix_one, i, j);
                        let temp_session = Ident::new(&format!("session{j}"), Span::call_site());
                        let temp_channel = match line {
                            m if m == i => {
                                Ident::new(&format!("channel_{line}_{column}"), Span::call_site())
                            }
                            _ => Ident::new(&format!("channel_{column}_{line}"), Span::call_site()),
                        };
                        quote! {
                            #temp_session : #temp_channel ,
                        }
                    })
                    .collect();

                let temp_meshedchannels =
                    Ident::new(&format!("meshedchannels_{i}"), Span::call_site());
                let temp_role = Ident::new(&format!("role_{i}"), Span::call_site());
                let temp_name = Ident::new(&format!("name_{i}"), Span::call_site());
                quote! {
                    let #temp_meshedchannels =
                        #copy_meshedchannels_name_one {
                            #(
                                #temp_sessions
                            )*
                            stack: #temp_role ,
                            name: #temp_name ,
                        };
                }
            })
            .collect();

        let new_meshedchannels_two: Vec<TokenStream> = (1..=(self.number_roles_two))
            .map(|i| {
                let temp_sessions: Vec<TokenStream> = (1..self.number_roles_two)
                    .map(|j| {
                        let (line, column, _) = get_tuple_matrix(&matrix_two, i, j);
                        let temp_session = Ident::new(&format!("session{j}"), Span::call_site());
                        let temp_channel = match line {
                            m if m == i => Ident::new(
                                &format!(
                                    "channel_{}_{}",
                                    self.number_roles_one + line,
                                    self.number_roles_one + column
                                ),
                                Span::call_site(),
                            ),
                            _ => Ident::new(
                                &format!(
                                    "channel_{}_{}",
                                    self.number_roles_one + column,
                                    self.number_roles_one + line
                                ),
                                Span::call_site(),
                            ),
                        };
                        quote! {
                            #temp_session : #temp_channel ,
                        }
                    })
                    .collect();

                let temp_meshedchannels = Ident::new(
                    &format!("meshedchannels_{}", self.number_roles_one + i),
                    Span::call_site(),
                );
                let temp_role = Ident::new(
                    &format!("role_{}", self.number_roles_one + i),
                    Span::call_site(),
                );
                let temp_name = Ident::new(
                    &format!("name_{}", self.number_roles_one + i),
                    Span::call_site(),
                );
                quote! {
                    let #temp_meshedchannels =
                        #copy_meshedchannels_name_two {
                            #(
                                #temp_sessions
                            )*
                            stack: #temp_role ,
                            name: #temp_name ,
                        };
                }
            })
            .collect();

        let new_threads: Vec<TokenStream> = (1..=(sum_nsessions - 2))
            .map(|i| {
                let offset = if i < self.index_tuple_one {
                    i
                } else if i >= self.index_tuple_one
                    && i < self.number_roles_one - 1 + self.index_tuple_two
                {
                    i + 1
                } else {
                    i + 2
                };
                let temp_function = Ident::new(&format!("f{i}"), Span::call_site());
                let temp_meshedchannels =
                    Ident::new(&format!("meshedchannels_{offset}"), Span::call_site());
                quote! {
                    std::thread::Builder::new()
                    .name(String::from(stringify!(#temp_function)))
                    .stack_size(64 * 1024 * 1024)
                    .spawn(move || {
                        std::panic::set_hook(Box::new(|_info| {
                            // do nothing
                        }));
                        match #temp_function(#temp_meshedchannels) {
                            Ok(()) => (),
                            Err(e) => panic!("{:?}", e),
                        }
                    }).unwrap(),
                }
            })
            .collect();

        let new_interleaved_thread: TokenStream = {
            let temp_meshedchannels_one = Ident::new(
                &format!("meshedchannels_{}", self.index_tuple_one),
                Span::call_site(),
            );
            let temp_meshedchannels_two = Ident::new(
                &format!(
                    "meshedchannels_{}",
                    (self.number_roles_one + self.index_tuple_two)
                ),
                Span::call_site(),
            );

            quote! {
                std::thread::Builder::new()
                .name(String::from("Interleaved thread"))
                .stack_size(64 * 1024 * 1024)
                .spawn(move || {
                    std::panic::set_hook(Box::new(|_info| {
                        // do nothing
                    }));
                    match f_interleaved(
                        #temp_meshedchannels_one,
                        #temp_meshedchannels_two
                    ) {
                        Ok(()) => (),
                        Err(e) => panic!("{:?}", e),
                    }
                }).unwrap(),
            }
        };

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

            // Create the new fork function
            fn #copy_func_name<
                #(
                    #sessions
                )*
                #(
                    #roles
                )*
                #(
                    #names
                )*
                #(
                    #functions
                )*
                FInterleaved,
            >(
                #(
                    #functions_input_one
                )*
                #(
                    #functions_input_two
                )*
                f_interleaved: FInterleaved,
            ) -> (
                #(
                    #join_handle
                )*
            )
            where
                #(
                    #roles_struct
                )*
                #(
                    #names_struct
                )*
                #(
                    #sessions_struct
                )*
                #(
                    #functions_struct_one
                )*
                #(
                    #functions_struct_two
                )*
                #function_interleaved_struct
            {
                #(
                    #new_channels_one
                )*
                #(
                    #new_channels_two
                )*

                #(
                    #new_roles
                )*

                #(
                    #new_names
                )*

                #(
                    #new_meshedchannels_one
                )*
                #(
                    #new_meshedchannels_two
                )*

                (
                    #(
                        #new_threads
                    )*
                    #new_interleaved_thread
                )
            }
        }
    }
}
