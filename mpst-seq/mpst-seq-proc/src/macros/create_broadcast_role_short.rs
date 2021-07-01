use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::Result;

#[derive(Debug)]
pub struct CreateBroadcastRoleShortMacroInput {
    role: syn::Ident,
}

impl Parse for CreateBroadcastRoleShortMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let role = syn::Ident::parse(input)?;

        Ok(CreateBroadcastRoleShortMacroInput { role })
    }
}

impl From<CreateBroadcastRoleShortMacroInput> for proc_macro2::TokenStream {
    fn from(input: CreateBroadcastRoleShortMacroInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl CreateBroadcastRoleShortMacroInput {
    fn expand(&self) -> proc_macro2::TokenStream {
        let role = self.role.clone();

        // Build the new names
        // role to all
        let role_to_all_name = syn::Ident::new(
            &format!("Role{}toAll", role),
            proc_macro2::Span::call_site(),
        );
        // dual to all
        let dual_to_all_name = syn::Ident::new(
            &format!("RoleAllto{}", role),
            proc_macro2::Span::call_site(),
        );

        quote! {
            ////////////////////////////////////////////
            /// The Role

            #[derive(Debug)]
            struct #role_to_all_name<R1, R2>
            where
                R1: mpstthree::role::Role,
                R2: mpstthree::role::Role,
                R1::Dual: mpstthree::role::Role,
                R2::Dual: mpstthree::role::Role,
            {
                sender1: crossbeam_channel::Sender<R1::Dual>,
                sender2: crossbeam_channel::Sender<R2::Dual>,
            }

            ////////////////////////////////////////////
            /// The Dual

            #[derive(Debug)]
            struct #dual_to_all_name<R1, R2>
            where
                R1: mpstthree::role::Role,
                R2: mpstthree::role::Role,
                R1::Dual: mpstthree::role::Role,
                R2::Dual: mpstthree::role::Role,
            {
                sender1: crossbeam_channel::Sender<R1::Dual>,
                sender2: crossbeam_channel::Sender<R2::Dual>,
            }

            ////////////////////////////////////////////
            /// The Role functions

            impl<R1: mpstthree::role::Role, R2: mpstthree::role::Role> mpstthree::role::Role for #role_to_all_name<R1, R2> {
                type Dual = #dual_to_all_name<
                    <R1 as mpstthree::role::Role>::Dual,
                    <R2 as mpstthree::role::Role>::Dual,
                >;

                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    let (sender_normal_1, _) = crossbeam_channel::bounded::<R1>(1);
                    let (sender_normal_2, _) = crossbeam_channel::bounded::<R2>(1);
                    let (sender_dual_1, _) = crossbeam_channel::bounded::<R1::Dual>(1);
                    let (sender_dual_2, _) = crossbeam_channel::bounded::<R2::Dual>(1);

                    (
                        #role_to_all_name {
                            sender1: sender_dual_1,
                            sender2: sender_dual_2,
                        },
                        #dual_to_all_name {
                            sender1: sender_normal_1,
                            sender2: sender_normal_2,
                        },
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    String::from(stringify!(#role_to_all_name))
                }

                #[doc(hidden)]
                fn tail_str() -> String {
                    format!(
                        "{}<{}> + {}<{}>",
                        <R1 as mpstthree::role::Role>::head_str(),
                        <R1 as mpstthree::role::Role>::tail_str(),
                        <R2 as mpstthree::role::Role>::head_str(),
                        <R2 as mpstthree::role::Role>::tail_str()
                    )
                }
            }

            ////////////////////////////////////////////
            /// The Dual functions

            impl<R1: mpstthree::role::Role, R2: mpstthree::role::Role> mpstthree::role::Role for #dual_to_all_name<R1, R2> {
                type Dual = #role_to_all_name<
                    <R1 as mpstthree::role::Role>::Dual,
                    <R2 as mpstthree::role::Role>::Dual,
                >;

                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    let (sender_normal_1, _) = crossbeam_channel::bounded::<R1>(1);
                    let (sender_normal_2, _) = crossbeam_channel::bounded::<R2>(1);
                    let (sender_dual_1, _) = crossbeam_channel::bounded::<R1::Dual>(1);
                    let (sender_dual_2, _) = crossbeam_channel::bounded::<R2::Dual>(1);

                    (
                        #dual_to_all_name {
                            sender1: sender_dual_1,
                            sender2: sender_dual_2,
                        },
                        #role_to_all_name {
                            sender1: sender_normal_1,
                            sender2: sender_normal_2,
                        },
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    String::from(stringify!(#dual_to_all_name))
                }

                #[doc(hidden)]
                fn tail_str() -> String {
                    format!(
                        "{}<{}> + {}<{}>",
                        <R1 as mpstthree::role::Role>::head_str(),
                        <R1 as mpstthree::role::Role>::tail_str(),
                        <R2 as mpstthree::role::Role>::head_str(),
                        <R2 as mpstthree::role::Role>::tail_str()
                    )
                }
            }
        }
    }
}
