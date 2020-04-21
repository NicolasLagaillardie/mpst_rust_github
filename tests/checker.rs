extern crate mpstthree;

use mpstthree::binary::{End, Recv, Send, Session};
use mpstthree::checking::checking::checker;
use mpstthree::sessionmpst::SessionMpst;

use mpstthree::role::a_to_b::RoleAtoB;
use mpstthree::role::a_to_c::RoleAtoC;
use mpstthree::role::b_to_a::RoleBtoA;
use mpstthree::role::b_to_c::RoleBtoC;
use mpstthree::role::c_to_a::RoleCtoA;
use mpstthree::role::c_to_b::RoleCtoB;
use mpstthree::role::end::RoleEnd;

/// A = !B.?C
/// B = ?A.!C
/// C = !A.?B

/// Creating the binary sessions
type AtoB<N> = Send<N, End>;
type AtoC<N> = Recv<N, End>;

type BtoA<N> = <AtoB<N> as Session>::Dual;
type BtoC<N> = Send<N, End>;

type CtoA<N> = <AtoC<N> as Session>::Dual;
type CtoB<N> = <BtoC<N> as Session>::Dual;

/// Queues
type QueueA = RoleAtoB<RoleAtoC<RoleEnd>>;
type QueueB = RoleBtoA<RoleBtoC<RoleEnd>>;
type QueueC = RoleCtoA<RoleCtoB<RoleEnd>>;

/// Creating the MP sessions
type EndpointA<N> = SessionMpst<AtoB<N>, AtoC<N>, QueueA>;
type EndpointB<N> = SessionMpst<BtoA<N>, BtoC<N>, QueueB>;
type EndpointC<N> = SessionMpst<CtoA<N>, CtoB<N>, QueueC>;

#[test]
fn simple_triple_endpoints() {
    let (s1, _): (EndpointA<i32>, _) = SessionMpst::new();
    let (s2, _): (EndpointB<i32>, _) = SessionMpst::new();
    let (s3, _): (EndpointC<i32>, _) = SessionMpst::new();

    checker(s1, s2, s3);

    // assert!(|| -> Result<(), Box<dyn Error>> {
    //     {
    //         let (thread_a, thread_b, thread_c) = run_processes(
    //             simple_triple_endpoint_a,
    //             simple_triple_endpoint_b,
    //             simple_triple_endpoint_c,
    //         );

    //         assert!(thread_a.is_ok());
    //         assert!(thread_b.is_ok());
    //         assert!(thread_c.is_ok());
    //     }
    //     Ok(())
    // }()
    // .is_ok());
}
