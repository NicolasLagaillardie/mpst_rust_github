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
generate!("rec_and_cancel", MeshedChannels, A, B, C);

// Types
// A
enum Branching0fromCtoA {
    Forward(MeshedChannels<Send<(), End>, RecursAtoC, RoleB<RoleC<RoleEnd>>, NameA>),
    Backward(MeshedChannels<Recv<(), End>, RecursAtoC, RoleB<RoleC<RoleEnd>>, NameA>),
    Done(MeshedChannels<End, End, RoleEnd, NameA>),
}
type RecursAtoC = Recv<Branching0fromCtoA, End>;

// B
enum Branching0fromCtoB {
    Forward(
        MeshedChannels<Recv<(), End>, Send<(), RecursBtoC>, RoleA<RoleC<RoleC<RoleEnd>>>, NameB>,
    ),
    Backward(
        MeshedChannels<Send<(), End>, Recv<(), RecursBtoC>, RoleC<RoleA<RoleC<RoleEnd>>>, NameB>,
    ),
    Done(MeshedChannels<End, End, RoleEnd, NameB>),
}
type RecursBtoC = Recv<Branching0fromCtoB, End>;

// C
type Choose0fromCtoA = Send<Branching0fromCtoA, End>;
type Choose0fromCtoB = Send<Branching0fromCtoB, End>;
type EndpointForwardC =
    MeshedChannels<Choose0fromCtoA, Recv<(), Choose0fromCtoB>, RoleB<RoleBroadcast>, NameC>;
type EndpointBackwardC =
    MeshedChannels<Choose0fromCtoA, Send<(), Choose0fromCtoB>, RoleB<RoleBroadcast>, NameC>;

// Creating the MP sessions
type EndpointA = MeshedChannels<End, RecursAtoC, RoleC<RoleEnd>, NameA>;
type EndpointB = MeshedChannels<End, RecursBtoC, RoleC<RoleEnd>, NameB>;
type EndpointC = MeshedChannels<Choose0fromCtoA, Choose0fromCtoB, RoleBroadcast, NameC>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromCtoA::Done(s) => {
            s.close()
        },
        Branching0fromCtoA::Forward(s) => {
            let s = s.send(())?;
            endpoint_a(s)
        },
        Branching0fromCtoA::Backward(s) => {
            let (_, s) = s.recv()?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromCtoB::Done(s) => {
            s.close()
        },
        Branching0fromCtoB::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_b(s)
        },
        Branching0fromCtoB::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    recurs_c(s, LOOPS)
}

fn recurs_c(s: EndpointC, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_c_to_all!(s, Branching0fromCtoA::Done, Branching0fromCtoB::Done);

            s.close()
        }
        i if i % 2 == 0 => {
            let s: EndpointForwardC =
                choose_mpst_c_to_all!(s, Branching0fromCtoA::Forward, Branching0fromCtoB::Forward);

            let (_, s) = s.recv()?;

            recurs_c(s, i - 1)
        }
        i => {
            let s: EndpointBackwardC = choose_mpst_c_to_all!(
                s,
                Branching0fromCtoA::Backward,
                Branching0fromCtoB::Backward
            );

            let s = s.send(())?;

            recurs_c(s, i - 1)
        }
    }
}

fn main() {
    let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
}
