use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result};

use crate::common_functions::expand::session::{
    create_function_details, create_functions, create_join_handle, create_name_structs,
    create_names, create_new_channels, create_new_meshedchannels, create_new_names,
    create_new_roles, create_role_structs, create_roles, create_session, create_session_structs,
};
use crate::common_functions::maths::{diag_and_matrix, diag_and_matrix_w_offset, get_tuple_matrix};
use crate::common_functions::parsing::parse_stream_sessions;

#[derive(Debug)]
pub(crate) struct ForkMPSTMulti {
    func_name: Ident,
    meshedchannels_name: Ident,
    n_sessions: u64,
}

impl Parse for ForkMPSTMulti {
    fn parse(input: ParseStream) -> Result<Self> {
        let (func_name, meshedchannels_name, n_sessions) = parse_stream_sessions(input)?;

        Ok(ForkMPSTMulti {
            func_name,
            meshedchannels_name,
            n_sessions,
        })
    }
}

impl From<ForkMPSTMulti> for TokenStream {
    fn from(input: ForkMPSTMulti) -> TokenStream {
        input.expand()
    }
}

impl ForkMPSTMulti {
    fn expand(&self) -> TokenStream {
        let func_name = &self.func_name;
        let meshedchannels_name = &self.meshedchannels_name;
        let (matrix, _diag) = diag_and_matrix(self.n_sessions);
        let (matrix_w_offset, diag_w_offset) = diag_and_matrix_w_offset(self.n_sessions);

        let sessions = create_session(1, (self.n_sessions - 1) * (self.n_sessions) / 2);

        let sessions_struct =
            create_session_structs(1, (self.n_sessions - 1) * (self.n_sessions) / 2);

        let roles = create_roles(1, self.n_sessions);

        let roles_struct = create_role_structs(1, self.n_sessions);

        let new_roles = create_new_roles(1, self.n_sessions);

        let names = create_names(1, self.n_sessions);

        let names_struct = create_name_structs(1, self.n_sessions);

        let new_names = create_new_names(1, self.n_sessions);

        let functions = create_functions(1, self.n_sessions);

        let functions_details = create_function_details(1, self.n_sessions);

        let functions_struct: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_sessions: Vec<TokenStream> = (1..self.n_sessions)
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
                                < #temp_ident  as mpstthree::binary::struct_trait::session::Session>::Dual ,
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
                    ) -> Result<(), Box<dyn std::error::Error>>
                    + std::marker::Send
                    + 'static,
                }
            })
            .collect();

        let join_handle = create_join_handle(1, self.n_sessions);

        let new_channels = create_new_channels(
            1,
            (self.n_sessions - 1) * (self.n_sessions) / 2,
            &diag_w_offset,
        );

        let new_meshedchannels =
            create_new_meshedchannels(1, self.n_sessions, meshedchannels_name, &matrix);

        let new_threads: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_function = Ident::new(&format!("f{i}"), Span::call_site());
                let temp_meshedchannels =
                    Ident::new(&format!("meshedchannels_{i}"), Span::call_site());
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
                    #roles_struct
                )*
                #(
                    #names_struct
                )*
                #(
                    #sessions_struct
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
}
