//! This module contains the macros
//! for creating receive functions for any number
//! of participants.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"macros_multiple"` feature.*

/// Creates a *recv* function to receive from a simple role on a given binary session type of a
/// MeshedChannels.
///
/// # Arguments
///
/// * The name of the new *recv* function
/// * The name of the sender
/// * The name of the receiver
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
/// * The index of the binary session type that will receive in the MeshedChannels for this specific
///   role. Index starts at 1.
///
/// # Example
///
/// ```
/// use mpstthree::{create_multiple_normal_role, create_normal_name, create_recv_mpst_session, create_meshedchannels};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_normal_name!(NameD);
///
/// create_meshedchannels!(MeshedChannels, 3);
///
/// create_recv_mpst_session!(recv_mpst_d_from_a, RoleA, NameD, MeshedChannels, 3, 1);
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! create_recv_mpst_session {
    (
        $func_name:ident,
        $sender:ident,
        $receiver:ident,
        $meshedchannels_name:ident,
        $n_sessions:literal,
        $exclusion:literal
    ) => {
        mpst_seq::create_recv_mpst_session!(
            $func_name,
            $sender,
            $receiver,
            $meshedchannels_name,
            $n_sessions,
            $exclusion
        );
    };
}

/// Creates multiple *recv* functions to receive from a simple role on a given binary session type
/// of a MeshedChannels.
///
/// # Arguments
///
/// * The name of the new *recv* functions
/// * The name of the senders
/// * The name of the receiver
/// * The index of the binary session types that will receive in the MeshedChannels for each
///   specific role. Index starts at 1.
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{create_multiple_normal_role, create_normal_name, create_meshedchannels, create_recv_mpst_session_bundle};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_normal_name!(NameD);
///
/// create_meshedchannels!(MeshedChannels, 3);
///
///
/// create_recv_mpst_session_bundle!(
///    recv_mpst_d_from_a,
///    RoleA,
///    1 |
///    recv_mpst_d_from_b,
///    RoleB,
///    2 | =>
///    NameD,
///    MeshedChannels,
///    3
/// );
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! create_recv_mpst_session_bundle {
    ($( $func_name: ident, $sender: ident, $exclusion: literal | )+ => $receiver: ident, $meshedchannels_name: ident, $n_sessions: literal) => {
       $(
            mpstthree::create_recv_mpst_session!(
                $func_name,
                $sender,
                $receiver,
                $meshedchannels_name,
                $n_sessions,
                $exclusion
            );
        )+
    }
}
