use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

use crate::common_functions::maths::{get_tuple_diag, get_tuple_matrix};

/// Create sessions
pub(crate) fn create_sessions(from: u64, to: u64) -> Vec<TokenStream> {
    (from..=to)
        .map(|i| {
            let temp_ident = Ident::new(&format!("S{i}"), Span::call_site());
            quote! {
                #temp_ident ,
            }
        })
        .collect()
}

/// Create session structs
pub(crate) fn create_session_structs(from: u64, to: u64) -> Vec<TokenStream> {
    (from..=to)
        .map(|i| {
            let temp_ident = Ident::new(&format!("S{i}"), Span::call_site());
            quote! {
                #temp_ident : mpstthree::binary::struct_trait::session::Session + 'static ,
            }
        })
        .collect()
}

/// Create roles
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

/// Create role struct
pub(crate) fn create_role_structs(from: u64, to: u64) -> Vec<TokenStream> {
    (from..=to)
        .map(|i| {
            let temp_ident = Ident::new(&format!("R{i}"), Span::call_site());
            quote! {
                #temp_ident : mpstthree::role::Role + 'static ,
            }
        })
        .collect()
}

/// Create role struct
pub(crate) fn create_new_roles(from: u64, to: u64) -> Vec<TokenStream> {
    (from..=to)
        .map(|i| {
            let temp_ident = Ident::new(&format!("R{i}"), Span::call_site());
            let temp_role = Ident::new(&format!("role_{i}"), Span::call_site());
            quote! {
                let ( #temp_role , _) = < #temp_ident as mpstthree::role::Role >::new() ;
            }
        })
        .collect()
}

/// Create names
pub(crate) fn create_names(from: u64, to: u64) -> Vec<TokenStream> {
    (from..=to)
        .map(|i| {
            let temp_ident = Ident::new(&format!("N{i}"), Span::call_site());
            quote! {
                #temp_ident ,
            }
        })
        .collect()
}

/// Create name structs
pub(crate) fn create_name_structs(from: u64, to: u64) -> Vec<TokenStream> {
    (from..=to)
        .map(|i| {
            let temp_ident = Ident::new(&format!("N{i}"), Span::call_site());
            quote! {
                #temp_ident : mpstthree::name::Name + 'static ,
            }
        })
        .collect()
}

/// Create new names
pub(crate) fn create_new_names(from: u64, to: u64) -> Vec<TokenStream> {
    (from..=to)
        .map(|i| {
            let temp_ident = Ident::new(&format!("N{i}"), Span::call_site());
            let temp_name = Ident::new(&format!("name_{i}"), Span::call_site());
            quote! {
                let ( #temp_name , _) = < #temp_ident as mpstthree::name::Name >::new() ;
            }
        })
        .collect()
}

/// Create functions
pub(crate) fn create_functions(from: u64, to: u64) -> Vec<TokenStream> {
    (from..=to)
        .map(|i| {
            let temp_ident = Ident::new(&format!("F{i}"), Span::call_site());
            quote! {
                #temp_ident ,
            }
        })
        .collect()
}

/// Create function details
pub(crate) fn create_function_details(from: u64, to: u64) -> Vec<TokenStream> {
    (from..=to)
        .map(|i| {
            let temp_ident = Ident::new(&format!("F{i}"), Span::call_site());
            let temp_expr = Ident::new(&format!("f{i}"), Span::call_site());
            quote! {
                #temp_expr : #temp_ident ,
            }
        })
        .collect()
}

/// Create join handle
pub(crate) fn create_join_handle(from: u64, to: u64) -> Vec<TokenStream> {
    (from..=to)
        .map(|_| {
            quote! {
                std::thread::JoinHandle<()> ,
            }
        })
        .collect()
}

/// Create new channels
pub(crate) fn create_new_channels(
    from: u64,
    to: u64,
    diag_w_offset: &[(u64, u64, u64)],
) -> Vec<TokenStream> {
    (from..=to)
        .map(|i| {
            let temp_ident = Ident::new(&format!("S{i}"), Span::call_site());
            let (line, column, _) = get_tuple_diag(diag_w_offset, i);
            let temp_channel_left =
                Ident::new(&format!("channel_{line}_{column}"), Span::call_site());
            let temp_channel_right =
                Ident::new(&format!("channel_{column}_{line}"), Span::call_site());
            quote! {
                let ( #temp_channel_left , #temp_channel_right ) =
                    < #temp_ident as mpstthree::binary::struct_trait::session::Session >::new();
            }
        })
        .collect()
}

/// Create new meshedchannels
pub(crate) fn create_new_meshedchannels(
    from: u64,
    to: u64,
    meshedchannels_name: &Ident,
    matrix: &[Vec<(u64, u64, u64)>],
) -> Vec<TokenStream> {
    (from..=to)
        .map(|i| {
            let temp_sessions: Vec<TokenStream> = (from..to)
                .map(|j| {
                    let (line, column, _) = get_tuple_matrix(matrix, i, j);
                    let temp_session = Ident::new(&format!("session{j}"), Span::call_site());
                    let temp_channel = match line {
                        m if m == i => {
                            Ident::new(&format!("channel_{line}_{column}"), Span::call_site())
                        }
                        _ => Ident::new(&format!("channel_{column}_{line}"), Span::call_site()),
                    };
                    quote! {
                        #temp_session : #temp_channel ,
                    }
                })
                .collect();

            let temp_meshedchannels = Ident::new(&format!("meshedchannels_{i}"), Span::call_site());
            let temp_role = Ident::new(&format!("role_{i}"), Span::call_site());
            let temp_name = Ident::new(&format!("name_{i}"), Span::call_site());
            quote! {
                let #temp_meshedchannels =
                    #meshedchannels_name {
                        #(
                            #temp_sessions
                        )*
                        stack: #temp_role ,
                        name: #temp_name ,
                    };
            }
        })
        .collect()
}
