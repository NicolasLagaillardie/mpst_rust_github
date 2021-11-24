use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct CreateMeshedChannels {
    meshedchannels_name: syn::Ident,
    nsessions: u64,
}

impl Parse for CreateMeshedChannels {
    fn parse(input: ParseStream) -> Result<Self> {
        let meshedchannels_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let nsessions = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(CreateMeshedChannels {
            meshedchannels_name,
            nsessions,
        })
    }
}

impl From<CreateMeshedChannels> for proc_macro2::TokenStream {
    fn from(input: CreateMeshedChannels) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl CreateMeshedChannels {
    fn expand(&self) -> proc_macro2::TokenStream {
        let meshedchannels_name = self.meshedchannels_name.clone();

        let sessions: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let sessions_dual: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                quote! {
                    <
                        #temp_ident as mpstthree::binary::struct_trait::session::Session
                    >::Dual,
                }
            })
            .collect();

        let sessions_struct: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident : mpstthree::binary::struct_trait::session::Session,
                }
            })
            .collect();

        let sessions_pub: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                let temp_field =
                    syn::Ident::new(&format!("session{}", i), proc_macro2::Span::call_site());
                quote! {
                    pub #temp_field :  #temp_ident,
                }
            })
            .collect();

        let senders_receivers: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_sender =
                    syn::Ident::new(&format!("sender{}", i), proc_macro2::Span::call_site());
                let temp_receiver =
                    syn::Ident::new(&format!("receiver{}", i), proc_macro2::Span::call_site());
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                quote! {
                    let ( #temp_sender , #temp_receiver ) = #temp_ident::new();
                }
            })
            .collect();

        let senders_sessions: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_sender =
                    syn::Ident::new(&format!("sender{}", i), proc_macro2::Span::call_site());
                let temp_field =
                    syn::Ident::new(&format!("session{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_field : #temp_sender ,
                }
            })
            .collect();

        let receivers_sessions: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_receiver =
                    syn::Ident::new(&format!("receiver{}", i), proc_macro2::Span::call_site());
                let temp_field =
                    syn::Ident::new(&format!("session{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_field : #temp_receiver ,
                }
            })
            .collect();

        let head_str: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
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

        let tail_str: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
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

        let stringify: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("session{}", i), proc_macro2::Span::call_site());
                quote! {
                    stringify!(#temp_ident),
                }
            })
            .collect();

        quote! {
            #[must_use]
            #[derive(Debug)]
            pub struct #meshedchannels_name<
                #(
                    #sessions
                )*
                R,
                N
            >
            where
                #(
                    #sessions_struct
                )*
                R: mpstthree::role::Role,
                N: mpstthree::role::Role
            {
                #(
                    #sessions_pub
                )*
                pub stack: R,
                pub name: N,
            }

            #[doc(hidden)]
            impl<
                #(
                    #sessions_struct
                )*
                R: mpstthree::role::Role,
                N: mpstthree::role::Role
            > mpstthree::binary::struct_trait::session::Session for #meshedchannels_name<
                #(
                    #sessions
                )*
                R,
                N
            > {
                type Dual =
                #meshedchannels_name<
                    #(
                        #sessions_dual
                    )*
                    <R as mpstthree::role::Role>::Dual,
                    <N as mpstthree::role::Role>::Dual,
                >;

                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    #(
                        #senders_receivers
                    )*

                    let (role_one, role_two) = R::new();
                    let (name_one, name_two) = N::new();

                    (
                        #meshedchannels_name {
                            #(
                                #senders_sessions
                            )*
                            stack: role_one,
                            name: name_one,
                        },
                        #meshedchannels_name {
                            #(
                                #receivers_sessions
                            )*
                            stack: role_two,
                            name: name_two,
                        }
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    let mut result = "".to_string();
                    #(
                        #head_str
                    )*
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
                    #(
                        #tail_str
                    )*
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
                    #(
                        #head_str
                    )*
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
                    #(
                        #tail_str
                    )*
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
                #(
                    #sessions_struct
                )*
                R: mpstthree::role::Role,
                N: mpstthree::role::Role
            > #meshedchannels_name<
                #(
                    #sessions
                )*
                R,
                N
            > {
                #[doc(hidden)]
                pub fn field_names(self) ->
                    (
                        &'static [&'static str],
                        #meshedchannels_name<
                            #(
                                #sessions
                            )*
                            R,
                            N
                        >
                    ) {
                    (
                        &[
                            #(
                                #stringify
                            )*
                        ],
                        self
                    )
                }
            }
        }
    }
}
