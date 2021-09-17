//! This module contains the macros
//! for creating receive functions for three
//! of participants, whatever are their name.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"macros_simple"` feature.*

/// Create a *recv* function to recv on the first binary
/// session from any kind of role.  Must be used with
/// [`MeshedChannels`].
///
/// # Arguments
///
/// * The name of the new *recv* function
/// * The name of the sender
/// * The name of the receiver
///
/// # Example
///
/// ```
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::{create_normal_role, create_recv_mpst_session_1};
///
/// create_normal_role!(RoleA, RoleADual);
/// create_normal_role!(RoleC, RoleCDual);
///
/// create_recv_mpst_session_1!(recv_mpst_c_from_a, RoleA, RoleC);
/// ```
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_recv_mpst_session_1 {
    ($func_name:ident, $sender:ident, $receiver:ident) => {
        mpst_seq_baking::create_recv_mpst_session!($func_name, $sender, $receiver, MeshedChannels, 3, 1);
    };
}

/// Create a *recv* function to recv on the second binary
/// session from any kind of role.  Must be used with
/// [`MeshedChannels`].
///
/// # Arguments
///
/// * The name of the new *recv* function
/// * The name of the sender
/// * The name of the receiver
///
/// # Example
///
/// ```
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::{create_normal_role, create_recv_mpst_session_2};
///
/// create_normal_role!(RoleA, RoleADual);
/// create_normal_role!(RoleC, RoleCDual);
///
/// create_recv_mpst_session_2!(recv_mpst_a_from_c, RoleC, RoleA);
/// ```
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_recv_mpst_session_2 {
    ($func_name:ident, $sender:ident, $receiver:ident) => {
        mpst_seq_baking::create_recv_mpst_session!($func_name, $sender, $receiver, MeshedChannels, 3, 2);
    };
}
