use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Token};

use crate::common_functions::maths::{diag_and_matrix, get_tuple_diag, get_tuple_matrix};

#[derive(Debug)]
pub(crate) struct ChooseTypeMultiRight {
    func_name: Ident,
    type_name: Ident,
    role_dual: Ident,
    name: Ident,
    meshedchannels_name: Ident,
    n_sessions: u64,
}

impl Parse for ChooseTypeMultiRight {
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

        Ok(ChooseTypeMultiRight {
            func_name,
            type_name,
            role_dual,
            name,
            meshedchannels_name,
            n_sessions,
        })
    }
}

impl From<ChooseTypeMultiRight> for TokenStream {
    fn from(input: ChooseTypeMultiRight) -> TokenStream {
        input.expand()
    }
}

impl ChooseTypeMultiRight {
    fn expand(&self) -> TokenStream {
        let func_name = &self.func_name;
        let type_name = &self.type_name;
        let role_dual = &self.role_dual;
        let name = &self.name;
        let meshedchannels_name = &self.meshedchannels_name;
        let diff = self.n_sessions - 1;
        let (matrix, diag) = diag_and_matrix(self.n_sessions);

        let all_sessions: Vec<TokenStream> = (1..=(diff * (diff + 1)))
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let all_sessions_struct: Vec<TokenStream> = (1..=(diff * (diff + 1)))
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                quote! {
                    #temp_ident : mpstthree::binary::struct_trait::session::Session + 'a ,
                }
            })
            .collect();

        let all_roles: Vec<TokenStream> = (1..(3 * self.n_sessions))
            .map(|i| {
                if i % 3 == 0 {
                    let temp_ident = Ident::new(&format!("N{}", i), Span::call_site());
                    quote! {
                        #temp_ident ,
                    }
                } else {
                    let temp_ident = Ident::new(&format!("R{}", i), Span::call_site());
                    quote! {
                        #temp_ident ,
                    }
                }
            })
            .collect();

        let all_roles_struct_and_struct: Vec<TokenStream> = (1..(3 * self.n_sessions))
            .map(|i| {
                if i % 3 == 0 {
                    let temp_ident = Ident::new(&format!("N{}", i), Span::call_site());
                    quote! {
                        #temp_ident : mpstthree::name::Name + 'a ,
                    }
                } else {
                    let temp_ident = Ident::new(&format!("R{}", i), Span::call_site());
                    quote! {
                        #temp_ident : mpstthree::role::Role + 'a ,
                    }
                }
            })
            .collect();

        let all_types: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let types_left: Vec<TokenStream> = (1..self.n_sessions)
                    .map(|j| {
                        let (k, _, m) = get_tuple_matrix(&matrix, i, j);

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
                        let (k, _, m) = get_tuple_matrix(&matrix, i, j);

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

                let temp_roles: Vec<TokenStream> = (1..3)
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

                let temp_names: TokenStream = {
                    let temp_ident = Ident::new(
                        &format!("N{}", 3 * (i - 1) + 3),
                        Span::call_site(),
                    );

                    quote! {
                        #temp_ident ,
                    }
                };

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
                        #temp_names
                    >,
                }
            })
            .collect();

        let stacks: Vec<TokenStream> = ((3 * self.n_sessions - 2)..(3 * self.n_sessions))
            .map(|i| {
                let temp_ident = Ident::new(&format!("R{}", i), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let sessions_struct: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let sum: u64 = (0..i).map(|j| self.n_sessions - 1 - j).sum();
                let temp_ident = Ident::new(
                    &format!("S{}", diff * (diff + 1) / 2 + sum),
                    Span::call_site(),
                );
                quote! {
                    < #temp_ident as mpstthree::binary::struct_trait::session::Session>::Dual,
                }
            })
            .collect();

        let role_right = Ident::new(&format!("R{}", 3 * self.n_sessions - 1), Span::call_site());

        let stack_right = Ident::new(&format!("stack_{}", self.n_sessions), Span::call_site());

        let name_right = Ident::new(&format!("name_{}", self.n_sessions), Span::call_site());

        let new_channels: Vec<TokenStream> = (1..=(diff * (diff + 1) / 2))
            .map(|i| {
                let (line, column, _) = get_tuple_diag(&diag, i);
                let channel_left =
                    Ident::new(&format!("channel_{}_{}", line, column), Span::call_site());
                let channel_right =
                    Ident::new(&format!("channel_{}_{}", column, line), Span::call_site());
                let temp_session = Ident::new(
                    &format!("S{}", diff * (diff + 1) / 2 + i),
                    Span::call_site(),
                );
                quote! {
                    let ( #channel_left , #channel_right ) = < #temp_session as mpstthree::binary::struct_trait::session::Session >::new();
                }
            })
            .collect();

        let new_stacks: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let temp_stack = Ident::new(&format!("stack_{}", i), Span::call_site());
                let temp_role = Ident::new(&format!("R{}", 3 * (i - 1) + 2), Span::call_site());
                quote! {
                    let (_, #temp_stack ) = < #temp_role as mpstthree::role::Role >::new();
                }
            })
            .collect();

        let new_names: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let temp_name = Ident::new(&format!("name_{}", i), Span::call_site());
                let temp_role = Ident::new(&format!("N{}", 3 * (i - 1) + 3), Span::call_site());
                quote! {
                    let ( #temp_name , _) = < #temp_role as mpstthree::name::Name >::new();
                }
            })
            .collect();

        let new_choices: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let types_sessions: Vec<TokenStream> = (1..self.n_sessions)
                    .map(|j| {
                        let (line, column, _) = get_tuple_matrix(&matrix, i, j);

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

                let temp_choice = Ident::new(&format!("choice_{}", i), Span::call_site());

                let temp_stack = Ident::new(&format!("stack_{}", i), Span::call_site());

                let temp_name = Ident::new(&format!("name_{}", i), Span::call_site());

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

        let new_sessions_right: Vec<TokenStream> = (1..self.n_sessions)
                .map(|i| {
                    let temp_new_session =
                        Ident::new(&format!("new_session_{}", i), Span::call_site());
                    let temp_session =
                        Ident::new(&format!("session{}", i), Span::call_site());
                    let temp_choice =
                        Ident::new(&format!("choice_{}", i), Span::call_site());
                    quote! {
                        let #temp_new_session =
                            mpstthree::binary::send::send(either::Either::Right(#temp_choice), s.#temp_session);
                    }
                })
                .collect();

        let old_session_meshedchannels: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let temp_new_session = Ident::new(&format!("new_session_{}", i), Span::call_site());
                let temp_session = Ident::new(&format!("session{}", i), Span::call_site());
                quote! {
                    #temp_session : #temp_new_session ,
                }
            })
            .collect();

        let new_session_meshedchannels: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let (line, column, _) = get_tuple_matrix(&matrix, self.n_sessions, i);
                let temp_channel = match line {
                    m if m == i => {
                        Ident::new(&format!("channel_{}_{}", column, line), Span::call_site())
                    }
                    _ => Ident::new(&format!("channel_{}_{}", line, column), Span::call_site()),
                };
                let temp_session = Ident::new(&format!("session{}", i), Span::call_site());
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

                    #name,
                >,
            )
            -> #meshedchannels_name<
                #(
                    #sessions_struct
                )*
                #role_right ,
                #name
            >
            where
                #(
                    #all_sessions_struct
                )*

                #(
                    #all_roles_struct_and_struct
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

                let ( #name_right , _) = < #name as mpstthree::name::Name>::new();

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
