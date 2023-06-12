use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

use crate::common_functions::expand::name::name;
use crate::common_functions::expand::role::role;
use crate::common_functions::expand::role_timed::role_timed;

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

/// Create timed role structs
pub(crate) fn create_timed_role_structs(all_roles: &[TokenStream]) -> Vec<TokenStream> {
    all_roles
        .iter()
        .map(|i| role_timed(format!("{i}")))
        .collect()
}

/// Create name structs
pub(crate) fn create_name_structs(all_roles: &[TokenStream]) -> Vec<TokenStream> {
    all_roles.iter().map(|i| name(format!("{i}"))).collect()
}
