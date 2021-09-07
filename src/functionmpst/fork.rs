//! This module contains the functions for forking the
//! different endpoints.

use std::error::Error;
use std::marker;
use std::panic::set_hook;
use std::thread::{spawn, JoinHandle};

use crate::binary::struct_trait::session::Session;
use crate::meshedchannels::MeshedChannels;
use crate::role::Role;

#[doc(hidden)]
fn fork_simple<S1, S2, R, N, P>(p: P, s: MeshedChannels<S1, S2, R, N>) -> JoinHandle<()>
where
    S1: Session + 'static,
    S2: Session + 'static,
    R: Role + 'static,
    N: Role + 'static,
    P: FnOnce(MeshedChannels<S1, S2, R, N>) -> Result<(), Box<dyn Error>> + marker::Send + 'static,
{
    spawn(move || {
        set_hook(Box::new(|_info| {
            // do nothing
        }));
        match p(s) {
            Ok(()) => (),
            Err(e) => panic!("{:?}", e),
        }
    })
}

/// Creates and returns a tuple of three child processes for
/// three [`MeshedChannels`] linked
/// together.
///
/// # Example
///
/// ```
/// use std::error::Error;
///
/// use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
/// use mpstthree::functionmpst::fork::fork_mpst;
/// use mpstthree::meshedchannels::MeshedChannels;
///
/// use mpstthree::functionmpst::close::close_mpst;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::functionmpst::recv::recv_mpst_a_from_c;
/// use mpstthree::functionmpst::recv::recv_mpst_b_from_a;
/// use mpstthree::functionmpst::recv::recv_mpst_c_from_b;
///
/// use mpstthree::functionmpst::send::send_mpst_a_to_b;
/// use mpstthree::functionmpst::send::send_mpst_b_to_c;
/// use mpstthree::functionmpst::send::send_mpst_c_to_a;
///
/// type AtoB<N> = Send<N, End>;
/// type AtoC<N> = Recv<N, End>;
///
/// type BtoA<N> = <AtoB<N> as Session>::Dual;
/// type BtoC<N> = Send<N, End>;
///
/// type CtoA<N> = <AtoC<N> as Session>::Dual;
/// type CtoB<N> = <BtoC<N> as Session>::Dual;
///
/// type StackA = RoleB<RoleC<RoleEnd>>;
/// type StackB = RoleA<RoleC<RoleEnd>>;
/// type StackC = RoleA<RoleB<RoleEnd>>;
///
/// type EndpointA<N> = MeshedChannels<AtoB<N>, AtoC<N>, StackA, RoleA<RoleEnd>>;
/// type EndpointB<N> = MeshedChannels<BtoA<N>, BtoC<N>, StackB, RoleB<RoleEnd>>;
/// type EndpointC<N> = MeshedChannels<CtoA<N>, CtoB<N>, StackC, RoleC<RoleEnd>>;
///
/// fn endpoint_a(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
///     let s = send_mpst_a_to_b(1, s);
///     let (_x, s) = recv_mpst_a_from_c(s)?;
///     close_mpst(s)
/// }
///
/// /// Single test for B
/// fn endpoint_b(s: EndpointB<i32>) -> Result<(), Box<dyn Error>> {
///     let (_x, s) = recv_mpst_b_from_a(s)?;
///     let s = send_mpst_b_to_c(2, s);
///     close_mpst(s)
/// }
///
/// /// Single test for C
/// fn endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
///     let s = send_mpst_c_to_a(3, s);
///     let (_x, s) = recv_mpst_c_from_b(s)?;
///     close_mpst(s)
/// }
/// let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);
///
/// thread_a.join().unwrap();
/// thread_b.join().unwrap();
/// thread_c.join().unwrap();
/// ```
///
/// Creates 3 pairs of endpoints, each pair of type `S` and
/// `S::Dual`. Creates 3 `Role` for each stack.
/// Creates 3 `MeshedChannels`, linked together with the pairs
/// of endpoints, and get the related child processes.
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
pub fn fork_mpst<S0, S1, S2, R0, R1, R2, N0, N1, N2, F0, F1, F2>(
    f0: F0,
    f1: F1,
    f2: F2,
) -> (JoinHandle<()>, JoinHandle<()>, JoinHandle<()>)
where
    S0: Session + 'static,
    S1: Session + 'static,
    S2: Session + 'static,
    R0: Role + 'static,
    R1: Role + 'static,
    R2: Role + 'static,
    N0: Role + 'static,
    N1: Role + 'static,
    N2: Role + 'static,
    F0: FnOnce(MeshedChannels<S0, S1, R0, N0>) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
    F1: FnOnce(MeshedChannels<<S0 as Session>::Dual, S2, R1, N1>) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
    F2: FnOnce(
            MeshedChannels<<S1 as Session>::Dual, <S2 as Session>::Dual, R2, N2>,
        ) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
{
    let (channel_ab, channel_ba) = S0::new();
    let (channel_ac, channel_ca) = S1::new();
    let (channel_bc, channel_cb) = S2::new();

    let (role_a, _) = R0::new();
    let (role_b, _) = R1::new();
    let (role_c, _) = R2::new();

    let (name_a, _) = N0::new();
    let (name_b, _) = N1::new();
    let (name_c, _) = N2::new();

    let a = MeshedChannels {
        session1: channel_ab,
        session2: channel_ac,
        stack: role_a,
        name: name_a,
    };
    let b = MeshedChannels {
        session1: channel_ba,
        session2: channel_bc,
        stack: role_b,
        name: name_b,
    };
    let c = MeshedChannels {
        session1: channel_ca,
        session2: channel_cb,
        stack: role_c,
        name: name_c,
    };

    (fork_simple(f0, a), fork_simple(f1, b), fork_simple(f2, c))
}
