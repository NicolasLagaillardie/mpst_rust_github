//! This module contains all the *receive* functions

use crate::binary::recv::recv;
use crate::binary::struct_trait::{Recv, Session};
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

/// Receive a value of type `T` on A from B. Can fail.
/// Returns either a pair of the received value and the
/// continuation of the `SessionMpst<S1, S2, R, N>`
/// or an error.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{End, Recv, Session};
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::functionmpst::recv::recv_mpst_a_to_b;
/// use mpstthree::functionmpst::send::send_mpst_b_to_a;
///
/// // Creating the binary sessions
/// type AtoB = Recv<(), End>;
/// type BtoA = <AtoB as Session>::Dual;
///
/// // Queue
/// type QueueA = RoleB<RoleEnd>;
/// type QueueB = RoleA<RoleEnd>;
///
/// // Name
/// type NameA = RoleA<RoleEnd>;
/// type NameB = RoleB<RoleEnd>;
///
/// // Creating the MP sessions
/// type EndpointA = SessionMpst<AtoB, End, QueueA, NameA>;
/// type EndpointB = SessionMpst<BtoA, End, QueueB, NameB>;
///
/// // From this point...
///
/// let (channel_ab, channel_ba) = AtoB::new();
/// let (channel_ac, _) = End::new();
/// let (channel_bc, _) = End::new();
///
/// let (role_a, _) = QueueA::new();
/// let (role_b, _) = QueueB::new();
///
/// let (name_a, _) = NameA::new();
/// let (name_b, _) = NameB::new();
///
/// let sess_a = SessionMpst {
///    session1: channel_ab,
///    session2: channel_ac,
///    stack: role_a,
///    name: name_a,
/// };
///
/// let sess_b = SessionMpst {
///    session1: channel_ba,
///    session2: channel_bc,
///    stack: role_b,
///    name: name_b,
/// };
/// // ...to this point, should not be written in general. Please look at [`mpstthree::fork`](../fork/index.html).
///
/// send_mpst_b_to_a((), sess_b);
/// recv_mpst_a_to_b(sess_a);
/// ```
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

/// Receive a value of type `T` on B from A. Can fail.
/// Returns either a pair of the received value and the
/// continuation of the `SessionMpst<S1, S2, R, N>`
/// or an error.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{End, Recv, Session};
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::functionmpst::recv::recv_mpst_b_to_a;
/// use mpstthree::functionmpst::send::send_mpst_a_to_b;
///
/// // Creating the binary sessions
/// type BtoA = Recv<(), End>;
/// type AtoB = <BtoA as Session>::Dual;
///
/// // Queue
/// type QueueB = RoleA<RoleEnd>;
/// type QueueA = RoleB<RoleEnd>;
///
/// // Name
/// type NameB = RoleB<RoleEnd>;
/// type NameA = RoleA<RoleEnd>;
///
/// // Creating the MP sessions
/// type EndpointB = SessionMpst<BtoA, End, QueueB, NameB>;
/// type EndpointA = SessionMpst<AtoB, End, QueueA, NameA>;
///
/// // From this point...
/// let (channel_ba, channel_ab) = BtoA::new();
/// let (channel_ac, _) = BtoA::new();
/// let (channel_bc, _) = End::new();
///
/// let (role_b, _) = QueueB::new();
/// let (role_a, _) = QueueA::new();
///
/// let (name_b, _) = NameB::new();
/// let (name_a, _) = NameA::new();
///
/// let sess_b = SessionMpst {
///    session1: channel_ba,
///    session2: channel_bc,
///    stack: role_b,
///    name: name_b,
/// };
///
/// let sess_a = SessionMpst {
///    session1: channel_ab,
///    session2: channel_ac,
///    stack: role_a,
///    name: name_a,
/// };
/// // ...to this point, should not be written in general. Please look at [`mpstthree::fork`](../fork/index.html).
///
/// send_mpst_a_to_b((), sess_a);
/// recv_mpst_b_to_a(sess_b);
/// ```
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

/// Receive a value of type `T` on C from A. Can fail.
/// Returns either a pair of the received value and the
/// continuation of the `SessionMpst<S1, S2, R, N>`
/// or an error.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{End, Recv, Session};
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::functionmpst::recv::recv_mpst_c_to_a;
/// use mpstthree::functionmpst::send::send_mpst_a_to_c;
///
/// // Creating the binary sessions
/// type CtoA = Recv<(), End>;
/// type AtoC = <CtoA as Session>::Dual;
///
/// // Queue
/// type QueueC = RoleA<RoleEnd>;
/// type QueueA = RoleC<RoleEnd>;
///
/// // Name
/// type NameC = RoleC<RoleEnd>;
/// type NameA = RoleA<RoleEnd>;
///
/// // Creating the MP sessions
/// type EndpointC = SessionMpst<CtoA, End, QueueC, NameC>;
/// type EndpointA = SessionMpst<End, AtoC, QueueA, NameA>;
///
/// // From this point...
/// let (channel_ca, channel_ac) = CtoA::new();
/// let (channel_cb, _) = End::new();
/// let (channel_ab, _) = End::new();
///
/// let (role_c, _) = QueueC::new();
/// let (role_a, _) = QueueA::new();
///
/// let (name_c, _) = NameC::new();
/// let (name_a, _) = NameA::new();
///
/// let sess_c = SessionMpst {
///    session1: channel_ca,
///    session2: channel_cb,
///    stack: role_c,
///    name: name_c,
/// };
///
/// let sess_a = SessionMpst {
///    session1: channel_ab,
///    session2: channel_ac,
///    stack: role_a,
///    name: name_a,
/// };
/// // ...to this point, should not be written in general. Please look at [`mpstthree::fork`](../fork/index.html).
///
/// send_mpst_a_to_c((), sess_a);
/// recv_mpst_c_to_a(sess_c);
/// ```
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

/// Receive a value of type `T` on A from C. Can fail.
/// Returns either a pair of the received value and the
/// continuation of the `SessionMpst<S1, S2, R, N>`
/// or an error.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{End, Recv, Session};
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::functionmpst::recv::recv_mpst_a_to_c;
/// use mpstthree::functionmpst::send::send_mpst_c_to_a;
///
/// // Creating the binary sessions
/// type AtoC = Recv<(), End>;
/// type CtoA = <AtoC as Session>::Dual;
///
/// // Queue
/// type QueueA = RoleC<RoleEnd>;
/// type QueueC = RoleA<RoleEnd>;
///
/// // Name
/// type NameA = RoleA<RoleEnd>;
/// type NameC = RoleC<RoleEnd>;
///
/// // Creating the MP sessions
/// type EndpointA = SessionMpst<End, AtoC, QueueA, NameA>;
/// type EndpointC = SessionMpst<CtoA, End, QueueC, NameC>;
///
/// // From this point...
/// let (channel_ab, _) = End::new();
/// let (channel_cb, _) = End::new();
/// let (channel_ac, channel_ca) = AtoC::new();
///
/// let (role_a, _) = QueueA::new();
/// let (role_c, _) = QueueC::new();
///
/// let (name_a, _) = NameA::new();
/// let (name_c, _) = NameC::new();
///
/// let sess_a = SessionMpst {
///    session1: channel_ab,
///    session2: channel_ac,
///    stack: role_a,
///    name: name_a,
/// };
///
/// let sess_c = SessionMpst {
///    session1: channel_ca,
///    session2: channel_cb,
///    stack: role_c,
///    name: name_c,
/// };
/// // ...to this point, should not be written in general. Please look at [`mpstthree::fork`](../fork/index.html).
///
/// send_mpst_c_to_a((), sess_c);
/// recv_mpst_a_to_c(sess_a);
/// ```
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

/// Receive a value of type `T` on B from C. Can fail.
/// Returns either a pair of the received value and the
/// continuation of the `SessionMpst<S1, S2, R, N>`
/// or an error.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{End, Recv, Session};
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::functionmpst::recv::recv_mpst_b_to_c;
/// use mpstthree::functionmpst::send::send_mpst_c_to_b;
///
/// // Creating the binary sessions
/// type BtoC = Recv<(), End>;
/// type CtoB = <BtoC as Session>::Dual;
///
/// // Queue
/// type QueueB = RoleC<RoleEnd>;
/// type QueueC = RoleB<RoleEnd>;
///
/// // Name
/// type NameB = RoleB<RoleEnd>;
/// type NameC = RoleC<RoleEnd>;
///
/// // Creating the MP sessions
/// type EndpointB = SessionMpst<End, BtoC, QueueB, NameB>;
/// type EndpointC = SessionMpst<End, CtoB, QueueC, NameC>;
///
/// // From this point...
/// let (channel_ba, _) = End::new();
/// let (channel_ca, _) = End::new();
/// let (channel_bc, channel_cb) = BtoC::new();
///
/// let (role_b, _) = QueueB::new();
/// let (role_c, _) = QueueC::new();
///
/// let (name_b, _) = NameB::new();
/// let (name_c, _) = NameC::new();
///
/// let sess_b = SessionMpst {
///    session1: channel_ba,
///    session2: channel_bc,
///    stack: role_b,
///    name: name_b,
/// };
///
/// let sess_c = SessionMpst {
///    session1: channel_ca,
///    session2: channel_cb,
///    stack: role_c,
///    name: name_c,
/// };
/// // ...to this point, should not be written in general. Please look at [`mpstthree::fork`](../fork/index.html).
///
/// send_mpst_c_to_b((), sess_c);
/// recv_mpst_b_to_c(sess_b);
/// ```
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

/// Receive a value of type `T` on C from B. Can fail.
/// Returns either a pair of the received value and the
/// continuation of the `SessionMpst<S1, S2, R, N>`
/// or an error.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{End, Recv, Session};
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::functionmpst::recv::recv_mpst_c_to_b;
/// use mpstthree::functionmpst::send::send_mpst_b_to_c;
///
/// // Creating the binary sessions
/// type CtoB = Recv<(), End>;
/// type BtoC = <CtoB as Session>::Dual;
///
/// // Queue
/// type QueueC = RoleB<RoleEnd>;
/// type QueueB = RoleC<RoleEnd>;
///
/// // Name
/// type NameC = RoleC<RoleEnd>;
/// type NameB = RoleB<RoleEnd>;
///
/// // Creating the MP sessions
/// type EndpointC = SessionMpst<End, CtoB, QueueC, NameC>;
/// type EndpointB = SessionMpst<End, BtoC, QueueB, NameB>;
///
/// // From this point...
/// let (channel_ba, _) = End::new();
/// let (channel_ca, _) = End::new();
/// let (channel_cb, channel_bc) = CtoB::new();
///
/// let (role_c, _) = QueueC::new();
/// let (role_b, _) = QueueB::new();
///
/// let (name_c, _) = NameC::new();
/// let (name_b, _) = NameB::new();
///
/// let sess_c = SessionMpst {
///    session1: channel_ca,
///    session2: channel_cb,
///    stack: role_c,
///    name: name_c,
/// };
///
/// let sess_b = SessionMpst {
///    session1: channel_ba,
///    session2: channel_bc,
///    stack: role_b,
///    name: name_b,
/// };
/// // ...to this point, should not be written in general. Please look at [`mpstthree::fork`](../fork/index.html).
///
/// send_mpst_b_to_c((), sess_b);
/// recv_mpst_c_to_b(sess_c);
/// ```
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

/// Receive a broadcasted value of type `T` on B from A. Can
/// fail. Returns either a pair of the received value and
/// the continuation of the `SessionMpst<S1, S2, R, N>` or
/// an error. Should not be used as a standalone, but rather
/// with [`mpstthree::offer::
/// offer_mpst_session_to_a_from_b`].
#[doc(hidden)]
pub fn recv_mpst_a_all_to_b<T, S1, S2, R, N>(
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

/// Receive a broadcasted value of type `T` on C from A. Can
/// fail. Returns either a pair of the received value and
/// the continuation of the `SessionMpst<S1, S2, R, N>` or
/// an error. Should not be used as a standalone, but rather
/// with [`mpstthree::offer::
/// offer_mpst_session_to_a_from_c`].
#[doc(hidden)]
pub fn recv_mpst_a_all_to_c<T, S1, S2, R, N>(
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

/// Receive a broadcasted value of type `T` on A from B. Can
/// fail. Returns either a pair of the received value and
/// the continuation of the `SessionMpst<S1, S2, R, N>` or
/// an error. Should not be used as a standalone, but rather
/// with [`mpstthree::offer::
/// offer_mpst_session_to_b_from_a`].
#[doc(hidden)]
pub fn recv_mpst_b_all_to_a<T, S1, S2, R, N>(
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

/// Receive a broadcasted value of type `T` on C from A. Can
/// fail. Returns either a pair of the received value and
/// the continuation of the `SessionMpst<S1, S2, R, N>` or
/// an error. Should not be used as a standalone, but rather
/// with [`mpstthree::offer::
/// offer_mpst_session_to_b_from_c`].
#[doc(hidden)]
pub fn recv_mpst_b_all_to_c<T, S1, S2, R, N>(
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

/// Receive a broadcasted value of type `T` on A from B. Can
/// fail. Returns either a pair of the received value and
/// the continuation of the `SessionMpst<S1, S2, R, N>` or
/// an error. Should not be used as a standalone, but rather
/// with [`mpstthree::offer::
/// offer_mpst_session_to_c_from_a`].
#[doc(hidden)]
pub fn recv_mpst_c_all_to_a<T, S1, S2, R, N>(
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

/// Receive a broadcasted value of type `T` on B from C. Can
/// fail. Returns either a pair of the received value and
/// the continuation of the `SessionMpst<S1, S2, R, N>` or
/// an error. Should not be used as a standalone, but rather
/// with [`mpstthree::offer::
/// offer_mpst_session_to_c_from_b`].
#[doc(hidden)]
pub fn recv_mpst_c_all_to_b<T, S1, S2, R, N>(
    s: SessionMpst<S1, Recv<T, S2>, RoleAlltoB<R, R>, N>,
) -> ResultBoxError<T, S1, S2, R, N>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
    N: Role,
{
    let (v, new_session) = recv(s.session2)?;
    let (new_queue, _) = next_all_to_b(s.stack);
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        stack: new_queue,
        name: s.name,
    };

    Ok((v, result))
}
