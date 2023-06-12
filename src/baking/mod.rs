#![cfg(feature = "baking")]

//! This module contains the macros for
//! creating the different structures and
//! associated functions for any number
//! of participants to simplify send/recv.

/// Create a new SessionMST structure, new roles and the baking environment.
/// This macro creates the related `fork_mpst` function.
///
/// # Arguments
///
/// * Type of baker (basic, recursive, cancel, rec_and_cancel, interleaved)
/// * Name of the new SessionMST
/// * Names of the new roles. They are called as RoleX where X is the name provided.
///
/// # Basic examples
///
/// ## Basic
///
/// ```
/// use mpstthree::baker;
///
/// baker!("basic", MeshedChannelsThree, A, B, C);
/// ```
///
/// ## Recursive
///
/// Also create the macros needed for choosing branches.
/// Each macro is linked to a role X and are called as followed:
///     choose_mpst_x_to_all!(
///         s, # the current session
///         enum_1::variant_1, # the first branch for the first passive role
///         enum_2::variant_2, # the first branch for the second passive role
///         ...
///         enum_n::variant_n, # the first branch for the n-th passive role
///     )
/// This macro creates the related `fork_mpst` function.
///
/// ```
/// use mpstthree::baker;
///
/// baker!("recursive", MeshedChannelsThree, A, B, C);
/// ```
///
/// ## Cancel
///
/// Also creates the primitives to handle an additional monitor that
/// checks the health of each participant and broadcasts any failure.
///
/// ```
/// use mpstthree::baker;
///
/// baker!("cancel", MeshedChannelsThree, A, B, C);
/// ```
///
/// ## Rec and Cancel
///
/// Also create the macros needed for choosing branches.
/// Each macro is linked to a role X and are called as followed:
///     choose_mpst_x_to_all!(
///         s, # the current session
///         enum_1::variant_1, # the first branch for the first passive role
///         enum_2::variant_2, # the first branch for the second passive role
///         ...
///         enum_n::variant_n, # the first branch for the n-th passive role
///     )
/// This macro creates the related `fork_mpst` function.
///
/// ```
/// use mpstthree::baker;
///
/// baker!("rec_and_cancel", MeshedChannelsThree, A, B, C);
/// ```
///
/// ## Interleaved
///
/// Also create the macros needed for interleaved sessions
/// (sessions where one participant is involved in two different
/// protocols)
///
/// ```
/// use mpstthree::baker;
///
/// baker!(
///     "interleaved",
///     MeshedChannelsBarber,
///     Barber,
///     ShopBarber,
///     2,
///     MeshedChannelsClient,
///     Client,
///     ShopClient,
///     2,
///     fork_mpst
/// );
/// ```
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "baking")))]
macro_rules! baker {
    (
        "basic",
        $meshedchannels_name: ident,
        $( $all_roles: ident ),+ $(,)?
    ) => {
        mpst_seq::baking!(
            $meshedchannels_name ,
            $( $all_roles , )+
        );
    };
    (
        "recursive",
        $meshedchannels_name: ident,
        $( $all_roles: ident ),+ $(,)?
    ) => {
        mpst_seq::baking_with_enum!(
            $meshedchannels_name ,
            $( $all_roles , )+
        );
    };
    (
        "cancel",
        $meshedchannels_name: ident,
        $( $all_roles: ident ),+ $(,)?
    ) => {
        mpst_seq::baking_with_cancel!(
            $meshedchannels_name ,
            $( $all_roles , )+
        );
    };
    (
        "rec_and_cancel",
        $meshedchannels_name: ident,
        $( $all_roles: ident ),+ $(,)?
    ) => {
        mpst_seq::baking_with_enum_and_cancel!(
            $meshedchannels_name ,
            $( $all_roles , )+
        );
    };
    (
        "interleaved",
        $meshedchannels_name_one: ident,
        $( $all_roles_one: ident , )+
        $index_tuple_one: literal,
        $meshedchannels_name_two: ident,
        $( $all_roles_two: ident , )+
        $index_tuple_two: literal,
        $func_name: ident
    ) => {
        mpst_seq::baking_interleaved_with_enum_and_cancel!(
            $meshedchannels_name_one ,
            ( $( ( $all_roles_one ) )+ ) ,
            $index_tuple_one ,
            $meshedchannels_name_two ,
            ( $( ( $all_roles_two ) )+ ) ,
            $index_tuple_two ,
            $func_name
        );
    };
    (
        "timed_interleaved",
        $meshedchannels_name_one: ident,
        $( $all_roles_one: ident , )+
        $index_tuple_one: literal,
        $meshedchannels_name_two: ident,
        $( $all_roles_two: ident , )+
        $index_tuple_two: literal,
        $func_name: ident
    ) => {
        mpst_seq::baking_timed_interleaved_with_enum_and_cancel!(
            $meshedchannels_name_one ,
            ( $( ( $all_roles_one ) )+ ) ,
            $index_tuple_one ,
            $meshedchannels_name_two ,
            ( $( ( $all_roles_two ) )+ ) ,
            $index_tuple_two ,
            $func_name
        );
    };
}
