use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send}; // The basic types
use mpstthree::checker_concat;
use mpstthree::generate; /* The macro for generating the roles and the MeshedChannels */
use mpstthree::role::broadcast::RoleBroadcast; /* Optional: used only for protocols with
                                                * choice/offer */
use mpstthree::role::end::RoleEnd; // The final type for the stacks and the names of the roles // Used for checking the protocol

use std::error::Error;

// Create new MeshedChannels for two participants
generate!("rec_and_cancel", MeshedChannels, A, B);

// Payload types
struct Request;
struct Response;
struct Stop;

// Types
// Binary types for A
type StartA0 = Recv<Request, Send<Branching0fromAtoB, End>>; // Recv a Request then Send a choice
type OrderingA0 = RoleB<RoleBroadcast>; // Stack for recv then sending a choice

type LoopA0 = Send<Branching0fromAtoB, End>; // Send a choice
type OrderingLoopA0 = RoleBroadcast; // Stack for sending a choice

type MoreA1 = Recv<Response, Send<Branching0fromAtoB, End>>; // Recv Response then send a choice
type OrderingMoreA1 = RoleB<RoleBroadcast>; // Stack for the previous binary type

type DoneA1 = Recv<Stop, End>; // Recv Stop
type OrderingDoneA1 = RoleB<RoleEnd>; // Stack for the previous binary type

// Binary types for B
type StartB0 = Send<Request, Recv<Branching0fromAtoB, End>>; // Send a Request then Recv a choice
type OrderingB0 = RoleA<RoleA<RoleEnd>>; // Stack for send then receiving a choice from A

type LoopB0 = Recv<Branching0fromAtoB, End>; // Recv a choice
type OrderingLoopB0 = RoleA<RoleEnd>; // Stack for recv a choice

type MoreB1 = Send<Response, Recv<Branching0fromAtoB, End>>; // Recv Request then Send Response then receive a choice
type OrderingMoreB1 = RoleA<RoleA<RoleEnd>>; // Stack for the previous binary type

type DoneB1 = Send<Stop, End>; // Send Stop
type OrderingDoneB1 = RoleA<RoleEnd>; // Stack for the previous binary type

enum Branching0fromAtoB {
    // Sum type containing the different paths of the choice
    More(MeshedChannels<MoreB1, OrderingMoreB1, NameB>),
    Done(MeshedChannels<DoneB1, OrderingDoneB1, NameB>),
}

// Creating the endpoints
// A
type EndpointAMore = MeshedChannels<MoreA1, OrderingMoreA1, NameA>;
type EndpointADone = MeshedChannels<DoneA1, OrderingDoneA1, NameA>;
type EndpointALoop = MeshedChannels<LoopA0, OrderingLoopA0, NameA>;
type EndpointA = MeshedChannels<StartA0, OrderingA0, NameA>;

// B
type EndpointBLoop = MeshedChannels<LoopB0, OrderingLoopB0, NameB>;
type EndpointB = MeshedChannels<StartB0, OrderingB0, NameB>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv()?;
    recurs_a(s, 5)
}

fn recurs_a(s: EndpointALoop, loops: i32) -> Result<(), Box<dyn Error>> {
    if loops > 0 {
        let s: EndpointAMore = choose_mpst_a_to_all!(s, Branching0fromAtoB::More);

        let (_, s) = s.recv()?;
        recurs_a(s, loops - 1)
    } else {
        let s: EndpointADone = choose_mpst_a_to_all!(s, Branching0fromAtoB::Done);

        let (_, s) = s.recv()?;
        s.close()
    }
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    let s = s.send(Request {})?;
    recurs_b(s)
}

fn recurs_b(s: EndpointBLoop) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromAtoB::More(s) => {
            let s = s.send(Response {})?;
            recurs_b(s)
        },
        Branching0fromAtoB::Done(s) => {
            let s = s.send(Stop {})?;
            s.close()
        },
    })
}

fn main() {
    let (thread_a, thread_b) = fork_mpst(endpoint_a, endpoint_b);

    thread_a.join().unwrap();
    thread_b.join().unwrap();

    let (_, _kmc) = checker_concat!(
        "basic",
        EndpointA,
        EndpointB
        =>
        [
            EndpointAMore,
            Branching0fromAtoB, More,
        ],
        [
            EndpointADone,
            Branching0fromAtoB, Done,
        ]
    )
    .unwrap();

    // println!("min kMC: {kmc:?}");
}
