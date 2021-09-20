use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

type VecOfTuple = Vec<(u64, u64, u64)>;

#[derive(Debug)]
pub struct ChooseTypeMultiRight {
    func_name: syn::Ident,
    type_name: syn::Ident,
    role_dual: syn::Ident,
    name: syn::Ident,
    meshedchannels_name: syn::Ident,
    nsessions: u64,
}

impl Parse for ChooseTypeMultiRight {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let type_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let role_dual = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let meshedchannels_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let nsessions = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(ChooseTypeMultiRight {
            func_name,
            type_name,
            role_dual,
            name,
            meshedchannels_name,
            nsessions,
        })
    }
}

impl From<ChooseTypeMultiRight> for proc_macro2::TokenStream {
    fn from(input: ChooseTypeMultiRight) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl ChooseTypeMultiRight {
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
        let type_name = self.type_name.clone();
        let role_dual = self.role_dual.clone();
        let name = self.name.clone();
        let meshedchannels_name = self.meshedchannels_name.clone();
        let diff = self.nsessions - 1;
        let (diag, matrix) = self.diag_and_matrix();

        let all_sessions: Vec<proc_macro2::TokenStream> = (1..=(diff * (diff + 1)))
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let all_sessions_struct: Vec<proc_macro2::TokenStream> = (1..=(diff * (diff + 1)))
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident : mpstthree::binary::struct_trait::session::Session + 'a ,
                }
            })
            .collect();

        let all_roles: Vec<proc_macro2::TokenStream> = (1..(3 * self.nsessions))
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("R{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let all_roles_struct: Vec<proc_macro2::TokenStream> = (1..(3 * self.nsessions))
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("R{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident : mpstthree::role::Role + 'a ,
                }
            })
            .collect();

        let all_types: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let types_left: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
                    .map(|j| {
                        let (k, _, m) = self.get_tuple_matrix(&matrix, i, j);

                        let temp_ident =
                            syn::Ident::new(&format!("S{}", m), proc_macro2::Span::call_site());

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

                let types_right: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
                    .map(|j| {
                        let (k, _, m) = self.get_tuple_matrix(&matrix, i, j);

                        let temp_ident = syn::Ident::new(
                            &format!("S{}", diff * (diff + 1) / 2 + m),
                            proc_macro2::Span::call_site(),
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

                let temp_roles: Vec<proc_macro2::TokenStream> = (1..4)
                    .map(|j| {
                        let temp_ident = syn::Ident::new(
                            &format!("R{}", 3 * (i - 1) + j),
                            proc_macro2::Span::call_site(),
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

        let stacks: Vec<proc_macro2::TokenStream> = ((3 * self.nsessions - 2)
            ..(3 * self.nsessions))
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("R{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let sessions_struct: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let sum: u64 = (0..i).map(|j| self.nsessions - 1 - j).sum();
                let temp_ident = syn::Ident::new(
                    &format!("S{}", diff * (diff + 1) / 2 + sum),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    < #temp_ident as mpstthree::binary::struct_trait::session::Session>::Dual,
                }
            })
            .collect();

        let role_right = syn::Ident::new(
            &format!("R{}", 3 * self.nsessions - 1),
            proc_macro2::Span::call_site(),
        );

        let stack_right = syn::Ident::new(
            &format!("stack_{}", self.nsessions),
            proc_macro2::Span::call_site(),
        );

        let name_right = syn::Ident::new(
            &format!("name_{}", self.nsessions),
            proc_macro2::Span::call_site(),
        );

        let new_channels: Vec<proc_macro2::TokenStream> = (1..=(diff * (diff + 1) / 2))
            .map(|i| {
                let (line, column, _) = self.get_tuple_diag(&diag, i);
                let channel_left = syn::Ident::new(
                    &format!("channel_{}_{}", line, column),
                    proc_macro2::Span::call_site(),
                );
                let channel_right = syn::Ident::new(
                    &format!("channel_{}_{}", column, line),
                    proc_macro2::Span::call_site(),
                );
                let temp_session = syn::Ident::new(
                    &format!("S{}", diff * (diff + 1) / 2 + i),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    let ( #channel_left , #channel_right ) = #temp_session::new();
                }
            })
            .collect();

        let new_stacks: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_stack =
                    syn::Ident::new(&format!("stack_{}", i), proc_macro2::Span::call_site());
                let temp_role = syn::Ident::new(
                    &format!("R{}", 3 * (i - 1) + 2),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    let (_, #temp_stack ) = #temp_role::new();
                }
            })
            .collect();

        let new_names: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_name =
                    syn::Ident::new(&format!("name_{}", i), proc_macro2::Span::call_site());
                let temp_role = syn::Ident::new(
                    &format!("R{}", 3 * (i - 1) + 3),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    let ( #temp_name , _) =
                        << #temp_role as mpstthree::role::Role>::Dual as mpstthree::role::Role>::new();
                }
            })
            .collect();

        let new_choices: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let types_sessions: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
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

                let temp_choice =
                    syn::Ident::new(&format!("choice_{}", i), proc_macro2::Span::call_site());

                let temp_stack =
                    syn::Ident::new(&format!("stack_{}", i), proc_macro2::Span::call_site());

                let temp_name =
                    syn::Ident::new(&format!("name_{}", i), proc_macro2::Span::call_site());

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

        let new_sessions_right: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
                .map(|i| {
                    let temp_new_session =
                        syn::Ident::new(&format!("new_session_{}", i), proc_macro2::Span::call_site());
                    let temp_session =
                        syn::Ident::new(&format!("session{}", i), proc_macro2::Span::call_site());
                    let temp_choice =
                        syn::Ident::new(&format!("choice_{}", i), proc_macro2::Span::call_site());
                    quote! {
                        let #temp_new_session =
                            mpstthree::binary::send::send(either::Either::Right(#temp_choice), s.#temp_session);
                    }
                })
                .collect();

        let old_session_meshedchannels: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_new_session = syn::Ident::new(
                    &format!("new_session_{}", i),
                    proc_macro2::Span::call_site(),
                );
                let temp_session =
                    syn::Ident::new(&format!("session{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_session : #temp_new_session ,
                }
            })
            .collect();

        let new_session_meshedchannels: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let (line, column, _) = self.get_tuple_matrix(&matrix, self.nsessions, i);
                let temp_channel = match line {
                    m if m == i => syn::Ident::new(
                        &format!("channel_{}_{}", column, line),
                        proc_macro2::Span::call_site(),
                    ),
                    _ => syn::Ident::new(
                        &format!("channel_{}_{}", line, column),
                        proc_macro2::Span::call_site(),
                    ),
                };
                let temp_session =
                    syn::Ident::new(&format!("session{}", i), proc_macro2::Span::call_site());
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
                #role_right ,
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

                let ( #stack_right , _) = < #role_right as mpstthree::role::Role>::new();

                #(
                    #new_names
                )*

                let ( #name_right , _) = <#name::<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

                #(
                    #new_choices
                )*

                #(
                    #new_sessions_right
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
                    stack: #stack_right ,
                    name: #name_right ,
                }
            }
        }
    }
}
