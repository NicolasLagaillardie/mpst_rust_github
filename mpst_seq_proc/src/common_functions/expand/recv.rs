use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::convert::TryFrom;
use syn::Ident;

/// Expand receive methods
pub(crate) fn recv(
    all_roles: &[TokenStream],
    receiver: u64,
    sender: u64,
    session_types: &[Ident],
    session_types_struct: &[TokenStream],
    meshedchannels_name: &Ident,
    number_roles: u64,
) -> TokenStream {
    let sender_ident = if let Some(elt) = all_roles.get(usize::try_from(sender - 1).unwrap()) {
        Ident::new(&format!("Role{}", elt), Span::call_site())
    } else {
        panic!("Not enough arguments for sender_ident in expand_recv")
    };

    let receiver_ident = if let Some(elt) = all_roles.get(usize::try_from(receiver - 1).unwrap()) {
        Ident::new(&format!("Name{}", elt), Span::call_site())
    } else {
        panic!("Not enough arguments for receiver_ident in expand_recv")
    };

    let send_sessions: Vec<TokenStream> = (1..number_roles)
        .map(|k| {
            let cond = if k >= receiver { sender - 1 } else { sender };

            let temp_type = Ident::new(&format!("S{}", k), Span::call_site());

            if k == cond {
                quote! { mpstthree::binary::struct_trait::recv::Recv<T, #temp_type > ,}
            } else {
                quote! { #temp_type ,}
            }
        })
        .collect();

    let new_sessions: Vec<TokenStream> = (1..number_roles)
        .map(|k| {
            let cond = if k >= receiver { sender - 1 } else { sender };

            let temp_session = Ident::new(&format!("session{}", k), Span::call_site());

            if k == cond {
                quote! { #temp_session : new_session , }
            } else {
                quote! { #temp_session : self.#temp_session , }
            }
        })
        .collect();

    let index = if sender >= receiver {
        sender - 1
    } else {
        sender
    };

    let new_session = Ident::new(&format!("session{}", index), Span::call_site());

    quote! {
        impl<#( #session_types_struct )* R: mpstthree::role::Role, T: std::marker::Send>
            #meshedchannels_name<
                #( #send_sessions )*
                #sender_ident<R>,
                #receiver_ident
            >
        {
            pub fn recv(self) -> Result<(
                T,
                #meshedchannels_name<
                    #( #session_types , )*
                    R,
                    #receiver_ident
                >),
                Box<dyn std::error::Error>
            > {
                let (v, new_session) = mpstthree::binary::recv::recv(self.#new_session)?;
                let new_stack = self.stack.continuation();
                Ok((
                    v,
                    #meshedchannels_name {
                        #( #new_sessions )*
                        stack: new_stack,
                        name: self.name,
                    }
                ))
            }
        }
    }
}

/// Expand receive from all methods
pub(crate) fn recv_from_all(
    all_roles: &[TokenStream],
    receiver: u64,
    sender: u64,
    session_types: &[Ident],
    session_types_struct: &[TokenStream],
    meshedchannels_name: &Ident,
    number_roles: u64,
) -> TokenStream {
    let sender_ident = if let Some(elt) = all_roles.get(usize::try_from(sender - 1).unwrap()) {
        Ident::new(&format!("RoleAllto{}", elt), Span::call_site())
    } else {
        panic!("Not enough arguments for sender_ident in expand_recv_from_all")
    };

    let receiver_ident = if let Some(elt) = all_roles.get(usize::try_from(receiver - 1).unwrap()) {
        Ident::new(&format!("Name{}", elt), Span::call_site())
    } else {
        panic!("Not enough arguments for receiver_ident in expand_recv_from_all")
    };

    let send_sessions: Vec<TokenStream> = (1..number_roles)
        .map(|k| {
            let cond = if k >= receiver { sender - 1 } else { sender };

            let temp_type = Ident::new(&format!("S{}", k), Span::call_site());

            if k == cond {
                quote! { mpstthree::binary::struct_trait::recv::Recv<T, #temp_type > ,}
            } else {
                quote! { #temp_type ,}
            }
        })
        .collect();

    let new_sessions: Vec<TokenStream> = (1..number_roles)
        .map(|k| {
            let cond = if k >= receiver { sender - 1 } else { sender };

            let temp_session = Ident::new(&format!("session{}", k), Span::call_site());

            if k == cond {
                quote! { #temp_session : new_session , }
            } else {
                quote! { #temp_session : self.#temp_session , }
            }
        })
        .collect();

    let index = if sender >= receiver {
        sender - 1
    } else {
        sender
    };

    let new_session = Ident::new(&format!("session{}", index), Span::call_site());

    quote! {
        impl<#( #session_types_struct )* T: std::marker::Send>
            #meshedchannels_name<
                #( #send_sessions )*
                #sender_ident<mpstthree::role::end::RoleEnd, mpstthree::role::end::RoleEnd>,
                #receiver_ident
            >
        {
            pub fn recv_from_all(self) -> Result<(
                T,
                #meshedchannels_name<
                    #( #session_types , )*
                    mpstthree::role::end::RoleEnd,
                    #receiver_ident
                >),
                Box<dyn std::error::Error>
            > {
                let (v, new_session) = mpstthree::binary::recv::recv(self.#new_session)?;

                let new_stack = self.stack.continuation_left();

                Ok((
                    v,
                    #meshedchannels_name {
                        #( #new_sessions )*
                        stack: new_stack,
                        name: self.name,
                    }
                ))
            }
        }
    }
}
