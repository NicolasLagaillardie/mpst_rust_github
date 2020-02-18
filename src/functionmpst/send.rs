use binary::{send, Send, Session};

use role::a_sends_to_b::{next_a_sends_to_b, RoleASendToB};
use role::b_sends_to_a::{next_b_sends_to_a, RoleBSendToA};
use role::c_sends_to_b::{next_c_sends_to_b, RoleCSendToB};
use role::b_sends_to_c::{next_b_sends_to_c, RoleBSendToC};
use role::a_sends_to_c::{next_a_sends_to_c, RoleASendToC};
use role::c_sends_to_a::{next_c_sends_to_a, RoleCSendToA};

use role::Role;
use sessionmpst::SessionMpst;
use std::marker;

pub fn send_mpst_session_one_a_to_b<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleASendToB<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = next_a_sends_to_b(s.queue);

    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    result
}

pub fn send_mpst_session_one_b_to_a<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleBSendToA<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = next_b_sends_to_a(s.queue);

    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    result
}

pub fn send_mpst_session_one_a_to_c<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleASendToC<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = next_a_sends_to_c(s.queue);

    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    result
}

pub fn send_mpst_session_one_c_to_a<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleCSendToA<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = next_c_sends_to_a(s.queue);

    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    result
}

pub fn send_mpst_session_one_c_to_b<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleCSendToB<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = next_c_sends_to_b(s.queue);

    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    result
}

pub fn send_mpst_session_one_b_to_c<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleBSendToC<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = next_b_sends_to_c(s.queue);

    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    result
}

/// Sending on session 2
pub fn send_mpst_session_two_a_to_b<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleASendToB<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = next_a_sends_to_b(s.queue);

    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    result
}

pub fn send_mpst_session_two_b_to_a<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleBSendToA<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = next_b_sends_to_a(s.queue);

    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    result
}

pub fn send_mpst_session_two_a_to_c<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleASendToC<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = next_a_sends_to_c(s.queue);

    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    result
}

pub fn send_mpst_session_two_c_to_a<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleCSendToA<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = next_c_sends_to_a(s.queue);

    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    result
}

pub fn send_mpst_session_two_b_to_c<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleBSendToC<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = next_b_sends_to_c(s.queue);

    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    result
}

pub fn send_mpst_session_two_c_to_b<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleCSendToB<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = next_c_sends_to_b(s.queue);
    
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    result
}
