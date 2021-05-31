use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

type VecOfTuple = Vec<(u64, u64, u64)>;

#[derive(Debug)]
pub struct ForkMPSTMultiMacroInput {
    func_name: syn::Ident,
    sessionmpst_name: syn::Ident,
    nsessions: u64,
}

impl Parse for ForkMPSTMultiMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let sessionmpst_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let nsessions = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(ForkMPSTMultiMacroInput {
            func_name,
            sessionmpst_name,
            nsessions,
        })
    }
}

impl From<ForkMPSTMultiMacroInput> for proc_macro2::TokenStream {
    fn from(input: ForkMPSTMultiMacroInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl ForkMPSTMultiMacroInput {
    /// Create the whole matrix of index according to line and column
    fn diag(&self) -> VecOfTuple {
        let diff = self.nsessions - 1;

        let mut column = 0;
        let mut line = 0;

        // Create the upper diag
        (0..(diff * (diff + 1) / 2))
            .map(|i| {
                if line == column {
                    column += 1;
                } else if column >= (self.nsessions - 1) {
                    line += 1;
                    column = line + 1;
                } else {
                    column += 1;
                }
                (line + 1, column + 1, i + 1)
            })
            .collect()
    }

    /// Create the whole matrix of index according to line and column
    fn diag_and_matrix(&self) -> (VecOfTuple, Vec<VecOfTuple>) {
        let diag = self.diag();

        // Create the whole matrix
        (
            diag.clone(),
            (1..=self.nsessions)
                .map(|i| {
                    diag.iter()
                        .filter_map(|(line, column, index)| {
                            if i == *line || i == *column {
                                std::option::Option::Some((*line, *column, *index))
                            } else {
                                std::option::Option::None
                            }
                        })
                        .collect()
                })
                .collect(),
        )
    }

    /// Create the whole matrix of index according to line and column
    fn diag_w_offset(&self) -> VecOfTuple {
        let diff = self.nsessions - 1;

        let mut column = 0;
        let mut line = 0;

        // Create the upper diag
        (0..=(diff * (diff + 1) / 2))
            .map(|i| {
                if line == column {
                    column += 1;
                } else if column >= (self.nsessions - 1) {
                    line += 1;
                    column = line + 1;
                } else {
                    column += 1;
                }
                (line + 1, column + 1, i + 1)
            })
            .collect()
    }

    /// Create the whole matrix of index according to line and column
    fn diag_and_matrix_w_offset(&self) -> (VecOfTuple, Vec<VecOfTuple>) {
        let diag_w_offset = self.diag_w_offset();

        // Create the whole matrix
        (
            diag_w_offset.clone(),
            (1..=self.nsessions)
                .map(|i| {
                    diag_w_offset
                        .iter()
                        .filter_map(|(line, column, index)| {
                            if i == *line || i == *column {
                                std::option::Option::Some((*line, *column, *index))
                            } else {
                                std::option::Option::None
                            }
                        })
                        .collect()
                })
                .collect(),
        )
    }

    /// Return (line, column, index) of diag
    fn get_tuple_diag(&self, diag: &[(u64, u64, u64)], i: u64) -> (u64, u64, u64) {
        if let Some((line, column, index)) = diag.get(usize::try_from(i - 1).unwrap()) {
            (*line, *column, *index)
        } else {
            panic!(
                "Error at get_tuple_diag for i = {:?} / diag = {:?}",
                i, diag
            )
        }
    }

    /// Return (line, column, index) of matrix
    fn get_tuple_matrix(&self, matrix: &[VecOfTuple], i: u64, j: u64) -> (u64, u64, u64) {
        let list: VecOfTuple = if let Some(temp) = matrix.get(usize::try_from(i - 1).unwrap()) {
            temp.to_vec()
        } else {
            panic!(
                "Error at get_tuple_matrix for i = {:?} / matrix = {:?}",
                i, matrix
            )
        };

        if let Some((line, column, index)) = list.get(usize::try_from(j - 1).unwrap()) {
            (*line, *column, *index)
        } else {
            panic!("Error at get_tuple_matrix for i = {:?} and j = {:?} with list = {:?} / matrix = {:?}", i, j, list, matrix)
        }
    }

    fn expand(&self) -> proc_macro2::TokenStream {
        let func_name = self.func_name.clone();
        let sessionmpst_name = self.sessionmpst_name.clone();
        let nsessions = self.nsessions;
        let (_diag, matrix) = self.diag_and_matrix();
        let (diag_w_offset, matrix_w_offset) = self.diag_and_matrix_w_offset();

        let sessions: Vec<proc_macro2::TokenStream> = (1..=((nsessions - 1) * (nsessions) / 2))
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let sessions_struct: Vec<proc_macro2::TokenStream> = (1..=((nsessions - 1) * (nsessions)
            / 2))
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident : mpstthree::binary::struct_trait::Session + 'static ,
                }
            })
            .collect();

        let roles: Vec<proc_macro2::TokenStream> = (1..=nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("R{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let roles_struct: Vec<proc_macro2::TokenStream> = (1..=nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("R{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident : mpstthree::role::Role + 'static ,
                }
            })
            .collect();

        let new_roles: Vec<proc_macro2::TokenStream> = (1..=nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("R{}", i), proc_macro2::Span::call_site());
                let temp_role =
                    syn::Ident::new(&format!("role_{}", i), proc_macro2::Span::call_site());
                quote! {
                    let ( #temp_role , _) = #temp_ident::new() ;
                }
            })
            .collect();

        let names: Vec<proc_macro2::TokenStream> = (1..=nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("N{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let names_struct: Vec<proc_macro2::TokenStream> = (1..=nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("N{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident : mpstthree::role::Role + 'static ,
                }
            })
            .collect();

        let new_names: Vec<proc_macro2::TokenStream> = (1..=nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("N{}", i), proc_macro2::Span::call_site());
                let temp_name =
                    syn::Ident::new(&format!("name_{}", i), proc_macro2::Span::call_site());
                quote! {
                    let ( #temp_name , _) = #temp_ident::new() ;
                }
            })
            .collect();

        let functions: Vec<proc_macro2::TokenStream> = (1..=nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("F{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let functions_detail: Vec<proc_macro2::TokenStream> = (1..=nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("F{}", i), proc_macro2::Span::call_site());
                let temp_expr = syn::Ident::new(&format!("f{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_expr : #temp_ident ,
                }
            })
            .collect();

        let functions_struct: Vec<proc_macro2::TokenStream> = (1..=nsessions)
            .map(|i| {
                let temp_sessions: Vec<proc_macro2::TokenStream> = (1..nsessions)
                    .map(|j| {
                        let (k, _, m) = self.get_tuple_matrix(&matrix_w_offset, i, j);
                        let temp_ident =
                            syn::Ident::new(&format!("S{}", m), proc_macro2::Span::call_site());
                        if k == i {
                            quote! {
                                #temp_ident ,
                            }
                        } else {
                            quote! {
                                < #temp_ident  as mpstthree::binary::struct_trait::Session>::Dual ,
                            }
                        }
                    })
                    .collect();

                let temp_function =
                    syn::Ident::new(&format!("F{}", i), proc_macro2::Span::call_site());
                let temp_role = syn::Ident::new(&format!("R{}", i), proc_macro2::Span::call_site());
                let temp_name = syn::Ident::new(&format!("N{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_function : FnOnce(
                        #sessionmpst_name<
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

        let join_handle: Vec<proc_macro2::TokenStream> = (1..=nsessions)
            .map(|_| {
                quote! {
                    std::thread::JoinHandle<()> ,
                }
            })
            .collect();

        let new_channels: Vec<proc_macro2::TokenStream> = (1..=((nsessions - 1) * (nsessions) / 2))
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                let (line, column, _) = self.get_tuple_diag(&diag_w_offset, i);
                let temp_channel_left = syn::Ident::new(
                    &format!("channel_{}_{}", line, column),
                    proc_macro2::Span::call_site(),
                );
                let temp_channel_right = syn::Ident::new(
                    &format!("channel_{}_{}", column, line),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    let ( #temp_channel_left , #temp_channel_right ) =
                        < #temp_ident as mpstthree::binary::struct_trait::Session>::new();
                }
            })
            .collect();

        let new_sessionmpst: Vec<proc_macro2::TokenStream> = (1..=nsessions)
            .map(|i| {
                let temp_sessions: Vec<proc_macro2::TokenStream> = (1..nsessions)
                    .map(|j| {
                        let (line, column, _) = self.get_tuple_matrix(&matrix, i, j);
                        let temp_session = syn::Ident::new(
                            &format!("session{}", j),
                            proc_macro2::Span::call_site(),
                        );
                        let temp_channel = match line {
                            m if m == i => syn::Ident::new(
                                &format!("channel_{}_{}", line, column),
                                proc_macro2::Span::call_site(),
                            ),
                            _ => syn::Ident::new(
                                &format!("channel_{}_{}", column, line),
                                proc_macro2::Span::call_site(),
                            ),
                        };
                        quote! {
                            #temp_session : #temp_channel ,
                        }
                    })
                    .collect();

                let temp_sessionmpst = syn::Ident::new(
                    &format!("sessionmpst_{}", i),
                    proc_macro2::Span::call_site(),
                );
                let temp_role =
                    syn::Ident::new(&format!("role_{}", i), proc_macro2::Span::call_site());
                let temp_name =
                    syn::Ident::new(&format!("name_{}", i), proc_macro2::Span::call_site());
                quote! {
                    let #temp_sessionmpst =
                        #sessionmpst_name {
                            #(
                                #temp_sessions
                            )*
                            stack: #temp_role ,
                            name: #temp_name ,
                        };
                }
            })
            .collect();

        let new_threads: Vec<proc_macro2::TokenStream> = (1..=nsessions)
            .map(|i| {
                let temp_function =
                    syn::Ident::new(&format!("f{}", i), proc_macro2::Span::call_site());
                let temp_sessionmpst = syn::Ident::new(
                    &format!("sessionmpst_{}", i),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    std::thread::spawn(move || {
                        std::panic::set_hook(Box::new(|_info| {
                            // do nothing
                        }));
                        match #temp_function(#temp_sessionmpst) {
                            Ok(()) => (),
                            Err(e) => panic!("{:?}", e),
                        }
                    }),
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
                    #new_sessionmpst
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
