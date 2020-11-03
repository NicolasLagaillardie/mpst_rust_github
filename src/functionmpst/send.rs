use crate::binary::{send, Send, Session};
use crate::role::a::{next_a, RoleA};
use crate::role::b::{next_b, RoleB};
use crate::role::c::{next_c, RoleC};
use crate::role::end::RoleEnd;
use crate::role::Role;
use crate::sessionmpst::SessionMpst;
use std::marker;

/// Send a value of type `T` from A to B. Always succeeds. Returns the continuation
/// `SessionMpst<S1, S2, R, N>`.
///
/// # Example
///
/// ```
/// use mpstthree::binary::{End, Send, Session};
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::functionmpst::send::send_mpst_a_to_b;
///
/// // Creating the binary sessions
/// type AtoB<N> = Send<N, End>;
/// type AtoC = End;
///
/// // Queue
/// type QueueA = RoleB<RoleEnd>;
///
/// // Name
/// type NameA = RoleA<RoleEnd>;
///
/// // Creating the MP sessions
/// type EndpointA<N> = SessionMpst<AtoB<N>, AtoC, QueueA, NameA>;
///
/// // From this point...
/// let (channel_ab, _) = AtoB::<i32>::new();
/// let (channel_ac, _) = AtoC::new();
///
/// let (role_a, _) = QueueA::new();
///
/// let (name_a, _) = NameA::new();
///
/// let sess = SessionMpst {
///    session1: channel_ab,
///    session2: channel_ac,
///    stack: role_a,
///    name: name_a,
/// };
/// // ...to this point, should not be written in general. Please look at [`fork`]
///
/// let s = send_mpst_a_to_b(1, sess);
/// ```
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
/// `SessionMpst<S1, S2, R, N>`.
///
/// # Example
///
/// ```
/// use mpstthree::binary::{End, Send, Session};
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::functionmpst::send::send_mpst_b_to_a;
///
/// // Creating the binary sessions
/// type BtoA<N> = Send<N, End>;
/// type BtoC = End;
///
/// // Queue
/// type QueueB = RoleA<RoleEnd>;
///
/// // Name
/// type NameB = RoleB<RoleEnd>;
///
/// // Creating the MP sessions
/// type EndpointB<N> = SessionMpst<BtoA<N>, BtoC, QueueB, NameB>;
///
/// // From this point...
/// let (channel_ba, _) = BtoA::<i32>::new();
/// let (channel_bc, _) = BtoC::new();
///
/// let (role_b, _) = QueueB::new();
///
/// let (name_b, _) = NameB::new();
///
/// let sess = SessionMpst {
///    session1: channel_ba,
///    session2: channel_bc,
///    stack: role_b,
///    name: name_b,
/// };
/// // ...to this point, should not be written in general. Please look at [`fork`]
///
/// let s = send_mpst_b_to_a(1, sess);
/// ```
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
/// `SessionMpst<S1, S2, R, N>`.
///
/// # Example
///
/// ```
/// use mpstthree::binary::{End, Send, Session};
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::functionmpst::send::send_mpst_c_to_a;
///
/// // Creating the binary sessions
/// type CtoA<N> = Send<N, End>;
/// type CtoB = End;
///
/// // Queue
/// type QueueC = RoleA<RoleEnd>;
///
/// // Name
/// type NameC = RoleC<RoleEnd>;
///
/// // Creating the MP sessions
/// type EndpointC<N> = SessionMpst<CtoA<N>, CtoB, QueueC, NameC>;
///
/// // From this point...
/// let (channel_ca, _) = CtoA::<i32>::new();
/// let (channel_cb, _) = CtoB::new();
///
/// let (role_c, _) = QueueC::new();
///
/// let (name_c, _) = NameC::new();
///
/// let sess = SessionMpst {
///    session1: channel_ca,
///    session2: channel_cb,
///    stack: role_c,
///    name: name_c,
/// };
/// // ...to this point, should not be written in general. Please look at [`fork`]
///
/// let s = send_mpst_c_to_a(1, sess);
/// ```
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
/// `SessionMpst<S1, S2, R, N>`.
///
/// # Example
///
/// ```
/// use mpstthree::binary::{End, Send, Session};
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::functionmpst::send::send_mpst_a_to_c;
///
/// // Creating the binary sessions
/// type AtoB = End;
/// type AtoC<N> = Send<N, End>;
///
/// // Queue
/// type QueueA = RoleC<RoleEnd>;
///
/// // Name
/// type NameA = RoleA<RoleEnd>;
///
/// // Creating the MP sessions
/// type EndpointA<N> = SessionMpst<AtoB, AtoC<N>, QueueA, NameA>;
///
/// // From this point...
/// let (channel_ab, _) = AtoB::new();
/// let (channel_ac, _) = AtoC::<i32>::new();
///
/// let (role_a, _) = QueueA::new();
///
/// let (name_a, _) = NameA::new();
///
/// let sess = SessionMpst {
///    session1: channel_ab,
///    session2: channel_ac,
///    stack: role_a,
///    name: name_a,
/// };
/// // ...to this point, should not be written in general. Please look at [`fork`]
///
/// let s = send_mpst_a_to_c(1, sess);
/// ```
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
/// `SessionMpst<S1, S2, R, N>`.
///
/// # Example
///
/// ```
/// use mpstthree::binary::{End, Send, Session};
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::functionmpst::send::send_mpst_b_to_c;
///
/// // Creating the binary sessions
/// type BtoA = End;
/// type BtoC<N> = Send<N, End>;
///
/// // Queue
/// type QueueB = RoleC<RoleEnd>;
///
/// // Name
/// type NameB = RoleB<RoleEnd>;
///
/// // Creating the MP sessions
/// type EndpointB<N> = SessionMpst<BtoA, BtoC<N>, QueueB, NameB>;
///
/// // From this point...
/// let (channel_ba, _) = BtoA::new();
/// let (channel_bc, _) = BtoC::<i32>::new();
///
/// let (role_b, _) = QueueB::new();
///
/// let (name_b, _) = NameB::new();
///
/// let sess = SessionMpst {
///    session1: channel_ba,
///    session2: channel_bc,
///    stack: role_b,
///    name: name_b,
/// };
/// // ...to this point, should not be written in general. Please look at [`fork`]
///
/// let s = send_mpst_b_to_c(1, sess);
/// ```
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
/// `SessionMpst<S1, S2, R, N>`.
///
/// # Example
///
/// ```
/// use mpstthree::binary::{End, Send, Session};
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::functionmpst::send::send_mpst_c_to_b;
///
/// // Creating the binary sessions
/// type CtoA = End;
/// type CtoB<N> = Send<N, End>;
///
/// // Queue
/// type QueueC = RoleB<RoleEnd>;
///
/// // Name
/// type NameC = RoleC<RoleEnd>;
///
/// // Creating the MP sessions
/// type EndpointC<N> = SessionMpst<CtoA, CtoB<N>, QueueC, NameC>;
///
/// // From this point...
/// let (channel_ca, _) = CtoA::new();
/// let (channel_cb, _) = CtoB::<i32>::new();
///
/// let (role_c, _) = QueueC::new();
///
/// let (name_c, _) = NameC::new();
///
/// let sess = SessionMpst {
///    session1: channel_ca,
///    session2: channel_cb,
///    stack: role_c,
///    name: name_c,
/// };
/// // ...to this point, should not be written in general. Please look at [`fork`]
///
/// let s = send_mpst_c_to_b(1, sess);
/// ```
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
