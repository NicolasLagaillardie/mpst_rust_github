use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::convert::TryFrom;
use syn::Ident;

use crate::common_functions::maths::{
    diag_and_matrix, get_line_column_from_diag, get_tuple_matrix,
};

/// Expand choose methods
pub(crate) fn choose(
    all_roles: Vec<TokenStream>,
    sender: u64,
    meshedchannels_name: Ident,
    number_roles: u64,
) -> TokenStream {
    let (diag, matrix) = diag_and_matrix(number_roles);

    let sender_ident = if let Some(elt) = all_roles.get(usize::try_from(sender - 1).unwrap()) {
        Ident::new(&format!("Name{}", elt), Span::call_site())
    } else {
        panic!("Not enough arguments for sender_ident in expand_choose")
    };

    let sender_stack = if let Some(elt) = all_roles.get(usize::try_from(sender - 1).unwrap()) {
        Ident::new(&format!("Role{}toAll", elt), Span::call_site())
    } else {
        panic!("Not enough arguments for sender_stack in expand_choose")
    };

    let choose_session_types_struct: Vec<TokenStream> = (1..=((number_roles - 1) * number_roles))
        .map(|i| {
            let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
            quote! { #temp_ident : mpstthree::binary::struct_trait::session::Session , }
        })
        .collect();

    let choose_roles_struct: Vec<TokenStream> = (1..=(2 * number_roles))
        .map(|i| {
            let temp_ident = Ident::new(&format!("R{}", i), Span::call_site());
            quote! { #temp_ident : mpstthree::role::Role , }
        })
        .collect();

    let choose_sessions: Vec<TokenStream> = (1..=number_roles)
            .map(|j| {
                if sender != j {
                    let left_sessions: Vec<TokenStream> = (1..number_roles)
                        .map(|k| {

                            let (l, _, _) = get_tuple_matrix(&matrix, j, k);

                            if l == 0 {
                                panic!("Erratum choose_sessions / left_sessions j = {:?}", j)
                            };

                            let (_, _, m1) = if j > sender {
                                get_tuple_matrix(&matrix, sender, j - 1)
                            } else {
                                get_tuple_matrix(&matrix, sender, j)
                            };
                            let (_, _, m2) = get_tuple_matrix(&matrix, j, k);

                            let (_, _, m) = get_tuple_matrix(&matrix, j, k);

                            let temp_ident = Ident::new(
                                &format!("S{}", m),
                                Span::call_site(),
                            );

                            if l == j || m1 == m2 {
                                quote! { #temp_ident , }
                            } else {
                                quote! { <#temp_ident as mpstthree::binary::struct_trait::session::Session>::Dual , }
                            }
                        })
                        .collect();

                    let right_sessions: Vec<TokenStream> = (1..number_roles)
                        .map(|k| {

                            let (l, _, _) = get_tuple_matrix(&matrix, j, k);

                            if l == 0 {
                                panic!("Erratum choose_sessions / right_sessions j = {:?}", j)
                            };

                            let (_, _, m1) = if j > sender {
                                get_tuple_matrix(&matrix, sender, j - 1)
                            } else {
                                get_tuple_matrix(&matrix, sender, j)
                            };
                            let (_, _, m2) = get_tuple_matrix(&matrix, j, k);

                            let (_, _, m) = get_tuple_matrix(&matrix, j, k);

                            let diff = number_roles - 1;

                            let temp_ident = Ident::new(
                                &format!("S{}", diff * (diff + 1) / 2 + m),
                                Span::call_site(),
                            );

                            if l == j || m1 == m2 {
                                quote! { #temp_ident , }
                            } else {
                                quote! { <#temp_ident as mpstthree::binary::struct_trait::session::Session>::Dual , }
                            }
                        })
                        .collect();

                    let stack_left = if j > sender {
                        let temp_ident = Ident::new(
                            &format!("R{}", 2 * (j - 1) - 1),
                            Span::call_site(),
                        );
                        quote! { #temp_ident , }
                    } else {
                        let temp_ident = Ident::new(
                            &format!("R{}", 2 * (j - 1) + 1),
                            Span::call_site(),
                        );
                        quote! { #temp_ident , }
                    };

                    let stack_right = if j > sender {
                        let temp_ident = Ident::new(
                            &format!("R{}", 2 * (j - 1)),
                            Span::call_site(),
                        );
                        quote! { #temp_ident , }
                    } else {
                        let temp_ident = Ident::new(
                            &format!("R{}", 2 * (j - 1) + 2),
                            Span::call_site(),
                        );
                        quote! { #temp_ident , }
                    };

                    let receiver_ident =
                        if let Some(elt) = all_roles.get(usize::try_from(j - 1).unwrap()) {
                            Ident::new(&format!("Name{}", elt), Span::call_site())
                        } else {
                            panic!("Not enough arguments for receiver_ident in choose_sessions in expand_choose")
                        };

                    quote! {
                        mpstthree::binary::struct_trait::send::Send<
                            either::Either<
                                #meshedchannels_name<
                                    #(
                                        #left_sessions
                                    )*
                                    #stack_left
                                    #receiver_ident
                                >,
                                #meshedchannels_name<
                                    #(
                                        #right_sessions
                                    )*
                                    #stack_right
                                    #receiver_ident
                                >
                            >,
                            mpstthree::binary::struct_trait::end::End,
                        >,
                    }
                } else {
                    quote! {
                        // Empty
                    }
                }
            })
            .collect();

    let new_stack_sender_left =
        Ident::new(&format!("R{}", 2 * number_roles - 1), Span::call_site());
    let new_stack_sender_right = Ident::new(&format!("R{}", 2 * number_roles), Span::call_site());
    let new_stacks_sender = quote! { #new_stack_sender_left , #new_stack_sender_right };

    let choose_left_session: Vec<TokenStream> = (1..=number_roles)
            .filter_map(|j| {
                if j == sender {
                    None
                } else {
                    let (_, _, m) = if j > sender {
                        get_tuple_matrix(&matrix, sender, j - 1)
                    } else {
                        get_tuple_matrix(&matrix, sender, j)
                    };
                    let temp_ident =
                        Ident::new(&format!("S{}", m), Span::call_site());
                    Some(
                        quote! { <#temp_ident as mpstthree::binary::struct_trait::session::Session>::Dual, },
                    )
                }
            })
            .collect();

    let choose_right_session: Vec<TokenStream> = (1..=number_roles)
            .filter_map(|j| {
                if j == sender {
                    None
                } else {
                    let (_, _, m) = if j > sender {
                        get_tuple_matrix(&matrix, sender, j - 1)
                    } else {
                        get_tuple_matrix(&matrix, sender, j)
                    };
                    let diff = number_roles - 1;
                    let temp_ident = Ident::new(
                        &format!("S{}", diff * (diff + 1) / 2 + m),
                        Span::call_site(),
                    );
                    Some(
                        quote! { <#temp_ident as mpstthree::binary::struct_trait::session::Session>::Dual, },
                    )
                }
            })
            .collect();
    let choose_left_channels: Vec<TokenStream> = (1..=((number_roles - 1) * number_roles / 2))
        .map(|j| {
            let (line, column) = get_line_column_from_diag(&diag, j);

            let first_channel = if sender != line {
                Ident::new(&format!("channel_{}_{}", line, column), Span::call_site())
            } else {
                Ident::new(&format!("channel_{}_{}", column, line), Span::call_site())
            };

            let second_channel = if sender != line {
                Ident::new(&format!("channel_{}_{}", column, line), Span::call_site())
            } else {
                Ident::new(&format!("channel_{}_{}", line, column), Span::call_site())
            };

            let temp_session = Ident::new(&format!("S{}", j), Span::call_site());

            quote! { let ( #first_channel , #second_channel ) =
            <#temp_session as mpstthree::binary::struct_trait::session::Session>::new() ; }
        })
        .collect();

    let choose_right_channels: Vec<TokenStream> = (1..=((number_roles - 1) * number_roles / 2))
        .map(|j| {
            let (line, column) = get_line_column_from_diag(&diag, j);
            let diff = number_roles - 1;

            let first_channel = if sender != line {
                Ident::new(&format!("channel_{}_{}", line, column), Span::call_site())
            } else {
                Ident::new(&format!("channel_{}_{}", column, line), Span::call_site())
            };

            let second_channel = if sender != line {
                Ident::new(&format!("channel_{}_{}", column, line), Span::call_site())
            } else {
                Ident::new(&format!("channel_{}_{}", line, column), Span::call_site())
            };

            let temp_session = Ident::new(
                &format!("S{}", diff * (diff + 1) / 2 + j),
                Span::call_site(),
            );

            quote! { let ( #first_channel , #second_channel ) = < #temp_session as mpstthree::binary::struct_trait::session::Session >::new() ; }
        })
        .collect();

    let new_stacks_receivers_left: Vec<TokenStream> = (1..number_roles)
        .map(|j| {
            let temp_stack = Ident::new(&format!("stack_{}", j), Span::call_site());
            let temp_role = Ident::new(&format!("R{}", 2 * (j - 1) + 1), Span::call_site());
            quote! { let (#temp_stack, _) = <#temp_role as mpstthree::role::Role>::new(); }
        })
        .collect();

    let new_stacks_receivers_right: Vec<TokenStream> = (1..number_roles)
        .map(|j| {
            let temp_stack = Ident::new(&format!("stack_{}", j), Span::call_site());
            let temp_role = Ident::new(&format!("R{}", 2 * (j - 1) + 2), Span::call_site());
            quote! { let (#temp_stack, _) = <#temp_role as mpstthree::role::Role>::new(); }
        })
        .collect();

    let new_names: Vec<TokenStream> = (1..=number_roles)
        .map(|j| {
            if sender != j {
                let receiver_ident = if let Some(elt) =
                    all_roles.get(usize::try_from(j - 1).unwrap())
                {
                    Ident::new(&format!("Name{}", elt), Span::call_site())
                } else {
                    panic!("Not enough arguments for receiver_ident in new_names in expand_choose")
                };

                let new_name = if let Some(elt) = all_roles.get(usize::try_from(j - 1).unwrap()) {
                    Ident::new(&format!("name_{}", elt), Span::call_site())
                } else {
                    panic!("Not enough arguments for new_name in new_names in expand_choose")
                };

                quote! {
                    let (#new_name, _) = < #receiver_ident as mpstthree::name::Name >::new();
                }
            } else {
                quote! {}
            }
        })
        .collect();

    let new_meshedchannels_receivers: Vec<TokenStream> = (1..=number_roles)
            .map(|j| {
                if sender != j {
                    let new_sessions_receiver: Vec<TokenStream> = (1..number_roles)
                        .map(|k| {
                            let new_session_receiver = Ident::new(
                                &format!("session{}", k),
                                Span::call_site(),
                            );
                            let new_channel_receiver = if j > k {
                                Ident::new(
                                    &format!("channel_{}_{}", j, k),
                                    Span::call_site(),
                                )
                            } else {
                                Ident::new(
                                    &format!("channel_{}_{}", j, k + 1),
                                    Span::call_site(),
                                )
                            };

                            quote! { #new_session_receiver : #new_channel_receiver , }
                        })
                        .collect();

                    let new_choice_receiver = if j > sender
                    {
                        Ident::new(&format!("choice_{}", j - 1), Span::call_site())
                    } else {
                        Ident::new(&format!("choice_{}", j), Span::call_site())
                    };

                    let new_stack_receiver = if j > sender
                    {
                        Ident::new(&format!("stack_{}", j - 1), Span::call_site())
                    } else {
                        Ident::new(&format!("stack_{}", j), Span::call_site())
                    };

                    let new_name_receiver = if let Some(elt) =
                        all_roles.get(usize::try_from(j - 1).unwrap())
                    {
                        Ident::new(&format!("name_{}", elt), Span::call_site())
                    } else {
                        panic!("Not enough arguments for new_name_receiver in new_meshedchannels_receivers in expand_choose")
                    };

                    quote! {
                        let #new_choice_receiver = #meshedchannels_name {
                            #(
                                #new_sessions_receiver
                            )*
                            stack: #new_stack_receiver,
                            name: #new_name_receiver,
                        };
                    }
                } else {
                    quote! {
                        // Empty
                    }
                }
            })
            .collect();

    let new_sessions_sender_left: Vec<TokenStream> = (1..number_roles)
        .map(|j| {
            let new_session_sender =
                Ident::new(&format!("new_session_{}", j - 1), Span::call_site());

            let new_choice_sender = Ident::new(&format!("choice_{}", j), Span::call_site());

            let session_sender = Ident::new(&format!("session{}", j), Span::call_site());

            quote! {
                let #new_session_sender = mpstthree::binary::send::send(
                    either::Either::Left(#new_choice_sender),
                    self.#session_sender
                );
            }
        })
        .collect();

    let new_sessions_sender_right: Vec<TokenStream> = (1..number_roles)
        .map(|j| {
            let new_session_sender =
                Ident::new(&format!("new_session_{}", j - 1), Span::call_site());

            let new_choice_sender = Ident::new(&format!("choice_{}", j), Span::call_site());

            let session_sender = Ident::new(&format!("session{}", j), Span::call_site());

            quote! {
                let #new_session_sender = mpstthree::binary::send::send(
                    either::Either::Right(#new_choice_sender),
                    self.#session_sender
                );
            }
        })
        .collect();

    let old_meshedchannels_sender: Vec<TokenStream> = (1..number_roles)
        .map(|j| {
            let new_session_sender =
                Ident::new(&format!("new_session_{}", j - 1), Span::call_site());

            let session_sender = Ident::new(&format!("session{}", j), Span::call_site());

            quote! {
                #session_sender : #new_session_sender ,
            }
        })
        .collect();

    let new_meshedchannels_sender: Vec<TokenStream> = (1..=number_roles)
        .map(|j| {
            if sender != j {
                let new_choice_sender = if j < sender {
                    Ident::new(&format!("session{}", j), Span::call_site())
                } else {
                    Ident::new(&format!("session{}", j - 1), Span::call_site())
                };

                let new_channel_sender =
                    Ident::new(&format!("channel_{}_{}", sender, j), Span::call_site());

                quote! {
                    #new_choice_sender : #new_channel_sender,
                }
            } else {
                quote! {
                    // Empty
                }
            }
        })
        .collect();

    let new_stack_sender = Ident::new(&format!("stack_{}", number_roles), Span::call_site());

    let new_name_sender = Ident::new(&format!("name_{}", number_roles), Span::call_site());

    quote! {
        impl<
            #(
                #choose_session_types_struct
            )*
            #(
                #choose_roles_struct
            )*
        >
            #meshedchannels_name<
                #(
                    #choose_sessions
                )*
                #sender_stack<
                    #new_stacks_sender
                >,
                #sender_ident,
            >
        {
            pub fn choose_left(self) -> #meshedchannels_name<
                #(
                    #choose_left_session
                )*
                #new_stack_sender_left ,
                #sender_ident
            >
            {
                #(
                    #choose_left_channels
                )*

                #(
                    #new_stacks_receivers_left
                )*

                let (#new_stack_sender, _) = <#new_stack_sender_left as mpstthree::role::Role>::new();

                #(
                    #new_names
                )*

                let (#new_name_sender, _) = < #sender_ident as mpstthree::name::Name >::new();

                #(
                    #new_meshedchannels_receivers
                )*

                #(
                    #new_sessions_sender_left
                )*

                let s = #meshedchannels_name {
                    #(
                        #old_meshedchannels_sender
                    )*
                    stack: self.stack,
                    name: self.name,
                };

                mpstthree::binary::cancel::cancel(s);

                #meshedchannels_name {
                    #(
                        #new_meshedchannels_sender
                    )*
                    stack: #new_stack_sender,
                    name: #new_name_sender,
                }
            }
            pub fn choose_right(self) -> #meshedchannels_name<
                #(
                    #choose_right_session
                )*
                #new_stack_sender_right ,
                #sender_ident
            >
            {
                #(
                    #choose_right_channels
                )*

                #(
                    #new_stacks_receivers_right
                )*

                let (#new_stack_sender, _) = <#new_stack_sender_right as mpstthree::role::Role>::new();

                #(
                    #new_names
                )*

                let (#new_name_sender, _) = < #sender_ident as mpstthree::name::Name >::new();

                #(
                    #new_meshedchannels_receivers
                )*

                #(
                    #new_sessions_sender_right
                )*

                let s = #meshedchannels_name {
                    #(
                        #old_meshedchannels_sender
                    )*
                    stack: self.stack,
                    name: self.name,
                };

                mpstthree::binary::cancel::cancel(s);

                #meshedchannels_name {
                    #(
                        #new_meshedchannels_sender
                    )*
                    stack: #new_stack_sender,
                    name: #new_name_sender,
                }
            }
        }
    }
}

/// Expand choose methods for interleaved sessions
pub(crate) fn choose_mpst_create_multi_to_all(
    meshedchannels_name: Ident,
    all_roles: Vec<TokenStream>,
    number_roles: u64,
) -> TokenStream {
    let choose_mpst_create_multi_to_all: Vec<TokenStream> = (1..=number_roles)
        .map(|sender| {

            let name_macro = if let Some(elt) =
                all_roles.get(usize::try_from(sender - 1).unwrap())
            {
                Ident::new(
                    &format!("choose_mpst_{}_to_all", elt).to_lowercase(),
                    Span::call_site(),
                )
            } else {
                panic!("Not enough arguments for name in expand_choose_mpst_create_multi_to_all")
            };

            let sender_name = if let Some(elt) =
                all_roles.get(usize::try_from(sender - 1).unwrap())
            {
                Ident::new(
                    &format!("Name{}", elt),
                    Span::call_site(),
                )
            } else {
                panic!("Not enough arguments for sender_name in expand_choose_mpst_create_multi_to_all")
            };

            let receivers: Vec<Ident> = (1..=number_roles)
                .filter_map(|receiver| {
                    if sender != receiver {
                        Some(
                            if let Some(elt) =
                                all_roles.get(usize::try_from(receiver - 1).unwrap())
                            {
                                Ident::new(
                                    &format!("Name{}", elt),
                                    Span::call_site(),
                                )
                            } else {
                                panic!("Not enough arguments for receivers in expand_choose_mpst_create_multi_to_all")
                            }
                        )
                    } else {
                        None
                    }
                })
                .collect();

            quote! {
                mpstthree::choose_mpst_create_multi_to_all!(
                    #name_macro ,
                    #( #receivers , )* =>
                    #sender_name ,
                    #meshedchannels_name ,
                    #sender
                );
            }
        })
        .collect();

    quote! {
        #( #choose_mpst_create_multi_to_all )*
    }
}
