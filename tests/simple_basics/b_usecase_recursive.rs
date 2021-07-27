use rand::{thread_rng, Rng};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::broadcast::RoleBroadcast;

use std::boxed::Box;
use std::error::Error;

use mpstthree::functionmpst::close::close_mpst;

// Get roles
use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

// Get recv functions
use mpstthree::functionmpst::recv::recv_mpst_a_from_c;
use mpstthree::functionmpst::recv::recv_mpst_b_from_c;
use mpstthree::functionmpst::recv::recv_mpst_c_from_a;
use mpstthree::functionmpst::recv::recv_mpst_c_from_b;

// Get send functions
use mpstthree::functionmpst::send::send_mpst_a_to_c;
use mpstthree::functionmpst::send::send_mpst_b_to_c;
use mpstthree::functionmpst::send::send_mpst_c_to_a;
use mpstthree::functionmpst::send::send_mpst_c_to_b;

use mpstthree::choose_mpst_b_to_all;
use mpstthree::offer_mpst_a_to_b;
use mpstthree::offer_mpst_c_to_b;

/// Test our usecase
/// Simple types
/// Client = B
/// Authenticator = C
/// Server = A

type CtoBClose = End;
type CtoAClose = End;
type CtoAVideo = Send<i32, Recv<i32, End>>;
type CtoBVideo = Recv<i32, Send<i32, RecursCtoB>>;

type InitC = Recv<i32, Send<i32, RecursCtoB>>;

type AtoCClose = <CtoAClose as Session>::Dual;
type AtoBClose = End;
type AtoCVideo = <CtoAVideo as Session>::Dual;

type RecursCtoB = Recv<Branches0CtoB, End>;
type RecursAtoB = Recv<Branches0AtoB, End>;

enum Branches0CtoB {
    End(MeshedChannels<CtoAClose, CtoBClose, StackCEnd, RoleC<RoleEnd>>),
    Video(MeshedChannels<CtoAVideo, CtoBVideo, StackCVideo, RoleC<RoleEnd>>),
}
enum Branches0AtoB {
    End(MeshedChannels<AtoBClose, AtoCClose, StackAEnd, RoleA<RoleEnd>>),
    Video(MeshedChannels<RecursAtoB, AtoCVideo, StackAVideo, RoleA<RoleEnd>>),
}
type Choose0fromBtoC = Send<Branches0CtoB, End>;
type Choose0fromBtoA = Send<Branches0AtoB, End>;

type InitB = Send<i32, Recv<i32, Choose0fromBtoC>>;

/// Stacks
type StackCEnd = RoleEnd;
type StackCVideo = RoleB<RoleA<RoleA<RoleB<RoleB<RoleEnd>>>>>;
type StackCRecurs = RoleB<RoleEnd>;
type StackCInit = RoleB<RoleB<RoleB<RoleEnd>>>;

type StackAEnd = RoleEnd;
type StackAVideo = RoleC<RoleC<RoleB<RoleEnd>>>;
type StackARecurs = RoleB<RoleEnd>;

type StackBRecurs = RoleBroadcast;
type StackBFull = RoleC<RoleC<StackBRecurs>>;

/// Creating the MP sessions

/// For B
type EndpointBEnd = MeshedChannels<End, End, RoleEnd, RoleB<RoleEnd>>;
type EndpointBVideo = MeshedChannels<
    Send<Branches0AtoB, End>,
    Send<i32, Recv<i32, Send<Branches0CtoB, End>>>,
    RoleC<RoleC<RoleBroadcast>>,
    RoleB<RoleEnd>,
>;
type EndpointBRecurs =
    MeshedChannels<Choose0fromBtoA, Choose0fromBtoC, StackBRecurs, RoleB<RoleEnd>>;
type EndpointBFull = MeshedChannels<Choose0fromBtoA, InitB, StackBFull, RoleB<RoleEnd>>;

/// For C
type EndpointCRecurs = MeshedChannels<End, RecursCtoB, StackCRecurs, RoleC<RoleEnd>>;
type EndpointCFull = MeshedChannels<End, InitC, StackCInit, RoleC<RoleEnd>>;

/// For A
type EndpointAFull = MeshedChannels<RecursAtoB, End, StackARecurs, RoleA<RoleEnd>>;

/// Functions related to endpoints
fn server(s: EndpointAFull) -> Result<(), Box<dyn Error>> {
    offer_mpst_a_to_b!(s, {
        Branches0AtoB::End(s) => {
            close_mpst(s)
        },
        Branches0AtoB::Video(s) => {
            let (request, s) = recv_mpst_a_from_c(s)?;
            let s = send_mpst_a_to_c(request + 1, s);
            server(s)
        },
    })
}

fn authenticator(s: EndpointCFull) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_c_from_b(s)?;
    let s = send_mpst_c_to_b(id + 1, s);

    authenticator_recurs(s)
}

fn authenticator_recurs(s: EndpointCRecurs) -> Result<(), Box<dyn Error>> {
    offer_mpst_c_to_b!(s, {
        Branches0CtoB::End(s) => {
            close_mpst(s)
        },
        Branches0CtoB::Video(s) => {
            let (request, s) = recv_mpst_c_from_b(s)?;
            let s = send_mpst_c_to_a(request + 1, s);
            let (video, s) = recv_mpst_c_from_a(s)?;
            let s = send_mpst_c_to_b(video + 1, s);
            authenticator_recurs(s)
        },
    })
}

fn client(s: EndpointBFull) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();

    let s = send_mpst_b_to_c(0, s);
    let (_, s) = recv_mpst_b_from_c(s)?;

    client_recurs(s, xs, 1)
}

fn client_recurs(s: EndpointBRecurs, mut xs: Vec<i32>, index: i32) -> Result<(), Box<dyn Error>> {
    match xs.pop() {
        Option::Some(_) => {
            let s = choose_mpst_b_to_all!(s, Branches0AtoB::Video, Branches0CtoB::Video);

            let s = send_mpst_b_to_c(1, s);
            let (_, s) = recv_mpst_b_from_c(s)?;

            client_recurs(s, xs, index + 1)
        }
        Option::None => {
            let s = choose_mpst_b_to_all!(s, Branches0AtoB::End, Branches0CtoB::End);

            assert_eq!(index, 100);

            close_mpst(s)
        }
    }
}

/////////////////////////////////////////

pub fn run_b_usecase_recursive() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(server, client, authenticator);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}

pub fn run_b_usecase_recursive_checker() {
    let graphs = mpstthree::checker_concat!(
        EndpointAFull,
        EndpointCFull,
        EndpointBFull
        =>
        [
            EndpointBVideo,
            Branches0CtoB, Video,
            Branches0AtoB, Video
        ],
        [
            EndpointBEnd,
            Branches0CtoB, End,
            Branches0AtoB, End
        ]
    )
    .unwrap();

    ////////////// Test graph A
    let graph_a = &graphs["RoleA"];

    assert_eq!(graph_a.node_count(), 5);
    assert_eq!(graph_a.edge_count(), 5);

    ////////////// Test graph B
    let graph_b = &graphs["RoleB"];

    assert_eq!(graph_b.node_count(), 7);
    assert_eq!(graph_b.edge_count(), 7);

    ////////////// Test graph C
    let graph_c = &graphs["RoleC"];

    assert_eq!(graph_c.node_count(), 9);
    assert_eq!(graph_c.edge_count(), 9);
}