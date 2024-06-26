//! This module contains the macros for
//! creating the different structures and
//! associated functions for any number
//! of participants in timed protocols to simplify send/recv.

pub mod choose;

/// Create new SessionMST structures, new roles and the baking environment,
/// with `send` functions that can fail, for timed protocols.
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
/// # Arguments
///
/// * Name of the new SessionMST
/// * Names of the new roles. They are called as RoleX where X is the name provided.
///
/// # Basic example
///
/// ```
/// use mpstthree::generate_atmp;
///
/// generate_atmp!(MeshedChannels, A, B, C);
/// ```
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "baking_atmp")))]
macro_rules! generate_atmp {
    (
        $meshedchannels_name: ident,
        $( $all_roles: ident),+ $(,)?
    ) => {
        mpst_seq_proc::baking_atmp_with_enum_and_cancel!(
            $meshedchannels_name ,
            $( $all_roles , )+
        );
    };
}
