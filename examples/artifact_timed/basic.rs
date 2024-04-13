use mpstthree::binary::struct_trait::end::End; // The basic End types
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed}; // The basic timed types
use mpstthree::generate_timed; /* The macro for generating the roles and
                                * the MeshedChannels */
use mpstthree::role::broadcast::RoleBroadcast; /* Optional: used only for protocols with
                                                * choice/offer */
use mpstthree::role::end::RoleEnd; // The final type for the stacks and the names of the roles

use std::collections::HashMap; // Used for storing clocks
use std::error::Error; // Used for functions returning _affine_ types
use std::time::Instant; // Used for clocks

// use std::thread::sleep; // Used for pausing a thread
// use std::time::Duration; // Used for declaring the duration for sleep

generate_timed!(MeshedChannels, A, B); // generates meshed channels for 3 roles

// Payload types
struct Request;
struct Response;
struct Stop;

// Binary types for A
type StartA0 = RecvTimed<
    Request,
    'a',
    0,
    true,
    1,
    true,
    ' ',
    SendTimed<Branching0fromAtoB, 'a', 0, true, 1, true, ' ', End>,
>; // RecvTimed a Request then SendTimed a choice
type OrderingA0 = RoleB<RoleBroadcast>; // Stack for recv then sending a choice

type LoopA0 = SendTimed<Branching0fromAtoB, 'a', 0, true, 1, true, ' ', End>; // SendTimed a choice
type OrderingLoopA0 = RoleBroadcast; // Stack for sending a choice

type MoreA1 = SendTimed<
    Response,
    'a',
    0,
    true,
    1,
    true,
    ' ',
    SendTimed<Branching0fromAtoB, 'a', 0, true, 1, true, ' ', End>,
>; // RecvTimed Response then send a choice
type OrderingMoreA1 = RoleB<RoleBroadcast>; // Stack for the previous binary type

type DoneA1 = SendTimed<Stop, 'a', 0, true, 1, true, ' ', End>; // RecvTimed Stop
type OrderingDoneA1 = RoleB<RoleEnd>; // Stack for the previous binary type

// Binary types for B
type StartB0 = SendTimed<
    Request,
    'a',
    0,
    true,
    1,
    true,
    ' ',
    RecvTimed<Branching0fromAtoB, 'a', 0, true, 1, true, ' ', End>,
>; // SendTimed a Request then RecvTimed a choice
type OrderingB0 = RoleA<RoleA<RoleEnd>>; // Stack for send then receiving a choice from A

type LoopB0 = RecvTimed<Branching0fromAtoB, 'a', 0, true, 1, true, ' ', End>; // RecvTimed a choice
type OrderingLoopB0 = RoleA<RoleEnd>; // Stack for recv a choice

type MoreB1 = RecvTimed<
    Response,
    'a',
    0,
    true,
    1,
    true,
    ' ',
    RecvTimed<Branching0fromAtoB, 'a', 0, true, 1, true, ' ', End>,
>; // RecvTimed Request then SendTimed Response then receive a choice
type OrderingMoreB1 = RoleA<RoleA<RoleEnd>>; // Stack for the previous binary type

type DoneB1 = RecvTimed<Stop, 'a', 0, true, 1, true, ' ', End>; // SendTimed Stop
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

fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (_, s) = s.recv(all_clocks)?;

    recurs_a(s, 5, all_clocks)
}

fn recurs_a(
    s: EndpointALoop,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    if loops > 0 {
        let s: EndpointAMore = choose_mpst_a_to_all!(s, all_clocks, Branching0fromAtoB::More);

        let s = s.send(Response {}, all_clocks)?;

        recurs_a(s, loops - 1, all_clocks)
    } else {
        let s: EndpointADone = choose_mpst_a_to_all!(s, all_clocks, Branching0fromAtoB::Done);

        let s = s.send(Stop {}, all_clocks)?;

        s.close()
    }
}

fn endpoint_b(s: EndpointB, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    // sleep(Duration::from_secs(2));

    let s = s.send(Request {}, all_clocks)?;

    recurs_b(s, all_clocks)
}

fn recurs_b(
    s: EndpointBLoop,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branching0fromAtoB::More(s) => {
            let (_, s) = s.recv(all_clocks)?;

            recurs_b(s, all_clocks)
        },
        Branching0fromAtoB::Done(s) => {
            let (_, s) = s.recv(all_clocks)?;

            s.close()
        },
    })
}

fn main() {
    let (thread_a, thread_b) = fork_mpst(endpoint_a, endpoint_b);

    println!("Thread a: {:?}", thread_a.join());
    println!("Thread b: {:?}", thread_b.join());
}
