//! This module contains the macros
//! for creating new participants.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"macros_simple"` feature.*

/// Create a new [`Role`] and its dual.
///
/// # Arguments
///
/// * The name of the new [`Role`]
/// * The name of the *dual* of the new [`Role`]
///
/// # Example
///
/// ```
/// use mpstthree::create_normal_role;
///
/// // Create the roles *A* and *ADual*
/// create_normal_role!(A, ADual);
/// ```
///
/// [`Role`]: crate::role::Role
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_normal_role {
    ($role_name:ident, $dual_name:ident) => {
        ////////////////////////////////////////////
        /// The Role

        #[derive(Debug)]
        struct $role_name<R>
        where
            R: mpstthree::role::Role,
            R::Dual: mpstthree::role::Role,
        {
            #[doc(hidden)]
            sender: crossbeam_channel::Sender<R::Dual>,
        }

        ////////////////////////////////////////////
        /// The Dual

        #[derive(Debug)]
        struct $dual_name<R>
        where
            R: mpstthree::role::Role,
            R::Dual: mpstthree::role::Role,
        {
            sender: crossbeam_channel::Sender<R::Dual>,
        }

        ////////////////////////////////////////////
        /// The Role functions

        impl<R: mpstthree::role::Role> mpstthree::role::Role for $role_name<R> {
            type Dual = $dual_name<<R as mpstthree::role::Role>::Dual>;

            #[doc(hidden)]
            fn new() -> (Self, Self::Dual) {
                let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
                let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);

                (
                    $role_name {
                        sender: sender_dual,
                    },
                    $dual_name {
                        sender: sender_normal,
                    },
                )
            }

            #[doc(hidden)]
            fn head_str() -> String {
                String::from(stringify!($role_name))
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
                String::from(stringify!($role_name))
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
        /// The Dual functions

        impl<R: mpstthree::role::Role> mpstthree::role::Role for $dual_name<R> {
            type Dual = $role_name<<R as mpstthree::role::Role>::Dual>;

            #[doc(hidden)]
            fn new() -> (Self, Self::Dual) {
                let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
                let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);

                (
                    $dual_name {
                        sender: sender_dual,
                    },
                    $role_name {
                        sender: sender_normal,
                    },
                )
            }

            #[doc(hidden)]
            fn head_str() -> String {
                String::from(stringify!($dual_name))
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
                String::from(stringify!($dual_name))
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

        impl<R: mpstthree::role::Role> $role_name<R> {
            pub fn continuation(&self) -> R {
                let (here, there) = R::new();
                self.sender.send(there).unwrap_or(());
                here
            }
        }

        ////////////////////////////////////////////
        /// The associated functions for Dual

        impl<R: mpstthree::role::Role> $dual_name<R> {
            pub fn continuation(&self) -> R {
                let (here, there) = R::new();
                self.sender.send(there).unwrap_or(());
                here
            }
        }
    };
}

/// Create multiple new [`Role`] and their respective dual.
///
/// # Arguments
///
/// * The name of the new [`Role`]
/// * The name of the *dual* of the new [`Role`]
///
/// # Example
///
/// ```
/// use mpstthree::create_multiple_normal_role;
///
/// // Create the roles *A*, *ADual*, *B* and *BDual*
/// create_multiple_normal_role!(
///    A, ADual |
///    B, BDual |
/// );
/// ```
///
/// [`Role`]: crate::role::Role
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_multiple_normal_role {
    ($( $role_name: ident, $dual_name: ident | )+ ) => {
        $(mpstthree::create_normal_role!($role_name, $dual_name);)+
     }
}

/// Create a new broadcast [`Role`] and its dual.
/// A broadcast [`Role`] is used for sending a
/// choice. Its dual is used for receving this choice.
///
/// # Arguments
///
/// * The name of the new [`Role`]
/// * The name of the *dual* of the new [`Role`]
///
/// # Example
///
/// ```
/// use mpstthree::create_broadcast_role;
///
/// // Create the roles *AlltoC* and *CtoAll*
/// create_broadcast_role!(AlltoC, CtoAll);
/// ```
///
/// [`Role`]: crate::role::Role
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_broadcast_role {
    ($role_name:ident, $dual_name:ident) => {
        ////////////////////////////////////////////
        /// The Role

        #[derive(Debug)]
        struct $role_name<R1, R2>
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
        struct $dual_name<R1, R2>
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

        impl<R1: mpstthree::role::Role, R2: mpstthree::role::Role> mpstthree::role::Role
            for $role_name<R1, R2>
        {
            type Dual = $dual_name<
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
                    $role_name {
                        sender1: sender_dual_1,
                        sender2: sender_dual_2,
                    },
                    $dual_name {
                        sender1: sender_normal_1,
                        sender2: sender_normal_2,
                    },
                )
            }

            #[doc(hidden)]
            fn head_str() -> String {
                String::from(stringify!($role_name))
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
                String::from(stringify!($role_name))
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
        /// The Dual functions

        impl<R1: mpstthree::role::Role, R2: mpstthree::role::Role> mpstthree::role::Role
            for $dual_name<R1, R2>
        {
            type Dual = $role_name<
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
                    $dual_name {
                        sender1: sender_dual_1,
                        sender2: sender_dual_2,
                    },
                    $role_name {
                        sender1: sender_normal_1,
                        sender2: sender_normal_2,
                    },
                )
            }

            #[doc(hidden)]
            fn head_str() -> String {
                String::from(stringify!($dual_name))
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
                String::from(stringify!($dual_name))
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

        impl<R1: mpstthree::role::Role, R2: mpstthree::role::Role> $role_name<R1, R2> {
            pub fn continuation_left(&self) -> R1 {
                let (here, there) = R1::new();
                self.sender1.send(there).unwrap_or(());
                here
            }
            pub fn continuation_right(&self) -> R2 {
                let (here, there) = R2::new();
                self.sender2.send(there).unwrap_or(());
                here
            }
        }

        ////////////////////////////////////////////
        /// The associated functions for Dual

        impl<R1: mpstthree::role::Role, R2: mpstthree::role::Role> $dual_name<R1, R2> {
            pub fn continuation_left(&self) -> R1 {
                let (here, there) = R1::new();
                self.sender1.send(there).unwrap_or(());
                here
            }
            pub fn continuation_right(&self) -> R2 {
                let (here, there) = R2::new();
                self.sender2.send(there).unwrap_or(());
                here
            }
        }
    };
}

/// Create multiple new broadcast [`Role`] and their respective dual.
/// A broadcast [`Role`] is used for
/// sending a choice. Its dual is used for receving this choice.
///
/// # Arguments
///
/// * The name of the new [`Role`]
/// * The name of the *dual* of the new [`Role`]
///
/// # Example
///
/// ```
/// use mpstthree::create_multiple_broadcast_role;
///
/// // Create the roles *AlltoC*, *CtoAll*, *AlltoD* and *DtoAll*
/// create_multiple_broadcast_role!(
///    AlltoC,
///    CtoAll |
///    AlltoD,
///    DtoAll|
/// );
/// ```
///
/// [`Role`]: crate::role::Role
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_multiple_broadcast_role {
    ($( $role_name: ident, $dual_name: ident | )+ ) => {
        $(
            mpstthree::create_broadcast_role!($role_name, $dual_name);
        )+
     }
}

///////////////////////////////

/// Create a new [`Role`] and its dual.
/// When a name *X* is given, the Roles created are
///
///  * RoleX
///  * RoleXDual
///
/// If you want to specify other names, please use [`create_normal_role`].
///
/// # Arguments
///
/// * The name of the new [`Role`]
///
/// # Example
///
/// ```
/// use mpstthree::create_normal_role_short;
///
/// // Create the roles *RoleA* and *RoleADual*
/// create_normal_role_short!(A);
/// ```
///
/// [`Role`]: crate::role::Role
/// [`create_normal_role`]: crate::create_normal_role
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_normal_role_short {
    ($role_name:ident) => {
        mpst_seq_proc::create_normal_role_short!($role_name);
    };
}

/// Create multiple new [`Role`] and their respective dual.
/// When a name *X* is given, the Roles created are
///
///  * RoleX
///  * RoleXDual
///
/// If you want to specify other names, please use [`create_multiple_normal_role`].
///
/// # Arguments
///
/// * The names of the new [`Role`]
///
/// # Example
///
/// ```
/// use mpstthree::create_multiple_normal_role_short;
///
/// // Create the roles *RoleA*, *RoleB*, *RoleC*,
/// // *RoleADual*, *RoleBDual* and *RoleCDual*
/// create_multiple_normal_role_short!(A, B, C);
/// ```
///
/// [`Role`]: crate::role::Role
/// [`create_multiple_normal_role`]: crate::create_multiple_normal_role
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_multiple_normal_role_short {
    ($( $role_name: ident),+ $(,)? ) => {
        $(
            mpstthree::create_normal_role_short!($role_name);
        )+
     }
}

/// Create a new broadcast [`Role`] and its dual.
/// A broadcast [`Role`] is used for sending a
/// choice. Its dual is used for receving this choice.
/// When a name *X* is given, the Roles created are
///
///  * RoleXtoAll
///  * RoleAlltoX
///
/// If you want to specify other names, please use [`create_broadcast_role`].
///
/// # Arguments
///
/// * The name of the new [`Role`]
///
/// # Example
///
/// ```
/// use mpstthree::create_broadcast_role_short;
///
/// // Create the roles *RoleAtoAll* and *RoleAlltoA*
/// create_broadcast_role_short!(A);
/// ```
///
/// [`Role`]: crate::role::Role
/// [`create_broadcast_role`]: crate::create_broadcast_role
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_broadcast_role_short {
    ($role_name:ident) => {
        mpst_seq_proc::create_broadcast_role_short!($role_name);
    };
}

/// Create multiple new broadcast [`Role`] and their respective dual.
/// A broadcast [`Role`] is used for
/// sending a choice. Its dual is used for receving this choice.
/// When a name *X* is given, the Roles created are
///
///  * RoleXtoAll
///  * RoleAlltoX
///
/// If you want to specify other names, please use
/// [`create_multiple_broadcast_role_short`]
///
/// # Arguments
///
/// * The name of the new [`Role`]
///
/// # Example
///
/// ```
/// use mpstthree::create_multiple_broadcast_role_short;
///
/// // Create the roles *RoleAtoAll*, *RoleBtoAll*, *RoleCtoAll*,
/// // *RoleAlltoA*, *RoleAlltoB* and *RoleAlltoC*
/// create_multiple_broadcast_role_short!(A, B, C);
/// ```
///
/// [`Role`]: crate::role::Role
/// [`create_multiple_broadcast_role_short`]: crate::create_multiple_broadcast_role_short
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_multiple_broadcast_role_short {
    ($( $role_name: ident),+ $(,)? ) => {
        $(
            mpstthree::create_broadcast_role_short!($role_name);
        )+
     }
}
