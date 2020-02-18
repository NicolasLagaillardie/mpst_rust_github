use binary::{recv, Recv, Session};

use role::a_receives_from_b::{next_a_receive_from_b, RoleAReceiveFromB};
use role::b_receives_from_a::{next_b_receive_from_a, RoleBReceiveFromA};
use role::c_receives_from_b::{next_c_receive_from_b, RoleCReceiveFromB};
use role::b_receives_from_c::{next_b_receive_from_c, RoleBReceiveFromC};
use role::a_receives_from_c::{next_a_receive_from_c, RoleAReceiveFromC};
use role::c_receives_from_a::{next_c_receive_from_a, RoleCReceiveFromA};

use role::Role;
use sessionmpst::SessionMpst;
use std::error::Error;
use std::marker;

pub fn recv_mpst_session_one_a_to_b<T, S1, S2, R>(
    s: SessionMpst<Recv<T, S1>, S2, RoleAReceiveFromB<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session1)?;
    let new_queue = next_a_receive_from_b(s.queue)?;

    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    Ok((v, result))
}

pub fn recv_mpst_session_one_b_to_a<T, S1, S2, R>(
    s: SessionMpst<Recv<T, S1>, S2, RoleBReceiveFromA<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session1)?;
    let new_queue = next_b_receive_from_a(s.queue)?;

    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    Ok((v, result))
}

pub fn recv_mpst_session_one_a_to_c<T, S1, S2, R>(
    s: SessionMpst<Recv<T, S1>, S2, RoleAReceiveFromC<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session1)?;
    let new_queue = next_a_receive_from_c(s.queue)?;

    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    Ok((v, result))
}

pub fn recv_mpst_session_one_c_to_a<T, S1, S2, R>(
    s: SessionMpst<Recv<T, S1>, S2, RoleCReceiveFromA<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session1)?;
    let new_queue = next_c_receive_from_a(s.queue)?;

    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    Ok((v, result))
}

pub fn recv_mpst_session_one_b_to_c<T, S1, S2, R>(
    s: SessionMpst<Recv<T, S1>, S2, RoleBReceiveFromC<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session1)?;
    let new_queue = next_b_receive_from_c(s.queue)?;

    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    Ok((v, result))
}

pub fn recv_mpst_session_one_c_to_b<T, S1, S2, R>(
    s: SessionMpst<Recv<T, S1>, S2, RoleCReceiveFromB<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session1)?;
    let new_queue = next_c_receive_from_b(s.queue)?;

    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    Ok((v, result))
}

/// Recving on session 2
pub fn recv_mpst_session_two_a_to_b<T, S1, S2, R>(
    s: SessionMpst<S1, Recv<T, S2>, RoleAReceiveFromB<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session2)?;
    let new_queue = next_a_receive_from_b(s.queue)?;

    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    Ok((v, result))
}

pub fn recv_mpst_session_two_b_to_a<T, S1, S2, R>(
    s: SessionMpst<S1, Recv<T, S2>, RoleBReceiveFromA<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session2)?;
    let new_queue = next_b_receive_from_a(s.queue)?;

    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    Ok((v, result))
}

pub fn recv_mpst_session_two_a_to_c<T, S1, S2, R>(
    s: SessionMpst<S1, Recv<T, S2>, RoleAReceiveFromC<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session2)?;
    let new_queue = next_a_receive_from_c(s.queue)?;

    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    Ok((v, result))
}

pub fn recv_mpst_session_two_c_to_a<T, S1, S2, R>(
    s: SessionMpst<S1, Recv<T, S2>, RoleCReceiveFromA<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session2)?;
    let new_queue = next_c_receive_from_a(s.queue)?;

    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    Ok((v, result))
}

pub fn recv_mpst_session_two_b_to_c<T, S1, S2, R>(
    s: SessionMpst<S1, Recv<T, S2>, RoleBReceiveFromC<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session2)?;
    let new_queue = next_b_receive_from_c(s.queue)?;

    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    Ok((v, result))
}

pub fn recv_mpst_session_two_c_to_b<T, S1, S2, R>(
    s: SessionMpst<S1, Recv<T, S2>, RoleCReceiveFromB<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session2)?;
    let new_queue = next_c_receive_from_b(s.queue)?;
    
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    Ok((v, result))
}
