////////////////////////////////////////////
/// SEND

/// Shorter way to call the code within the send function instead of having to create the function
/// itself.
///
/// # Arguments
///
/// * The session that will be used
/// * The payload that will be send
/// * The name of the receiver
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
/// * The index of the binary session type that will receive in the SessionMpst for this specific
///   role. Index starts at 1.
///
/// # Example
///
/// ```ignore
/// use mpstthree::{create_multiple_normal_role, create_sessionmpst, send_mpst};
///
/// create_sessionmpst!(SessionMpstThree, 3);
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// fn main(s: Endpoint) {
///    let _s = send_mpst!(s, (), RoleB, RoleA, SessionMpstThree, 3, 1);
/// }
/// ```
#[macro_export]
macro_rules! send_mpst {
    ($session: expr, $payload: expr, $receiver: ident, $sender: ident, $sessionmpst_name: ident, $nsessions: literal, $exclusion: literal) => {
        mpst_seq::send_mpst!(
            ( $session ),
            ( $payload ),
            $receiver,
            $sender,
            $sessionmpst_name,
            $nsessions,
            $exclusion
        );
    }
}

/// Creates a *send* function to send from a given binary session type of a SessionMpst with more
/// than 3 participants.
///
/// # Arguments
///
/// * The name of the new *send* function
/// * The name of the receiver
/// * The name of the sender
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
/// * The index of the binary session type that will receive in the SessionMpst for this specific
///   role. Index starts at 1.
///
/// # Example
///
/// ```
/// use mpstthree::role::Role;
/// use mpstthree::{create_multiple_normal_role, create_send_mpst_session, create_sessionmpst};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_sessionmpst!(SessionMpst, 3);
///
/// create_send_mpst_session!(send_mpst_d_to_a, RoleA, RoleD, SessionMpst, 3, 1);
/// ```
#[macro_export]
macro_rules! create_send_mpst_session {
    ($func_name: ident, $receiver: ident, $sender: ident, $sessionmpst_name: ident, $nsessions: literal, $exclusion: literal) => {
        mpst_seq::create_send_mpst_session!(
            $func_name,
            $receiver,
            $sender,
            $sessionmpst_name,
            $nsessions,
            $exclusion
        );
    }
}

/// Creates a *send* function to send from a given binary session type of a SessionMpst with more
/// than 3 participants. The send function will try to send and panic if not possible (canceled
/// session).
///
/// # Arguments
///
/// * The name of the new *send* function
/// * The name of the receiver
/// * The name of the sender
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
/// * The index of the binary session type that will receive in the SessionMpst for this specific
///   role. Index starts at 1.
///
/// # Example
///
/// ```
/// use mpstthree::role::Role;
/// use mpstthree::{create_multiple_normal_role, create_send_mpst_cancel, create_sessionmpst};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_sessionmpst!(SessionMpst, 3);
///
/// create_send_mpst_cancel!(send_cancel_d_to_a, RoleA, RoleD, SessionMpst, 3, 1);
/// ```
#[macro_export]
macro_rules! create_send_mpst_cancel {
    ($func_name: ident, $receiver: ident, $sender: ident, $sessionmpst_name: ident, $nsessions: literal, $exclusion: literal) => {
        mpst_seq::create_send_mpst_cancel!(
            $func_name,
            $receiver,
            $sender,
            $sessionmpst_name,
            $nsessions,
            $exclusion
        );
    }
}

/// Creates a *send* function to send from a given binary session type of a SessionMpst with more
/// than 3 participants. Checks if the first binary session has a "cancel" signal and panic if yes.
/// The send function will try to send and panic if not possible (canceled session).
///
/// # Arguments
///
/// * The name of the new *send* function
/// * The name of the receiver
/// * The name of the sender
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
/// * The index of the binary session type that will receive in the SessionMpst for this specific
///   role. Index starts at 2 as 1 is an End.
///
/// # Example
///
/// ```
/// use mpstthree::role::Role;
/// use mpstthree::{create_normal_role, create_send_check_cancel, create_sessionmpst};
///
/// create_normal_role!(RoleB, RoleBDual);
/// create_normal_role!(RoleD, RoleDDual);
///
/// create_sessionmpst!(SessionMpst, 3);
///
/// create_send_check_cancel!(send_cancel_d_to_b, RoleB, RoleD, SessionMpst, 3, 2);
/// ```
///
/// # Compile fail
///
/// ```compile_fail
/// use mpstthree::role::Role;
/// use mpstthree::{create_normal_role, create_send_check_cancel, create_sessionmpst};
///
/// create_normal_role!(RoleA, RoleADual);
/// create_normal_role!(RoleD, RoleDDual);
///
/// create_sessionmpst!(SessionMpst, 3);
///
/// create_send_check_cancel!(send_cancel_d_to_b, RoleA, RoleD, SessionMpst, 3, 1);
/// ```
#[macro_export]
macro_rules! create_send_check_cancel {
    ($func_name: ident, $role: ident, $name: ident, $sessionmpst_name: ident, $nsessions: literal, $exclusion: literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion {
            fn $func_name<T, #(S#N:0,)16:0 R>(
                x: T,
                s: $sessionmpst_name<
                    mpstthree::binary::struct_trait::End,
                    %(
                        S#N:0,
                    )(
                        mpstthree::binary::struct_trait::Send<T, S#N:0>,
                    )3*
                    $role<R>,
                    $name<mpstthree::role::end::RoleEnd>,
                >,
            ) -> Result<$sessionmpst_name<
                mpstthree::binary::struct_trait::End,
                #(S#N:0,)16:0
                R,
                $name<mpstthree::role::end::RoleEnd>
            >, std::boxed::Box<dyn std::error::Error>>
            where
                T: std::marker::Send,
                #(
                    S#N:0: mpstthree::binary::struct_trait::Session,
                )16:0
                R: mpstthree::role::Role,
            {
                match s.session1.receiver.try_recv() {
                    Ok(mpstthree::binary::struct_trait::Signal::Cancel) => {
                        mpstthree::binary::cancel::cancel(s);
                        panic!("Error")
                    },
                    _ => {}
                };

                %(
                )(
                    let new_session = mpstthree::binary::send::send_canceled(x, s.session#N:0)?;
                )0*

                let new_stack = {
                    fn temp<R>(r: $role<R>) -> R
                    where
                        R: mpstthree::role::Role,
                    {
                        let (here, there) = <R as mpstthree::role::Role>::new();
                        r.sender.send(there).unwrap_or(());
                        here
                    }
                    temp(s.stack)
                };

                Ok($sessionmpst_name {
                    %(
                        session#N:0: s.session#N:0,
                    )(
                        session#N:0: new_session,
                    )0*
                    stack: new_stack,
                    name: s.name,
                })
            }
        });
    }
}

/// Creates a *send* function to send from a given binary session type of a SessionMpst with more
/// than 3 participants.
///
/// # Arguments
///
/// * The name of the new *send* function
/// * The name of the receiver
/// * The name of the sender
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
/// * The index of the binary session type that will receive in the SessionMpst for this specific
///   role. Index starts at 1.
///
/// # Example
///
/// ```
/// use mpstthree::role::Role;
/// use mpstthree::{create_multiple_normal_role, create_send_http_session, create_sessionmpst};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_sessionmpst!(SessionMpst, 3);
///
/// create_send_http_session!(send_http_d_to_a, RoleA, RoleD, SessionMpst, 3, 1);
/// ```
#[macro_export]
macro_rules! create_send_http_session {
    ($func_name: ident, $receiver: ident, $sender: ident, $sessionmpst_name: ident, $nsessions: literal, $exclusion: literal) => {
        mpst_seq::create_send_http_session!(
            $func_name,
            $receiver,
            $sender,
            $sessionmpst_name,
            $nsessions,
            $exclusion
        );
    }
}

/// Creates multiple *send* functions to send from a given binary session type of a SessionMpst with
/// more than 3 participants.
///
/// # Arguments
///
/// * The name of the new *send* functions
/// * The name of the receivers
/// * The name of the senders
/// * The index of the binary session types that will receive in the SessionMpst for this specific
///   role. Index starts at 1.
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::role::Role;
/// use mpstthree::{create_multiple_normal_role, create_sessionmpst, create_send_mpst_session, create_send_mpst_session_bundle};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_sessionmpst!(SessionMpst, 3);
///
/// create_send_mpst_session_bundle!(
///    send_mpst_d_to_a,
///    RoleA,
///    1 |
///    send_mpst_d_to_b,
///    RoleB,
///    2 | =>
///    RoleD,
///    SessionMpst,
///    3
/// );
/// ```
#[macro_export]
macro_rules! create_send_mpst_session_bundle {
    ($($func_name: ident, $role: ident, $exclusion: literal | )+ => $name: ident, $sessionmpst_name: ident, $nsessions: literal) => {
       $(mpstthree::create_send_mpst_session!($func_name, $role, $name, $sessionmpst_name, $nsessions, $exclusion);)+
    }
}

/// Creates multiple *send* functions to send from a given binary session type of a SessionMpst with
/// more than 3 participants.
///
/// # Arguments
///
/// * The name of the new *send* functions
/// * The name of the receivers
/// * The name of the senders
/// * The index of the binary session types that will receive in the SessionMpst for this specific
///   role. Index starts at 1.
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::role::Role;
/// use mpstthree::{create_multiple_normal_role, create_sessionmpst, create_send_mpst_cancel_bundle};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_sessionmpst!(SessionMpst, 3);
///
/// create_send_mpst_cancel_bundle!(
///    send_cancel_d_to_a,
///    RoleA,
///    1 |
///    send_cancel_d_to_b,
///    RoleB,
///    2 | =>
///    RoleD,
///    SessionMpst,
///    3
/// );
/// ```
#[macro_export]
macro_rules! create_send_mpst_cancel_bundle {
    ($($func_name: ident, $role: ident, $exclusion: literal | )+ => $name: ident, $sessionmpst_name: ident, $nsessions: literal) => {
       $(mpstthree::create_send_mpst_cancel!($func_name, $role, $name, $sessionmpst_name, $nsessions, $exclusion);)+
    }
}

/// Creates multiple *send* functions to send from a given binary session type of a SessionMpst with
/// more than 3 participants.
///
/// # Arguments
///
/// * The name of the new *send* functions
/// * The name of the receivers
/// * The name of the senders
/// * The index of the binary session types that will receive in the SessionMpst for this specific
///   role. Index starts at 2 as 1 is an End.
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::role::Role;
/// use mpstthree::{create_normal_role, create_sessionmpst, create_send_check_cancel_bundle};
///
/// create_normal_role!(RoleA, RoleADual);
/// create_normal_role!(RoleB, RoleBDual);
/// create_normal_role!(RoleD, RoleDDual);
///
/// create_sessionmpst!(SessionMpst, 3);
///
/// create_send_check_cancel_bundle!(
///    send_cancel_d_to_b,
///    RoleB,
///    2 | =>
///    RoleD,
///    SessionMpst,
///    3
/// );
/// ```
///
/// # Compile fail
///
/// ```compile_fail
/// use mpstthree::role::Role;
/// use mpstthree::{create_multiple_normal_role, create_sessionmpst, create_send_check_cancel_bundle};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_sessionmpst!(SessionMpst, 3);
///
/// create_send_check_cancel_bundle!(
///    send_cancel_d_to_a,
///    RoleA,
///    1 |
///    send_cancel_d_to_b,
///    RoleB,
///    2 | =>
///    RoleD,
///    SessionMpst,
///    3
/// );
/// ```
#[macro_export]
macro_rules! create_send_check_cancel_bundle {
    ($($func_name: ident, $role: ident, $exclusion: literal | )+ => $name: ident, $sessionmpst_name: ident, $nsessions: literal) => {
       $(mpstthree::create_send_check_cancel!($func_name, $role, $name, $sessionmpst_name, $nsessions, $exclusion);)+
    }
}

/// Creates multiple *send* functions to send from a given binary session type of a SessionMpst with
/// more than 3 participants.
///
/// # Arguments
///
/// * The name of the new *send* functions
/// * The name of the receivers
/// * The name of the senders
/// * The index of the binary session types that will receive in the SessionMpst for this specific
///   role. Index starts at 1.
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::role::Role;
/// use mpstthree::{create_multiple_normal_role, create_sessionmpst, create_send_mpst_cancel, create_send_mpst_http_bundle};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_sessionmpst!(SessionMpst, 3);
///
/// create_send_mpst_http_bundle!(
///    send_http_d_to_a,
///    RoleA,
///    1 |
///    send_http_d_to_b,
///    RoleB,
///    2 | =>
///    RoleD,
///    SessionMpst,
///    3
/// );
/// ```
#[macro_export]
macro_rules! create_send_mpst_http_bundle {
    ($($func_name: ident, $role: ident, $exclusion: literal | )+ => $name: ident, $sessionmpst_name: ident, $nsessions: literal) => {
       $(mpstthree::create_send_http_session!($func_name, $role, $name, $sessionmpst_name, $nsessions, $exclusion);)+
    }
}
