use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

/// Expand choose methods
pub(crate) fn cancel(meshedchannels_name: &Ident, number_roles: u64) -> TokenStream {
    let temp_types: Vec<TokenStream> = (1..number_roles)
        .map(|i| {
            let temp_session = Ident::new(&format!("S{i}"), Span::call_site());
            quote! { #temp_session , }
        })
        .collect();

    let temp_detail_types: Vec<TokenStream> = (1..number_roles)
        .map(|i| {
            let temp_session = Ident::new(&format!("S{i}"), Span::call_site());
            quote! { #temp_session : mpstthree::binary::struct_trait::session::Session , }
        })
        .collect();

    quote! {
        impl<
            #(
                #temp_detail_types
            )*
            R: mpstthree::role::Role,
            N: mpstthree::name::Name,
        >
            #meshedchannels_name<
                #(
                    #temp_types
                )*
                R,
                N
            >
        {
            /// Cancel the session
            pub fn cancel(self) {
                std::mem::drop(self);
            }
        }
    }
}
