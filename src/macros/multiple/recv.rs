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
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
/// * The index of the binary session type that will receive in the SessionMpst for this specific
///   role. Index starts at 1.
///
/// # Example
///
/// ```ignore
/// use mpstthree::{create_multiple_normal_role, create_sessionmpst, recv_mpst};
///
/// create_sessionmpst!(SessionMpstThree, 3);
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// fn main(s: Endpoint) -> Result<(), Box<dyn std::error::Error>> {
///    let (_payload, _s) = recv_mpst!(s, RoleB, RoleA, SessionMpstThree, 3, 1)()?;
/// }
/// ```
#[macro_export]
macro_rules! recv_mpst {
    ($session:expr, $sender:ident, $receiver:ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion { || -> Result<_, Box<dyn std::error::Error>> {
            %(
            )(
                let (v, new_session) = mpstthree::binary::recv::recv($session.session#N:0)?;
            )0*

            let new_stack = {
                fn temp<R>(r: $sender<R>) -> R
                where
                    R: mpstthree::role::Role,
                {
                    let (here, there) = <R as mpstthree::role::Role>::new();
                    r.sender.send(there).unwrap_or(());
                    here
                }
                temp($session.stack)
            };

            {
                fn temp(_s: &$receiver<mpstthree::role::end::RoleEnd>) -> Result<(), Box<dyn std::error::Error>> {
                    Ok(())
                }
                temp(&$session.name)
            }.unwrap();

            Ok((
                v,
                $sessionmpst_name {
                    %(
                        session#N:0: $session.session#N:0,
                    )(
                        session#N:0: new_session,
                    )0*
                    stack: new_stack,
                    name: $session.name,
                }
            ))
        }});
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! recv_aux {
    ($session:expr, $role:ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion { || -> Result<_, Box<dyn std::error::Error>> {
            %(
            )(
                let (v, new_session) = mpstthree::binary::recv::recv($session.session#N:0)?;
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
                temp($session.stack)
            };

            Ok((
                v,
                $sessionmpst_name {
                    %(
                        session#N:0: $session.session#N:0,
                    )(
                        session#N:0: new_session,
                    )0*
                    stack: new_stack,
                    name: $session.name,
                }
            ))
        }});
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! recv_all_aux {
    ($session:expr, $role:ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion { || -> Result<_, Box<dyn std::error::Error>> {
            %(
            )(
                let (v, new_session) = mpstthree::binary::recv::recv($session.session#N:0)?;
            )0*

            let (new_stack_left, _new_stack_right) = { // new_stack_right = new_stack_left
                fn temp(r: $role<crate::role::end::RoleEnd, crate::role::end::RoleEnd>) -> (crate::role::end::RoleEnd, crate::role::end::RoleEnd)
                {
                    let (here1, there1) = <crate::role::end::RoleEnd as crate::role::Role>::new();
                    let (here2, there2) = <crate::role::end::RoleEnd as crate::role::Role>::new();
                    r.sender1.send(there1).unwrap_or(());
                    r.sender2.send(there2).unwrap_or(());
                    (here1, here2)
                }
                temp($session.stack)
            };

            Ok((
                v,
                $sessionmpst_name {
                    %(
                        session#N:0: $session.session#N:0,
                    )(
                        session#N:0: new_session,
                    )0*
                    stack: new_stack_left,
                    name: $session.name,
                }
            ))
        }});
    }
}

/// Creates a *recv* function to receive from a simple role on a given binary session type of a
/// SessionMpst with more than 3 participants.
///
/// # Arguments
///
/// * The name of the new *recv* function
/// * The name of the sender
/// * The name of the related *next* function
/// * The name of the receiver
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
/// * The index of the binary session type that will receive in the SessionMpst for this specific
///   role. Index starts at 1.
///
/// # Example
///
/// ```
/// use mpstthree::{create_multiple_normal_role, create_recv_mpst_session, create_sessionmpst};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_sessionmpst!(SessionMpst, 3);
///
/// create_recv_mpst_session!(recv_mpst_d_from_a, RoleA, RoleD, SessionMpst, 3, 1);
/// ```
#[macro_export]
macro_rules! create_recv_mpst_session {
    ($func_name:ident, $role:ident, $name:ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion {
            fn $func_name<T, #(S#N:0,)0:0 R>(
                s: $sessionmpst_name<
                    %(
                        S#N:0,
                    )(
                        mpstthree::binary::struct_trait::Recv<T, S#N:0>,
                    )0*
                    $role<R>,
                    $name<mpstthree::role::end::RoleEnd>,
                >,
            ) -> Result<
                (
                    T,
                    $sessionmpst_name<#(S#N:0,)0:0 R, $name<mpstthree::role::end::RoleEnd>,
                    >,
                ),
                Box<dyn std::error::Error>,
            >
            where
                T: std::marker::Send,
                #(
                    S#N:0: mpstthree::binary::struct_trait::Session,
                )0:0
                R: mpstthree::role::Role,
            {
                mpstthree::recv_aux!(s, $role, $sessionmpst_name, $nsessions, $exclusion)()
            }
        });
    }
}

/// Creates multiple *recv* functions to receive from a simple role on a given binary session type
/// of a SessionMpst with more than 3 participants.
///
/// # Arguments
///
/// * The name of the new *recv* functions
/// * The name of the senders
/// * The name of the related *next* functions
/// * The name of the receiver
/// * The index of the binary session types that will receive in the SessionMpst for each specific
///   role. Index starts at 1.
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{create_multiple_normal_role, create_sessionmpst, create_recv_mpst_session_bundle};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_sessionmpst!(SessionMpst, 3);
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
///    SessionMpst,
///    3
/// );
/// ```
#[macro_export]
macro_rules! create_recv_mpst_session_bundle {
    ($($func_name:ident, $role:ident, $exclusion:literal | )+ => $name:ident, $sessionmpst_name:ident, $nsessions:literal) => {
       $(mpstthree::create_recv_mpst_session!($func_name, $role, $name, $sessionmpst_name, $nsessions, $exclusion);)+
    }
}

/// Creates a *recv* function to receive from a broadcasting role on a given binary session type of
/// a SessionMpst with more than 3 participants.
///
/// # Arguments
///
/// * The name of the new *recv* function
/// * The name of the broadcasting sender
/// * The name of the related *next* function
/// * The name of the receiver
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
/// * The index of the binary session type that will receive in the SessionMpst for this specific
///   role. Index starts at 1.
///
/// # Example
///
/// ```
/// use mpstthree::{
///     create_broadcast_role, create_normal_role, create_recv_mpst_all_session, create_sessionmpst,
/// };
///
/// create_normal_role!(RoleA, RoleADual);
/// create_broadcast_role!(RoleAlltoD, RoleDtoAll);
///
/// create_sessionmpst!(SessionMpst, 3);
///
/// create_recv_mpst_all_session!(recv_mpst_a_all_to_d, RoleAlltoD, RoleA, SessionMpst, 3, 2);
/// ```
#[macro_export]
macro_rules! create_recv_mpst_all_session {
    ($func_name:ident, $role:ident, $name:ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion {
            fn $func_name<T, #(S#N:0,)0:0 R>(
                s: $sessionmpst_name<
                    %(
                        S#N:0,
                    )(
                        mpstthree::binary::struct_trait::Recv<T, S#N:0>,
                    )0*
                    $role<R, R>,
                    $name<mpstthree::role::end::RoleEnd>,
                >,
            ) -> Result<
                (
                    T,
                    $sessionmpst_name<
                        #(
                            S#N:0,
                        )0:0
                        R,
                        $name<mpstthree::role::end::RoleEnd>,
                    >,
                ),
                Box<dyn std::error::Error>,
            >
            where
                T: std::marker::Send,
                #(
                    S#N:0: mpstthree::binary::struct_trait::Session,
                )0:0
                R: mpstthree::role::Role,
            {
                %(
                )(
                    let (v, new_session) = mpstthree::binary::recv::recv(s.session#N:0)?;
                )0*
                let (new_stack, _) = {
                    fn temp<R1, R2>(r: $role<R1, R2>) -> (R1, R2)
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
                    temp(s.stack)
                };

                let result = $sessionmpst_name {
                    %(
                        session#N:0: s.session#N:0,
                    )(
                        session#N:0: new_session,
                    )0*
                    stack: new_stack,
                    name: s.name,
                };

                Ok((v, result))
            }
        });
    }
}

/// Creates a *recv* function to receive from a simple role on a given binary session type of a
/// SessionMpst with more than 3 participants.
///
/// # Arguments
///
/// * The name of the new *recv* function
/// * The name of the sender
/// * The name of the related *next* function
/// * The name of the receiver
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
/// * The index of the binary session type that will receive in the SessionMpst for this specific
///   role. Index starts at 1.
///
/// # Example
///
/// ```
/// use mpstthree::{create_multiple_normal_role, create_recv_http_session, create_sessionmpst};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_sessionmpst!(SessionMpst, 3);
///
/// create_recv_http_session!(recv_mpst_d_from_a, RoleA, RoleD, SessionMpst, 3, 1);
/// ```
#[macro_export]
macro_rules! create_recv_http_session {
    ($func_name:ident, $role:ident, $name:ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion {
            fn $func_name<T, #(S#N:0,)0:0 R>(
                s: $sessionmpst_name<
                    %(
                        S#N:0,
                    )(
                        mpstthree::binary::struct_trait::Recv<T, S#N:0>,
                    )0*
                    $role<R>,
                    $name<mpstthree::role::end::RoleEnd>,
                >,
                http: bool,
                respfuture: Vec::<hyper::client::ResponseFuture>,
            ) -> Result<
                (
                    T,
                    $sessionmpst_name<#(S#N:0,)0:0 R, $name<mpstthree::role::end::RoleEnd>,
                    >,
                    hyper::Response<hyper::Body>
                ),
                Box<dyn std::error::Error>,
            >
            where
                T: std::marker::Send,
                #(
                    S#N:0: mpstthree::binary::struct_trait::Session,
                )0:0
                R: mpstthree::role::Role,
            {
                if respfuture.len() > 1 {
                    panic!("Too many futures: {:?}", respfuture.len())
                }

                let resp = match http {
                    true => {
                        let rt = tokio::runtime::Runtime::new()?;
                        rt.block_on(async move { respfuture[0].await })?
                    },
                    false => hyper::Response::default(),
                };

                let (v, s) = mpstthree::recv_aux!(s, $role, $sessionmpst_name, $nsessions, $exclusion)()?;

                Ok((v, s, resp))
            }
        });
    }
}

/// Creates multiple *recv* functions to receive from a
/// simple role on a given binary session type of a
/// SessionMpst with more than 3 participants.
///
/// # Arguments
///
/// * The name of the new *recv* functions
/// * The name of the senders
/// * The name of the related *next* functions
/// * The name of the receiver
/// * The index of the binary session types that will receive in the SessionMpst for each specific
///   role. Index starts at 1.
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{create_multiple_normal_role, create_sessionmpst, create_recv_http_session_bundle};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_sessionmpst!(SessionMpst, 3);
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
///    SessionMpst,
///    3
/// );
/// ```
#[macro_export]
macro_rules! create_recv_http_session_bundle {
    ($($func_name:ident, $role:ident, $exclusion:literal | )+ => $name:ident, $sessionmpst_name:ident, $nsessions:literal) => {
       $(mpstthree::create_recv_http_session!($func_name, $role, $name, $sessionmpst_name, $nsessions, $exclusion);)+
    }
}
