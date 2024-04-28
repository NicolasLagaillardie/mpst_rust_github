use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::functionmpst::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

use mpstthree::name::a::NameA;
use mpstthree::name::b::NameB;
use mpstthree::name::c::NameC;

use rand::random;
use std::error::Error;

// A --> B canceled
// A --> B.B--> C

// Types
type EndpointA = MeshedChannels<Send<i32, End>, End, RoleB<RoleEnd>, NameA>;
type EndpointB = MeshedChannels<Recv<i32, End>, Send<i32, End>, RoleA<RoleC<RoleEnd>>, NameB>;
type EndpointC = MeshedChannels<End, Recv<i32, End>, RoleB<RoleEnd>, NameC>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    let s = s.send(random());
    s.close()
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    s.cancel();

    // let (_, s) = recv_mpst_b_from_a(s)?;
    // let s = send_mpst_b_to_c(random(), s);
    // close_mpst_multi(s)

    panic!("Session dropped");
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv()?;
    s.close()
}

pub fn main() {
    let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);

    assert!(thread_a.join().is_err());
    assert!(thread_b.join().is_err());
    assert!(thread_c.join().is_err());
}
