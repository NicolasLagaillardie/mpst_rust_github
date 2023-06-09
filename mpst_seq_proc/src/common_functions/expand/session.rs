use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

// Common sessions
pub(crate) fn create_session(from: u64, to: u64) -> Vec<TokenStream> {
    (from..=to)
        .map(|i| {
            let temp_ident = Ident::new(&format!("S{i}"), Span::call_site());
            quote! {
                #temp_ident ,
            }
        })
        .collect()
}

// Common session structs
pub(crate) fn create_session_struct(from: u64, to: u64) -> Vec<TokenStream> {
    (from..=to)
        .map(|i| {
            let temp_ident = Ident::new(&format!("S{i}"), Span::call_site());
            quote! {
                #temp_ident : mpstthree::binary::struct_trait::session::Session + 'static ,
            }
        })
        .collect()
}

// Common roles
pub(crate) fn create_roles(from: u64, to: u64) -> Vec<TokenStream> {
    (from..=to)
        .map(|i| {
            let temp_ident = Ident::new(&format!("R{i}"), Span::call_site());
            quote! {
                #temp_ident ,
            }
        })
        .collect()
}
