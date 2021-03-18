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
/// create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
/// ```
///
/// [`mpstthree::role::Role`]: ../role/trait.Role.html
#[macro_export]
macro_rules! create_normal_role {
    ($role_name:ident, $role_next:ident, $dual_name:ident, $dual_next:ident) => {
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

        fn $role_next<R>(r: $role_name<R>) -> R
        where
            R: mpstthree::role::Role,
        {
            let (here, there) = <R as mpstthree::role::Role>::new();
            r.sender.send(there).unwrap_or(());
            here
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

        fn $dual_next<R>(r: $dual_name<R>) -> R
        where
            R: mpstthree::role::Role,
        {
            let (here, there) = <R as mpstthree::role::Role>::new();
            r.sender.send(there).unwrap_or(());
            here
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
///    RoleA, next_a, RoleADual, next_a_dual |
///    RoleB, next_b, RoleBDual, next_b_dual |
/// );
/// ```
///
/// [`mpstthree::role::Role`]: ../role/trait.Role.html
#[macro_export]
macro_rules! create_multiple_normal_role {
    ($($role_name:ident, $role_next:ident, $dual_name:ident, $dual_next:ident | )+ ) => {
        $(mpstthree::create_normal_role!($role_name, $role_next, $dual_name, $dual_next);)+
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
/// create_broadcast_role!(RoleAlltoC, next_all_to_c, RoleCtoAll, next_c_to_all);
/// ```
///
/// [`mpstthree::role::Role`]: ../role/trait.Role.html
#[macro_export]
macro_rules! create_broadcast_role {
    ($role_name:ident, $role_next:ident, $dual_name:ident, $dual_next:ident) => {
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

        fn $role_next<R1, R2>(r: $role_name<R1, R2>) -> (R1, R2)
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

        fn $dual_next<R1, R2>(r: $dual_name<R1, R2>) -> (R1, R2)
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
///    next_all_to_c,
///    RoleCtoAll,
///    next_c_to_all |
///    RoleAlltoD,
///    next_all_to_D,
///    RoleDtoAll,
///    next_d_to_all |
/// );
/// ```
///
/// [`mpstthree::role::Role`]: ../role/trait.Role.html
#[macro_export]
macro_rules! create_multiple_broadcast_role {
    ($($role_name:ident, $role_next:ident, $dual_name:ident, $dual_next:ident | )+ ) => {
        $(mpstthree::create_broadcast_role!($role_name, $role_next, $dual_name, $dual_next);)+
     }
}

///////////////////////////////

/// Create a new [`mpstthree::role::Role`], its dual and the related *next* function to process
/// stacks start with them.
/// When a name *X* is given, the Roles and next functions created are
///  * RoleX
///  * RoleXDual
///  * next_x
///  * next_x_dual
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
#[macro_export]
macro_rules! create_normal_role_short {
    ($role_name:ident) => {
        mpst_seq::seq!(N in 1..3 > ($role_name) {
            ////////////////////////////////////////////
            /// The Role

            #[derive(Debug)]
            struct unused^N:4<R>
            where
                R: mpstthree::role::Role,
                R::Dual: mpstthree::role::Role,
            {
                sender: crossbeam_channel::Sender<R::Dual>,
            }

            ////////////////////////////////////////////
            /// The Dual

            #[derive(Debug)]
            struct unused^N:6<R>
            where
                R: mpstthree::role::Role,
                R::Dual: mpstthree::role::Role,
            {
                sender: crossbeam_channel::Sender<R::Dual>,
            }

            ////////////////////////////////////////////
            /// The Role functions

            impl<R: mpstthree::role::Role> mpstthree::role::Role for unused^N:4<R> {
                type Dual = unused^N:6<<R as mpstthree::role::Role>::Dual>;

                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
                    let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);

                    (
                        unused^N:4 {
                            sender: sender_dual,
                        },
                        unused^N:6 {
                            sender: sender_normal,
                        },
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    String::from(stringify!(unused^N:4))
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

            fn unused^N:5<R>(r: unused^N:4<R>) -> R
            where
                R: mpstthree::role::Role,
            {
                let (here, there) = <R as mpstthree::role::Role>::new();
                r.sender.send(there).unwrap_or(());
                here
            }

            ////////////////////////////////////////////
            /// The Dual functions

            impl<R: mpstthree::role::Role> mpstthree::role::Role for unused^N:6<R> {
                type Dual = unused^N:4<<R as mpstthree::role::Role>::Dual>;

                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
                    let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);

                    (
                        unused^N:6 {
                            sender: sender_dual,
                        },
                        unused^N:4 {
                            sender: sender_normal,
                        },
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    String::from(stringify!(unused^N:6))
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

            fn unused^N:7<R>(r: unused^N:6<R>) -> R
            where
                R: mpstthree::role::Role,
            {
                let (here, there) = <R as mpstthree::role::Role>::new();
                r.sender.send(there).unwrap_or(());
                here
            }
        });
    }
}

/// Create multiple new [`mpstthree::role::Role`], its dual and the related *next* function to
/// process stacks start with them.
/// When a name *X* is given, the Roles and next functions created are
///  * RoleX
///  * RoleXDual
///  * next_x
///  * next_x_dual
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
///  * RoleXtoAll
///  * RoleAlltoX
///  * next_x_to_all
///  * next_all_to_x
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

            fn unused^N:9<R1, R2>(r: unused^N:8<R1, R2>) -> (R1, R2)
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

            fn unused^N:11<R1, R2>(r: unused^N:10<R1, R2>) -> (R1, R2)
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
        });
    }
}

/// Create multiple new broadcast [`mpstthree::role::Role`], its dual and the related *next*
/// function to process stacks start with them. A broadcast [`mpstthree::role::Role`] is used for
/// sending a choice. Its dual is used for receving this choice.
/// When a name *X* is given, the Roles and next functions created are
///  * RoleXtoAll
///  * RoleAlltoX
///  * next_x_to_all
///  * next_all_to_x
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
#[macro_export]
macro_rules! create_multiple_broadcast_role_short {
    ($($role_name:ident),+ $(,)? ) => {
        $(mpstthree::create_broadcast_role_short!($role_name);)+
     }
}
