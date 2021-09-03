#![cfg(feature = "baking")]

//! This module contains the macros for
//! creating the different structures and
//! associated functions for any number
//! of participants to simplify send/recv.

/// Create a new SessionMST structure, new roles and the baking environment.
///
/// # Arguments
///
/// * Name of the new SessionMST
/// * Names of the new roles
/// * \[Optional\] Name of the new fork function
///
/// # Basic example
///
/// ```
/// use mpstthree::bundle_impl;
///
/// bundle_impl!(MeshedChannelsThree => A, B, C);
/// ```
///
/// # Example with fork
///
/// ```
/// use mpstthree::bundle_impl;
///
/// bundle_impl!(MeshedChannelsThree => A, B, C => fork_mpst);
/// ```
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "baking")))]
macro_rules! bundle_impl {
    (
        $meshedchannels_name: ident =>
        $( $all_roles: ident),+ $(,)?
    ) => {
        mpst_seq::baking!(
            $meshedchannels_name,
            ( $( $all_roles , )+ )
        );
    };
    (
        $meshedchannels_name: ident =>
        $( $all_roles: ident),+ $(,)? =>
        $fork_mpst: ident
    ) => {
        mpst_seq::baking!(
            $meshedchannels_name ,
            ( $( $all_roles , )+ ) ,
            $fork_mpst
        );
    };
}

/// Create a new SessionMST structuren, new roles and the baking environment.
/// Also create the macros needed for choosing branches.
/// Each macro is linked to a role X and are called as followed:
///     choose_mpst_x_to_all!(
///         s, # the current session
///         enum_1::variant_1, # the first branch for the first passive role
///         enum_2::variant_2, # the first branch for the second passive role
///         ...
///         enum_n::variant_n, # the first branch for the n-th passive role
///     )
///
/// # Arguments
///
/// * Name of the new SessionMST
/// * Names of the new roles
/// * \[Optional\] Name of the new fork function
///
/// # Basic example
///
/// ```
/// use mpstthree::bundle_impl_with_enum;
///
/// bundle_impl_with_enum!(MeshedChannelsThree => A, B, C);
/// ```
///
/// # Example with fork
///
/// ```
/// use mpstthree::bundle_impl_with_enum;
///
/// bundle_impl_with_enum!(MeshedChannelsThree => A, B, C => fork_mpst);
/// ```
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "baking")))]
macro_rules! bundle_impl_with_enum {
    (
        $meshedchannels_name: ident =>
        $( $all_roles: ident),+ $(,)?
    ) => {
        mpst_seq::baking_with_enum!(
            $meshedchannels_name,
            ( $( $all_roles , )+ )
        );
    };
    (
        $meshedchannels_name: ident =>
        $( $all_roles: ident),+ $(,)? =>
        $fork_mpst: ident
    ) => {
        mpst_seq::baking_with_enum!(
            $meshedchannels_name ,
            ( $( $all_roles , )+ ) ,
            $fork_mpst
        );
    };
}
