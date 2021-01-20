////////////////////////////////////////////
/// SEND

/// Creates a *send* function to send from a given binary session type of a SessionMpst with more than 3 participants.
///
///  # Arguments
///  
///  * The name of the new *send* function
///  * The name of the receiver
///  * The name of the related *next* function
///  * The name of the sender
///  * The name of the *SessionMpst* type that will be used
///  * The number of participants (all together)
///  * The index of the binary session type that will receive in the SessionMpst for this specific role. Index starts at 1.
///  
///  # Example
///  
///  ```
///  use mpstthree::role::Role;
///  use mpstthree::{create_normal_role, create_sessionmpst, create_send_mpst_session};
///
///  create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
///  create_normal_role!(RoleD, next_d, RoleDDual, next_d_dual);
///
///  create_sessionmpst!(SessionMpst, 3);
///
///  create_send_mpst_session!(send_mpst_d_to_a, RoleA, next_a, RoleD, SessionMpst, 3, 1);
///  ```
#[macro_export]
macro_rules! create_send_mpst_session {
    ($func_name:ident, $role:ident, $next:ident, $name:ident, $struct_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion {
            fn $func_name<T, #(S#N:0,)0:0 R>(
                x: T,
                s: $struct_name<
                    %(
                        S#N:0,
                    )(
                        mpstthree::binary::Send<T, S#N:0>,
                    )0*
                    $role<R>,
                    $name<mpstthree::role::end::RoleEnd>,
                >,
            ) -> $struct_name<#(S#N:0,)0:0 R, $name<mpstthree::role::end::RoleEnd>>
            where
                T: std::marker::Send,
                #(
                    S#N:0: mpstthree::binary::Session,
                )0:0
                R: mpstthree::role::Role,
            {
                %(
                )(
                    let new_session = mpstthree::binary::send(x, s.session#N:0);
                )0*
                let new_queue = $next(s.stack);

                $struct_name {
                    %(
                        session#N:0: s.session#N:0,
                    )(
                        session#N:0: new_session,
                    )0*
                    stack: new_queue,
                    name: s.name,
                }
            }
        });
    }
}

/// Creates multiple *send* functions to send from a given binary session type of a SessionMpst with more than 3 participants.
///
///  # Arguments
///  
///  * The name of the new *send* functions
///  * The name of the receivers
///  * The name of the related *next* functions
///  * The name of the senders
///  * The index of the binary session types that will receive in the SessionMpst for this specific role. Index starts at 1.
///  * The name of the *SessionMpst* type that will be used
///  * The number of participants (all together)
///  
///  # Example
///  
///  ```
///  use mpstthree::role::Role;
///  use mpstthree::{create_normal_role, create_sessionmpst, create_send_mpst_session, create_send_mpst_session_bundle};
///
///  create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
///  create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
///  create_normal_role!(RoleD, next_d, RoleDDual, next_d_dual);
///
///  create_sessionmpst!(SessionMpst, 3);
///
///  create_send_mpst_session_bundle!(
///     send_mpst_d_to_a,
///     RoleA,
///     next_a,
///     1, |
///     send_mpst_d_to_b,
///     RoleB,
///     next_b,
///     2, |
///     RoleD,
///     SessionMpst,
///     3
/// );
///  ```
#[macro_export]
macro_rules! create_send_mpst_session_bundle {
    ($($func_name:ident, $role:ident, $next:ident, $exclusion:literal, | )+ => $name:ident, $struct_name:ident, $nsessions:literal) => {
       $(create_send_mpst_session!($func_name, $role, $next, $name, $struct_name, $nsessions, $exclusion);)+
    }
}
