/// Create a new [`mpstthree::role::Role`], its dual and the related *next* function to process
/// stacks start with them.
///
/// # Arguments
///
/// * The name of the new [`mpstthree::role::Role`]
/// * The name of the *next* function that is related
/// * The name of the *dual* of the new [`mpstthree::role::Role`]
/// * The name of the *next* function that is related
///
/// # Example
///
/// ```
/// use mpstthree::create_normal_role;
///
/// create_normal_role!(RoleA, RoleADual);
/// ```
///
/// [`mpstthree::role::Role`]: ../role/trait.Role.html
#[macro_export]
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
        }
    };
}

/// Create multiple new [`mpstthree::role::Role`], its dual and the related *next* function to
/// process stacks start with them.
///
/// # Arguments
///
/// * The name of the new [`mpstthree::role::Role`]
/// * The name of the *next* function that is related
/// * The name of the *dual* of the new [`mpstthree::role::Role`]
/// * The name of the *next* function that is related
///
/// # Example
///
/// ```
/// use mpstthree::create_multiple_normal_role;
///
/// create_multiple_normal_role!(
///    RoleA, RoleADual |
///    RoleB, RoleBDual |
/// );
/// ```
///
/// [`mpstthree::role::Role`]: ../role/trait.Role.html
#[macro_export]
macro_rules! create_multiple_normal_role {
    ($($role_name:ident, $dual_name:ident | )+ ) => {
        $(mpstthree::create_normal_role!($role_name, $dual_name);)+
     }
}

/// Create a new broadcast [`mpstthree::role::Role`], its dual and the related *next* function to
/// process stacks start with them. A broadcast [`mpstthree::role::Role`] is used for sending a
/// choice. Its dual is used for receving this choice.
///
/// # Arguments
///
/// * The name of the new [`mpstthree::role::Role`]
/// * The name of the *next* function that is related
/// * The name of the *dual* of the new [`mpstthree::role::Role`]
/// * The name of the *next* function that is related
///
/// # Example
///
/// ```
/// use mpstthree::create_broadcast_role;
///
/// create_broadcast_role!(RoleAlltoC, RoleCtoAll);
/// ```
///
/// [`mpstthree::role::Role`]: ../role/trait.Role.html
#[macro_export]
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
        }
    };
}

/// Create multiple new broadcast [`mpstthree::role::Role`], its dual and the related *next*
/// function to process stacks start with them. A broadcast [`mpstthree::role::Role`] is used for
/// sending a choice. Its dual is used for receving this choice.
///
/// # Arguments
///
/// * The name of the new [`mpstthree::role::Role`]
/// * The name of the *next* function that is related
/// * The name of the *dual* of the new [`mpstthree::role::Role`]
/// * The name of the *next* function that is related
///
/// # Example
///
/// ```
/// use mpstthree::create_multiple_broadcast_role;
///
/// create_multiple_broadcast_role!(
///    RoleAlltoC,
///    RoleCtoAll |
///    RoleAlltoD,
///    RoleDtoAll|
/// );
/// ```
///
/// [`mpstthree::role::Role`]: ../role/trait.Role.html
#[macro_export]
macro_rules! create_multiple_broadcast_role {
    ($($role_name:ident, $dual_name:ident | )+ ) => {
        $(mpstthree::create_broadcast_role!($role_name, $dual_name);)+
     }
}

///////////////////////////////

/// Create a new [`mpstthree::role::Role`], its dual and the related *next* function to process
/// stacks start with them.
/// When a name *X* is given, the Roles and next functions created are
///
///  * RoleX
///  * RoleXDual
///
/// If you want to specify other names, please use [`mpstthree::create_normal_role`].
///
/// # Arguments
///
/// * The name of the new [`mpstthree::role::Role`]
///
/// # Example
///
/// ```
/// use mpstthree::create_normal_role_short;
///
/// create_normal_role_short!(A);
/// ```
///
/// [`mpstthree::role::Role`]: ../role/trait.Role.html
/// [`mpstthree::create_normal_role`]: ../macro.create_normal_role!.html
#[macro_export]
macro_rules! create_normal_role_short {
    ($role_name:ident) => {
        mpst_seq::create_normal_role_short!(($role_name));
    };
}

/// Create multiple new [`mpstthree::role::Role`], its dual and the related *next* function to
/// process stacks start with them.
/// When a name *X* is given, the Roles and next functions created are
/// 
///  * RoleX
///  * RoleXDual
/// 
/// If you want to specify other names, please use [`mpstthree::create_multiple_normal_role`].
///
/// # Arguments
///
/// * The names of the new [`mpstthree::role::Role`]
///
/// # Example
///
/// ```
/// use mpstthree::create_multiple_normal_role_short;
///
/// create_multiple_normal_role_short!(A, B, C);
/// ```
///
/// [`mpstthree::role::Role`]: ../role/trait.Role.html
/// [`mpstthree::create_normal_role_short`]: ../macro.create_normal_role_short.html
/// [`mpstthree::create_multiple_normal_role`]: ../macro.create_multiple_normal_role.html
#[macro_export]
macro_rules! create_multiple_normal_role_short {
    ($($role_name:ident),+ $(,)? ) => {
        $(mpstthree::create_normal_role_short!($role_name);)+
     }
}

/// Create a new broadcast [`mpstthree::role::Role`], its dual and the related *next* function to
/// process stacks start with them. A broadcast [`mpstthree::role::Role`] is used for sending a
/// choice. Its dual is used for receving this choice.
/// When a name *X* is given, the Roles and next functions created are
///
///  * RoleXtoAll
///  * RoleAlltoX
///
/// If you want to specify other names, please use [`mpstthree::create_broadcast_role`].
///
/// # Arguments
///
/// * The name of the new [`mpstthree::role::Role`]
///
/// # Example
///
/// ```
/// use mpstthree::create_broadcast_role_short;
///
/// create_broadcast_role_short!(A);
/// ```
///
/// [`mpstthree::role::Role`]: ../role/trait.Role.html
/// [`mpstthree::create_broadcast_role`]: ../macro.create_broadcast_role.html
#[macro_export]
macro_rules! create_broadcast_role_short {
    ($role_name:ident) => {
        mpst_seq::seq!(N in 1..3 > ($role_name) {
            ////////////////////////////////////////////
            /// The Role

            #[derive(Debug)]
            struct unused^N:8<R1, R2>
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
            struct unused^N:10<R1, R2>
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
                for unused^N:8<R1, R2>
            {
                type Dual = unused^N:10<
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
                        unused^N:8 {
                            sender1: sender_dual_1,
                            sender2: sender_dual_2,
                        },
                        unused^N:10 {
                            sender1: sender_normal_1,
                            sender2: sender_normal_2,
                        },
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    String::from(stringify!(unused^N:8))
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

            impl<R1: mpstthree::role::Role, R2: mpstthree::role::Role> mpstthree::role::Role
                for unused^N:10<R1, R2>
            {
                type Dual = unused^N:8<
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
                        unused^N:10 {
                            sender1: sender_dual_1,
                            sender2: sender_dual_2,
                        },
                        unused^N:8 {
                            sender1: sender_normal_1,
                            sender2: sender_normal_2,
                        },
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    String::from(stringify!(unused^N:10))
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
        });
    }
}

/// Create multiple new broadcast [`mpstthree::role::Role`], its dual and the related *next*
/// function to process stacks start with them. A broadcast [`mpstthree::role::Role`] is used for
/// sending a choice. Its dual is used for receving this choice.
/// When a name *X* is given, the Roles and next functions created are
///
///  * RoleXtoAll
///  * RoleAlltoX
///
/// If you want to specify other names, please use
/// [`mpstthree::create_multiple_broadcast_role_short`]
///
/// # Arguments
///
/// * The name of the new [`mpstthree::role::Role`]
///
/// # Example
///
/// ```
/// use mpstthree::create_multiple_broadcast_role_short;
///
/// create_multiple_broadcast_role_short!(A, B, C);
/// ```
///
/// [`mpstthree::role::Role`]: ../role/trait.Role.html
/// [`mpstthree::create_multiple_broadcast_role_short`]: ../macro.create_multiple_broadcast_role_short.html
#[macro_export]
macro_rules! create_multiple_broadcast_role_short {
    ($($role_name:ident),+ $(,)? ) => {
        $(mpstthree::create_broadcast_role_short!($role_name);)+
     }
}
