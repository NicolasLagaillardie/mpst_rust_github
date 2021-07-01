use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::Result;

#[derive(Debug)]
pub struct CreateNormalRoleShortMacroInput {
    role: syn::Ident,
}

impl Parse for CreateNormalRoleShortMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let role = syn::Ident::parse(input)?;

        Ok(CreateNormalRoleShortMacroInput { role })
    }
}

impl From<CreateNormalRoleShortMacroInput> for proc_macro2::TokenStream {
    fn from(input: CreateNormalRoleShortMacroInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl CreateNormalRoleShortMacroInput {
    fn expand(&self) -> proc_macro2::TokenStream {
        let role = self.role.clone();

        // Build the new names
        // role
        let concatenated_role = format!("Role{}", role);
        let role_name = syn::Ident::new(&concatenated_role, proc_macro2::Span::call_site());
        // dual
        let concatenated_dual = format!("Role{}Dual", role);
        let dual_name = syn::Ident::new(&concatenated_dual, proc_macro2::Span::call_site());

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
            }
        }
    }
}
