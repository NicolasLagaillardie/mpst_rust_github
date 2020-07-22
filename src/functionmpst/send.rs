use binary::{send, Send, Session};
use role::a_to_b::{next_a_to_b, RoleAtoB};
use role::a_to_c::{next_a_to_c, RoleAtoC};
use role::b_to_a::{next_b_to_a, RoleBtoA};
use role::b_to_c::{next_b_to_c, RoleBtoC};
use role::c_to_a::{next_c_to_a, RoleCtoA};
use role::c_to_b::{next_c_to_b, RoleCtoB};
use role::Role;
use sessionmpst::SessionMpst;
use std::marker;

/// Send a value of type `T` from A to B. Always succeeds. Returns the continuation of the
/// `SessionMpst<S1, S2, R>`.
pub fn send_mpst_a_to_b<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleAtoB<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = next_a_to_b(s.stack);

    SessionMpst {
        session1: new_session,
        session2: s.session2,
        stack: new_queue,
    }
}

/// Send a value of type `T` from B to A. Always succeeds. Returns the continuation of the
/// `SessionMpst<S1, S2, R>`.
pub fn send_mpst_b_to_a<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleBtoA<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = next_b_to_a(s.stack);

    SessionMpst {
        session1: new_session,
        session2: s.session2,
        stack: new_queue,
    }
}

/// Send a value of type `T` from C to A. Always succeeds. Returns the continuation of the
/// `SessionMpst<S1, S2, R>`.
pub fn send_mpst_c_to_a<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleCtoA<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = next_c_to_a(s.stack);

    SessionMpst {
        session1: new_session,
        session2: s.session2,
        stack: new_queue,
    }
}

/// Send a value of type `T` from A to C. Always succeeds. Returns the continuation of the
/// `SessionMpst<S1, S2, R>`.
pub fn send_mpst_a_to_c<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleAtoC<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = next_a_to_c(s.stack);

    SessionMpst {
        session1: s.session1,
        session2: new_session,
        stack: new_queue,
    }
}

/// Send a value of type `T` from B to C. Always succeeds. Returns the continuation of the
/// `SessionMpst<S1, S2, R>`.
pub fn send_mpst_b_to_c<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleBtoC<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = next_b_to_c(s.stack);

    SessionMpst {
        session1: s.session1,
        session2: new_session,
        stack: new_queue,
    }
}

/// Send a value of type `T` from C to B. Always succeeds. Returns the continuation of the
/// `SessionMpst<S1, S2, R>`.
pub fn send_mpst_c_to_b<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleCtoB<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = next_c_to_b(s.stack);

    SessionMpst {
        session1: s.session1,
        session2: new_session,
        stack: new_queue,
    }
}

// // create a function send_mpst for the first session
// #[macro_export]
// macro_rules! create_send_mpst_session_1 {
//     ($func_name:ident, $session:expr, $role:ty, $next:expr) => {
//         fn $func_name<T, S1, S2, R>(
//             x: T,
//             s: SessionMpst<Send<T, S1>, S2,  $role<R>>,
//         ) -> SessionMpst<S1, S2, R>
//         where
//             T: marker::Send,
//             S1: Session,
//             S2: Session,
//             R: Role,
//         {
//             let new_session = send(x, s.session1);
//             let new_queue = $next(s.stack);

//             SessionMpst {
//                 session1: new_session,
//                 session2: s.session2,
//                 stack: new_queue,
//             }
//         }
//     }
// }

// // create a function send_mpst for the first session
// #[macro_export]
// macro_rules! create_send_mpst_session_2 {
//     ($func_name:ident, $session:expr, $role:ty, $next:expr) => {
//         fn $func_name<T, S1, S2, R>(
//             x: T,
//             s: SessionMpst<S1,  Send<T, S2>, $role<R>>,
//         ) -> SessionMpst<S1, S2, R>
//         where
//             T: marker::Send,
//             S1: Session,
//             S2: Session,
//             R: Role,
//         {
//             let new_session = send(x, s.session2);
//             let new_queue = $next(s.stack);

//             SessionMpst {
//                 session1: s.session1,
//                 session2: new_session,
//                 stack: new_queue,
//             }
//         }
//     }
// }

// // create a function send_mpst for a session depending of the
// #[macro_export]
// macro_rules! create_send_mpst {
//     ($func_name:ident, $session:expr, $role:ty, $next:expr, $number:expr) => {
//         if $number
//             create_send_mpst!()
//     }
// }
