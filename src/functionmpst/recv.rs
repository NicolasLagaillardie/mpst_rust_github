use crate::binary::{recv, Recv, Session};
use crate::role::a::{next_a, RoleA};
use crate::role::all_to_a::{next_all_to_a, RoleAlltoA};
use crate::role::all_to_b::{next_all_to_b, RoleAlltoB};
use crate::role::all_to_c::{next_all_to_c, RoleAlltoC};
use crate::role::b::{next_b, RoleB};
use crate::role::c::{next_c, RoleC};
use crate::role::end::RoleEnd;
use crate::role::Role;
use crate::sessionmpst::SessionMpst;
use std::error::Error;
use std::marker;

type ResultBoxError<T, S1, S2, R, N> = Result<(T, SessionMpst<S1, S2, R, N>), Box<dyn Error>>;

/// Receive a value of type `T` on A from B. Can fail. Returns either a pair of the received
/// value and the continuation of the `SessionMpst<S1, S2, R>` or an error.
pub fn recv_mpst_a_to_b<T, S1, S2, R>(
    s: SessionMpst<Recv<T, S1>, S2, RoleB<R>, RoleA<RoleEnd>>,
) -> ResultBoxError<T, S1, S2, R, RoleA<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session1)?;
    let new_queue = next_b(s.stack);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        stack: new_queue,
        name: s.name,
    };

    Ok((v, result))
}

/// Receive a value of type `T` on B from A. Can fail. Returns either a pair of the received
/// value and the continuation of the `SessionMpst<S1, S2, R>` or an error.
pub fn recv_mpst_b_to_a<T, S1, S2, R>(
    s: SessionMpst<Recv<T, S1>, S2, RoleA<R>, RoleB<RoleEnd>>,
) -> ResultBoxError<T, S1, S2, R, RoleB<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session1)?;
    let new_queue = next_a(s.stack);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        stack: new_queue,
        name: s.name,
    };

    Ok((v, result))
}

/// Receive a value of type `T` on C from A. Can fail. Returns either a pair of the received
/// value and the continuation of the `SessionMpst<S1, S2, R>` or an error.
pub fn recv_mpst_c_to_a<T, S1, S2, R>(
    s: SessionMpst<Recv<T, S1>, S2, RoleA<R>, RoleC<RoleEnd>>,
) -> ResultBoxError<T, S1, S2, R, RoleC<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session1)?;
    let new_queue = next_a(s.stack);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        stack: new_queue,
        name: s.name,
    };

    Ok((v, result))
}

/// Receive a value of type `T` on A from C. Can fail. Returns either a pair of the received
/// value and the continuation of the `SessionMpst<S1, S2, R>` or an error.
pub fn recv_mpst_a_to_c<T, S1, S2, R>(
    s: SessionMpst<S1, Recv<T, S2>, RoleC<R>, RoleA<RoleEnd>>,
) -> ResultBoxError<T, S1, S2, R, RoleA<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session2)?;
    let new_queue = next_c(s.stack);
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        stack: new_queue,
        name: s.name,
    };

    Ok((v, result))
}

/// Receive a value of type `T` on B from C. Can fail. Returns either a pair of the received
/// value and the continuation of the `SessionMpst<S1, S2, R>` or an error.
pub fn recv_mpst_b_to_c<T, S1, S2, R>(
    s: SessionMpst<S1, Recv<T, S2>, RoleC<R>, RoleB<RoleEnd>>,
) -> ResultBoxError<T, S1, S2, R, RoleB<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session2)?;
    let new_queue = next_c(s.stack);
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        stack: new_queue,
        name: s.name,
    };

    Ok((v, result))
}

/// Receive a value of type `T` on C from B. Can fail. Returns either a pair of the received
/// value and the continuation of the `SessionMpst<S1, S2, R>` or an error.
pub fn recv_mpst_c_to_b<T, S1, S2, R>(
    s: SessionMpst<S1, Recv<T, S2>, RoleB<R>, RoleC<RoleEnd>>,
) -> ResultBoxError<T, S1, S2, R, RoleC<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session2)?;
    let new_queue = next_b(s.stack);
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        stack: new_queue,
        name: s.name,
    };

    Ok((v, result))
}

/// Receive a broadcasted value of type `T` on B from A. Can fail. Returns either a pair of the received
/// value and the continuation of the `SessionMpst<S1, S2, R>` or an error.
pub fn recv_mpst_all_to_b<T, S1, S2, R, N>(
    s: SessionMpst<Recv<T, S1>, S2, RoleAlltoB<R, R>, N>,
) -> ResultBoxError<T, S1, S2, R, N>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
    N: Role,
{
    let (v, new_session) = recv(s.session1)?;
    let (new_queue, _) = next_all_to_b(s.stack);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        stack: new_queue,
        name: s.name,
    };

    Ok((v, result))
}

/// Receive a broadcasted value of type `T` on A from B. Can fail. Returns either a pair of the received
/// value and the continuation of the `SessionMpst<S1, S2, R>` or an error.
pub fn recv_mpst_all_to_a<T, S1, S2, R, N>(
    s: SessionMpst<Recv<T, S1>, S2, RoleAlltoA<R, R>, N>,
) -> ResultBoxError<T, S1, S2, R, N>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
    N: Role,
{
    let (v, new_session) = recv(s.session1)?;
    let (new_queue, _) = next_all_to_a(s.stack);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        stack: new_queue,
        name: s.name,
    };

    Ok((v, result))
}

/// Receive a broadcasted value of type `T` on C from A. Can fail. Returns either a pair of the received
/// value and the continuation of the `SessionMpst<S1, S2, R>` or an error.
pub fn recv_mpst_all_to_c<T, S1, S2, R, N>(
    s: SessionMpst<S1, Recv<T, S2>, RoleAlltoC<R, R>, N>,
) -> ResultBoxError<T, S1, S2, R, N>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
    N: Role,
{
    let (v, new_session) = recv(s.session2)?;
    let (new_queue, _) = next_all_to_c(s.stack);
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        stack: new_queue,
        name: s.name,
    };

    Ok((v, result))
}
