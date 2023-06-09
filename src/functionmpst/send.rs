//! This module contains all the *send* functions

use crate::binary::struct_trait::{send::Send, session::Session};
use crate::meshedchannels::MeshedChannels;
use crate::name::a::NameA;
use crate::name::b::NameB;
use crate::name::c::NameC;
use crate::role::a::RoleA;
use crate::role::b::RoleB;
use crate::role::c::RoleC;
use crate::role::Role;
use std::marker;

#[doc(hidden)]
#[macro_export]
macro_rules! send_aux_simple {
    ($session:expr, $payload:expr, $exclusion: literal) => {
        mpst_seq::send_aux_simple!($session, $payload, $exclusion)
    };
}

/// Send a value of type `T` from A to B. Always succeeds.
/// Returns the continuation `MeshedChannels<S1, S2, R, N>`.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{end::End, send::Send, session::Session};
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::name::Name;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::name::a::NameA;
///
/// use mpstthree::functionmpst::send::send_mpst_a_to_b;
///
/// // Creating the binary sessions
/// type AtoB<N> = Send<N, End>;
/// type AtoC = End;
///
/// // Stack
/// type StackA = RoleB<RoleEnd>;
///
/// // From this point...
///
/// let (channel_ab, _) = AtoB::<i32>::new();
/// let (channel_ac, _) = AtoC::new();
///
/// let (role_a, _) = StackA::new();
///
/// let (name_a, _) = NameA::new();
///
/// let sess = MeshedChannels {
///     session1: channel_ab,
///     session2: channel_ac,
///     stack: role_a,
///     name: name_a,
/// };
///
/// // ...to this point, should not be written in general. Please look at the *fork* function.
///
/// let _s = send_mpst_a_to_b(1, sess);
/// ```
pub fn send_mpst_a_to_b<T, S1, S2, R>(
    x: T,
    s: MeshedChannels<Send<T, S1>, S2, RoleB<R>, NameA>,
) -> MeshedChannels<S1, S2, R, NameA>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    send_aux_simple!(s, x, 1)
}

/// Send a value of type `T` from B to A. Always succeeds.
/// Returns the continuation of the `MeshedChannels<S1, S2, R,
/// N>`.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{end::End, send::Send, session::Session};
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::name::Name;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::name::b::NameB;
///
/// use mpstthree::functionmpst::send::send_mpst_b_to_a;
///
/// // Creating the binary sessions
/// type BtoA<N> = Send<N, End>;
/// type BtoC = End;
///
/// // Stack
/// type StackB = RoleA<RoleEnd>;
///
/// // From this point...
///
/// let (channel_ba, _) = BtoA::<i32>::new();
/// let (channel_bc, _) = BtoC::new();
///
/// let (role_b, _) = StackB::new();
///
/// let (name_b, _) = NameB::new();
///
/// let sess = MeshedChannels {
///     session1: channel_ba,
///     session2: channel_bc,
///     stack: role_b,
///     name: name_b,
/// };
///
/// // ...to this point, should not be written in general. Please look at the *fork* function.
///
/// let _s = send_mpst_b_to_a(1, sess);
/// ```
pub fn send_mpst_b_to_a<T, S1, S2, R>(
    x: T,
    s: MeshedChannels<Send<T, S1>, S2, RoleA<R>, NameB>,
) -> MeshedChannels<S1, S2, R, NameB>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    send_aux_simple!(s, x, 1)
}

/// Send a value of type `T` from C to A. Always succeeds.
/// Returns the continuation of the `MeshedChannels<S1, S2, R,
/// N>`.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{end::End, send::Send, session::Session};
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::name::Name;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::name::c::NameC;
///
/// use mpstthree::functionmpst::send::send_mpst_c_to_a;
///
/// // Creating the binary sessions
/// type CtoA<N> = Send<N, End>;
/// type CtoB = End;
///
/// // Stack
/// type StackC = RoleA<RoleEnd>;
///
/// // From this point...
///
/// let (channel_ca, _) = CtoA::<i32>::new();
/// let (channel_cb, _) = CtoB::new();
///
/// let (role_c, _) = StackC::new();
///
/// let (name_c, _) = NameC::new();
///
/// let sess = MeshedChannels {
///     session1: channel_ca,
///     session2: channel_cb,
///     stack: role_c,
///     name: name_c,
/// };
///
/// // ...to this point, should not be written in general. Please look at the *fork* function.
///
/// let _s = send_mpst_c_to_a(1, sess);
/// ```
pub fn send_mpst_c_to_a<T, S1, S2, R>(
    x: T,
    s: MeshedChannels<Send<T, S1>, S2, RoleA<R>, NameC>,
) -> MeshedChannels<S1, S2, R, NameC>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    send_aux_simple!(s, x, 1)
}

/// Send a value of type `T` from A to C. Always succeeds.
/// Returns the continuation of the `MeshedChannels<S1, S2, R,
/// N>`.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{end::End, send::Send, session::Session};
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::name::Name;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::name::a::NameA;
///
/// use mpstthree::functionmpst::send::send_mpst_a_to_c;
///
/// // Creating the binary sessions
/// type AtoB = End;
/// type AtoC<N> = Send<N, End>;
///
/// // Stack
/// type StackA = RoleC<RoleEnd>;
///
/// // From this point...
///
/// let (channel_ab, _) = AtoB::new();
/// let (channel_ac, _) = AtoC::<i32>::new();
///
/// let (role_a, _) = StackA::new();
///
/// let (name_a, _) = NameA::new();
///
/// let sess = MeshedChannels {
///     session1: channel_ab,
///     session2: channel_ac,
///     stack: role_a,
///     name: name_a,
/// };
///
/// // ...to this point, should not be written in general. Please look at the *fork* function.
///
/// let _s = send_mpst_a_to_c(1, sess);
/// ```
pub fn send_mpst_a_to_c<T, S1, S2, R>(
    x: T,
    s: MeshedChannels<S1, Send<T, S2>, RoleC<R>, NameA>,
) -> MeshedChannels<S1, S2, R, NameA>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    send_aux_simple!(s, x, 2)
}

/// Send a value of type `T` from B to C. Always succeeds.
/// Returns the continuation of the `MeshedChannels<S1, S2, R,
/// N>`.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{end::End, send::Send, session::Session};
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::name::Name;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::name::b::NameB;
///
/// use mpstthree::functionmpst::send::send_mpst_b_to_c;
///
/// // Creating the binary sessions
/// type BtoA = End;
/// type BtoC<N> = Send<N, End>;
///
/// // Stack
/// type StackB = RoleC<RoleEnd>;
///
/// // From this point...
///
/// let (channel_ba, _) = BtoA::new();
/// let (channel_bc, _) = BtoC::<i32>::new();
///
/// let (role_b, _) = StackB::new();
///
/// let (name_b, _) = NameB::new();
///
/// let sess = MeshedChannels {
///     session1: channel_ba,
///     session2: channel_bc,
///     stack: role_b,
///     name: name_b,
/// };
///
/// // ...to this point, should not be written in general. Please look at the *fork* function.
///
/// let _s = send_mpst_b_to_c(1, sess);
/// ```
pub fn send_mpst_b_to_c<T, S1, S2, R>(
    x: T,
    s: MeshedChannels<S1, Send<T, S2>, RoleC<R>, NameB>,
) -> MeshedChannels<S1, S2, R, NameB>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    send_aux_simple!(s, x, 2)
}

/// Send a value of type `T` from C to B. Always succeeds.
/// Returns the continuation of the `MeshedChannels<S1, S2, R,
/// N>`.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{end::End, send::Send, session::Session};
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::name::Name;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::name::c::NameC;
///
/// use mpstthree::functionmpst::send::send_mpst_c_to_b;
///
/// // Creating the binary sessions
/// type CtoA = End;
/// type CtoB<N> = Send<N, End>;
///
/// // Stack
/// type StackC = RoleB<RoleEnd>;
///
/// // From this point...
///
/// let (channel_ca, _) = CtoA::new();
/// let (channel_cb, _) = CtoB::<i32>::new();
///
/// let (role_c, _) = StackC::new();
///
/// let (name_c, _) = NameC::new();
///
/// let sess = MeshedChannels {
///     session1: channel_ca,
///     session2: channel_cb,
///     stack: role_c,
///     name: name_c,
/// };
///
/// // ...to this point, should not be written in general. Please look at the *fork* function.
///
/// let _s = send_mpst_c_to_b(1, sess);
/// ```
pub fn send_mpst_c_to_b<T, S1, S2, R>(
    x: T,
    s: MeshedChannels<S1, Send<T, S2>, RoleB<R>, NameC>,
) -> MeshedChannels<S1, S2, R, NameC>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    send_aux_simple!(s, x, 2)
}
