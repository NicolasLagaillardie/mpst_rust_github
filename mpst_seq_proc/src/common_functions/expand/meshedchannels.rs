use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

/// Expand choose methods
pub(crate) fn meshedchannels(meshedchannels_name: &Ident, number_roles: u64) -> TokenStream {
    let session_types: Vec<Ident> = (1..number_roles)
        .map(|i| Ident::new(&format!("S{}", i), Span::call_site()))
        .collect();

    let session_types_struct: Vec<TokenStream> = (1..number_roles)
        .map(|i| {
            let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
            quote! { #temp_ident : mpstthree::binary::struct_trait::session::Session , }
        })
        .collect();

    let session_types_pub: Vec<TokenStream> = (1..number_roles)
        .map(|i| {
            let temp_session = Ident::new(&format!("session{}", i), Span::call_site());
            let temp_type = Ident::new(&format!("S{}", i), Span::call_site());
            quote! { pub #temp_session : #temp_type , }
        })
        .collect();

    let session_types_dual_struct: Vec<TokenStream> = (1..number_roles)
        .map(|i| {
            let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
            quote! { <#temp_ident as mpstthree::binary::struct_trait::session::Session>::Dual , }
        })
        .collect();

    let sender_receiver: Vec<TokenStream> = (1..number_roles)
        .map(|i| {
            let temp_sender = Ident::new(&format!("sender{}", i), Span::call_site());
            let temp_receiver = Ident::new(&format!("receiver{}", i), Span::call_site());
            let temp_type = Ident::new(&format!("S{}", i), Span::call_site());
            quote! { let ( #temp_sender , #temp_receiver ) =
            <#temp_type as mpstthree::binary::struct_trait::session::Session>::new() ; }
        })
        .collect();

    let sender_struct: Vec<TokenStream> = (1..number_roles)
        .map(|i| {
            let temp_session = Ident::new(&format!("session{}", i), Span::call_site());
            let temp_sender = Ident::new(&format!("sender{}", i), Span::call_site());
            quote! { #temp_session : #temp_sender , }
        })
        .collect();

    let receiver_struct: Vec<TokenStream> = (1..number_roles)
        .map(|i| {
            let temp_session = Ident::new(&format!("session{}", i), Span::call_site());
            let temp_receiver = Ident::new(&format!("receiver{}", i), Span::call_site());
            quote! { #temp_session : #temp_receiver , }
        })
        .collect();

    let tail_str: Vec<TokenStream> = (1..number_roles)
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

    let head_str: Vec<TokenStream> = (1..number_roles)
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

    let stringify: Vec<TokenStream> = (1..number_roles)
        .map(|i| {
            let temp_session = Ident::new(&format!("session{}", i), Span::call_site());
            quote! { stringify!( #temp_session ) , }
        })
        .collect();

    quote! {
        #[must_use]
        #[derive(Debug)]
        pub struct #meshedchannels_name<
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
            #( #session_types , )*
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
                let (name_one, _) = N::new();
                let (name_two, _) = N::new();
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
                    <N as mpstthree::name::Name>::head_str()
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
                    <N as mpstthree::name::Name>::head_str(),
                    <N as mpstthree::name::Name>::tail_str()
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
                    <N as mpstthree::name::Name>::head_str()
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
                    <N as mpstthree::name::Name>::head_str(),
                    <N as mpstthree::name::Name>::tail_str()
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
    }
}
