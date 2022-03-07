use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::convert::TryFrom;
use syn::Ident;

/// Expand offer methods
pub(crate) fn offer(
    all_roles: Vec<TokenStream>,
    sender: u64,
    receiver: u64,
    meshedchannels_name: Ident,
    number_roles: u64,
) -> TokenStream {
    let sender_ident = if let Some(elt) = all_roles.get(usize::try_from(sender - 1).unwrap()) {
        Ident::new(&format!("RoleAllto{}", elt), Span::call_site())
    } else {
        panic!("Not enough arguments for sender_ident in expand_offer")
    };

    let receiver_ident = if let Some(elt) = all_roles.get(usize::try_from(receiver - 1).unwrap()) {
        Ident::new(&format!("Role{}", elt), Span::call_site())
    } else {
        panic!("Not enough arguments for receiver_ident in expand_offer")
    };

    let offer_session_types_struct: Vec<TokenStream> = (1..(2 * number_roles - 1))
        .map(|i| {
            let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
            quote! { #temp_ident : mpstthree::binary::struct_trait::session::Session , }
        })
        .collect();

    let left_sessions: Vec<TokenStream> = (1..number_roles)
        .map(|i| {
            let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
            quote! { #temp_ident , }
        })
        .collect();

    let right_sessions: Vec<TokenStream> = (number_roles..(2 * number_roles - 1))
        .map(|i| {
            let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
            quote! { #temp_ident , }
        })
        .collect();

    let offer_sessions: Vec<TokenStream> = (1..number_roles)
        .map(|k| {
            let cond = if k >= receiver { sender - 1 } else { sender };
            if k == cond {
                quote! {
                    mpstthree::binary::struct_trait::recv::Recv<
                        either::Either<
                            #meshedchannels_name<
                                #( #left_sessions )*
                                R1,
                                #receiver_ident<mpstthree::role::end::RoleEnd>
                            >,
                            #meshedchannels_name<
                                #( #right_sessions )*
                                R2,
                                #receiver_ident<mpstthree::role::end::RoleEnd>
                            >
                        >,
                        mpstthree::binary::struct_trait::end::End
                    >,
                }
            } else {
                quote! { mpstthree::binary::struct_trait::end::End, }
            }
        })
        .collect();

    quote! {
        impl<
            'a,
            #( #offer_session_types_struct )*
            R1: mpstthree::role::Role,
            R2: mpstthree::role::Role,
        >
            #meshedchannels_name<
                #( #offer_sessions )*
                #sender_ident<mpstthree::role::end::RoleEnd, mpstthree::role::end::RoleEnd>,
                #receiver_ident<mpstthree::role::end::RoleEnd>,
            >
        {
            pub fn offer<F, G, U>(self, f: F, g: G) -> Result<U, Box<dyn std::error::Error + 'a>>
            where
                F: FnOnce(
                    #meshedchannels_name<
                        #( #left_sessions )*
                        R1,
                        #receiver_ident<mpstthree::role::end::RoleEnd>,
                    >,
                ) -> Result<U, Box<dyn std::error::Error + 'a>>,
                G: FnOnce(
                    #meshedchannels_name<
                        #( #right_sessions )*
                        R2,
                        #receiver_ident<mpstthree::role::end::RoleEnd>,
                    >,
                ) -> Result<U, Box<dyn std::error::Error + 'a>>,
            {
                let (e, s) = self.recv_from_all()?;
                mpstthree::binary::cancel::cancel(s);
                e.either(f, g)
            }
        }
    }
}
