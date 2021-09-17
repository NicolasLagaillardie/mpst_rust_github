//! This module contains the macros
//! for creating send functions for any number
//! of participants.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"macros_multiple"` feature.*

/// Shorter way to call the code within the send function instead of having to create the function
/// itself.
///
/// # Arguments
///
/// * The session that will be used
/// * The payload that will be send
/// * The name of the receiver
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
/// * The index of the binary session type that will receive in the MeshedChannels for this specific
///   role. Index starts at 1.
///
/// # Example
///
/// ```ignore
/// use mpstthree::{create_multiple_normal_role, create_meshedchannels, send_mpst};
///
/// create_meshedchannels!(MeshedChannelsThree, 3);
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// fn main(s: Endpoint) {
///    let _s = send_mpst!(s, (), RoleB, RoleA, MeshedChannelsThree, 3, 1);
/// }
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! send_mpst {
    (
        $session:expr,
        $payload:expr,
        $receiver:ident,
        $sender:ident,
        $meshedchannels_name:ident,
        $nsessions:literal,
        $exclusion:literal
    ) => {
        mpst_seq_baking::send_mpst!(
            $session,
            $payload,
            $receiver,
            $sender,
            $meshedchannels_name,
            $nsessions,
            $exclusion
        );
    };
}

/// Creates a *send* function to send from a given binary session type of a MeshedChannels with more
/// than 3 participants.
///
/// # Arguments
///
/// * The name of the new *send* function
/// * The name of the receiver
/// * The name of the sender
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
/// * The index of the binary session type that will receive in the MeshedChannels for this specific
///   role. Index starts at 1.
///
/// # Example
///
/// ```
/// use mpstthree::{create_multiple_normal_role, create_send_mpst_session, create_meshedchannels};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_meshedchannels!(MeshedChannels, 3);
///
/// create_send_mpst_session!(send_mpst_d_to_a, RoleA, RoleD, MeshedChannels, 3, 1);
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! create_send_mpst_session {
    (
        $func_name:ident,
        $receiver:ident,
        $sender:ident,
        $meshedchannels_name:ident,
        $nsessions:literal,
        $exclusion:literal
    ) => {
        mpst_seq_baking::create_send_mpst_session!(
            $func_name,
            $receiver,
            $sender,
            $meshedchannels_name,
            $nsessions,
            $exclusion
        );
    };
}

/// Creates a *send* function to send from a given binary session type of a MeshedChannels with more
/// than 3 participants. The send function will try to send and panic if not possible (canceled
/// session).
///
/// # Arguments
///
/// * The name of the new *send* function
/// * The name of the receiver
/// * The name of the sender
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
/// * The index of the binary session type that will receive in the MeshedChannels for this specific
///   role. Index starts at 1.
///
/// # Example
///
/// ```
/// use mpstthree::{create_multiple_normal_role, create_send_mpst_cancel, create_meshedchannels};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_meshedchannels!(MeshedChannels, 3);
///
/// create_send_mpst_cancel!(send_cancel_d_to_a, RoleA, RoleD, MeshedChannels, 3, 1);
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! create_send_mpst_cancel {
    (
        $func_name:ident,
        $receiver:ident,
        $sender:ident,
        $meshedchannels_name:ident,
        $nsessions:literal,
        $exclusion:literal
    ) => {
        mpst_seq_baking::create_send_mpst_cancel!(
            $func_name,
            $receiver,
            $sender,
            $meshedchannels_name,
            $nsessions,
            $exclusion
        );
    };
}

/// Creates a *send* function to send from a given binary session type of a MeshedChannels with more
/// than 3 participants. Checks if the first binary session has a "cancel" signal and panic if yes.
/// The send function will try to send and panic if not possible (canceled session).
///
/// # Arguments
///
/// * The name of the new *send* function
/// * The name of the receiver
/// * The name of the sender
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
/// * The index of the binary session type that will receive in the MeshedChannels for this specific
///   role. Index starts at 2 as 1 is an End.
///
/// # Example
///
/// ```
/// use mpstthree::{create_meshedchannels, create_normal_role, create_send_check_cancel};
///
/// create_normal_role!(RoleB, RoleBDual);
/// create_normal_role!(RoleD, RoleDDual);
///
/// create_meshedchannels!(MeshedChannels, 3);
///
/// create_send_check_cancel!(send_cancel_d_to_b, RoleB, RoleD, MeshedChannels, 3, 2);
/// ```
///
/// # Compile fail
///
/// ```compile_fail
/// use mpstthree::{create_meshedchannels, create_normal_role, create_send_check_cancel};
///
/// create_normal_role!(RoleA, RoleADual);
/// create_normal_role!(RoleD, RoleDDual);
///
/// create_meshedchannels!(MeshedChannels, 3);
///
/// create_send_check_cancel!(send_cancel_d_to_b, RoleA, RoleD, MeshedChannels, 3, 1);
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! create_send_check_cancel {
    (
        $func_name:ident,
        $receiver:ident,
        $sender:ident,
        $meshedchannels_name:ident,
        $nsessions:literal,
        $exclusion:literal
    ) => {
        mpst_seq_baking::create_send_check_cancel!(
            $func_name,
            $receiver,
            $sender,
            $meshedchannels_name,
            $nsessions,
            $exclusion
        );
    };
}

/// Creates multiple *send* functions to send from a given binary session type of a MeshedChannels
/// with more than 3 participants.
///
/// # Arguments
///
/// * The name of the new *send* functions
/// * The name of the receivers
/// * The name of the senders
/// * The index of the binary session types that will receive in the MeshedChannels for this
///   specific role. Index starts at 1.
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{create_multiple_normal_role, create_meshedchannels, create_send_mpst_session_bundle};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_meshedchannels!(MeshedChannels, 3);
///
/// create_send_mpst_session_bundle!(
///    send_mpst_d_to_a,
///    RoleA,
///    1 |
///    send_mpst_d_to_b,
///    RoleB,
///    2 | =>
///    RoleD,
///    MeshedChannels,
///    3
/// );
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! create_send_mpst_session_bundle {
    ($( $func_name: ident, $receiver: ident, $exclusion: literal | )+ => $sender: ident, $meshedchannels_name: ident, $nsessions: literal) => {
       $(
           mpstthree::create_send_mpst_session!(
               $func_name,
               $receiver,
               $sender,
               $meshedchannels_name,
               $nsessions,
               $exclusion
            );
        )+
    }
}

/// Creates multiple *send* functions to send from a given binary session type of a MeshedChannels
/// with more than 3 participants.
///
/// # Arguments
///
/// * The name of the new *send* functions
/// * The name of the receivers
/// * The name of the senders
/// * The index of the binary session types that will receive in the MeshedChannels for this
///   specific role. Index starts at 1.
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{create_multiple_normal_role, create_meshedchannels, create_send_mpst_cancel_bundle};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_meshedchannels!(MeshedChannels, 3);
///
/// create_send_mpst_cancel_bundle!(
///    send_cancel_d_to_a,
///    RoleA,
///    1 |
///    send_cancel_d_to_b,
///    RoleB,
///    2 | =>
///    RoleD,
///    MeshedChannels,
///    3
/// );
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! create_send_mpst_cancel_bundle {
    ($( $func_name: ident, $receiver: ident, $exclusion: literal | )+ => $sender: ident, $meshedchannels_name: ident, $nsessions: literal) => {
       $(
           mpstthree::create_send_mpst_cancel!(
            $func_name,
            $receiver,
            $sender,
            $meshedchannels_name,
            $nsessions,
            $exclusion
            );
        )+
    }
}

/// Creates multiple *send* functions to send from a given binary session type of a MeshedChannels
/// with more than 3 participants.
///
/// # Arguments
///
/// * The name of the new *send* functions
/// * The name of the receivers
/// * The name of the senders
/// * The index of the binary session types that will receive in the MeshedChannels for this
///   specific role. Index starts at 2 as 1 is an End.
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{create_normal_role, create_meshedchannels, create_send_check_cancel_bundle};
///
/// create_normal_role!(RoleA, RoleADual);
/// create_normal_role!(RoleB, RoleBDual);
/// create_normal_role!(RoleD, RoleDDual);
///
/// create_meshedchannels!(MeshedChannels, 3);
///
/// create_send_check_cancel_bundle!(
///    send_cancel_d_to_b,
///    RoleB,
///    2 | =>
///    RoleD,
///    MeshedChannels,
///    3
/// );
/// ```
///
/// # Compile fail
///
/// ```compile_fail
/// use mpstthree::{create_multiple_normal_role, create_meshedchannels, create_send_check_cancel_bundle};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_meshedchannels!(MeshedChannels, 3);
///
/// create_send_check_cancel_bundle!(
///    send_cancel_d_to_a,
///    RoleA,
///    1 |
///    send_cancel_d_to_b,
///    RoleB,
///    2 | =>
///    RoleD,
///    MeshedChannels,
///    3
/// );
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! create_send_check_cancel_bundle {
    ($( $func_name: ident, $receiver: ident, $exclusion: literal | )+ => $sender: ident, $meshedchannels_name: ident, $nsessions: literal) => {
       $(
           mpstthree::create_send_check_cancel!(
               $func_name,
               $receiver,
               $sender,
               $meshedchannels_name,
               $nsessions,
               $exclusion
            );
        )+
    }
}
