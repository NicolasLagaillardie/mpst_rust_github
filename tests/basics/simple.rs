use mpstthree::checking::checker;

use std::boxed::Box;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;

use mpstthree::functionmpst::close::close_mpst;

use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

use mpstthree::functionmpst::recv::recv_mpst_a_from_c;
use mpstthree::functionmpst::recv::recv_mpst_b_from_a;
use mpstthree::functionmpst::recv::recv_mpst_c_from_b;

use mpstthree::functionmpst::send::send_mpst_a_to_b;
use mpstthree::functionmpst::send::send_mpst_b_to_c;
use mpstthree::functionmpst::send::send_mpst_c_to_a;

/// Creating the binary sessions
type AtoB<N> = Send<N, End>;
type AtoC<N> = Recv<N, End>;

type BtoA<N> = <AtoB<N> as Session>::Dual;
type BtoC<N> = Send<N, End>;

type CtoA<N> = <AtoC<N> as Session>::Dual;
type CtoB<N> = <BtoC<N> as Session>::Dual;

/// Stacks
type StackA = RoleB<RoleC<RoleEnd>>;
type StackB = RoleA<RoleC<RoleEnd>>;
type StackC = RoleA<RoleB<RoleEnd>>;

/// Creating the MP sessions
type EndpointA<N> = MeshedChannels<AtoB<N>, AtoC<N>, StackA, RoleA<RoleEnd>>;
type EndpointB<N> = MeshedChannels<BtoA<N>, BtoC<N>, StackB, RoleB<RoleEnd>>;
type EndpointC<N> = MeshedChannels<CtoA<N>, CtoB<N>, StackC, RoleC<RoleEnd>>;

/// Single test for A
fn endpoint_a(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
    let (size, s) = s.field_names();
    assert_eq!(size.len(), 2);
    let s = send_mpst_a_to_b(1, s);
    let (x, s) = recv_mpst_a_from_c(s)?;

    assert_eq!(x, 3);

    close_mpst(s)
}

/// Single test for B
fn endpoint_b(s: EndpointB<i32>) -> Result<(), Box<dyn Error>> {
    let (size, s) = s.field_names();
    assert_eq!(size.len(), 2);
    let (x, s) = recv_mpst_b_from_a(s)?;
    let s = send_mpst_b_to_c(2, s);

    assert_eq!(x, 1);

    close_mpst(s)
}

/// Single test for C
fn endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
    let (size, s) = s.field_names();
    assert_eq!(size.len(), 2);
    let s = send_mpst_c_to_a(3, s);
    let (x, s) = recv_mpst_c_from_b(s)?;

    assert_eq!(x, 2);

    close_mpst(s)
}

/////////////////////////////////////////

pub fn simple_triple_endpoints() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}

pub fn simple_triple_endpoints_checker() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let s = RandomState::new();
            let hm: HashMap<String, &Vec<String>> = HashMap::with_hasher(s);

            let (s1, _): (EndpointA<i32>, _) = MeshedChannels::new();
            let (s2, _): (EndpointB<i32>, _) = MeshedChannels::new();
            let (s3, _): (EndpointC<i32>, _) = MeshedChannels::new();

            let (a, b, c) = checker(s1, s2, s3, &hm, &HashMap::new())?;

            assert_eq!(a, "A: A!B.A?C.0");
            assert_eq!(b, "B: B?A.B!C.0");
            assert_eq!(c, "C: C!A.C?B.0");
        }
        Ok(())
    }()
    .is_ok());
}
