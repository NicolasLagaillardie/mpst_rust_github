use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

/// Expand role structures and methods
pub(crate) fn role_timed(role: String) -> TokenStream {
    // role
    let role_name = Ident::new(&format!("Role{}", role), Span::call_site());
    // dual
    let dual_name = Ident::new(&format!("Role{}Dual", role), Span::call_site());
    // role to all
    let role_to_all_name = Ident::new(&format!("Role{}toAll", role), Span::call_site());
    // dual to all
    let dual_to_all_name = Ident::new(&format!("RoleAllto{}", role), Span::call_site());

    quote! {
        ////////////////////////////////////////////
        /// The normal Role
        #[derive(Debug)]
        struct #role_name<R>
        where
            R: mpstthree::role::Role,
            R::Dual: mpstthree::role::Role,
        {
            sender: crossbeam_channel::Sender<R::Dual>,
        }

        ////////////////////////////////////////////
        /// The normal Dual
        #[derive(Debug)]
        struct #dual_name<R>
        where
            R: mpstthree::role::Role,
            R::Dual: mpstthree::role::Role,
        {
            sender: crossbeam_channel::Sender<R::Dual>,
        }

        ////////////////////////////////////////////
        /// The normal Role implementation of Role
        impl<R: mpstthree::role::Role> mpstthree::role::Role for #role_name<R> {
            type Dual = #dual_name<<R as mpstthree::role::Role>::Dual>;
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
        /// The normal Dual implementation of Role
        impl<R: mpstthree::role::Role> mpstthree::role::Role for #dual_name<R> {
            type Dual = #role_name<<R as mpstthree::role::Role>::Dual>;
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
            pub fn continuation(&self) -> std::result::Result<
                R,
                Box<dyn std::error::Error>
            > {
                let (here, there) = R::new();
                match self.sender.send(there) {
                    Ok(_) => Ok(here),
                    Err(e) => {
                        mpstthree::binary::cancel::cancel(self);
                        panic!("{}", e.to_string())
                    }
                }
            }
        }

        ////////////////////////////////////////////
        /// The associated functions for Dual
        impl<R: mpstthree::role::Role> #dual_name<R> {
            pub fn continuation(&self) -> std::result::Result<
                R,
                Box<dyn std::error::Error>
            > {
                let (here, there) = R::new();
                match self.sender.send(there) {
                    Ok(_) => Ok(here),
                    Err(e) => {
                        mpstthree::binary::cancel::cancel(self);
                        panic!("{}", e.to_string())
                    }
                }
            }
        }

        ////////////////////////////////////////////
        /// The all Role
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
        /// The all Dual
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
        /// The all Role implementation of Role
        impl<R1: mpstthree::role::Role, R2: mpstthree::role::Role> mpstthree::role::Role
            for #role_to_all_name<R1, R2>
        {
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

            #[doc(hidden)]
            fn self_head_str(&self) -> String {
                String::from(stringify!(#role_to_all_name))
            }

            #[doc(hidden)]
            fn self_tail_str(&self) -> String {
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
        /// The all Dual implementation of Role
        impl<R1: mpstthree::role::Role, R2: mpstthree::role::Role> mpstthree::role::Role
            for #dual_to_all_name<R1, R2>
        {
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

            #[doc(hidden)]
            fn self_head_str(&self) -> String {
                String::from(stringify!(#dual_to_all_name))
            }

            #[doc(hidden)]
            fn self_tail_str(&self) -> String {
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
        /// The associated functions for Role
        impl<R1: mpstthree::role::Role, R2: mpstthree::role::Role> #role_to_all_name<R1, R2> {
            pub fn continuation_left(&self) -> std::result::Result<
                R1,
                Box<dyn std::error::Error>
             > {
                let (here, there) = R1::new();
                match self.sender1.send(there) {
                    Ok(_) => Ok(here),
                    Err(e) => {
                        mpstthree::binary::cancel::cancel(self);
                        panic!("{}", e.to_string())
                    }
                }
            }
            pub fn continuation_right(&self) -> std::result::Result<
                R2,
                Box<dyn std::error::Error>
            > {
                let (here, there) = R2::new();
                match self.sender2.send(there) {
                    Ok(_) => Ok(here),
                    Err(e) => {
                        mpstthree::binary::cancel::cancel(self);
                        panic!("{}", e.to_string())
                    }
                }
            }
        }

        ////////////////////////////////////////////
        /// The associated functions for Dual
        impl<R1: mpstthree::role::Role, R2: mpstthree::role::Role> #dual_to_all_name<R1, R2> {
            pub fn continuation_left(&self) -> std::result::Result<
                R1,
                Box<dyn std::error::Error>
            > {
                let (here, there) = R1::new();
                match self.sender1.send(there) {
                    Ok(_) => Ok(here),
                    Err(e) => {
                        mpstthree::binary::cancel::cancel(self);
                        panic!("{}", e.to_string())
                    }
                }
            }
            pub fn continuation_right(&self) -> std::result::Result<
                R2,
                Box<dyn std::error::Error>
            > {
                let (here, there) = R2::new();
                match self.sender2.send(there) {
                    Ok(_) => Ok(here),
                    Err(e) => {
                        mpstthree::binary::cancel::cancel(self);
                        panic!("{}", e.to_string())
                    }
                }
            }
        }
    }
}
