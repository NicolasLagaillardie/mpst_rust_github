use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token, Ident, LitInt};
use proc_macro2::{TokenStream, Span};

type VecOfTuple = Vec<(u64, u64, u64)>;

#[derive(Debug)]
pub struct ChooseTypeMultiLeft {
    func_name: Ident,
    type_name: Ident,
    role_dual: Ident,
    name: Ident,
    meshedchannels_name: Ident,
    n_sessions: u64,
}

impl Parse for ChooseTypeMultiLeft {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let type_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let role_dual = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let meshedchannels_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let n_sessions = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(ChooseTypeMultiLeft {
            func_name,
            type_name,
            role_dual,
            name,
            meshedchannels_name,
            n_sessions,
        })
    }
}

impl From<ChooseTypeMultiLeft> for TokenStream {
    fn from(input: ChooseTypeMultiLeft) -> TokenStream {
        input.expand()
    }
}

impl ChooseTypeMultiLeft {
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

    fn expand(&self) -> TokenStream {
        let func_name = self.func_name.clone();
        let type_name = self.type_name.clone();
        let role_dual = self.role_dual.clone();
        let name = self.name.clone();
        let meshedchannels_name = self.meshedchannels_name.clone();
        let diff = self.n_sessions - 1;
        let (diag, matrix) = self.diag_and_matrix();

        let all_sessions: Vec<TokenStream> = (1..=(diff * (diff + 1)))
            .map(|i| {
                let temp_ident =
                    Ident::new(&format!("S{}", i), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let all_sessions_struct: Vec<TokenStream> = (1..=(diff * (diff + 1)))
            .map(|i| {
                let temp_ident =
                    Ident::new(&format!("S{}", i), Span::call_site());
                quote! {
                    #temp_ident : mpstthree::binary::struct_trait::session::Session + 'a ,
                }
            })
            .collect();

        let all_roles: Vec<TokenStream> = (1..(3 * self.n_sessions))
            .map(|i| {
                let temp_ident =
                    Ident::new(&format!("R{}", i), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let all_roles_struct: Vec<TokenStream> = (1..(3 * self.n_sessions))
            .map(|i| {
                let temp_ident =
                    Ident::new(&format!("R{}", i), Span::call_site());
                quote! {
                    #temp_ident : mpstthree::role::Role + 'a ,
                }
            })
            .collect();

        let all_types: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let types_left: Vec<TokenStream> = (1..self.n_sessions)
                    .map(|j| {
                        let (k, _, m) = self.get_tuple_matrix(&matrix, i, j);

                        let temp_ident =
                            Ident::new(&format!("S{}", m), Span::call_site());

                        if k == i {
                            quote! {
                                < #temp_ident as mpstthree::binary::struct_trait::session::Session>::Dual,
                            }
                        } else {
                            quote! {
                                #temp_ident ,
                            }
                        }
                    })
                    .collect();

                let types_right: Vec<TokenStream> = (1..self.n_sessions)
                    .map(|j| {
                        let (k, _, m) = self.get_tuple_matrix(&matrix, i, j);

                        let temp_ident = Ident::new(
                            &format!("S{}", diff * (diff + 1) / 2 + m),
                            Span::call_site(),
                        );

                        if k == i {
                            quote! {
                                < #temp_ident as mpstthree::binary::struct_trait::session::Session>::Dual,
                            }
                        } else {
                            quote! {
                                #temp_ident ,
                            }
                        }
                    })
                    .collect();

                let temp_roles: Vec<TokenStream> = (1..4)
                    .map(|j| {
                        let temp_ident = Ident::new(
                            &format!("R{}", 3 * (i - 1) + j),
                            Span::call_site(),
                        );

                        quote! {
                            #temp_ident ,
                        }
                    })
                    .collect();

                quote! {
                    #type_name<
                        #(
                            #types_left
                        )*
                        #(
                            #types_right
                        )*
                        #(
                            #temp_roles
                        )*
                    >,
                }
            })
            .collect();

        let stacks: Vec<TokenStream> = ((3 * self.n_sessions - 2)
            ..(3 * self.n_sessions))
            .map(|i| {
                let temp_ident =
                    Ident::new(&format!("R{}", i), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let sessions_struct: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let sum: u64 = (0..i).map(|j| self.n_sessions - 1 - j).sum();
                let temp_ident =
                    Ident::new(&format!("S{}", sum), Span::call_site());
                quote! {
                    < #temp_ident as mpstthree::binary::struct_trait::session::Session>::Dual,
                }
            })
            .collect();

        let role_left = Ident::new(
            &format!("R{}", 3 * self.n_sessions - 2),
            Span::call_site(),
        );

        let stack_left = Ident::new(
            &format!("stack_{}", self.n_sessions),
            Span::call_site(),
        );

        let name_left = Ident::new(
            &format!("name_{}", self.n_sessions),
            Span::call_site(),
        );

        let new_channels: Vec<TokenStream> = (1..=(diff * (diff + 1) / 2))
            .map(|i| {
                let (line, column, _) = self.get_tuple_diag(&diag, i);
                let channel_left = Ident::new(
                    &format!("channel_{}_{}", line, column),
                    Span::call_site(),
                );
                let channel_right = Ident::new(
                    &format!("channel_{}_{}", column, line),
                    Span::call_site(),
                );
                let temp_session =
                    Ident::new(&format!("S{}", i), Span::call_site());
                quote! {
                    let ( #channel_left , #channel_right ) = #temp_session::new();
                }
            })
            .collect();

        let new_stacks: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let temp_stack =
                    Ident::new(&format!("stack_{}", i), Span::call_site());
                let temp_role = Ident::new(
                    &format!("R{}", 3 * (i - 1) + 1),
                    Span::call_site(),
                );
                quote! {
                    let (_, #temp_stack ) = #temp_role::new();
                }
            })
            .collect();

        let new_names: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let temp_name =
                    Ident::new(&format!("name_{}", i), Span::call_site());
                let temp_role = Ident::new(
                    &format!("R{}", 3 * (i - 1) + 3),
                    Span::call_site(),
                );
                quote! {
                    let ( #temp_name , _) =
                        << #temp_role as mpstthree::role::Role>::Dual as mpstthree::role::Role>::new();
                }
            })
            .collect();

        let new_choices: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let types_sessions: Vec<TokenStream> = (1..self.n_sessions)
                    .map(|j| {
                        let (line, column, _) = self.get_tuple_matrix(&matrix, i, j);

                        let temp_session = Ident::new(
                            &format!("session{}", j),
                            Span::call_site(),
                        );

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

                let temp_choice =
                    Ident::new(&format!("choice_{}", i), Span::call_site());

                let temp_stack =
                    Ident::new(&format!("stack_{}", i), Span::call_site());

                let temp_name =
                    Ident::new(&format!("name_{}", i), Span::call_site());

                quote! {
                    let #temp_choice = #meshedchannels_name {
                        #(
                            #types_sessions
                        )*
                        stack: #temp_stack ,
                        name: #temp_name ,
                    };
                }
            })
            .collect();

        let new_sessions_left: Vec<TokenStream> = (1..self.n_sessions)
                .map(|i| {
                    let temp_new_session =
                        Ident::new(&format!("new_session_{}", i), Span::call_site());
                    let temp_session =
                        Ident::new(&format!("session{}", i), Span::call_site());
                    let temp_choice =
                        Ident::new(&format!("choice_{}", i), Span::call_site());
                    quote! {
                        let #temp_new_session =
                            mpstthree::binary::send::send(either::Either::Left(#temp_choice), s.#temp_session);
                    }
                })
                .collect();

        let old_session_meshedchannels: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let temp_new_session = Ident::new(
                    &format!("new_session_{}", i),
                    Span::call_site(),
                );
                let temp_session =
                    Ident::new(&format!("session{}", i), Span::call_site());
                quote! {
                    #temp_session : #temp_new_session ,
                }
            })
            .collect();

        let new_session_meshedchannels: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let (line, column, _) = self.get_tuple_matrix(&matrix, self.n_sessions, i);
                let temp_channel = match line {
                    m if m == i => Ident::new(
                        &format!("channel_{}_{}", column, line),
                        Span::call_site(),
                    ),
                    _ => Ident::new(
                        &format!("channel_{}_{}", line, column),
                        Span::call_site(),
                    ),
                };
                let temp_session =
                    Ident::new(&format!("session{}", i), Span::call_site());
                quote! {
                    #temp_session : #temp_channel ,
                }
            })
            .collect();

        quote! {
            fn #func_name<
                'a,
                #(
                    #all_sessions
                )*
                #(
                    #all_roles
                )*
            >(
                s: #meshedchannels_name<
                    #(
                        #all_types
                    )*

                    #role_dual<
                        #(
                            #stacks
                        )*
                    >,

                    #name<mpstthree::role::end::RoleEnd>,
                >,
            )
            -> #meshedchannels_name<
                #(
                    #sessions_struct
                )*
                #role_left ,
                #name<mpstthree::role::end::RoleEnd>
            >
            where
                #(
                    #all_sessions_struct
                )*

                #(
                    #all_roles_struct
                )*
            {
                #(
                    #new_channels
                )*

                #(
                    #new_stacks
                )*

                let ( #stack_left , _) = < #role_left as mpstthree::role::Role>::new();

                #(
                    #new_names
                )*

                let ( #name_left , _) = <#name::<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

                #(
                    #new_choices
                )*

                #(
                    #new_sessions_left
                )*

                let s = #meshedchannels_name {
                    #(
                        #old_session_meshedchannels
                    )*
                    stack: s.stack,
                    name: s.name,
                };

                mpstthree::binary::cancel::cancel(s);

                #meshedchannels_name {
                    #(
                        #new_session_meshedchannels
                    )*
                    stack: #stack_left ,
                    name: #name_left ,
                }
            }
        }
    }
}
