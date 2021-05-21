use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::Result;

#[derive(Debug)]
pub struct CreateBroadcastRoleShortMacroInput {
    role: proc_macro2::TokenStream,
}

impl Parse for CreateBroadcastRoleShortMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let content_role;
        let _parentheses = syn::parenthesized!(content_role in input);
        let role = proc_macro2::TokenStream::parse(&content_role)?;

        Ok(CreateBroadcastRoleShortMacroInput { role })
    }
}

impl From<CreateBroadcastRoleShortMacroInput> for proc_macro2::TokenStream {
    fn from(input: CreateBroadcastRoleShortMacroInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl CreateBroadcastRoleShortMacroInput {
    fn expand_options(
        &self,
        tt: proc_macro2::TokenTree,
        _rest: &mut proc_macro2::token_stream::IntoIter,
    ) -> std::option::Option<proc_macro2::TokenStream> {
        match tt {
            proc_macro2::TokenTree::Group(g) => Some(g.stream()),
            _ => None,
        }
    }

    fn expand_role(&self, stream: &proc_macro2::TokenStream) -> Vec<proc_macro2::TokenStream> {
        let mut out: Vec<proc_macro2::TokenStream> = Vec::new();
        let mut tts = stream.clone().into_iter();
        while let Some(tt) = tts.next() {
            let elt = self.expand_options(tt, &mut tts);
            if let Some(elt_tt) = elt {
                out.push(elt_tt)
            }
        }
        out
    }

    fn lowercase(&self, elt: &str) -> String {
        let temp = &format!("{}", elt);
        let mut result = String::from("");
        for c in temp.chars() {
            result.push_str(&c.to_lowercase().to_string());
        }
        result
    }

    fn expand(&self) -> proc_macro2::TokenStream {
        // Get all the roles provided into a Vec
        let all_roles = self.expand_role(&self.role.clone());
        // Get the first role, panic if it does not exist
        let first_role = if let Some(elt) = all_roles.get(usize::try_from(0).unwrap()) {
            format!("{}", elt)
        } else {
            panic!("Not enough arguments")
        };
        // Lowercase the name of the role
        let lower_case_role = self.lowercase(&first_role);

        // Build the new names
        // role
        let concatenated_role = format!("Role{}toAll", first_role);
        let role_name = syn::Ident::new(&concatenated_role, proc_macro2::Span::call_site());
        // next function
        let concatenated_next = format!("next_{}_to_all", lower_case_role);
        let next_role_name = syn::Ident::new(&concatenated_next, proc_macro2::Span::call_site());
        // dual
        let concatenated_dual = format!("RoleAllto{}", first_role);
        let dual_name = syn::Ident::new(&concatenated_dual, proc_macro2::Span::call_site());
        // next dual function
        let concatenated_next_dual = format!("next_all_to_{}", lower_case_role);
        let next_dual_name =
            syn::Ident::new(&concatenated_next_dual, proc_macro2::Span::call_site());

        quote! {
            ////////////////////////////////////////////
            /// The Role

            #[derive(Debug)]
            struct #role_name<R1, R2>
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
            struct #dual_name<R1, R2>
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

            impl<R1: mpstthree::role::Role, R2: mpstthree::role::Role> mpstthree::role::Role for #role_name<R1, R2> {
                type Dual = #dual_name<
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
                        #role_name {
                            sender1: sender_dual_1,
                            sender2: sender_dual_2,
                        },
                        #dual_name {
                            sender1: sender_normal_1,
                            sender2: sender_normal_2,
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
                        "{}<{}> + {}<{}>",
                        <R1 as mpstthree::role::Role>::head_str(),
                        <R1 as mpstthree::role::Role>::tail_str(),
                        <R2 as mpstthree::role::Role>::head_str(),
                        <R2 as mpstthree::role::Role>::tail_str()
                    )
                }
            }

            #[doc(hidden)]
            fn #next_role_name<R1, R2>(r: #role_name<R1, R2>) -> (R1, R2)
            where
                R1: mpstthree::role::Role,
                R2: mpstthree::role::Role,
            {
                let (here1, there1) = <R1 as mpstthree::role::Role>::new();
                let (here2, there2) = <R2 as mpstthree::role::Role>::new();
                r.sender1.send(there1).unwrap_or(());
                r.sender2.send(there2).unwrap_or(());
                (here1, here2)
            }

            ////////////////////////////////////////////
            /// The Dual functions

            impl<R1: mpstthree::role::Role, R2: mpstthree::role::Role> mpstthree::role::Role for #dual_name<R1, R2> {
                type Dual = #role_name<
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
                        #dual_name {
                            sender1: sender_dual_1,
                            sender2: sender_dual_2,
                        },
                        #role_name {
                            sender1: sender_normal_1,
                            sender2: sender_normal_2,
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
                        "{}<{}> + {}<{}>",
                        <R1 as mpstthree::role::Role>::head_str(),
                        <R1 as mpstthree::role::Role>::tail_str(),
                        <R2 as mpstthree::role::Role>::head_str(),
                        <R2 as mpstthree::role::Role>::tail_str()
                    )
                }
            }

            #[doc(hidden)]
            fn #next_dual_name<R1, R2>(r: #dual_name<R1, R2>) -> (R1, R2)
            where
                R1: mpstthree::role::Role,
                R2: mpstthree::role::Role,
            {
                let (here1, there1) = <R1 as mpstthree::role::Role>::new();
                let (here2, there2) = <R2 as mpstthree::role::Role>::new();
                r.sender1.send(there1).unwrap_or(());
                r.sender2.send(there2).unwrap_or(());
                (here1, here2)
            }
        }
    }
}
