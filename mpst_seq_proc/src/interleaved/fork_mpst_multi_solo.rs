use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Token};

type VecOfTuple = Vec<(u64, u64, u64)>;

#[derive(Debug)]
pub(crate) struct ForkMPSTMultiSolo {
    func_name: Ident,
    meshedchannels_name: Ident,
    n_sessions: u64,
}

impl Parse for ForkMPSTMultiSolo {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let meshedchannels_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let n_sessions = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(ForkMPSTMultiSolo {
            func_name,
            meshedchannels_name,
            n_sessions,
        })
    }
}

impl From<ForkMPSTMultiSolo> for TokenStream {
    fn from(input: ForkMPSTMultiSolo) -> TokenStream {
        input.expand()
    }
}

impl ForkMPSTMultiSolo {
    /// Create the whole matrix of index according to line and column
    fn diag(&self) -> VecOfTuple {
        let diff = self.n_sessions - 1;

        let mut column = 0;
        let mut line = 0;

        // Create the upper diag
        (0..(diff * (diff + 1) / 2))
            .map(|i| {
                if line == column {
                    column += 1;
                } else if column >= (self.n_sessions - 1) {
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
            (1..=self.n_sessions)
                .map(|i| {
                    diag.iter()
                        .filter_map(|(line, column, index)| {
                            if i == *line || i == *column {
                                Some((*line, *column, *index))
                            } else {
                                None
                            }
                        })
                        .collect()
                })
                .collect(),
        )
    }

    /// Create the whole matrix of index according to line and column
    fn diag_w_offset(&self) -> VecOfTuple {
        let diff = self.n_sessions - 1;

        let mut column = 0;
        let mut line = 0;

        // Create the upper diag
        (0..=(diff * (diff + 1) / 2))
            .map(|i| {
                if line == column {
                    column += 1;
                } else if column >= (self.n_sessions - 1) {
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
            (1..=self.n_sessions)
                .map(|i| {
                    diag_w_offset
                        .iter()
                        .filter_map(|(line, column, index)| {
                            if i == *line || i == *column {
                                Some((*line, *column, *index))
                            } else {
                                None
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

    fn expand(&self) -> TokenStream {
        let func_name = self.func_name.clone();
        let meshedchannels_name = self.meshedchannels_name.clone();
        let (_diag, matrix) = self.diag_and_matrix();
        let (diag_w_offset, matrix_w_offset) = self.diag_and_matrix_w_offset();

        let sessions: Vec<TokenStream> = (1..=((self.n_sessions - 1) * (self.n_sessions) / 2))
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let sessions_struct: Vec<TokenStream> = (1..=((self.n_sessions - 1) * (self.n_sessions)
            / 2))
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                quote! {
                    #temp_ident : mpstthree::binary::struct_trait::session::Session + 'static ,
                }
            })
            .collect();

        let roles: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("R{}", i), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let roles_struct: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("R{}", i), Span::call_site());
                quote! {
                    #temp_ident : mpstthree::role::Role + 'static ,
                }
            })
            .collect();

        let new_roles: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("R{}", i), Span::call_site());
                let temp_role = Ident::new(&format!("role_{}", i), Span::call_site());
                quote! {
                    let ( #temp_role , _) = #temp_ident::new() ;
                }
            })
            .collect();

        let names: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("N{}", i), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let names_struct: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("N{}", i), Span::call_site());
                quote! {
                    #temp_ident : mpstthree::name::Name + 'static ,
                }
            })
            .collect();

        let new_names: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("N{}", i), Span::call_site());
                let temp_name = Ident::new(&format!("name_{}", i), Span::call_site());
                quote! {
                    let ( #temp_name , _) = #temp_ident::new() ;
                }
            })
            .collect();

        let functions_struct: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_sessions: Vec<TokenStream> = (1..self.n_sessions)
                    .map(|j| {
                        let (k, _, m) = self.get_tuple_matrix(&matrix_w_offset, i, j);
                        let temp_ident =
                            Ident::new(&format!("S{}", m), Span::call_site());
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

                let temp_role = Ident::new(&format!("R{}", i), Span::call_site());
                let temp_name = Ident::new(&format!("N{}", i), Span::call_site());
                quote! {
                    #meshedchannels_name<
                        #(
                            #temp_sessions
                        )*
                        #temp_role ,
                        #temp_name
                    >,
                }
            })
            .collect();

        let new_channels: Vec<TokenStream> = (1..=((self.n_sessions - 1) * (self.n_sessions) / 2))
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                let (line, column, _) = self.get_tuple_diag(&diag_w_offset, i);
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
                        let (line, column, _) = self.get_tuple_matrix(&matrix, i, j);
                        let temp_session = Ident::new(&format!("session{}", j), Span::call_site());
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
                    Ident::new(&format!("meshedchannels_{}", i), Span::call_site());
                let temp_role = Ident::new(&format!("role_{}", i), Span::call_site());
                let temp_name = Ident::new(&format!("name_{}", i), Span::call_site());
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

        let use_meshedchannels: Vec<TokenStream> = (1..=self.n_sessions)
            .map(|i| {
                let temp_meshedchannels =
                    Ident::new(&format!("meshedchannels_{}", i), Span::call_site());
                quote! {
                    #temp_meshedchannels,
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
                F
            >(
                f: F
            ) -> Result<(), Box<dyn std::error::Error>>
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
                F : FnOnce(
                    #(
                        #functions_struct
                    )*
                ) -> Result<(), Box<dyn std::error::Error>>
                + std::marker::Send
                + 'static,
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

                f(
                    #(
                        #use_meshedchannels
                    )*
                )
            }
        }
    }
}
