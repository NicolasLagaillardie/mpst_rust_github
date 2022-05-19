use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

/// Expand close methods
pub(crate) fn close(meshedchannels_name: &Ident, number_roles: u64) -> TokenStream {
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
        impl<
            N: mpstthree::name::Name
        >
            #meshedchannels_name<
                #(
                    #close_session_types
                )*
                mpstthree::role::end::RoleEnd,
                N
            >
        {
            pub fn close(self) -> std::result::Result<(), Box<dyn std::error::Error>> {

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
