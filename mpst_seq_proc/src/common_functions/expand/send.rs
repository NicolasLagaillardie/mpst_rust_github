use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::convert::TryFrom;
use syn::Ident;

/// Expand send methods for basic baking
pub(crate) fn send_basic(
    all_roles: &[TokenStream],
    sender: u64,
    receiver: u64,
    session_types: &[Ident],
    session_types_struct: &[TokenStream],
    meshedchannels_name: &Ident,
    number_roles: u64,
) -> TokenStream {
    let sender_ident = if let Some(elt) = all_roles.get(usize::try_from(sender - 1).unwrap()) {
        Ident::new(&format!("Name{}", elt), Span::call_site())
    } else {
        panic!("Not enough arguments for sender_ident in expand_send")
    };

    let receiver_ident = if let Some(elt) = all_roles.get(usize::try_from(receiver - 1).unwrap()) {
        Ident::new(&format!("Role{}", elt), Span::call_site())
    } else {
        panic!("Not enough arguments for receiver_ident in expand_send")
    };

    let send_sessions: Vec<TokenStream> = (1..number_roles)
        .map(|k| {
            let cond = if k >= sender { receiver - 1 } else { receiver };

            let temp_type = Ident::new(&format!("S{}", k), Span::call_site());

            if k == cond {
                quote! { mpstthree::binary::struct_trait::send::Send<T, #temp_type > , }
            } else {
                quote! { #temp_type , }
            }
        })
        .collect();

    let new_sessions: Vec<TokenStream> = (1..number_roles)
        .map(|k| {
            let cond = if k >= sender { receiver - 1 } else { receiver };

            let temp_session = Ident::new(&format!("session{}", k), Span::call_site());

            if k == cond {
                quote! { #temp_session : new_session , }
            } else {
                quote! { #temp_session : self.#temp_session , }
            }
        })
        .collect();

    let index = if receiver >= sender {
        receiver - 1
    } else {
        receiver
    };

    let new_session = Ident::new(&format!("session{}", index), Span::call_site());

    quote! {
        impl<#( #session_types_struct )* R: mpstthree::role::Role, T: std::marker::Send>
            #meshedchannels_name<
                #( #send_sessions )*
                #receiver_ident<R>,
                #sender_ident
            >
        {
            pub fn send(self, payload: T) -> #meshedchannels_name<
                #( #session_types , )*
                R,
                #sender_ident
            > {
                let new_session = mpstthree::binary::send::send(payload, self.#new_session);
                let new_stack = self.stack.continuation();
                #meshedchannels_name {
                    #( #new_sessions )*
                    stack: new_stack,
                    name: self.name,
                }
            }
        }
    }
}

/// Expand send methods for baking with cancelation
pub(crate) fn send_canceled(
    all_roles: &[TokenStream],
    sender: u64,
    receiver: u64,
    session_types: &[Ident],
    session_types_struct: &[TokenStream],
    meshedchannels_name: &Ident,
    number_roles: u64,
) -> TokenStream {
    let sender_ident = if let Some(elt) = all_roles.get(usize::try_from(sender - 1).unwrap()) {
        Ident::new(&format!("Name{}", elt), Span::call_site())
    } else {
        panic!("Not enough arguments for sender_ident in expand_send")
    };

    let receiver_ident = if let Some(elt) = all_roles.get(usize::try_from(receiver - 1).unwrap()) {
        Ident::new(&format!("Role{}", elt), Span::call_site())
    } else {
        panic!("Not enough arguments for receiver_ident in expand_send")
    };

    let send_sessions: Vec<TokenStream> = (1..number_roles)
        .map(|k| {
            let cond = if k >= sender { receiver - 1 } else { receiver };

            let temp_type = Ident::new(&format!("S{}", k), Span::call_site());

            if k == cond {
                quote! { mpstthree::binary::struct_trait::send::Send<T, #temp_type > , }
            } else {
                quote! { #temp_type , }
            }
        })
        .collect();

    let new_sessions: Vec<TokenStream> = (1..number_roles)
        .map(|k| {
            let cond = if k >= sender { receiver - 1 } else { receiver };

            let temp_session = Ident::new(&format!("session{}", k), Span::call_site());

            if k == cond {
                quote! { #temp_session : new_session , }
            } else {
                quote! { #temp_session : self.#temp_session , }
            }
        })
        .collect();

    let index = if receiver >= sender {
        receiver - 1
    } else {
        receiver
    };

    let new_session = Ident::new(&format!("session{}", index), Span::call_site());

    quote! {
        impl<#( #session_types_struct )* R: mpstthree::role::Role, T: std::marker::Send>
            #meshedchannels_name<
                #( #send_sessions )*
                #receiver_ident<R>,
                #sender_ident
            >
        {
            pub fn send(self, payload: T) -> std::result::Result<
                #meshedchannels_name<
                    #( #session_types , )*
                    R,
                    #sender_ident
                >,
                Box<dyn std::error::Error>
            > {
                let new_session = mpstthree::binary::send::send_canceled(payload, self.#new_session)?;
                let new_stack = self.stack.continuation();
                Ok(
                    #meshedchannels_name {
                        #( #new_sessions )*
                        stack: new_stack,
                        name: self.name,
                    }
                )
            }
        }
    }
}

/// Expand send methods for baking with cancelation and timed
pub(crate) fn send_timed_canceled(
    all_roles: &[TokenStream],
    sender: u64,
    receiver: u64,
    session_types: &[Ident],
    session_types_struct: &[TokenStream],
    meshedchannels_name: &Ident,
    number_roles: u64,
) -> TokenStream {
    let sender_ident = if let Some(elt) = all_roles.get(usize::try_from(sender - 1).unwrap()) {
        Ident::new(&format!("Name{}", elt), Span::call_site())
    } else {
        panic!("Not enough arguments for sender_ident in expand_send")
    };

    let receiver_ident = if let Some(elt) = all_roles.get(usize::try_from(receiver - 1).unwrap()) {
        Ident::new(&format!("Role{}", elt), Span::call_site())
    } else {
        panic!("Not enough arguments for receiver_ident in expand_send")
    };

    let send_sessions: Vec<TokenStream> = (1..number_roles)
        .map(|k| {
            let cond = if k >= sender { receiver - 1 } else { receiver };

            let temp_type = Ident::new(&format!("S{}", k), Span::call_site());

            if k == cond {
                quote! {
                    mpstthree::binary_timed::struct_trait::send::SendTimed<
                        T,
                        #temp_type,
                        CLOCK,
                        START,
                        INCLUDE_START,
                        END,
                        INCLUDE_END,
                        RESET
                    > ,
                }
            } else {
                quote! { #temp_type , }
            }
        })
        .collect();

    let new_sessions: Vec<TokenStream> = (1..number_roles)
        .map(|k| {
            let cond = if k >= sender { receiver - 1 } else { receiver };

            let temp_session = Ident::new(&format!("session{}", k), Span::call_site());

            if k == cond {
                quote! { #temp_session : new_session , }
            } else {
                quote! { #temp_session : self.#temp_session , }
            }
        })
        .collect();

    let index = if receiver >= sender {
        receiver - 1
    } else {
        receiver
    };

    let new_session = Ident::new(&format!("session{}", index), Span::call_site());

    quote! {
        impl<
            #( #session_types_struct )*
            R: mpstthree::role::Role,
            T: std::marker::Send,
            const CLOCK: char,
            const START: i128,
            const INCLUDE_START: bool,
            const END: i128,
            const INCLUDE_END: bool,
            const RESET: bool,
        >
            #meshedchannels_name<
                #( #send_sessions )*
                #receiver_ident<R>,
                #sender_ident
            >
        {
            pub fn send(
                self,
                payload: T,
                all_clocks: &mut std::collections::HashMap<char, std::time::Instant>,
            ) -> std::result::Result<
                #meshedchannels_name<
                    #( #session_types , )*
                    R,
                    #sender_ident
                >,
                Box<dyn std::error::Error>
            > {
                let new_session = mpstthree::binary_timed::send::send(
                    payload,
                    all_clocks,
                    self.#new_session
                )?;
                let new_stack = self.stack.continuation();
                Ok(
                    #meshedchannels_name {
                        #( #new_sessions )*
                        stack: new_stack,
                        name: self.name,
                    }
                )
            }
        }
    }
}
