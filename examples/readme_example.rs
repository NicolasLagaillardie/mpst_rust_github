// Used for the functions that will process the protocol
use std::boxed::Box;
use std::error::Error;

// Used for creating the types
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::meshedchannels::MeshedChannels;

// Used for creating the stack of each role
use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

// Importing the names of the participants
use mpstthree::name::a::NameA;
use mpstthree::name::b::NameB;
use mpstthree::name::c::NameC;

// Used for connecting all the roles, represented as MeshedChannels, together
use mpstthree::functionmpst::fork::fork_mpst;

// Creating the binary sessions
// for A
type AtoB = Send<i32, End>;
type AtoC = Recv<i32, End>;

// for B
type BtoA = Recv<i32, End>;
type BtoC = Send<i32, End>;

// for C
type CtoA = Send<i32, End>;
type CtoB = Recv<i32, End>;

// Stacks
// for A
type StackA = RoleB<RoleC<RoleEnd>>;
// for B
type StackB = RoleA<RoleC<RoleEnd>>;
// for C
type StackC = RoleA<RoleB<RoleEnd>>;

// Creating the MP sessions
// for A
type EndpointA = MeshedChannels<AtoB, AtoC, StackA, NameA>;
// for B
type EndpointB = MeshedChannels<BtoA, BtoC, StackB, NameB>;
// for C
type EndpointC = MeshedChannels<CtoA, CtoB, StackC, NameC>;

// Function to process Endpoint of A
fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    let s = s.send(1);
    let (_x, s) = s.recv()?;

    s.close()
}

// Function to process Endpoint of B
fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    let (_x, s) = s.recv()?;
    let s = s.send(2);

    s.close()
}

// Function to process Endpoint of C
fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let s = s.send(3);
    let (_x, s) = s.recv()?;

    s.close()
}

// Fork all endpoints
fn main() {
    let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();

    println!("Done");
}
