//! This module contains the macros
//! for creating send functions for three
//! of participants, whatever are their name.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"macros_simple"` feature.*

/// Create a *send* function to send on the first binary
/// session from any kind of role.  Must be used with
/// [`MeshedChannels`].
///
/// # Arguments
///
/// * The name of the new *send* function
/// * The role of the receiver
/// * The name of the sender
///
/// # Example
///
/// ```
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::{create_normal_name, create_normal_role, create_send_mpst_session_1};
///
/// create_normal_role!(RoleA, RoleADual);
/// create_normal_name!(NameC);
///
/// create_send_mpst_session_1!(send_mpst_c_to_a, RoleA, NameC);
/// ```
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_send_mpst_session_1 {
    ($func_name:ident, $receiver:ident, $sender:ident) => {
        mpst_seq_proc::create_send_mpst_session!(
            $func_name,
            $receiver,
            $sender,
            MeshedChannels,
            3,
            1
        );
    };
}

/// Create a *send* function to send on the second binary
/// session from any kind of role.  Must be used with
/// [`MeshedChannels`].
///
/// # Arguments
///
/// * The name of the new *send* function
/// * The role of the receiver
/// * The name of the sender
///
/// # Example
///
/// ```
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::{create_normal_name, create_normal_role, create_send_mpst_session_2};
///
/// create_normal_name!(NameA);
/// create_normal_role!(RoleC, RoleCDual);
///
/// create_send_mpst_session_2!(send_mpst_a_to_c, RoleC, NameA);
/// ```
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_send_mpst_session_2 {
    ($func_name:ident, $receiver:ident, $sender:ident) => {
        mpst_seq_proc::create_send_mpst_session!(
            $func_name,
            $receiver,
            $sender,
            MeshedChannels,
            3,
            2
        );
    };
}
