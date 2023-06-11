use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::convert::TryFrom;
use syn::Ident;

use crate::common_functions::expand::session::{
    create_function_details, create_functions, create_join_handle, create_name_structs,
    create_names, create_new_channels, create_new_meshedchannels, create_new_names,
    create_new_roles, create_role_structs, create_roles, create_session, create_session_structs,
};
use crate::common_functions::maths::{
    diag_and_matrix, diag_and_matrix_w_offset, get_tuple_diag, get_tuple_matrix,
};

/// Expand fork methods
pub(crate) fn fork_mpst(meshedchannels_name: &Ident, number_roles: u64) -> TokenStream {
    let (matrix, _diag) = diag_and_matrix(number_roles);
    let (matrix_w_offset, diag_w_offset) = diag_and_matrix_w_offset(number_roles);

    let sessions = create_session(1, (number_roles - 1) * (number_roles) / 2);

    let session_structs = create_session_structs(1, (number_roles - 1) * (number_roles) / 2);

    let roles = create_roles(1, number_roles);

    let role_structs = create_role_structs(1, number_roles);

    let new_roles = create_new_roles(1, number_roles);

    let names = create_names(1, number_roles);

    let name_structs = create_name_structs(1, number_roles);

    let new_names = create_new_names(1, number_roles);

    let functions = create_functions(1, number_roles);

    let functions_details = create_function_details(1, number_roles);

    let functions_struct: Vec<TokenStream> = (1..=number_roles)
            .map(|i| {
                let temp_sessions: Vec<TokenStream> = (1..number_roles)
                    .map(|j| {
                        let (k, _, m) = get_tuple_matrix(&matrix_w_offset, i, j);
                        let temp_ident =
                            Ident::new(&format!("S{m}"), Span::call_site());
                        if k == i {
                            quote! {
                                #temp_ident ,
                            }
                        } else {
                            quote! {
                                < #temp_ident  as mpstthree::binary::struct_trait::session::Session >::Dual ,
                            }
                        }
                    })
                    .collect();

                let temp_function =
                    Ident::new(&format!("F{i}"), Span::call_site());
                let temp_role = Ident::new(&format!("R{i}"), Span::call_site());
                let temp_name = Ident::new(&format!("N{i}"), Span::call_site());
                quote! {
                    #temp_function : FnOnce(
                        #meshedchannels_name<
                            #(
                                #temp_sessions
                            )*
                            #temp_role ,
                            #temp_name
                        >
                    ) -> std::result::Result<(), Box<dyn std::error::Error>>
                    + std::marker::Send
                    + 'static,
                }
            })
            .collect();

    let join_handle = create_join_handle(1, number_roles);

    let new_channels =
        create_new_channels(1, (number_roles - 1) * (number_roles) / 2, &diag_w_offset);

    let new_meshedchannels =
        create_new_meshedchannels(1, number_roles, meshedchannels_name, &matrix);

    let new_threads: Vec<TokenStream> = (1..=number_roles)
        .map(|i| {
            let temp_function = Ident::new(&format!("f{i}"), Span::call_site());
            let temp_meshedchannels = Ident::new(&format!("meshedchannels_{i}"), Span::call_site());
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

    quote! {
        fn fork_mpst<
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
        >(
            #(
                #functions_details
            )*
        ) -> (
            #(
                #join_handle
            )*
        )
        where
            #(
                #role_structs
            )*
            #(
                #name_structs
            )*
            #(
                #session_structs
            )*
            #(
                #functions_struct
            )*
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

            #(
                #new_meshedchannels
            )*

            (
                #(
                    #new_threads
                )*
            )
        }
    }
}

/// Expand fork timed methods
pub(crate) fn fork_timed_mpst(meshedchannels_name: &Ident, number_roles: u64) -> TokenStream {
    let (matrix, _diag) = diag_and_matrix(number_roles);
    let (matrix_w_offset, diag_w_offset) = diag_and_matrix_w_offset(number_roles);

    let sessions = create_session(1, (number_roles - 1) * (number_roles) / 2);

    let session_structs = create_session_structs(1, (number_roles - 1) * (number_roles) / 2);

    let roles = create_roles(1, number_roles);

    let role_structs = create_role_structs(1, number_roles);

    let new_roles = create_new_roles(1, number_roles);

    let names = create_names(1, number_roles);

    let name_structs = create_name_structs(1, number_roles);

    let new_names = create_new_names(1, number_roles);

    let functions = create_functions(1, number_roles);

    let functions_details = create_function_details(1, number_roles);

    let functions_struct: Vec<TokenStream> = (1..=number_roles)
            .map(|i| {
                let temp_sessions: Vec<TokenStream> = (1..number_roles)
                    .map(|j| {
                        let (k, _, m) = get_tuple_matrix(&matrix_w_offset, i, j);
                        let temp_ident =
                            Ident::new(&format!("S{m}"), Span::call_site());
                        if k == i {
                            quote! {
                                #temp_ident ,
                            }
                        } else {
                            quote! {
                                < #temp_ident  as mpstthree::binary::struct_trait::session::Session >::Dual ,
                            }
                        }
                    })
                    .collect();

                let temp_function =
                    Ident::new(&format!("F{i}"), Span::call_site());
                let temp_role = Ident::new(&format!("R{i}"), Span::call_site());
                let temp_name = Ident::new(&format!("N{i}"), Span::call_site());
                quote! {
                    #temp_function : FnOnce(
                        #meshedchannels_name<
                            #(
                                #temp_sessions
                            )*
                            #temp_role ,
                            #temp_name
                        >,
                        &mut std::collections::HashMap<char, std::time::Instant>
                    ) -> std::result::Result<(), Box<dyn std::error::Error>>
                    + std::marker::Send
                    + 'static,
                }
            })
            .collect();

    let join_handle = create_join_handle(1, number_roles);

    let new_channels =
        create_new_channels(1, (number_roles - 1) * (number_roles) / 2, &diag_w_offset);

    let new_meshedchannels =
        create_new_meshedchannels(1, number_roles, meshedchannels_name, &matrix);

    let new_threads: Vec<TokenStream> = (1..=number_roles)
        .map(|i| {
            let temp_function = Ident::new(&format!("f{i}"), Span::call_site());
            let temp_meshedchannels = Ident::new(&format!("meshedchannels_{i}"), Span::call_site());
            quote! {
                std::thread::Builder::new()
                .name(String::from(stringify!(#temp_function)))
                .stack_size(64 * 1024 * 1024)
                .spawn(move || {
                    std::panic::set_hook(Box::new(|_info| {
                        // do nothing
                    }));
                    match #temp_function(
                        #temp_meshedchannels,
                        &mut std::collections::HashMap::<char, std::time::Instant>::new()
                    ) {
                        Ok(()) => (),
                        Err(e) => panic!("{:?}", e),
                    }
                }).unwrap(),
            }
        })
        .collect();

    quote! {
        fn fork_mpst<
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
        >(
            #(
                #functions_details
            )*
        ) -> (
            #(
                #join_handle
            )*
        )
        where
            #(
                #role_structs
            )*
            #(
                #name_structs
            )*
            #(
                #session_structs
            )*
            #(
                #functions_struct
            )*
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

            #(
                #new_meshedchannels
            )*

            (
                #(
                    #new_threads
                )*
            )
        }
    }
}

/// Expand fork timed methods
pub(crate) fn fork_interleaved_mpst(
    func_name: &Ident,
    meshedchannels_name_one: &Ident,
    number_roles_one: u64,
    index_tuple_one: u64,
    meshedchannels_name_two: &Ident,
    number_roles_two: u64,
    index_tuple_two: u64,
) -> TokenStream {
    let sum_nsessions = number_roles_one + number_roles_two;
    let (matrix_one, _diag_one) = diag_and_matrix(number_roles_one);
    let (matrix_two, _diag_two) = diag_and_matrix(number_roles_two);
    let (matrix_w_offset_one, diag_w_offset_one) = diag_and_matrix_w_offset(number_roles_one);
    let (matrix_w_offset_two, diag_w_offset_two) = diag_and_matrix_w_offset(number_roles_two);

    // Generate
    // S0, S1, S2, ...
    let sessions = create_session(
        1,
        (number_roles_one - 1) * (number_roles_one) / 2
            + (number_roles_two - 1) * (number_roles_two) / 2,
    );

    // Generate
    // S0: Session + `static ,
    // S1: Session + `static ,
    // ... ,
    let session_structs = create_session_structs(
        1,
        (number_roles_one - 1) * (number_roles_one) / 2
            + (number_roles_two - 1) * (number_roles_two) / 2,
    );

    let roles = create_roles(1, sum_nsessions);

    let role_structs = create_role_structs(1, sum_nsessions);

    let new_roles = create_new_roles(1, sum_nsessions);

    let names = create_names(1, sum_nsessions);

    let name_structs = create_name_structs(1, sum_nsessions);

    let new_names = create_new_names(1, sum_nsessions);

    let functions = create_functions(1, sum_nsessions - 2);

    let functions_input_one: Vec<TokenStream> = (1..number_roles_one)
        .map(|i| {
            let temp_ident = Ident::new(&format!("F{i}"), Span::call_site());
            let temp_expr = Ident::new(&format!("f{i}"), Span::call_site());
            quote! {
                #temp_expr : #temp_ident ,
            }
        })
        .collect();

    let functions_input_two: Vec<TokenStream> = (1..number_roles_two)
        .map(|i| {
            let temp_ident =
                Ident::new(&format!("F{}", number_roles_one - 1 + i), Span::call_site());
            let temp_expr =
                Ident::new(&format!("f{}", number_roles_one - 1 + i), Span::call_site());
            quote! {
                #temp_expr : #temp_ident ,
            }
        })
        .collect();

    let mut functions_struct_one: Vec<TokenStream> = (1..=number_roles_one)
        .map(|i| {
            let temp_sessions: Vec<TokenStream> = (1..number_roles_one)
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

            if i != index_tuple_one {
                let offset = if i < index_tuple_one { i } else { i - 1 };
                let temp_function =
                    Ident::new(&format!("F{offset}"), Span::call_site());
                quote! {
                    #temp_function : FnOnce(
                        #meshedchannels_name_one<
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
                    #meshedchannels_name_one<
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

    let mut functions_struct_two: Vec<TokenStream> = (1..=number_roles_two)
        .map(|i| {
            let temp_sessions: Vec<TokenStream> = (1..number_roles_two)
                .map(|j| {
                    let (k, _, m) = get_tuple_matrix(&matrix_w_offset_two, i, j);
                    let temp_ident =
                        Ident::new(&format!("S{}", ((number_roles_one - 1) * (number_roles_one) / 2) + m), Span::call_site());
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

                let temp_role = Ident::new(&format!("R{}", number_roles_one + i), Span::call_site());
                let temp_name = Ident::new(&format!("N{}", number_roles_one + i), Span::call_site());

            if i != index_tuple_two {
                let offset = if i < index_tuple_two { i - 1} else { i - 2 };
                let temp_function =
                    Ident::new(&format!("F{}", number_roles_one + offset), Span::call_site());
               quote! {
                    #temp_function : FnOnce(
                        #meshedchannels_name_two<
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
                    #meshedchannels_name_two<
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
            functions_struct_one.remove(usize::try_from(index_tuple_one - 1).unwrap());
        let temp_meshedchannels_name_two =
            functions_struct_two.remove(usize::try_from(index_tuple_two - 1).unwrap());

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

    let new_channels_one: Vec<TokenStream> = (1..=((number_roles_one - 1) * (number_roles_one)
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

    let new_channels_two: Vec<TokenStream> = (1..=((number_roles_two - 1) * (number_roles_two)
        / 2))
        .map(|i| {
            let temp_ident = Ident::new(
                &format!("S{}", ((number_roles_one - 1) * (number_roles_one) / 2) + i),
                Span::call_site(),
            );
            let (line, column, _) = get_tuple_diag(&diag_w_offset_two, i);
            let temp_channel_left = Ident::new(
                &format!(
                    "channel_{}_{}",
                    number_roles_one + line,
                    number_roles_one + column
                ),
                Span::call_site(),
            );
            let temp_channel_right = Ident::new(
                &format!(
                    "channel_{}_{}",
                    number_roles_one + column,
                    number_roles_one + line
                ),
                Span::call_site(),
            );
            quote! {
                let ( #temp_channel_left , #temp_channel_right ) =
                    < #temp_ident as mpstthree::binary::struct_trait::session::Session>::new();
            }
        })
        .collect();

    let new_meshedchannels_one: Vec<TokenStream> = (1..=(number_roles_one))
        .map(|i| {
            let temp_sessions: Vec<TokenStream> = (1..number_roles_one)
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

            let temp_meshedchannels = Ident::new(&format!("meshedchannels_{i}"), Span::call_site());
            let temp_role = Ident::new(&format!("role_{i}"), Span::call_site());
            let temp_name = Ident::new(&format!("name_{i}"), Span::call_site());
            quote! {
                let #temp_meshedchannels =
                    #meshedchannels_name_one {
                        #(
                            #temp_sessions
                        )*
                        stack: #temp_role ,
                        name: #temp_name ,
                    };
            }
        })
        .collect();

    let new_meshedchannels_two: Vec<TokenStream> = (1..=(number_roles_two))
        .map(|i| {
            let temp_sessions: Vec<TokenStream> = (1..number_roles_two)
                .map(|j| {
                    let (line, column, _) = get_tuple_matrix(&matrix_two, i, j);
                    let temp_session = Ident::new(&format!("session{j}"), Span::call_site());
                    let temp_channel = match line {
                        m if m == i => Ident::new(
                            &format!(
                                "channel_{}_{}",
                                number_roles_one + line,
                                number_roles_one + column
                            ),
                            Span::call_site(),
                        ),
                        _ => Ident::new(
                            &format!(
                                "channel_{}_{}",
                                number_roles_one + column,
                                number_roles_one + line
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
                &format!("meshedchannels_{}", number_roles_one + i),
                Span::call_site(),
            );
            let temp_role =
                Ident::new(&format!("role_{}", number_roles_one + i), Span::call_site());
            let temp_name =
                Ident::new(&format!("name_{}", number_roles_one + i), Span::call_site());
            quote! {
                let #temp_meshedchannels =
                    #meshedchannels_name_two {
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
            let offset = if i < index_tuple_one {
                i
            } else if i >= index_tuple_one && i < number_roles_one - 1 + index_tuple_two {
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
            &format!("meshedchannels_{}", index_tuple_one),
            Span::call_site(),
        );
        let temp_meshedchannels_two = Ident::new(
            &format!("meshedchannels_{}", (number_roles_one + index_tuple_two)),
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
    // Create the new fork function
               fn #func_name<
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
                       #role_structs
                   )*
                   #(
                       #name_structs
                   )*
                   #(
                       #session_structs
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
