////////////////////////////////////////////
/// RECV

/// Shorter way to call the code within the recv function
/// instead of having to create the function itself.
///
/// # Example
///
/// ```ignore
/// use mpstthree::{create_normal_role, create_sessionmpst, recv_mpst};
///
/// create_sessionmpst!(SessionMpstThree, 3);
///
/// create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
/// create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
///
/// fn main(s: Endpoint) -> Result<(), Box<dyn std::error::Error>> {
///     let (_payload, _s) = recv_mpst!(s, next_b, SessionMpstThree, 3, 1)()?;
/// }
/// ```
#[macro_export]
macro_rules! recv_mpst {
    ($session:expr, $next:ident, $struct_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion { || -> Result<_, Box<dyn std::error::Error>> {
            %(
            )(
                let (v, new_session) = mpstthree::binary::recv($session.session#N:0)?;
            )0*
            let new_queue = $next($session.stack);

            let result = $struct_name {
                %(
                    session#N:0: $session.session#N:0,
                )(
                    session#N:0: new_session,
                )0*
                stack: new_queue,
                name: $session.name,
            };

            Ok((v, result))
        }});
    }
}

/// Creates a *recv* function to receive from a simple role
/// on a given binary session type of a SessionMpst with
/// more than 3 participants.
///
///  # Arguments
///  
///  * The name of the new *recv* function
///  * The name of the sender
///  * The name of the related *next* function
///  * The name of the receiver
///  * The name of the *SessionMpst* type that will be used
///  * The number of participants (all together)
///  * The index of the binary session type that will receive in the SessionMpst for this specific
///    role. Index starts at 1.
///  
///  # Example
///  
///  ```
///  use mpstthree::role::Role;
///  use mpstthree::{create_normal_role, create_sessionmpst,
/// create_recv_mpst_session};
///
///  create_normal_role!(RoleA, next_a, RoleADual,
/// next_a_dual);  create_normal_role!(RoleD, next_d,
/// RoleDDual, next_d_dual);
///
///  create_sessionmpst!(SessionMpst, 3);
///
///  create_recv_mpst_session!(recv_mpst_d_to_a, RoleA,
/// next_a, RoleD, SessionMpst, 3, 1);
/// ```
#[macro_export]
macro_rules! create_recv_mpst_session {
    ($func_name:ident, $role:ident, $next:ident, $name:ident, $struct_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion {
            fn $func_name<T, #(S#N:0,)0:0 R>(
                s: $struct_name<
                    %(
                        S#N:0,
                    )(
                        mpstthree::binary::Recv<T, S#N:0>,
                    )0*
                    $role<R>,
                    $name<mpstthree::role::end::RoleEnd>,
                >,
            ) -> Result<
                (
                    T,
                    $struct_name<#(S#N:0,)0:0 R, $name<mpstthree::role::end::RoleEnd>,
                    >,
                ),
                Box<dyn std::error::Error>,
            >
            where
                T: std::marker::Send,
                #(
                    S#N:0: mpstthree::binary::Session,
                )0:0
                R: mpstthree::role::Role,
            {
                mpstthree::recv_mpst!(s, $next, $struct_name, $nsessions, $exclusion)()
            }
        });
    }
}

/// Creates multiple *recv* functions to receive from a
/// simple role on a given binary session type of a
/// SessionMpst with more than 3 participants.
///
///  # Arguments
///  
///  * The name of the new *recv* functions
///  * The name of the senders
///  * The name of the related *next* functions
///  * The name of the receiver
///  * The index of the binary session types that will receive in the SessionMpst for each specific
///    role. Index starts at 1.
///  * The name of the *SessionMpst* type that will be used
///  * The number of participants (all together)
///  
///  # Example
///  
///  ```
///  use mpstthree::role::Role;
///  use mpstthree::{create_normal_role, create_sessionmpst,
/// create_recv_mpst_session,
/// create_recv_mpst_session_bundle};
///
///  create_normal_role!(RoleA, next_a, RoleADual,
/// next_a_dual);  create_normal_role!(RoleB, next_b,
/// RoleBDual, next_b_dual);  create_normal_role!(RoleD,
/// next_d, RoleDDual, next_d_dual);
///
///  create_sessionmpst!(SessionMpst, 3);
///
///
///  create_recv_mpst_session_bundle!(
///     recv_mpst_d_to_a,
///     RoleA,
///     next_a,
///     1 |
///     recv_mpst_d_to_b,
///     RoleB,
///     next_b,
///     2 | =>
///     RoleD,
///     SessionMpst,
///     3
/// );
///  ```
#[macro_export]
macro_rules! create_recv_mpst_session_bundle {
    ($($func_name:ident, $role:ident, $next:ident, $exclusion:literal | )+ => $name:ident, $struct_name:ident, $nsessions:literal) => {
       $(mpstthree::create_recv_mpst_session!($func_name, $role, $next, $name, $struct_name, $nsessions, $exclusion);)+
    }
}

/// Creates a *recv* function to receive from a broadcasting
/// role on a given binary session type of a SessionMpst
/// with more than 3 participants.
///
///  # Arguments
///  
///  * The name of the new *recv* function
///  * The name of the broadcasting sender
///  * The name of the related *next* function
///  * The name of the receiver
///  * The name of the *SessionMpst* type that will be used
///  * The number of participants (all together)
///  * The index of the binary session type that will receive in the SessionMpst for this specific
///    role. Index starts at 1.
///  
///  # Example
///  
///  ```
///  use mpstthree::role::Role;
///  use mpstthree::{create_normal_role,
/// create_broadcast_role, create_sessionmpst,
/// create_recv_mpst_all_session};
///
///  create_normal_role!(RoleA, next_a, RoleADual,
/// next_a_dual);  create_broadcast_role!(RoleAlltoD,
/// next_all_to_d, RoleDtoAll, next_d_to_all);
///
///  create_sessionmpst!(SessionMpst, 3);
///
///  create_recv_mpst_all_session!(
///      recv_mpst_a_all_to_d,
///      RoleAlltoD,
///      next_all_to_d,
///      RoleA,
///      SessionMpst,
///      3,
///      2
///  );
///  ```
#[macro_export]
macro_rules! create_recv_mpst_all_session {
    ($func_name:ident, $role:ident, $next:ident, $name:ident, $struct_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion {
            fn $func_name<T, #(S#N:0,)0:0 R>(
                s: $struct_name<
                    %(
                        S#N:0,
                    )(
                        mpstthree::binary::Recv<T, S#N:0>,
                    )0*
                    $role<R, R>,
                    $name<mpstthree::role::end::RoleEnd>,
                >,
            ) -> Result<
                (
                    T,
                    $struct_name<
                        #(S#N:0,)0:0
                        R,
                        $name<mpstthree::role::end::RoleEnd>,
                    >,
                ),
                Box<dyn std::error::Error>,
            >
            where
                T: std::marker::Send,
                #(
                    S#N:0: mpstthree::binary::Session,
                )0:0
                R: mpstthree::role::Role,
            {
                %(
                )(
                    let (v, new_session) = mpstthree::binary::recv(s.session#N:0)?;
                )0*
                let (new_queue, _) = $next(s.stack);
                let result = $struct_name {
                    %(
                        session#N:0: s.session#N:0,
                    )(
                        session#N:0: new_session,
                    )0*
                    stack: new_queue,
                    name: s.name,
                };

                Ok((v, result))
            }
        });
    }
}

/// Creates a *recv* function to receive from a broadcasting
/// role on a given binary session type of a SessionMpst
/// with more than 3 participants.
///
///  # Arguments
///  
///  * The name of the new *recv* functions
///  * The name of the broadcasting senders
///  * The name of the related *next* functions
///  * The index of the binary session types that will receive in the SessionMpst for this specific
///    role. Index starts at 1.
///  * The name of the receiver
///  * The name of the *SessionMpst* type that will be used
///  * The number of participants (all together)
///  
///  # Example
///  
///  ```
///  use mpstthree::role::Role;
///  use mpstthree::{create_normal_role,
/// create_broadcast_role, create_sessionmpst,
/// create_recv_mpst_all_session,
/// create_recv_mpst_all_session_bundle};
///
///  create_normal_role!(RoleA, next_a, RoleADual,
/// next_a_dual);  create_broadcast_role!(RoleAlltoB,
/// next_all_to_b, RoleBtoAll, next_b_to_all);
/// create_broadcast_role!(RoleAlltoD, next_all_to_d,
/// RoleDtoAll, next_d_to_all);
///
///  create_sessionmpst!(SessionMpst, 3);
///
///  create_recv_mpst_all_session_bundle!(
///     recv_mpst_a_all_to_b,
///     RoleAlltoB,
///     next_all_to_b,
///     1 |
///     recv_mpst_a_all_to_d,
///     RoleAlltoD,
///     next_all_to_d,
///     2 | =>
///     RoleA,
///     SessionMpst,
///     3
/// );
///  ```
#[macro_export]
macro_rules! create_recv_mpst_all_session_bundle {
    ($($func_name:ident, $role:ident, $next:ident, $exclusion:literal | )+ => $name:ident, $struct_name:ident, $nsessions:literal) => {
       $(mpstthree::create_recv_mpst_all_session!($func_name, $role, $next, $name, $struct_name, $nsessions, $exclusion);)+
    }
}
