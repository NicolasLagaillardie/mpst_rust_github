use crate::binary::{send, Send, Session};
use crate::role::a::{next_a, RoleA};
use crate::role::b::{next_b, RoleB};
use crate::role::c::{next_c, RoleC};
use crate::role::end::RoleEnd;
use crate::role::Role;
use crate::sessionmpst::SessionMpst;
use std::marker;

/// Send a value of type `T` from A to B. Always succeeds. Returns the continuation of the
/// `SessionMpst<S1, S2, R>`.
pub fn send_mpst_a_to_b<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleB<R>, RoleA<RoleEnd>>,
) -> SessionMpst<S1, S2, R, RoleA<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = next_b(s.stack);

    SessionMpst {
        session1: new_session,
        session2: s.session2,
        stack: new_queue,
        name: s.name,
    }
}

/// Send a value of type `T` from B to A. Always succeeds. Returns the continuation of the
/// `SessionMpst<S1, S2, R>`.
pub fn send_mpst_b_to_a<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleA<R>, RoleB<RoleEnd>>,
) -> SessionMpst<S1, S2, R, RoleB<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = next_a(s.stack);

    SessionMpst {
        session1: new_session,
        session2: s.session2,
        stack: new_queue,
        name: s.name,
    }
}

/// Send a value of type `T` from C to A. Always succeeds. Returns the continuation of the
/// `SessionMpst<S1, S2, R>`.
pub fn send_mpst_c_to_a<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleA<R>, RoleC<RoleEnd>>,
) -> SessionMpst<S1, S2, R, RoleC<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = next_a(s.stack);

    SessionMpst {
        session1: new_session,
        session2: s.session2,
        stack: new_queue,
        name: s.name,
    }
}

/// Send a value of type `T` from A to C. Always succeeds. Returns the continuation of the
/// `SessionMpst<S1, S2, R>`.
pub fn send_mpst_a_to_c<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleC<R>, RoleA<RoleEnd>>,
) -> SessionMpst<S1, S2, R, RoleA<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = next_c(s.stack);

    SessionMpst {
        session1: s.session1,
        session2: new_session,
        stack: new_queue,
        name: s.name,
    }
}

/// Send a value of type `T` from B to C. Always succeeds. Returns the continuation of the
/// `SessionMpst<S1, S2, R>`.
pub fn send_mpst_b_to_c<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleC<R>, RoleB<RoleEnd>>,
) -> SessionMpst<S1, S2, R, RoleB<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = next_c(s.stack);

    SessionMpst {
        session1: s.session1,
        session2: new_session,
        stack: new_queue,
        name: s.name,
    }
}

/// Send a value of type `T` from C to B. Always succeeds. Returns the continuation of the
/// `SessionMpst<S1, S2, R>`.
pub fn send_mpst_c_to_b<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleB<R>, RoleC<RoleEnd>>,
) -> SessionMpst<S1, S2, R, RoleC<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = next_b(s.stack);

    SessionMpst {
        session1: s.session1,
        session2: new_session,
        stack: new_queue,
        name: s.name,
    }
}
