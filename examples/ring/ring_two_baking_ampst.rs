#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::error::Error;

static LOOPS: i64 = 100;

// Create new roles
generate!("rec_and_cancel", MeshedChannels, A, B);

// Types
// A
enum Branching0fromBtoA {
    Forward(MeshedChannels<Send<(), RecursAtoB>, RoleB<RoleB<RoleEnd>>, NameA>),
    Backward(MeshedChannels<Recv<(), RecursAtoB>, RoleB<RoleB<RoleEnd>>, NameA>),
    Done(MeshedChannels<End, RoleEnd, NameA>),
}
type RecursAtoB = Recv<Branching0fromBtoA, End>;

// B
type Choose0fromBtoA = Send<Branching0fromBtoA, End>;
type EndpointForwardB = MeshedChannels<Recv<(), Choose0fromBtoA>, RoleA<RoleBroadcast>, NameB>;
type EndpointBackwardB = MeshedChannels<Send<(), Choose0fromBtoA>, RoleA<RoleBroadcast>, NameB>;

// Creating the MP sessions
type EndpointA = MeshedChannels<RecursAtoB, RoleB<RoleEnd>, NameA>;
type EndpointB = MeshedChannels<Choose0fromBtoA, RoleBroadcast, NameB>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromBtoA::Done(s) => {
            s.close()
        },
        Branching0fromBtoA::Forward(s) => {
            let s = s.send(())?;
            endpoint_a(s)
        },
        Branching0fromBtoA::Backward(s) => {
            let (_, s) = s.recv()?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    recurs_b(s, LOOPS)
}

fn recurs_b(s: EndpointB, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_b_to_all!(s, Branching0fromBtoA::Done);

            s.close()
        }
        i if i % 2 == 0 => {
            let s: EndpointForwardB = choose_mpst_b_to_all!(s, Branching0fromBtoA::Forward);

            let (_, s) = s.recv()?;

            recurs_b(s, i - 1)
        }
        i => {
            let s: EndpointBackwardB = choose_mpst_b_to_all!(s, Branching0fromBtoA::Backward);

            let s = s.send(())?;

            recurs_b(s, i - 1)
        }
    }
}

fn main() {
    let (thread_a, thread_b) = fork_mpst(endpoint_a, endpoint_b);

    thread_a.join().unwrap();
    thread_b.join().unwrap();
}
