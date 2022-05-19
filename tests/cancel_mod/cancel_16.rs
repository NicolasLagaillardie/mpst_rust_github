use mpstthree::baker;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::end::RoleEnd;

use rand::random;
use std::error::Error;

// A --> B canceled
// A --> B.B--> C

// Create new MeshedChannels for three participants
baker!("rec_and_cancel", MeshedChannelsThree, A, B, C);

// Types
type EndpointA = MeshedChannelsThree<Send<i32, End>, End, RoleB<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsThree<Recv<i32, End>, Send<i32, End>, RoleA<RoleC<RoleEnd>>, NameB>;
type EndpointC = MeshedChannelsThree<End, Recv<i32, End>, RoleB<RoleEnd>, NameC>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    let s = s.send(random())?;
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
