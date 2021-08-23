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

use mpstthree::choose_mpst_c_to_all;
use mpstthree::offer_mpst_a_to_c;
use mpstthree::offer_mpst_b_to_c;

use petgraph::dot::Dot;

/// Test our usecase
/// Simple types
/// Client = C
/// Authenticator = A
/// Server = B

type AtoCClose = End;
type AtoBClose = End;
type AtoBVideo = Send<i32, Recv<i32, End>>;
type AtoCVideo = Recv<i32, Send<i32, RecursAtoC>>;

type InitA = Recv<i32, Send<i32, RecursAtoC>>;

type BtoAClose = <AtoBClose as Session>::Dual;
type BtoCClose = End;
type BtoAVideo = <AtoBVideo as Session>::Dual;

type RecursAtoC = Recv<Branches0AtoC, End>;
type RecursBtoC = Recv<Branches0BtoC, End>;

enum Branches0AtoC {
    End(MeshedChannels<AtoBClose, AtoCClose, StackAEnd, RoleA<RoleEnd>>),
    Video(MeshedChannels<AtoBVideo, AtoCVideo, StackAVideo, RoleA<RoleEnd>>),
}
enum Branches0BtoC {
    End(MeshedChannels<BtoAClose, BtoCClose, StackBEnd, RoleB<RoleEnd>>),
    Video(MeshedChannels<BtoAVideo, RecursBtoC, StackBVideo, RoleB<RoleEnd>>),
}
type Choose0fromCtoA = Send<Branches0AtoC, End>;
type Choose0fromCtoB = Send<Branches0BtoC, End>;

type InitC = Send<i32, Recv<i32, Choose0fromCtoA>>;

/// Stacks
type StackAEnd = RoleEnd;
type StackAVideo = RoleC<RoleB<RoleB<RoleC<RoleC<RoleEnd>>>>>;
type StackARecurs = RoleC<RoleEnd>;
type StackAInit = RoleC<RoleC<RoleC<RoleEnd>>>;

type StackBEnd = RoleEnd;
type StackBVideo = RoleA<RoleA<RoleC<RoleEnd>>>;
type StackBRecurs = RoleC<RoleEnd>;

type StackCRecurs = RoleBroadcast;
type StackCFull = RoleA<RoleA<StackCRecurs>>;

/// Creating the MP sessions

/// For C
type EndpointCEnd = MeshedChannels<End, End, RoleEnd, RoleC<RoleEnd>>;
type EndpointCVideo = MeshedChannels<
    Send<i32, Recv<i32, Send<Branches0AtoC, End>>>,
    Send<Branches0BtoC, End>,
    RoleA<RoleA<RoleBroadcast>>,
    RoleC<RoleEnd>,
>;
type EndpointCRecurs =
    MeshedChannels<Choose0fromCtoA, Choose0fromCtoB, StackCRecurs, RoleC<RoleEnd>>;
type EndpointCFull = MeshedChannels<InitC, Choose0fromCtoB, StackCFull, RoleC<RoleEnd>>;

/// For A
type EndpointARecurs = MeshedChannels<End, RecursAtoC, StackARecurs, RoleA<RoleEnd>>;
type EndpointAFull = MeshedChannels<End, InitA, StackAInit, RoleA<RoleEnd>>;

/// For B
type EndpointBFull = MeshedChannels<End, RecursBtoC, StackBRecurs, RoleB<RoleEnd>>;

/// Functions related to endpoints
fn server(s: EndpointBFull) -> Result<(), Box<dyn Error>> {
    offer_mpst_b_to_c!(s, {
        Branches0BtoC::End(s) => {
            s.close()
        },
        Branches0BtoC::Video(s) => {
            let (request, s) = s.recv()?;
            let s = s.send(request + 1);
            server(s)
        },
    })
}

fn authenticator(s: EndpointAFull) -> Result<(), Box<dyn Error>> {
    let (id, s) = s.recv()?;
    let s = s.send(id + 1);

    authenticator_recurs(s)
}

fn authenticator_recurs(s: EndpointARecurs) -> Result<(), Box<dyn Error>> {
    offer_mpst_a_to_c!(s, {
        Branches0AtoC::End(s) => {
            s.close()
        },
        Branches0AtoC::Video(s) => {
            let (request, s) = s.recv()?;
            let (video, s) = s.send(request + 1).recv()?;
            let s = s.send(video + 1);
            authenticator_recurs(s)
        },
    })
}

fn client(s: EndpointCFull) -> Result<(), Box<dyn Error>> {
    let xs: Vec<i32> = (1..100).map(|_| thread_rng().gen()).collect();

    let (_, s) = s.send(0).recv()?;

    client_recurs(s, xs, 1)
}

fn client_recurs(s: EndpointCRecurs, mut xs: Vec<i32>, index: i32) -> Result<(), Box<dyn Error>> {
    match xs.pop() {
        Option::Some(_) => {
            let s: EndpointCFull =
                choose_mpst_c_to_all!(s, Branches0AtoC::Video, Branches0BtoC::Video);

            let (_, s) = s.send(1).recv()?;

            client_recurs(s, xs, index + 1)
        }
        Option::None => {
            let s = choose_mpst_c_to_all!(s, Branches0AtoC::End, Branches0BtoC::End);

            assert_eq!(index, 100);

            s.close()
        }
    }
}

/////////////////////////////////////////

pub fn run_c_usecase_recursive() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}

pub fn run_c_usecase_recursive_checker() {
    let graphs = mpstthree::checker_concat!(
        EndpointAFull,
        EndpointCFull,
        EndpointBFull
        =>
        [
            EndpointCVideo,
            Branches0AtoC, Video,
            Branches0BtoC, Video
        ],
        [
            EndpointCEnd,
            Branches0AtoC, End,
            Branches0BtoC, End
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
            3 [ label = \"\\\"2.0\\\"\" ]\n    \
            4 [ label = \"\\\"2.1\\\"\" ]\n    \
            5 [ label = \"\\\"2.1\\\"\" ]\n    \
            6 [ label = \"\\\"2.2\\\"\" ]\n    \
            7 [ label = \"\\\"2.3\\\"\" ]\n    \
            8 [ label = \"\\\"2.4\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleA?RoleC: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleA!RoleC: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"& RoleC\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"0\\\"\" ]\n    \
            3 -> 5 [ label = \"\\\"RoleA?RoleC: i32\\\"\" ]\n    \
            5 -> 6 [ label = \"\\\"RoleA!RoleB: i32\\\"\" ]\n    \
            6 -> 7 [ label = \"\\\"RoleA?RoleB: i32\\\"\" ]\n    \
            7 -> 8 [ label = \"\\\"RoleA!RoleC: i32\\\"\" ]\n    \
            8 -> 3 [ label = \"\\\"µ\\\"\" ]\n\
        }\n"
    );

    ////////////// Test graph B
    let graph_b = &graphs["RoleB"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_b)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"0.0\\\"\" ]\n    \
            2 [ label = \"\\\"0.1\\\"\" ]\n    \
            3 [ label = \"\\\"0.1\\\"\" ]\n    \
            4 [ label = \"\\\"0.2\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"& RoleC\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"0\\\"\" ]\n    \
            1 -> 3 [ label = \"\\\"RoleB?RoleA: i32\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"RoleB!RoleA: i32\\\"\" ]\n    \
            4 -> 1 [ label = \"\\\"µ\\\"\" ]\n\
        }\n"
    );

    ////////////// Test graph C
    let graph_c = &graphs["RoleC"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_c)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"1\\\"\" ]\n    \
            2 [ label = \"\\\"2\\\"\" ]\n    \
            3 [ label = \"\\\"2.0\\\"\" ]\n    \
            4 [ label = \"\\\"2.1\\\"\" ]\n    \
            5 [ label = \"\\\"2.1\\\"\" ]\n    \
            6 [ label = \"\\\"2.2\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleC!RoleA: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleC?RoleA: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"+ RoleC\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"0\\\"\" ]\n    \
            3 -> 5 [ label = \"\\\"RoleC!RoleA: i32\\\"\" ]\n    \
            5 -> 6 [ label = \"\\\"RoleC?RoleA: i32\\\"\" ]\n    \
            6 -> 3 [ label = \"\\\"µ\\\"\" ]\n\
        }\n"
    );
}