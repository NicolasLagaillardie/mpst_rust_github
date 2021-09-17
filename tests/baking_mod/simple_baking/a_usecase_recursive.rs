use rand::{thread_rng, Rng};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::functionmpst::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::broadcast::RoleBroadcast;

use std::boxed::Box;
use std::error::Error;

// Get roles
use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

use mpstthree::choose_mpst_a_to_all;
use mpstthree::offer_mpst_b_to_a;
use mpstthree::offer_mpst_c_to_a;

use petgraph::dot::Dot;

// Test our usecase
// Simple types
// Client = B → Y → A
// Authenticator = C → Z → B
// Server = A → X → C

type BtoAClose = End;
type BtoCClose = End;
type BtoCVideo = Send<i32, Recv<i32, End>>;
type BtoAVideo = Recv<i32, Send<i32, RecursBtoA>>;

type InitB = Recv<i32, Send<i32, RecursBtoA>>;

type CtoBClose = <BtoCClose as Session>::Dual;
type CtoAClose = End;
type CtoBVideo = <BtoCVideo as Session>::Dual;

type RecursBtoA = Recv<Branches0BtoA, End>;
type RecursCtoA = Recv<Branches0CtoA, End>;

enum Branches0BtoA {
    End(MeshedChannels<BtoAClose, BtoCClose, StackBEnd, RoleB<RoleEnd>>),
    Video(MeshedChannels<BtoAVideo, BtoCVideo, StackBVideo, RoleB<RoleEnd>>),
}
enum Branches0CtoA {
    End(MeshedChannels<CtoAClose, CtoBClose, StackCEnd, RoleC<RoleEnd>>),
    Video(MeshedChannels<RecursCtoA, CtoBVideo, StackCVideo, RoleC<RoleEnd>>),
}
type Choose0fromAtoB = Send<Branches0BtoA, End>;
type Choose0fromAtoC = Send<Branches0CtoA, End>;

type InitA = Send<i32, Recv<i32, Choose0fromAtoB>>;

// Stacks
type StackBEnd = RoleEnd;
type StackBVideo = RoleA<RoleC<RoleC<RoleA<RoleA<RoleEnd>>>>>;
type StackBRecurs = RoleA<RoleEnd>;
type StackBInit = RoleA<RoleA<RoleA<RoleEnd>>>;

type StackCEnd = RoleEnd;
type StackCVideo = RoleB<RoleB<RoleA<RoleEnd>>>;
type StackCRecurs = RoleA<RoleEnd>;

type StackARecurs = RoleBroadcast;
type StackAFull = RoleB<RoleB<StackARecurs>>;

// Creating the MP sessions

// For A
type EndpointAEnd = MeshedChannels<End, End, RoleEnd, RoleA<RoleEnd>>;
type EndpointAVideo = MeshedChannels<
    Send<i32, Recv<i32, Send<Branches0BtoA, End>>>,
    Send<Branches0CtoA, End>,
    RoleB<RoleB<RoleBroadcast>>,
    RoleA<RoleEnd>,
>;
type EndpointARecurs =
    MeshedChannels<Choose0fromAtoB, Choose0fromAtoC, StackARecurs, RoleA<RoleEnd>>;
type EndpointAFull = MeshedChannels<InitA, Choose0fromAtoC, StackAFull, RoleA<RoleEnd>>;

// For B
type EndpointBRecurs = MeshedChannels<RecursBtoA, End, StackBRecurs, RoleB<RoleEnd>>;
type EndpointBFull = MeshedChannels<InitB, End, StackBInit, RoleB<RoleEnd>>;

// For C
type EndpointCFull = MeshedChannels<RecursCtoA, End, StackCRecurs, RoleC<RoleEnd>>;

// Functions related to endpoints
fn server(s: EndpointCFull) -> Result<(), Box<dyn Error>> {
    offer_mpst_c_to_a!(s, {
        Branches0CtoA::End(s) => {
            s.close()
        },
        Branches0CtoA::Video(s) => {
            let (request, s) = s.recv()?;
            let s = s.send(request + 1);
            server(s)
        },
    })
}

fn authenticator(s: EndpointBFull) -> Result<(), Box<dyn Error>> {
    let (id, s) = s.recv()?;
    let s = s.send(id + 1);

    authenticator_recurs(s)
}

fn authenticator_recurs(s: EndpointBRecurs) -> Result<(), Box<dyn Error>> {
    offer_mpst_b_to_a!(s, {
        Branches0BtoA::End(s) => {
            s.close()
        },
        Branches0BtoA::Video(s) => {
            let (request, s) = s.recv()?;
            let (video, s) = s.send(request + 1).recv()?;
            let s = s.send(video + 1);
            authenticator_recurs(s)
        },
    })
}

fn client(s: EndpointAFull) -> Result<(), Box<dyn Error>> {
    let xs: Vec<i32> = (1..100).map(|_| thread_rng().gen()).collect();

    let (_, s) = s.send(0).recv()?;

    client_recurs(s, xs, 1)
}

fn client_recurs(s: EndpointARecurs, mut xs: Vec<i32>, index: i32) -> Result<(), Box<dyn Error>> {
    match xs.pop() {
        Option::Some(_) => {
            let s: EndpointAFull =
                choose_mpst_a_to_all!(s, Branches0BtoA::Video, Branches0CtoA::Video);

            let (_, s) = s.send(1).recv()?;

            client_recurs(s, xs, index + 1)
        }
        Option::None => {
            let s = choose_mpst_a_to_all!(s, Branches0BtoA::End, Branches0CtoA::End);

            assert_eq!(index, 100);

            s.close()
        }
    }
}

/////////////////////////////////////////

pub fn run_a_usecase_recursive() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(client, authenticator, server);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}

pub fn run_a_usecase_recursive_checker() {
    let graphs = mpstthree::checker_concat!(
        EndpointAFull,
        EndpointCFull,
        EndpointBFull
        =>
        [
            EndpointAVideo,
            Branches0BtoA, Video,
            Branches0CtoA, Video
        ],
        [
            EndpointAEnd,
            Branches0BtoA, End,
            Branches0CtoA, End
        ]
    )
    .unwrap();

    ////////////// Test graph A
    let graph_a = &graphs["RoleA"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_a)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"1\\\"\" ]\n    \
            2 [ label = \"\\\"2\\\"\" ]\n    \
            3 [ label = \"\\\"2.1\\\"\" ]\n    \
            4 [ label = \"\\\"2.1\\\"\" ]\n    \
            5 [ label = \"\\\"2.2\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleA!RoleB: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleA?RoleB: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"0\\\"\" ]\n    \
            2 -> 4 [ label = \"\\\"RoleA!RoleB: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleA?RoleB: i32\\\"\" ]\n    \
            5 -> 2 [ label = \"\\\"µ\\\"\" ]\n\
        }\n"
    );

    ////////////// Test graph B
    let graph_b = &graphs["RoleB"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_b)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"1\\\"\" ]\n    \
            2 [ label = \"\\\"2\\\"\" ]\n    \
            3 [ label = \"\\\"2.1\\\"\" ]\n    \
            4 [ label = \"\\\"2.1\\\"\" ]\n    \
            5 [ label = \"\\\"2.2\\\"\" ]\n    \
            6 [ label = \"\\\"2.3\\\"\" ]\n    \
            7 [ label = \"\\\"2.4\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleB?RoleA: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleB!RoleA: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"0\\\"\" ]\n    \
            2 -> 4 [ label = \"\\\"RoleB?RoleA: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleB!RoleC: i32\\\"\" ]\n    \
            5 -> 6 [ label = \"\\\"RoleB?RoleC: i32\\\"\" ]\n    \
            6 -> 7 [ label = \"\\\"RoleB!RoleA: i32\\\"\" ]\n    \
            7 -> 2 [ label = \"\\\"µ\\\"\" ]\n\
        }\n"
    );

    ////////////// Test graph C
    let graph_c = &graphs["RoleC"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_c)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"0.1\\\"\" ]\n    \
            2 [ label = \"\\\"0.1\\\"\" ]\n    \
            3 [ label = \"\\\"0.2\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 2 [ label = \"\\\"RoleC?RoleB: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleC!RoleB: i32\\\"\" ]\n    \
            3 -> 0 [ label = \"\\\"µ\\\"\" ]\n\
        }\n"
    );
}
