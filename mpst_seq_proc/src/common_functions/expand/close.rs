use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::convert::TryFrom;
use syn::Ident;

/// Expand close methods
pub(crate) fn close(
    all_roles: Vec<TokenStream>,
    sender: u64,
    meshedchannels_name: Ident,
    number_roles: u64,
) -> TokenStream {
    let sender_ident = if let Some(elt) = all_roles.get(usize::try_from(sender - 1).unwrap()) {
        let concatenated_elt = format!("Role{}", elt);
        Ident::new(&concatenated_elt, Span::call_site())
    } else {
        panic!("Not enough arguments for sender_ident in expand_close")
    };

    let close_session_types: Vec<TokenStream> = (1..number_roles)
        .map(|_i| {
            quote! { mpstthree::binary::struct_trait::end::End, }
        })
        .collect();

    let close_session_send: Vec<TokenStream> = (1..number_roles)
                .map(|i| {
                    let temp_session = Ident::new(
                        &format!("session{}", i),
                        Span::call_site(),
                    );
                    quote! { self.#temp_session.sender.send(mpstthree::binary::struct_trait::end::Signal::Stop).unwrap_or(()); }
                })
                .collect();

    let close_session_recv: Vec<TokenStream> = (1..number_roles)
        .map(|i| {
            let temp_session = Ident::new(&format!("session{}", i), Span::call_site());
            quote! { self.#temp_session.receiver.recv()?; }
        })
        .collect();

    quote! {
        impl
            #meshedchannels_name<
                #(
                    #close_session_types
                )*
                mpstthree::role::end::RoleEnd,
                #sender_ident<mpstthree::role::end::RoleEnd>
            >
        {
            pub fn close(self) -> Result<(), Box<dyn std::error::Error>> {

                #(
                    #close_session_send
                )*

                #(
                    #close_session_recv
                )*

                Ok(())
            }
        }
    }
}
