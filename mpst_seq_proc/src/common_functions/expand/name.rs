use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

/// Expand name methods
pub(crate) fn name(role: String) -> TokenStream {
    // Name
    let name_role = Ident::new(&format!("Name{}", role), Span::call_site());

    quote! {
        ////////////////////////////////////////////
        /// The Name
        #[derive(Debug)]
        pub struct #name_role
        {
            #[doc(hidden)]
            sender: crossbeam_channel::Sender<()>,
            #[doc(hidden)]
            receiver: crossbeam_channel::Receiver<()>,
        }
        ////////////////////////////////////////////
        /// The normal Name implementation of Name
        impl mpstthree::name::Name for #name_role {
            type Dual = #name_role;

            #[doc(hidden)]
            fn new() -> (Self, Self::Dual) {
                let (sender1, receiver1) = crossbeam_channel::bounded::<()>(1);
                let (sender2, receiver2) = crossbeam_channel::bounded::<()>(1);

                (
                    #name_role {
                        sender: sender1,
                        receiver: receiver2,
                    },
                    #name_role {
                        sender: sender2,
                        receiver: receiver1,
                    },
                )
            }

            #[doc(hidden)]
            fn head_str() -> String {
                String::from(stringify!(#name_role))
            }

            #[doc(hidden)]
            fn tail_str() -> String {
                "".to_string()
            }

            #[doc(hidden)]
            fn self_head_str(&self) -> String {
                String::from(stringify!(#name_role))
            }

            #[doc(hidden)]
            fn self_tail_str(&self) -> String {
                "".to_string()
            }
        }
    }
}
