use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result, Token};

use crate::common_functions::expand::choose::choose;
use crate::common_functions::expand::close::close;
use crate::common_functions::expand::fork::fork_mpst;
use crate::common_functions::expand::name::name;
use crate::common_functions::expand::offer::offer;
use crate::common_functions::expand::recv::{recv, recv_from_all};
use crate::common_functions::expand::role::role;
use crate::common_functions::expand::send::send_basic;
use crate::common_functions::expand::token_stream::token_stream;

#[derive(Debug)]
pub(crate) struct Baking {
    meshedchannels_name: Ident,
    all_roles: Vec<TokenStream>,
    number_roles: u64,
}

impl Parse for Baking {
    fn parse(input: ParseStream) -> Result<Self> {
        let meshedchannels_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;
        let all_roles = token_stream(<&syn::parse::ParseBuffer>::clone(&input))?;

        let number_roles = u64::try_from(all_roles.len()).unwrap();

        Ok(Baking {
            meshedchannels_name,
            all_roles,
            number_roles,
        })
    }
}

impl From<Baking> for TokenStream {
    fn from(input: Baking) -> TokenStream {
        input.expand()
    }
}

impl Baking {
    fn expand(&self) -> TokenStream {
        let meshedchannels_name = self.meshedchannels_name.clone();

        // Get all the roles provided into a Vec
        let all_roles = self.all_roles.clone();

        let quote_fork_mpst = fork_mpst(meshedchannels_name.clone(), self.number_roles);

        let session_types: Vec<Ident> = (1..self.number_roles)
            .map(|i| Ident::new(&format!("S{}", i), Span::call_site()))
            .collect();

        let session_types_struct: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                quote! { #temp_ident : mpstthree::binary::struct_trait::session::Session , }
            })
            .collect();

        let session_types_dual_struct: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_ident =
                    Ident::new(&format!("S{}", i), Span::call_site());
                quote! { <#temp_ident as mpstthree::binary::struct_trait::session::Session>::Dual , }
            })
            .collect();

        let session_types_pub: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_session = Ident::new(&format!("session{}", i), Span::call_site());
                let temp_type = Ident::new(&format!("S{}", i), Span::call_site());
                quote! { pub #temp_session : #temp_type , }
            })
            .collect();

        let sender_receiver: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_sender = Ident::new(&format!("sender{}", i), Span::call_site());
                let temp_receiver = Ident::new(&format!("receiver{}", i), Span::call_site());
                let temp_type = Ident::new(&format!("S{}", i), Span::call_site());
                quote! { let ( #temp_sender , #temp_receiver ) =
                <#temp_type as mpstthree::binary::struct_trait::session::Session>::new() ; }
            })
            .collect();

        let sender_struct: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_session = Ident::new(&format!("session{}", i), Span::call_site());
                let temp_sender = Ident::new(&format!("sender{}", i), Span::call_site());
                quote! { #temp_session : #temp_sender , }
            })
            .collect();

        let receiver_struct: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_session = Ident::new(&format!("session{}", i), Span::call_site());
                let temp_receiver = Ident::new(&format!("receiver{}", i), Span::call_site());
                quote! { #temp_session : #temp_receiver , }
            })
            .collect();

        let head_str: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_ident =
                    Ident::new(&format!("S{}", i), Span::call_site());
                quote! {
                    if result.is_empty() {
                        result = format!(
                            "{}",
                            <#temp_ident as mpstthree::binary::struct_trait::session::Session>::head_str()
                        ) ;
                    } else {
                        result = format!(
                            "{}\n{}",
                            result,
                            <#temp_ident as mpstthree::binary::struct_trait::session::Session>::head_str()
                        );
                    }
                }
            })
            .collect();

        let tail_str: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_ident =
                    Ident::new(&format!("S{}", i), Span::call_site());
                quote! {
                    if result.is_empty() {
                        result = format!(
                            "{}<{}>",
                            <#temp_ident as mpstthree::binary::struct_trait::session::Session>::head_str(),
                            <#temp_ident as mpstthree::binary::struct_trait::session::Session>::tail_str()
                        ) ;
                    } else {
                        result = format!(
                            "{}\n{}<{}>",
                            result,
                            <#temp_ident as mpstthree::binary::struct_trait::session::Session>::head_str(),
                            <#temp_ident as mpstthree::binary::struct_trait::session::Session>::tail_str()
                        ) ;
                    }
                }
            })
            .collect();

        let stringify: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_session = Ident::new(&format!("session{}", i), Span::call_site());
                quote! { stringify!( #temp_session ) , }
            })
            .collect();

        let roles_struct: Vec<TokenStream> =
            all_roles.iter().map(|i| role(format!("{}", i))).collect();

        let names_struct: Vec<TokenStream> =
            all_roles.iter().map(|i| name(format!("{}", i))).collect();

        let send_methods: Vec<TokenStream> = (1..=self.number_roles)
            .map(|sender| {
                (1..=self.number_roles)
                    .filter_map(|receiver| {
                        if sender != receiver {
                            Some(send_basic(
                                all_roles.clone(),
                                sender,
                                receiver,
                                session_types.clone(),
                                session_types_struct.clone(),
                                meshedchannels_name.clone(),
                                self.number_roles,
                            ))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let recv_methods: Vec<TokenStream> = (1..=self.number_roles)
            .map(|receiver| {
                (1..=self.number_roles)
                    .filter_map(|sender| {
                        if receiver != sender {
                            Some(recv(
                                all_roles.clone(),
                                receiver,
                                sender,
                                session_types.clone(),
                                session_types_struct.clone(),
                                meshedchannels_name.clone(),
                                self.number_roles,
                            ))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let recv_from_all_methods: Vec<TokenStream> = (1..=self.number_roles)
            .map(|receiver| {
                (1..=self.number_roles)
                    .filter_map(|sender| {
                        if receiver != sender {
                            Some(recv_from_all(
                                all_roles.clone(),
                                receiver,
                                sender,
                                session_types.clone(),
                                session_types_struct.clone(),
                                meshedchannels_name.clone(),
                                self.number_roles,
                            ))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let offer_methods: Vec<TokenStream> = (1..=self.number_roles)
            .map(|receiver| {
                (1..=self.number_roles)
                    .filter_map(|sender| {
                        if receiver != sender {
                            Some(offer(
                                all_roles.clone(),
                                sender,
                                receiver,
                                meshedchannels_name.clone(),
                                self.number_roles,
                            ))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let choose_methods: Vec<TokenStream> = (1..=self.number_roles)
            .map(|sender| {
                choose(
                    all_roles.clone(),
                    sender,
                    meshedchannels_name.clone(),
                    self.number_roles,
                )
            })
            .collect();

        let close_methods: Vec<TokenStream> = (1..=self.number_roles)
            .map(|sender| {
                close(
                    all_roles.clone(),
                    sender,
                    meshedchannels_name.clone(),
                    self.number_roles,
                )
            })
            .collect();

        quote! {
            #[must_use]
            #[derive(Debug)]
            pub(crate) struct  #meshedchannels_name<
                #( #session_types , )*
                R,
                N
            >
            where
                #( #session_types_struct )*
                R: mpstthree::role::Role,
                N: mpstthree::name::Name
            {
                #( #session_types_pub )*
                pub stack: R,
                pub name: N,
            }
            #[doc(hidden)]
            impl<
                #( #session_types_struct )*
                R: mpstthree::role::Role,
                N: mpstthree::name::Name
            > mpstthree::binary::struct_trait::session::Session for #meshedchannels_name<
                #(
                    #session_types , )*
                    R,
                    N
                > {
                type Dual =
                #meshedchannels_name<
                    #( #session_types_dual_struct )*
                    <R as mpstthree::role::Role>::Dual,
                    N,
                >;

                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    #( #sender_receiver )*

                    let (role_one, role_two) = R::new();
                    let (name_one, name_two) = N::new();
                    (
                        #meshedchannels_name {
                            #( #sender_struct )*
                            stack: role_one,
                            name: name_one,
                        },
                        #meshedchannels_name {
                            #( #receiver_struct )*
                            stack: role_two,
                            name: name_two,
                        }
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    let mut result = "".to_string();
                    #( #head_str )*
                    format!(
                        "{}\n{}\n{}",
                        result,
                        <R as mpstthree::role::Role>::head_str(),
                        <N as mpstthree::role::Role>::head_str()
                    )
                }

                #[doc(hidden)]
                fn tail_str() -> String {
                    let mut result = "".to_string();
                    #( #tail_str )*
                    format!(
                        "{}\n{}<{}>\n{}<{}>",
                        result,
                        <R as mpstthree::role::Role>::head_str(),
                        <R as mpstthree::role::Role>::tail_str(),
                        <N as mpstthree::role::Role>::head_str(),
                        <N as mpstthree::role::Role>::tail_str()
                    )
                }

                #[doc(hidden)]
                fn self_head_str(&self) -> String {
                    let mut result = "".to_string();
                    #( #head_str )*
                    format!(
                        "{}\n{}\n{}",
                        result,
                        <R as mpstthree::role::Role>::head_str(),
                        <N as mpstthree::role::Role>::head_str()
                    )
                }

                #[doc(hidden)]
                fn self_tail_str(&self) -> String {
                    let mut result = "".to_string();
                    #( #tail_str )*
                    format!(
                        "{}\n{}<{}>\n{}<{}>",
                        result,
                        <R as mpstthree::role::Role>::head_str(),
                        <R as mpstthree::role::Role>::tail_str(),
                        <N as mpstthree::role::Role>::head_str(),
                        <N as mpstthree::role::Role>::tail_str()
                    )
                }
            }
            #[doc(hidden)]
            impl<
                    #( #session_types_struct )*
                    R: mpstthree::role::Role,
                    N: mpstthree::name::Name
                > #meshedchannels_name<#( #session_types , )* R, N> {
                #[doc(hidden)]
                pub fn field_names(self) ->
                    (
                        &'static [&'static str],
                        #meshedchannels_name<#( #session_types , )* R, N>
                    ) {
                    (
                        &[
                            #( #stringify )*
                        ],
                        self
                    )
                }
            }

            #( #roles_struct )*

            #( #names_struct )*

            #( #send_methods )*

            #( #recv_methods )*

            #( #recv_from_all_methods )*

            #( #offer_methods )*

            #( #choose_methods )*

            #( #close_methods )*

            #[allow(unused_macros)]
            macro_rules! offer_mpst {
                ($session: expr, { $( $pat: pat => $result: expr, )+ }) => {
                    (move || -> Result<_, _> {
                        let (l, s) = $session.recv()?;
                        mpstthree::binary::cancel::cancel(s);
                        match l {
                            $(
                                $pat => $result,
                            )+
                            _ => panic!("Unexpected payload") ,
                        }
                    })()
                };
            }

            #quote_fork_mpst

        }
    }
}
