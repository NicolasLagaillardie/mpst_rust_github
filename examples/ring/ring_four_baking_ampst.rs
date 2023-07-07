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
generate!("rec_and_cancel", MeshedChannelsFour, A, B, C, D);

// Types
// A
enum Branching0fromDtoA {
    Forward(MeshedChannelsFour<Send<(), End>, End, RecursAtoD, RoleB<RoleD<RoleEnd>>, NameA>),
    Backward(MeshedChannelsFour<Recv<(), End>, End, RecursAtoD, RoleB<RoleD<RoleEnd>>, NameA>),
    Done(MeshedChannelsFour<End, End, End, RoleEnd, NameA>),
}
type RecursAtoD = Recv<Branching0fromDtoA, End>;

// B
enum Branching0fromDtoB {
    Forward(
        MeshedChannelsFour<
            Recv<(), End>,
            Send<(), End>,
            RecursBtoD,
            RoleA<RoleC<RoleD<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannelsFour<
            Send<(), End>,
            Recv<(), End>,
            RecursBtoD,
            RoleC<RoleA<RoleD<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsFour<End, End, End, RoleEnd, NameB>),
}
type RecursBtoD = Recv<Branching0fromDtoB, End>;

// C
enum Branching0fromDtoC {
    Forward(
        MeshedChannelsFour<
            End,
            Recv<(), End>,
            Send<(), RecursCtoD>,
            RoleB<RoleD<RoleD<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannelsFour<
            End,
            Send<(), End>,
            Recv<(), RecursCtoD>,
            RoleD<RoleB<RoleD<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsFour<End, End, End, RoleEnd, NameC>),
}
type RecursCtoD = Recv<Branching0fromDtoC, End>;

// D
type Choose0fromDtoA = Send<Branching0fromDtoA, End>;
type Choose0fromDtoB = Send<Branching0fromDtoB, End>;
type Choose0fromDtoC = Send<Branching0fromDtoC, End>;
type EndpointForwardD = MeshedChannelsFour<
    Choose0fromDtoA,
    Choose0fromDtoB,
    Recv<(), Choose0fromDtoC>,
    RoleC<RoleBroadcast>,
    NameD,
>;
type EndpointBackwardD = MeshedChannelsFour<
    Choose0fromDtoA,
    Choose0fromDtoB,
    Send<(), Choose0fromDtoC>,
    RoleC<RoleBroadcast>,
    NameD,
>;

// Creating the MP sessions
type EndpointA = MeshedChannelsFour<End, End, RecursAtoD, RoleD<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsFour<End, End, RecursBtoD, RoleD<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsFour<End, End, RecursCtoD, RoleD<RoleEnd>, NameC>;
type EndpointD =
    MeshedChannelsFour<Choose0fromDtoA, Choose0fromDtoB, Choose0fromDtoC, RoleBroadcast, NameD>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromDtoA::Done(s) => {
            s.close()
        },
        Branching0fromDtoA::Forward(s) => {
            let s = s.send(())?;
            endpoint_a(s)
        },
        Branching0fromDtoA::Backward(s) => {
            let (_, s) = s.recv()?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromDtoB::Done(s) => {
            s.close()
        },
        Branching0fromDtoB::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_b(s)
        },
        Branching0fromDtoB::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromDtoC::Done(s) => {
            s.close()
        },
        Branching0fromDtoC::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_c(s)
        },
        Branching0fromDtoC::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    recurs_d(s, LOOPS)
}

fn recurs_d(s: EndpointD, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_d_to_all!(
                s,
                Branching0fromDtoA::Done,
                Branching0fromDtoB::Done,
                Branching0fromDtoC::Done
            );

            s.close()
        }
        i if i % 2 == 0 => {
            let s: EndpointForwardD = choose_mpst_d_to_all!(
                s,
                Branching0fromDtoA::Forward,
                Branching0fromDtoB::Forward,
                Branching0fromDtoC::Forward
            );

            let (_, s) = s.recv()?;

            recurs_d(s, i - 1)
        }
        i => {
            let s: EndpointBackwardD = choose_mpst_d_to_all!(
                s,
                Branching0fromDtoA::Backward,
                Branching0fromDtoB::Backward,
                Branching0fromDtoC::Backward
            );

            let s = s.send(())?;

            recurs_d(s, i - 1)
        }
    }
}

fn main() {
    let (thread_a, thread_b, thread_c, thread_d) =
        fork_mpst(endpoint_a, endpoint_b, endpoint_c, endpoint_d);

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
}
