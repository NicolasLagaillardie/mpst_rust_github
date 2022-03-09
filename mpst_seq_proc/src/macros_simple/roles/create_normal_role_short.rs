use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result};

#[derive(Debug)]
pub(crate) struct CreateNormalRoleShort {
    role: Ident,
}

impl Parse for CreateNormalRoleShort {
    fn parse(input: ParseStream) -> Result<Self> {
        let role = Ident::parse(input)?;

        Ok(CreateNormalRoleShort { role })
    }
}

impl From<CreateNormalRoleShort> for TokenStream {
    fn from(input: CreateNormalRoleShort) -> TokenStream {
        input.expand()
    }
}

impl CreateNormalRoleShort {
    fn expand(&self) -> TokenStream {
        let role = self.role.clone();

        // Build the new names
        // role
        let concatenated_role = format!("Role{}", role);
        let role_name = Ident::new(&concatenated_role, Span::call_site());
        // dual
        let concatenated_dual = format!("Role{}Dual", role);
        let dual_name = Ident::new(&concatenated_dual, Span::call_site());

        quote! {
            ////////////////////////////////////////////
            /// The Role

            #[derive(Debug)]
            struct #role_name<R>
            where
                R: mpstthree::role::Role,
                R::Dual: mpstthree::role::Role,
            {
                sender: crossbeam_channel::Sender<R::Dual>,
            }

            ////////////////////////////////////////////
            /// The Dual

            #[derive(Debug)]
            struct #dual_name<R>
            where
                R: mpstthree::role::Role,
                R::Dual: mpstthree::role::Role,
            {
                sender: crossbeam_channel::Sender<R::Dual>,
            }

            ////////////////////////////////////////////
            /// The Role methods

            impl<R: mpstthree::role::Role> mpstthree::role::Role for #role_name<R> {
                type Dual = #dual_name::<<R as mpstthree::role::Role>::Dual>;

                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
                    let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);

                    (
                        #role_name {
                            sender: sender_dual,
                        },
                        #dual_name {
                            sender: sender_normal,
                        },
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    String::from(stringify!(#role_name))
                }

                #[doc(hidden)]
                fn tail_str() -> String {
                    format!(
                        "{}<{}>",
                        <R as mpstthree::role::Role>::head_str(),
                        <R as mpstthree::role::Role>::tail_str()
                    )
                }

                #[doc(hidden)]
                fn self_head_str(&self) -> String {
                    String::from(stringify!(#role_name))
                }

                #[doc(hidden)]
                fn self_tail_str(&self) -> String {
                    format!(
                        "{}<{}>",
                        <R as mpstthree::role::Role>::head_str(),
                        <R as mpstthree::role::Role>::tail_str()
                    )
                }
            }

            ////////////////////////////////////////////
            /// The Dual methods

            impl<R: mpstthree::role::Role> mpstthree::role::Role for #dual_name<R> {
                type Dual = #role_name::<<R as mpstthree::role::Role>::Dual>;

                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
                    let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);

                    (
                        #dual_name {
                            sender: sender_dual,
                        },
                        #role_name {
                            sender: sender_normal,
                        },
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    String::from(stringify!(#dual_name))
                }

                #[doc(hidden)]
                fn tail_str() -> String {
                    format!(
                        "{}<{}>",
                        <R as mpstthree::role::Role>::head_str(),
                        <R as mpstthree::role::Role>::tail_str()
                    )
                }

                #[doc(hidden)]
                fn self_head_str(&self) -> String {
                    String::from(stringify!(#dual_name))
                }

                #[doc(hidden)]
                fn self_tail_str(&self) -> String {
                    format!(
                        "{}<{}>",
                        <R as mpstthree::role::Role>::head_str(),
                        <R as mpstthree::role::Role>::tail_str()
                    )
                }
            }

            ////////////////////////////////////////////
            /// The associated functions for Role

            impl<R: mpstthree::role::Role> #role_name<R> {
                pub fn continuation(&self) -> R {
                    let (here, there) = R::new();
                    self.sender.send(there).unwrap_or(());
                    here
                }
            }

            ////////////////////////////////////////////
            /// The associated functions for Dual

            impl<R: mpstthree::role::Role> #dual_name<R> {
                pub fn continuation(&self) -> R {
                    let (here, there) = R::new();
                    self.sender.send(there).unwrap_or(());
                    here
                }
            }
        }
    }
}
