//! This module contains all the *send* functions

use crate::binary::struct_trait::{Send, Session};
use crate::role::a::RoleA;
use crate::role::b::RoleB;
use crate::role::c::RoleC;
use crate::role::end::RoleEnd;
use crate::role::Role;
use crate::sessionmpst::SessionMpst;
use std::marker;

#[doc(hidden)]
macro_rules! send_aux {
    ($session:expr, $payload:expr, $role:ident, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..3 ! $exclusion {{ // exclusion: index of binary channel among the 2 others
            %(
            )(
                let new_session = crate::binary::send::send($payload, $session.session#N:0);
            )0*

            let new_queue = {
                fn temp<R>(r: $role<R>) -> R
                where
                    R: crate::role::Role,
                {
                    let (here, there) = <R as crate::role::Role>::new();
                    r.sender.send(there).unwrap_or(());
                    here
                }
                temp($session.stack)
            };

            crate::sessionmpst::SessionMpst {
                %(
                    session#N:0: $session.session#N:0,
                )(
                    session#N:0: new_session,
                )0*
                stack: new_queue,
                name: $session.name,
            }
        }});
    }
}

/// Send a value of type `T` from A to B. Always succeeds.
/// Returns the continuation `SessionMpst<S1, S2, R, N>`.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{End, Send, Session};
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
///
/// let (channel_ab, _) = AtoB::<i32>::new();
/// let (channel_ac, _) = AtoC::new();
///
/// let (role_a, _) = QueueA::new();
///
/// let (name_a, _) = NameA::new();
///
/// let sess = SessionMpst {
///   session1: channel_ab,
///   session2: channel_ac,
///   stack: role_a,
///   name: name_a,
/// };
///
/// // ...to this point, should not be written in general. Please look at [`mpstthree::fork`](../fork/index.html).
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
    send_aux!(s, x, RoleB, 1)
}

/// Send a value of type `T` from B to A. Always succeeds.
/// Returns the continuation of the `SessionMpst<S1, S2, R,
/// N>`.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{End, Send, Session};
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
///
/// let (channel_ba, _) = BtoA::<i32>::new();
/// let (channel_bc, _) = BtoC::new();
///
/// let (role_b, _) = QueueB::new();
///
/// let (name_b, _) = NameB::new();
///
/// let sess = SessionMpst {
///   session1: channel_ba,
///   session2: channel_bc,
///   stack: role_b,
///   name: name_b,
/// };
///
/// // ...to this point, should not be written in general. Please look at [`mpstthree::fork`](../fork/index.html).
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
    send_aux!(s, x, RoleA, 1)
}

/// Send a value of type `T` from C to A. Always succeeds.
/// Returns the continuation of the `SessionMpst<S1, S2, R,
/// N>`.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{End, Send, Session};
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
///
/// let (channel_ca, _) = CtoA::<i32>::new();
/// let (channel_cb, _) = CtoB::new();
///
/// let (role_c, _) = QueueC::new();
///
/// let (name_c, _) = NameC::new();
///
/// let sess = SessionMpst {
///   session1: channel_ca,
///   session2: channel_cb,
///   stack: role_c,
///   name: name_c,
/// };
///
/// // ...to this point, should not be written in general. Please look at [`mpstthree::fork`](../fork/index.html).
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
    send_aux!(s, x, RoleA, 1)
}

/// Send a value of type `T` from A to C. Always succeeds.
/// Returns the continuation of the `SessionMpst<S1, S2, R,
/// N>`.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{End, Send, Session};
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
///
/// let (channel_ab, _) = AtoB::new();
/// let (channel_ac, _) = AtoC::<i32>::new();
///
/// let (role_a, _) = QueueA::new();
///
/// let (name_a, _) = NameA::new();
///
/// let sess = SessionMpst {
///   session1: channel_ab,
///   session2: channel_ac,
///   stack: role_a,
///   name: name_a,
/// };
///
/// // ...to this point, should not be written in general. Please look at [`mpstthree::fork`](../fork/index.html).
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
    send_aux!(s, x, RoleC, 2)
}

/// Send a value of type `T` from B to C. Always succeeds.
/// Returns the continuation of the `SessionMpst<S1, S2, R,
/// N>`.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{End, Send, Session};
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
///
/// let (channel_ba, _) = BtoA::new();
/// let (channel_bc, _) = BtoC::<i32>::new();
///
/// let (role_b, _) = QueueB::new();
///
/// let (name_b, _) = NameB::new();
///
/// let sess = SessionMpst {
///   session1: channel_ba,
///   session2: channel_bc,
///   stack: role_b,
///   name: name_b,
/// };
///
/// // ...to this point, should not be written in general. Please look at [`mpstthree::fork`](../fork/index.html).
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
    send_aux!(s, x, RoleC, 2)
}

/// Send a value of type `T` from C to B. Always succeeds.
/// Returns the continuation of the `SessionMpst<S1, S2, R,
/// N>`.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{End, Send, Session};
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
///
/// let (channel_ca, _) = CtoA::new();
/// let (channel_cb, _) = CtoB::<i32>::new();
///
/// let (role_c, _) = QueueC::new();
///
/// let (name_c, _) = NameC::new();
///
/// let sess = SessionMpst {
///   session1: channel_ca,
///   session2: channel_cb,
///   stack: role_c,
///   name: name_c,
/// };
///
/// // ...to this point, should not be written in general. Please look at [`mpstthree::fork`](../fork/index.html).
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
    send_aux!(s, x, RoleB, 2)
}
