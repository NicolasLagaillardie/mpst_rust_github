use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result};

#[derive(Debug)]
pub(crate) struct CreateNormalNameShort {
    name: Ident,
}

impl Parse for CreateNormalNameShort {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = Ident::parse(input)?;

        Ok(CreateNormalNameShort { name })
    }
}

impl From<CreateNormalNameShort> for TokenStream {
    fn from(input: CreateNormalNameShort) -> TokenStream {
        input.expand()
    }
}

impl CreateNormalNameShort {
    fn expand(&self) -> TokenStream {
        let name = &self.name;

        // Build the new names
        // name
        let concatenated_role = format!("Name{name}");
        let role_name = Ident::new(&concatenated_role, Span::call_site());

        quote! {
            ////////////////////////////////////////////
            /// The Name

            #[derive(Debug)]
            struct #role_name
            {
                #[doc(hidden)]
                sender: crossbeam_channel::Sender<()>,
                #[doc(hidden)]
                receiver: crossbeam_channel::Receiver<()>,
            }

            ////////////////////////////////////////////
            /// The Name methods

            impl mpstthree::name::Name for #role_name {
                type Dual = #role_name;

                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    let (sender1, receiver1) = crossbeam_channel::bounded::<()>(1);
                    let (sender2, receiver2) = crossbeam_channel::bounded::<()>(1);

                    (
                        #role_name {
                            sender: sender1,
                            receiver: receiver2,
                        },
                        #role_name {
                            sender: sender2,
                            receiver: receiver1,
                        },
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    String::from(stringify!(#role_name))
                }

                #[doc(hidden)]
                fn tail_str() -> String {
                    "".to_string()
                }

                #[doc(hidden)]
                fn self_head_str(&self) -> String {
                    String::from(stringify!(#role_name))
                }

                #[doc(hidden)]
                fn self_tail_str(&self) -> String {
                    "".to_string()
                }
            }
        }
    }
}
