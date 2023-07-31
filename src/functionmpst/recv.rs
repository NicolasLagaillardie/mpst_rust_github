//! This module contains all the *receive* functions

use crate::binary::struct_trait::{recv::Recv, session::Session};
use crate::meshedchannels::MeshedChannels;
use crate::name::a::NameA;
use crate::name::b::NameB;
use crate::name::c::NameC;
use crate::role::a::RoleA;
use crate::role::all_to_a::RoleAlltoA;
use crate::role::all_to_b::RoleAlltoB;
use crate::role::all_to_c::RoleAlltoC;
use crate::role::b::RoleB;
use crate::role::c::RoleC;
use crate::role::end::RoleEnd;
use crate::role::Role;
use std::error::Error;
use std::marker;

type ResultBoxError<T, S1, S2, R, N> = Result<(T, MeshedChannels<S1, S2, R, N>), Box<dyn Error>>;

#[doc(hidden)]
#[macro_export]
macro_rules! recv_aux_simple {
    ($session:expr, $exclusion: literal) => {
        mpst_seq_proc::recv_aux_simple!($session, $exclusion)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! recv_all_aux_simple {
    ($session:expr, $exclusion: literal) => {
        mpst_seq_proc::recv_all_aux_simple!($session, $exclusion)
    };
}

/// Receive a value of type `T` on A from B. Can fail.
/// Returns either a pair of the received value and the
/// continuation of the `MeshedChannels<S1, S2, R, N>`
/// or an error.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{end::End, recv::Recv, session::Session};
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::name::Name;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::name::a::NameA;
/// use mpstthree::name::b::NameB;
///
/// use mpstthree::functionmpst::recv::recv_mpst_a_from_b;
/// use mpstthree::functionmpst::send::send_mpst_b_to_a;
///
/// // Creating the binary sessions
/// type AtoB = Recv<(), End>;
///
/// // Stack
/// type StackA = RoleB<RoleEnd>;
/// type StackB = RoleA<RoleEnd>;
///
/// // From this point...
///
/// let (channel_ab, channel_ba) = AtoB::new();
/// let (channel_ac, _) = End::new();
/// let (channel_bc, _) = End::new();
///
/// let (role_a, _) = StackA::new();
/// let (role_b, _) = StackB::new();
///
/// let (name_a, _) = NameA::new();
/// let (name_b, _) = NameB::new();
///
/// let sess_a = MeshedChannels {
///     session1: channel_ab,
///     session2: channel_ac,
///     stack: role_a,
///     name: name_a,
/// };
///
/// let sess_b = MeshedChannels {
///     session1: channel_ba,
///     session2: channel_bc,
///     stack: role_b,
///     name: name_b,
/// };
///
/// // ...to this point, should not be written in general. Please look at the *fork* function.
///
/// let _ = send_mpst_b_to_a((), sess_b);
/// let _ = recv_mpst_a_from_b(sess_a).unwrap();
/// ```
pub fn recv_mpst_a_from_b<T, S1, S2, R>(
    s: MeshedChannels<Recv<T, S1>, S2, RoleB<R>, NameA>,
) -> ResultBoxError<T, S1, S2, R, NameA>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    recv_aux_simple!(s, 1)()
}

/// Receive a value of type `T` on B from A. Can fail.
/// Returns either a pair of the received value and the
/// continuation of the `MeshedChannels<S1, S2, R, N>`
/// or an error.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{end::End, recv::Recv, session::Session};
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::name::Name;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::name::a::NameA;
/// use mpstthree::name::b::NameB;
///
/// use mpstthree::functionmpst::recv::recv_mpst_b_from_a;
/// use mpstthree::functionmpst::send::send_mpst_a_to_b;
///
/// // Creating the binary sessions
/// type BtoA = Recv<(), End>;
///
/// // Stack
/// type StackB = RoleA<RoleEnd>;
/// type StackA = RoleB<RoleEnd>;
///
/// // From this point...
///
/// let (channel_ba, channel_ab) = BtoA::new();
/// let (channel_ac, _) = BtoA::new();
/// let (channel_bc, _) = End::new();
///
/// let (role_b, _) = StackB::new();
/// let (role_a, _) = StackA::new();
///
/// let (name_b, _) = NameB::new();
/// let (name_a, _) = NameA::new();
///
/// let sess_b = MeshedChannels {
///     session1: channel_ba,
///     session2: channel_bc,
///     stack: role_b,
///     name: name_b,
/// };
///
/// let sess_a = MeshedChannels {
///     session1: channel_ab,
///     session2: channel_ac,
///     stack: role_a,
///     name: name_a,
/// };
///
/// // ...to this point, should not be written in general. Please look at the *fork* function.
///
/// let _ = send_mpst_a_to_b((), sess_a);
/// let _ = recv_mpst_b_from_a(sess_b).unwrap();
/// ```
pub fn recv_mpst_b_from_a<T, S1, S2, R>(
    s: MeshedChannels<Recv<T, S1>, S2, RoleA<R>, NameB>,
) -> ResultBoxError<T, S1, S2, R, NameB>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    recv_aux_simple!(s, 1)()
}

/// Receive a value of type `T` on C from A. Can fail.
/// Returns either a pair of the received value and the
/// continuation of the `MeshedChannels<S1, S2, R, N>`
/// or an error.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{end::End, recv::Recv, session::Session};
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::name::Name;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::name::a::NameA;
/// use mpstthree::name::c::NameC;
///
/// use mpstthree::functionmpst::recv::recv_mpst_c_from_a;
/// use mpstthree::functionmpst::send::send_mpst_a_to_c;
///
/// // Creating the binary sessions
/// type CtoA = Recv<(), End>;
///
/// // Stack
/// type StackC = RoleA<RoleEnd>;
/// type StackA = RoleC<RoleEnd>;
///
/// // From this point...
///
/// let (channel_ca, channel_ac) = CtoA::new();
/// let (channel_cb, _) = End::new();
/// let (channel_ab, _) = End::new();
///
/// let (role_c, _) = StackC::new();
/// let (role_a, _) = StackA::new();
///
/// let (name_c, _) = NameC::new();
/// let (name_a, _) = NameA::new();
///
/// let sess_c = MeshedChannels {
///     session1: channel_ca,
///     session2: channel_cb,
///     stack: role_c,
///     name: name_c,
/// };
///
/// let sess_a = MeshedChannels {
///     session1: channel_ab,
///     session2: channel_ac,
///     stack: role_a,
///     name: name_a,
/// };
///
/// // ...to this point, should not be written in general. Please look at the *fork* function.
///
/// let _ = send_mpst_a_to_c((), sess_a);
/// let _ = recv_mpst_c_from_a(sess_c).unwrap();
/// ```
pub fn recv_mpst_c_from_a<T, S1, S2, R>(
    s: MeshedChannels<Recv<T, S1>, S2, RoleA<R>, NameC>,
) -> ResultBoxError<T, S1, S2, R, NameC>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    recv_aux_simple!(s, 1)()
}

/// Receive a value of type `T` on A from C. Can fail.
/// Returns either a pair of the received value and the
/// continuation of the `MeshedChannels<S1, S2, R, N>`
/// or an error.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{end::End, recv::Recv, session::Session};
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::name::Name;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::name::a::NameA;
/// use mpstthree::name::c::NameC;
///
/// use mpstthree::functionmpst::recv::recv_mpst_a_from_c;
/// use mpstthree::functionmpst::send::send_mpst_c_to_a;
///
/// // Creating the binary sessions
/// type AtoC = Recv<(), End>;
///
/// // Stack
/// type StackA = RoleC<RoleEnd>;
/// type StackC = RoleA<RoleEnd>;
///
/// // From this point...
///
/// let (channel_ab, _) = End::new();
/// let (channel_cb, _) = End::new();
/// let (channel_ac, channel_ca) = AtoC::new();
///
/// let (role_a, _) = StackA::new();
/// let (role_c, _) = StackC::new();
///
/// let (name_a, _) = NameA::new();
/// let (name_c, _) = NameC::new();
///
/// let sess_a = MeshedChannels {
///     session1: channel_ab,
///     session2: channel_ac,
///     stack: role_a,
///     name: name_a,
/// };
///
/// let sess_c = MeshedChannels {
///     session1: channel_ca,
///     session2: channel_cb,
///     stack: role_c,
///     name: name_c,
/// };
///
/// // ...to this point, should not be written in general. Please look at the *fork* function.
///
/// let _ = send_mpst_c_to_a((), sess_c);
/// let _ = recv_mpst_a_from_c(sess_a).unwrap();
/// ```
pub fn recv_mpst_a_from_c<T, S1, S2, R>(
    s: MeshedChannels<S1, Recv<T, S2>, RoleC<R>, NameA>,
) -> ResultBoxError<T, S1, S2, R, NameA>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    recv_aux_simple!(s, 2)()
}

/// Receive a value of type `T` on B from C. Can fail.
/// Returns either a pair of the received value and the
/// continuation of the `MeshedChannels<S1, S2, R, N>`
/// or an error.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{end::End, recv::Recv, session::Session};
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::name::Name;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::name::b::NameB;
/// use mpstthree::name::c::NameC;
///
/// use mpstthree::functionmpst::recv::recv_mpst_b_from_c;
/// use mpstthree::functionmpst::send::send_mpst_c_to_b;
///
/// // Creating the binary sessions
/// type BtoC = Recv<(), End>;
///
/// // Stack
/// type StackB = RoleC<RoleEnd>;
/// type StackC = RoleB<RoleEnd>;
///
/// // From this point...
///
/// let (channel_ba, _) = End::new();
/// let (channel_ca, _) = End::new();
/// let (channel_bc, channel_cb) = BtoC::new();
///
/// let (role_b, _) = StackB::new();
/// let (role_c, _) = StackC::new();
///
/// let (name_b, _) = NameB::new();
/// let (name_c, _) = NameC::new();
///
/// let sess_b = MeshedChannels {
///     session1: channel_ba,
///     session2: channel_bc,
///     stack: role_b,
///     name: name_b,
/// };
///
/// let sess_c = MeshedChannels {
///     session1: channel_ca,
///     session2: channel_cb,
///     stack: role_c,
///     name: name_c,
/// };
///
/// // ...to this point, should not be written in general. Please look at the *fork* function.
///
/// let _ = send_mpst_c_to_b((), sess_c);
/// let _ = recv_mpst_b_from_c(sess_b).unwrap();
/// ```
pub fn recv_mpst_b_from_c<T, S1, S2, R>(
    s: MeshedChannels<S1, Recv<T, S2>, RoleC<R>, NameB>,
) -> ResultBoxError<T, S1, S2, R, NameB>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    recv_aux_simple!(s, 2)()
}

/// Receive a value of type `T` on C from B. Can fail.
/// Returns either a pair of the received value and the
/// continuation of the `MeshedChannels<S1, S2, R, N>`
/// or an error.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{end::End, recv::Recv, session::Session};
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::name::Name;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::name::b::NameB;
/// use mpstthree::name::c::NameC;
///
/// use mpstthree::functionmpst::recv::recv_mpst_c_from_b;
/// use mpstthree::functionmpst::send::send_mpst_b_to_c;
///
/// // Creating the binary sessions
/// type CtoB = Recv<(), End>;
///
/// // Stack
/// type StackC = RoleB<RoleEnd>;
/// type StackB = RoleC<RoleEnd>;
///
/// // From this point...
///
/// let (channel_ba, _) = End::new();
/// let (channel_ca, _) = End::new();
/// let (channel_cb, channel_bc) = CtoB::new();
///
/// let (role_c, _) = StackC::new();
/// let (role_b, _) = StackB::new();
///
/// let (name_c, _) = NameC::new();
/// let (name_b, _) = NameB::new();
///
/// let sess_c = MeshedChannels {
///     session1: channel_ca,
///     session2: channel_cb,
///     stack: role_c,
///     name: name_c,
/// };
///
/// let sess_b = MeshedChannels {
///     session1: channel_ba,
///     session2: channel_bc,
///     stack: role_b,
///     name: name_b,
/// };
///
/// // ...to this point, should not be written in general. Please look at the *fork* function.
///
/// let _ = send_mpst_b_to_c((), sess_b);
/// let _ = recv_mpst_c_from_b(sess_c).unwrap();
/// ```
pub fn recv_mpst_c_from_b<T, S1, S2, R>(
    s: MeshedChannels<S1, Recv<T, S2>, RoleB<R>, NameC>,
) -> ResultBoxError<T, S1, S2, R, NameC>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    recv_aux_simple!(s, 2)()
}

// Receive a broadcasted value of type `T` on B from A. Can
// fail. Returns either a pair of the received value and
// the continuation of the `MeshedChannels<S1, S2, R, N>` or
// an error. Should not be used as a standalone, but rather
// with [`offer_mpst_session_to_a_from_b`](crate::functionmpst::offer::
// offer_mpst_session_to_a_from_b).
#[doc(hidden)]
pub fn recv_mpst_a_all_from_b<T, S1, S2>(
    s: MeshedChannels<Recv<T, S1>, S2, RoleAlltoB<RoleEnd, RoleEnd>, NameA>,
) -> ResultBoxError<T, S1, S2, RoleEnd, NameA>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
{
    recv_all_aux_simple!(s, 1)()
}

// Receive a broadcasted value of type `T` on C from A. Can
// fail. Returns either a pair of the received value and
// the continuation of the `MeshedChannels<S1, S2, R, N>` or
// an error. Should not be used as a standalone, but rather
// with [`offer_mpst_session_to_a_from_c`](crate::functionmpst::offer::
// offer_mpst_session_to_a_from_c).
#[doc(hidden)]
pub fn recv_mpst_a_all_from_c<T, S1, S2>(
    s: MeshedChannels<S1, Recv<T, S2>, RoleAlltoC<RoleEnd, RoleEnd>, NameA>,
) -> ResultBoxError<T, S1, S2, RoleEnd, NameA>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
{
    recv_all_aux_simple!(s, 2)()
}

// Receive a broadcasted value of type `T` on A from B. Can
// fail. Returns either a pair of the received value and
// the continuation of the `MeshedChannels<S1, S2, R, N>` or
// an error. Should not be used as a standalone, but rather
// with [`offer_mpst_session_to_b_from_a`](crate::functionmpst::offer::
// offer_mpst_session_to_b_from_a).
#[doc(hidden)]
pub fn recv_mpst_b_all_from_a<T, S1, S2>(
    s: MeshedChannels<Recv<T, S1>, S2, RoleAlltoA<RoleEnd, RoleEnd>, NameB>,
) -> ResultBoxError<T, S1, S2, RoleEnd, NameB>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
{
    recv_all_aux_simple!(s, 1)()
}

// Receive a broadcasted value of type `T` on C from A. Can
// fail. Returns either a pair of the received value and
// the continuation of the `MeshedChannels<S1, S2, R, N>` or
// an error. Should not be used as a standalone, but rather
// with [`offer_mpst_session_to_b_from_c`](crate::functionmpst::offer::
// offer_mpst_session_to_b_from_c).
#[doc(hidden)]
pub fn recv_mpst_b_all_from_c<T, S1, S2>(
    s: MeshedChannels<S1, Recv<T, S2>, RoleAlltoC<RoleEnd, RoleEnd>, NameB>,
) -> ResultBoxError<T, S1, S2, RoleEnd, NameB>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
{
    recv_all_aux_simple!(s, 2)()
}

// Receive a broadcasted value of type `T` on A from B. Can
// fail. Returns either a pair of the received value and
// the continuation of the `MeshedChannels<S1, S2, R, N>` or
// an error. Should not be used as a standalone, but rather
// with [`offer_mpst_session_to_c_from_a`](crate::functionmpst::offer::
// offer_mpst_session_to_c_from_a).
#[doc(hidden)]
pub fn recv_mpst_c_all_from_a<T, S1, S2>(
    s: MeshedChannels<Recv<T, S1>, S2, RoleAlltoA<RoleEnd, RoleEnd>, NameC>,
) -> ResultBoxError<T, S1, S2, RoleEnd, NameC>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
{
    recv_all_aux_simple!(s, 1)()
}

// Receive a broadcasted value of type `T` on B from C. Can
// fail. Returns either a pair of the received value and
// the continuation of the `MeshedChannels<S1, S2, R, N>` or
// an error. Should not be used as a standalone, but rather
// with [`offer_mpst_session_to_c_from_b`](crate::functionmpst::offer::
// offer_mpst_session_to_c_from_b).
#[doc(hidden)]
pub fn recv_mpst_c_all_from_b<T, S1, S2>(
    s: MeshedChannels<S1, Recv<T, S2>, RoleAlltoB<RoleEnd, RoleEnd>, NameC>,
) -> ResultBoxError<T, S1, S2, RoleEnd, NameC>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
{
    recv_all_aux_simple!(s, 2)()
}
