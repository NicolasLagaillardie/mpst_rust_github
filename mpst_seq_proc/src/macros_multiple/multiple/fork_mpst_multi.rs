use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Token};

use crate::common_functions::maths::{
    diag_and_matrix, diag_and_matrix_w_offset, get_tuple_diag, get_tuple_matrix,
};

#[derive(Debug)]
pub(crate) struct ForkMPSTMulti {
    func_name: Ident,
    meshedchannels_name: Ident,
    n_sessions: u64,
}

impl Parse for ForkMPSTMulti {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let meshedchannels_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let n_sessions = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();

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

        let sessions: Vec<TokenStream> = (1..=((self.n_sessions - 1) * (self.n_sessions) / 2))
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{i}"), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let sessions_struct: Vec<TokenStream> = (1..=((self.n_sessions - 1) * (self.n_sessions)
            / 2))
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{i}"), Span::call_site());
                quote! {
                    #temp_ident : mpstthree::binary::struct_trait::session::Session + 'static ,
                }
            })
            .collect();

        let roles: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("R{i}"), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let roles_struct: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("R{i}"), Span::call_site());
                quote! {
                    #temp_ident : mpstthree::role::Role + 'static ,
                }
            })
            .collect();

        let new_roles: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("R{i}"), Span::call_site());
                let temp_role = Ident::new(&format!("role_{i}"), Span::call_site());
                quote! {
                    let ( #temp_role , _) = < #temp_ident as mpstthree::role::Role >::new() ;
                }
            })
            .collect();

        let names: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("N{i}"), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let names_struct: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("N{i}"), Span::call_site());
                quote! {
                    #temp_ident : mpstthree::name::Name + 'static ,
                }
            })
            .collect();

        let new_names: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("N{i}"), Span::call_site());
                let temp_name = Ident::new(&format!("name_{i}"), Span::call_site());
                quote! {
                    let ( #temp_name , _) = < #temp_ident as mpstthree::name::Name >::new() ;
                }
            })
            .collect();

        let functions: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("F{i}"), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let functions_detail: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("F{i}"), Span::call_site());
                let temp_expr = Ident::new(&format!("f{i}"), Span::call_site());
                quote! {
                    #temp_expr : #temp_ident ,
                }
            })
            .collect();

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

        let join_handle: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|_| {
                quote! {
                    std::thread::JoinHandle<()> ,
                }
            })
            .collect();

        let new_channels: Vec<TokenStream> = (1..=((self.n_sessions - 1) * (self.n_sessions) / 2))
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{i}"), Span::call_site());
                let (line, column, _) = get_tuple_diag(&diag_w_offset, i);
                let temp_channel_left =
                    Ident::new(&format!("channel_{}_{}", line, column), Span::call_site());
                let temp_channel_right =
                    Ident::new(&format!("channel_{}_{}", column, line), Span::call_site());
                quote! {
                    let ( #temp_channel_left , #temp_channel_right ) =
                        < #temp_ident as mpstthree::binary::struct_trait::session::Session>::new();
                }
            })
            .collect();

        let new_meshedchannels: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_sessions: Vec<TokenStream> = (1..self.n_sessions)
                    .map(|j| {
                        let (line, column, _) = get_tuple_matrix(&matrix, i, j);
                        let temp_session = Ident::new(&format!("session{j}"), Span::call_site());
                        let temp_channel = match line {
                            m if m == i => Ident::new(
                                &format!("channel_{}_{}", line, column),
                                Span::call_site(),
                            ),
                            _ => Ident::new(
                                &format!("channel_{}_{}", column, line),
                                Span::call_site(),
                            ),
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
                        #meshedchannels_name {
                            #(
                                #temp_sessions
                            )*
                            stack: #temp_role ,
                            name: #temp_name ,
                        };
                }
            })
            .collect();

        let new_threads: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_function =
                    Ident::new(&format!("f{i}"), Span::call_site());
                let temp_meshedchannels = Ident::new(
                    &format!("meshedchannels_{i}"),
                    Span::call_site(),
                );
                quote! {
                    std::thread::Builder::new().name(String::from(stringify!(#temp_function))).stack_size(64 * 1024 * 1024).spawn(move || {
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
                    #functions_detail
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
