//! This module contains the functions for forking the different endpoints.

use std::any::Any;
use std::error::Error;
use std::marker;
use std::panic;
use std::thread::{spawn, JoinHandle};

use crate::binary;
use crate::role;
use crate::sessionmpst;

type ResultAnySend = Result<(), Box<(dyn Any + marker::Send + 'static)>>;

#[doc(hidden)]
pub fn fork_simple<S1, S2, R, N, P>(
    p: P,
    s: sessionmpst::SessionMpst<S1, S2, R, N>,
) -> JoinHandle<()>
where
    S1: binary::Session + 'static,
    S2: binary::Session + 'static,
    R: role::Role + 'static,
    N: role::Role + 'static,
    P: FnOnce(sessionmpst::SessionMpst<S1, S2, R, N>) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
{
    spawn(move || {
        panic::set_hook(Box::new(|_info| {
            // do nothing
        }));
        match p(s) {
            Ok(()) => (),
            Err(e) => panic!("{:?}", e),
        }
    })
}

/// Creates and returns a tuple of three child processes for three
/// [`mpstthree::sessionmpst::SessionMpst`](../sessionmpst/struct.SessionMpst.html) linked together.
///
///  # Example
///
///  ```
/// use std::boxed::Box;
/// use std::collections::hash_map::RandomState;
/// use std::collections::HashMap;
/// use std::error::Error;
///
/// use mpstthree::binary::{End, Recv, Send, Session};
/// use mpstthree::fork::fork_mpst;
/// use mpstthree::sessionmpst::SessionMpst;
///
/// use mpstthree::functionmpst::close::close_mpst;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::functionmpst::recv::recv_mpst_a_to_c;
/// use mpstthree::functionmpst::recv::recv_mpst_b_to_a;
/// use mpstthree::functionmpst::recv::recv_mpst_c_to_b;
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
/// type QueueA = RoleB<RoleC<RoleEnd>>;
/// type QueueB = RoleA<RoleC<RoleEnd>>;
/// type QueueC = RoleA<RoleB<RoleEnd>>;
///
/// type EndpointA<N> = SessionMpst<AtoB<N>, AtoC<N>, QueueA, RoleA<RoleEnd>>;
/// type EndpointB<N> = SessionMpst<BtoA<N>, BtoC<N>, QueueB, RoleB<RoleEnd>>;
/// type EndpointC<N> = SessionMpst<CtoA<N>, CtoB<N>, QueueC, RoleC<RoleEnd>>;
///
/// fn simple_triple_endpoint_a(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
///     let s = send_mpst_a_to_b(1, s);
///     let (x, s) = recv_mpst_a_to_c(s)?;
///     close_mpst(s)?;
///     Ok(())
/// }
///
/// /// Single test for B
/// fn simple_triple_endpoint_b(s: EndpointB<i32>) -> Result<(), Box<dyn Error>> {
///     let (x, s) = recv_mpst_b_to_a(s)?;
///     let s = send_mpst_b_to_c(2, s);
///     close_mpst(s)?;
///     Ok(())
/// }
///
/// /// Single test for C
/// fn simple_triple_endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
///     let s = send_mpst_c_to_a(3, s);
///     let (x, s) = recv_mpst_c_to_b(s)?;
///     close_mpst(s)?;
///     Ok(())
/// }
/// let (thread_a, thread_b, thread_c) = fork_mpst(
///                 simple_triple_endpoint_a,
///                 simple_triple_endpoint_b,
///                 simple_triple_endpoint_c,
/// );
///  ```
///
/// Creates 3 pairs of endpoints, each pair of type `S` and `S::Dual`.
/// Creates 3 `Role` for each queue.
/// Creates 3 `SessionMpst`, linked together with the pairs of endpoints, and get the related child processes.
pub fn fork_mpst<S0, S1, S2, R0, R1, R2, N0, N1, N2, F0, F1, F2>(
    f0: F0,
    f1: F1,
    f2: F2,
) -> (ResultAnySend, ResultAnySend, ResultAnySend)
where
    S0: binary::Session + 'static,
    S1: binary::Session + 'static,
    S2: binary::Session + 'static,
    R0: role::Role + 'static,
    R1: role::Role + 'static,
    R2: role::Role + 'static,
    N0: role::Role + 'static,
    N1: role::Role + 'static,
    N2: role::Role + 'static,
    F0: FnOnce(sessionmpst::SessionMpst<S0, S1, R0, N0>) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
    F1: FnOnce(
            sessionmpst::SessionMpst<<S0 as binary::Session>::Dual, S2, R1, N1>,
        ) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
    F2: FnOnce(
            sessionmpst::SessionMpst<
                <S1 as binary::Session>::Dual,
                <S2 as binary::Session>::Dual,
                R2,
                N2,
            >,
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

    let a = sessionmpst::SessionMpst {
        session1: channel_ab,
        session2: channel_ac,
        stack: role_a,
        name: name_a,
    };
    let b = sessionmpst::SessionMpst {
        session1: channel_ba,
        session2: channel_bc,
        stack: role_b,
        name: name_b,
    };
    let c = sessionmpst::SessionMpst {
        session1: channel_ca,
        session2: channel_cb,
        stack: role_c,
        name: name_c,
    };

    let thread_a = fork_simple(f0, a);
    let thread_b = fork_simple(f1, b);
    let thread_c = fork_simple(f2, c);

    (thread_a.join(), thread_b.join(), thread_c.join())
}
