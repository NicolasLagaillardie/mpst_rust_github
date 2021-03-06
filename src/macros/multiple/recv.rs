////////////////////////////////////////////
/// RECV

/// Shorter way to call the code within the recv function instead of having to create the function
/// itself.
///
/// # Arguments
///
/// * The session that will be used
/// * The name of the sender
/// * The name of the receiver
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
/// * The index of the binary session type that will receive in the MeshedChannels for this specific
///   role. Index starts at 1.
///
/// # Example
///
/// ```ignore
/// use mpstthree::{create_multiple_normal_role, create_meshedchannels, recv_mpst};
///
/// create_meshedchannels!(MeshedChannelsThree, 3);
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// fn main(s: Endpoint) -> Result<(), Box<dyn std::error::Error>> {
///    let (_payload, _s) = recv_mpst!(s, RoleB, RoleA, MeshedChannelsThree, 3, 1)()?;
/// }
/// ```
#[macro_export]
macro_rules! recv_mpst {
    ($session: expr, $sender: ident, $receiver: ident, $meshedchannels_name: ident, $nsessions: literal, $exclusion: literal) => {
        mpst_seq::recv_mpst!(
            $session,
            $sender,
            $receiver,
            $meshedchannels_name,
            $nsessions,
            $exclusion
        );
    };
}

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
/// use mpstthree::{create_multiple_normal_role, create_recv_mpst_session, create_meshedchannels};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_meshedchannels!(MeshedChannels, 3);
///
/// create_recv_mpst_session!(recv_mpst_d_from_a, RoleA, RoleD, MeshedChannels, 3, 1);
/// ```
#[macro_export]
macro_rules! create_recv_mpst_session {
    ($func_name: ident, $sender: ident, $receiver: ident, $meshedchannels_name: ident, $nsessions: literal, $exclusion: literal) => {
        mpst_seq::create_recv_mpst_session!(
            $func_name,
            $sender,
            $receiver,
            $meshedchannels_name,
            $nsessions,
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
/// * The index of the binary session types that will receive in the MeshedChannels for each specific
///   role. Index starts at 1.
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{create_multiple_normal_role, create_meshedchannels, create_recv_mpst_session_bundle};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
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
///    RoleD,
///    MeshedChannels,
///    3
/// );
/// ```
#[macro_export]
macro_rules! create_recv_mpst_session_bundle {
    ($( $func_name: ident, $sender: ident, $exclusion: literal | )+ => $receiver: ident, $meshedchannels_name: ident, $nsessions: literal) => {
       $(
            mpstthree::create_recv_mpst_session!(
                $func_name,
                $sender,
                $receiver,
                $meshedchannels_name,
                $nsessions,
                $exclusion
            );
        )+
    }
}

/// Creates a *recv* function to receive from a broadcasting role on a given binary session type of
/// a MeshedChannels.
///
/// # Arguments
///
/// * The name of the new *recv* function
/// * The name of the broadcasting sender
/// * The name of the receiver
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
/// * The index of the binary session type that will receive in the MeshedChannels for this specific
///   role. Index starts at 1.
///
/// # Example
///
/// ```
/// use mpstthree::{
///     create_broadcast_role, create_normal_role, create_recv_mpst_all_session, create_meshedchannels,
/// };
///
/// create_normal_role!(RoleA, RoleADual);
/// create_broadcast_role!(RoleAlltoD, RoleDtoAll);
///
/// create_meshedchannels!(MeshedChannels, 3);
///
/// create_recv_mpst_all_session!(recv_mpst_a_all_from_d, RoleAlltoD, RoleA, MeshedChannels, 3, 2);
/// ```
#[macro_export]
macro_rules! create_recv_mpst_all_session {
    ($func_name: ident, $sender: ident, $receiver: ident, $meshedchannels_name: ident, $nsessions: literal, $exclusion: literal) => {
        mpst_seq::create_recv_mpst_all_session!(
            $func_name,
            $sender,
            $receiver,
            $meshedchannels_name,
            $nsessions,
            $exclusion
        );
    };
}

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
/// use mpstthree::{create_multiple_normal_role, create_recv_http_session, create_meshedchannels};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_meshedchannels!(MeshedChannels, 3);
///
/// create_recv_http_session!(recv_mpst_d_from_a, RoleA, RoleD, MeshedChannels, 3, 1);
/// ```
#[macro_export]
macro_rules! create_recv_http_session {
    ($func_name: ident, $sender: ident, $receiver: ident, $meshedchannels_name: ident, $nsessions: literal, $exclusion: literal) => {
        mpst_seq::create_recv_http_session!(
            $func_name,
            $sender,
            $receiver,
            $meshedchannels_name,
            $nsessions,
            $exclusion
        );
    };
}

/// Creates multiple *recv* functions to receive from a
/// simple role on a given binary session type of a
/// MeshedChannels.
///
/// # Arguments
///
/// * The name of the new *recv* functions
/// * The name of the senders
/// * The name of the receiver
/// * The index of the binary session types that will receive in the MeshedChannels for each specific
///   role. Index starts at 1.
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{create_multiple_normal_role, create_meshedchannels, create_recv_http_session_bundle};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_meshedchannels!(MeshedChannels, 3);
///
///
/// create_recv_http_session_bundle!(
///    recv_mpst_d_from_a,
///    RoleA,
///    1 |
///    recv_mpst_d_from_b,
///    RoleB,
///    2 | =>
///    RoleD,
///    MeshedChannels,
///    3
/// );
/// ```
#[macro_export]
macro_rules! create_recv_http_session_bundle {
    ($( $func_name: ident, $sender: ident, $exclusion: literal | )+ => $receiver: ident, $meshedchannels_name: ident, $nsessions: literal) => {
       $(
           mpstthree::create_recv_http_session!(
               $func_name,
               $sender,
               $receiver,
               $meshedchannels_name,
               $nsessions,
               $exclusion
            );
        )+
    }
}
