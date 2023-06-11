use crate::common_functions::expand::role::role;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

/// Create session types
pub(crate) fn create_session_types(from: u64, to: u64) -> Vec<Ident> {
    (from..to)
        .map(|i| Ident::new(&format!("S{i}"), Span::call_site()))
        .collect()
}

/// Create session type structs
pub(crate) fn create_session_type_structs(from: u64, to: u64) -> Vec<TokenStream> {
    (from..to)
        .map(|i| {
            let temp_ident = Ident::new(&format!("S{i}"), Span::call_site());
            quote! { #temp_ident : mpstthree::binary::struct_trait::session::Session , }
        })
        .collect()
}

/// Create role structs
pub(crate) fn create_role_structs(all_roles: &[TokenStream]) -> Vec<TokenStream> {
    all_roles.iter().map(|i| role(format!("{i}"))).collect()
}
