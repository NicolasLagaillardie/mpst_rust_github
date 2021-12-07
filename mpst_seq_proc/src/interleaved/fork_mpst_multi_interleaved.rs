use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Token};

type VecOfTuple = Vec<(u64, u64, u64)>;

#[derive(Debug)]
pub struct ForkMPSTMultiInterleaved {
    func_name: Ident,
    meshedchannels_name_one: Ident,
    nsessions_one: u64,
    index_tuple_one: u64,
    meshedchannels_name_two: Ident,
    nsessions_two: u64,
    index_tuple_two: u64,
}

impl Parse for ForkMPSTMultiInterleaved {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let meshedchannels_name_one = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let nsessions_one = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();
        <Token![,]>::parse(input)?;

        let index_tuple_one = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();
        <Token![,]>::parse(input)?;

        let meshedchannels_name_two = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let nsessions_two = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();
        <Token![,]>::parse(input)?;

        let index_tuple_two = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(ForkMPSTMultiInterleaved {
            func_name,
            meshedchannels_name_one,
            nsessions_one,
            index_tuple_one,
            meshedchannels_name_two,
            nsessions_two,
            index_tuple_two,
        })
    }
}

impl From<ForkMPSTMultiInterleaved> for TokenStream {
    fn from(input: ForkMPSTMultiInterleaved) -> TokenStream {
        input.expand()
    }
}

impl ForkMPSTMultiInterleaved {
    /// Create the whole matrix of index according to line and column
    fn diag(&self, n_sessions: u64) -> VecOfTuple {
        let diff = n_sessions - 1;

        let mut column = 0;
        let mut line = 0;

        // Create the upper diag
        (0..(diff * (diff + 1) / 2))
            .map(|i| {
                if line == column {
                    column += 1;
                } else if column >= (n_sessions - 1) {
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
    fn diag_and_matrix(&self, n_sessions: u64) -> (VecOfTuple, Vec<VecOfTuple>) {
        let diag = self.diag(n_sessions);

        // Create the whole matrix
        (
            diag.clone(),
            (1..=n_sessions)
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
    fn diag_w_offset(&self, n_sessions: u64) -> VecOfTuple {
        let diff = n_sessions - 1;

        let mut column = 0;
        let mut line = 0;

        // Create the upper diag
        (0..=(diff * (diff + 1) / 2))
            .map(|i| {
                if line == column {
                    column += 1;
                } else if column >= (n_sessions - 1) {
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
    fn diag_and_matrix_w_offset(&self, n_sessions: u64) -> (VecOfTuple, Vec<VecOfTuple>) {
        let diag_w_offset = self.diag_w_offset(n_sessions);

        // Create the whole matrix
        (
            diag_w_offset.clone(),
            (1..=n_sessions)
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
        let meshedchannels_name_one = self.meshedchannels_name_one.clone();
        let meshedchannels_name_two = self.meshedchannels_name_two.clone();
        let sum_nsessions = self.nsessions_one + self.nsessions_two;
        let (_diag_one, matrix_one) = self.diag_and_matrix(self.nsessions_one);
        let (_diag_two, matrix_two) = self.diag_and_matrix(self.nsessions_two);
        let (diag_w_offset_one, matrix_w_offset_one) =
            self.diag_and_matrix_w_offset(self.nsessions_one);
        let (diag_w_offset_two, matrix_w_offset_two) =
            self.diag_and_matrix_w_offset(self.nsessions_two);

        // Generate
        // S0, S1, S2, ...
        let sessions: Vec<TokenStream> = (1..=((self.nsessions_one - 1) * (self.nsessions_one)
            / 2
            + (self.nsessions_two - 1) * (self.nsessions_two) / 2))
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
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
            (1..=((self.nsessions_one - 1) * (self.nsessions_one) / 2
                + (self.nsessions_two - 1) * (self.nsessions_two) / 2))
                .map(|i| {
                    let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                    quote! {
                        #temp_ident : mpstthree::binary::struct_trait::session::Session + 'static ,
                    }
                })
                .collect();

        let roles: Vec<TokenStream> = (1..=sum_nsessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("R{}", i), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let roles_struct: Vec<TokenStream> = (1..=sum_nsessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("R{}", i), Span::call_site());
                quote! {
                    #temp_ident : mpstthree::role::Role + 'static ,
                }
            })
            .collect();

        let new_roles: Vec<TokenStream> = (1..=sum_nsessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("R{}", i), Span::call_site());
                let temp_role = Ident::new(&format!("role_{}", i), Span::call_site());
                quote! {
                    let ( #temp_role , _) = #temp_ident::new() ;
                }
            })
            .collect();

        let names: Vec<TokenStream> = (1..=sum_nsessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("N{}", i), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let names_struct: Vec<TokenStream> = (1..=sum_nsessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("N{}", i), Span::call_site());
                quote! {
                    #temp_ident : mpstthree::role::Role + 'static ,
                }
            })
            .collect();

        let new_names: Vec<TokenStream> = (1..=sum_nsessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("N{}", i), Span::call_site());
                let temp_name = Ident::new(&format!("name_{}", i), Span::call_site());
                quote! {
                    let ( #temp_name , _) = #temp_ident::new() ;
                }
            })
            .collect();

        let functions: Vec<TokenStream> = (1..=(sum_nsessions - 2))
            .map(|i| {
                let temp_ident = Ident::new(&format!("F{}", i), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let functions_input_one: Vec<TokenStream> = (1..self.nsessions_one)
            .map(|i| {
                let temp_ident = Ident::new(&format!("F{}", i), Span::call_site());
                let temp_expr = Ident::new(&format!("f{}", i), Span::call_site());
                quote! {
                    #temp_expr : #temp_ident ,
                }
            })
            .collect();

        let functions_input_two: Vec<TokenStream> = (1..self.nsessions_two)
            .map(|i| {
                let temp_ident = Ident::new(
                    &format!("F{}", self.nsessions_one - 1 + i),
                    Span::call_site(),
                );
                let temp_expr = Ident::new(
                    &format!("f{}", self.nsessions_one - 1 + i),
                    Span::call_site(),
                );
                quote! {
                    #temp_expr : #temp_ident ,
                }
            })
            .collect();

        let mut functions_struct_one: Vec<TokenStream> = (1..=self.nsessions_one)
            .map(|i| {
                let temp_sessions: Vec<TokenStream> = (1..self.nsessions_one)
                    .map(|j| {
                        let (k, _, m) = self.get_tuple_matrix(&matrix_w_offset_one, i, j);
                        let temp_ident =
                            Ident::new(&format!("S{}", m), Span::call_site());
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

                    let temp_role = Ident::new(&format!("R{}", i), Span::call_site());
                    let temp_name = Ident::new(&format!("N{}", i), Span::call_site());

                if i != self.index_tuple_one {
                    let offset = if i < self.index_tuple_one { i } else { i - 1 };
                    let temp_function =
                        Ident::new(&format!("F{}", offset), Span::call_site());
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

        let mut functions_struct_two: Vec<TokenStream> = (1..=self.nsessions_two)
            .map(|i| {
                let temp_sessions: Vec<TokenStream> = (1..self.nsessions_two)
                    .map(|j| {
                        let (k, _, m) = self.get_tuple_matrix(&matrix_w_offset_two, i, j);
                        let temp_ident =
                            Ident::new(&format!("S{}", ((self.nsessions_one - 1) * (self.nsessions_one) / 2) + m), Span::call_site());
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

                    let temp_role = Ident::new(&format!("R{}", self.nsessions_one + i), Span::call_site());
                    let temp_name = Ident::new(&format!("N{}", self.nsessions_one + i), Span::call_site());

                if i != self.index_tuple_two {
                    let offset = if i < self.index_tuple_two { i - 1} else { i - 2 };
                    let temp_function =
                        Ident::new(&format!("F{}", self.nsessions_one + offset), Span::call_site());
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

        let new_channels_one: Vec<TokenStream> = (1..=((self.nsessions_one - 1)
            * (self.nsessions_one)
            / 2))
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                let (line, column, _) = self.get_tuple_diag(&diag_w_offset_one, i);
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

        let new_channels_two: Vec<TokenStream> = (1..=((self.nsessions_two - 1)
            * (self.nsessions_two)
            / 2))
            .map(|i| {
                let temp_ident = Ident::new(
                    &format!(
                        "S{}",
                        ((self.nsessions_one - 1) * (self.nsessions_one) / 2) + i
                    ),
                    Span::call_site(),
                );
                let (line, column, _) = self.get_tuple_diag(&diag_w_offset_two, i);
                let temp_channel_left = Ident::new(
                    &format!(
                        "channel_{}_{}",
                        self.nsessions_one + line,
                        self.nsessions_one + column
                    ),
                    Span::call_site(),
                );
                let temp_channel_right = Ident::new(
                    &format!(
                        "channel_{}_{}",
                        self.nsessions_one + column,
                        self.nsessions_one + line
                    ),
                    Span::call_site(),
                );
                quote! {
                    let ( #temp_channel_left , #temp_channel_right ) =
                        < #temp_ident as mpstthree::binary::struct_trait::session::Session>::new();
                }
            })
            .collect();

        let new_meshedchannels_one: Vec<TokenStream> = (1..=(self.nsessions_one))
            .map(|i| {
                let temp_sessions: Vec<TokenStream> = (1..self.nsessions_one)
                    .map(|j| {
                        let (line, column, _) = self.get_tuple_matrix(&matrix_one, i, j);
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

        let new_meshedchannels_two: Vec<TokenStream> = (1..=(self.nsessions_two))
            .map(|i| {
                let temp_sessions: Vec<TokenStream> = (1..self.nsessions_two)
                    .map(|j| {
                        let (line, column, _) = self.get_tuple_matrix(&matrix_two, i, j);
                        let temp_session = Ident::new(&format!("session{}", j), Span::call_site());
                        let temp_channel = match line {
                            m if m == i => Ident::new(
                                &format!(
                                    "channel_{}_{}",
                                    self.nsessions_one + line,
                                    self.nsessions_one + column
                                ),
                                Span::call_site(),
                            ),
                            _ => Ident::new(
                                &format!(
                                    "channel_{}_{}",
                                    self.nsessions_one + column,
                                    self.nsessions_one + line
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
                    &format!("meshedchannels_{}", self.nsessions_one + i),
                    Span::call_site(),
                );
                let temp_role = Ident::new(
                    &format!("role_{}", self.nsessions_one + i),
                    Span::call_site(),
                );
                let temp_name = Ident::new(
                    &format!("name_{}", self.nsessions_one + i),
                    Span::call_site(),
                );
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
                let offset = if i < self.index_tuple_one {
                    i
                } else if i >= self.index_tuple_one
                    && i < self.nsessions_one - 1 + self.index_tuple_two
                {
                    i + 1
                } else {
                    i + 2
                };
                let temp_function =
                    Ident::new(&format!("f{}", i), Span::call_site());
                let temp_meshedchannels = Ident::new(
                    &format!("meshedchannels_{}", offset),
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

        let new_interleaved_thread: TokenStream = {
            let temp_meshedchannels_one = Ident::new(
                &format!("meshedchannels_{}", self.index_tuple_one),
                Span::call_site(),
            );
            let temp_meshedchannels_two = Ident::new(
                &format!(
                    "meshedchannels_{}",
                    (self.nsessions_one + self.index_tuple_two)
                ),
                Span::call_site(),
            );

            quote! {
                std::thread::Builder::new().name(String::from("Interleaved thread")).stack_size(64 * 1024 * 1024).spawn(move || {
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
