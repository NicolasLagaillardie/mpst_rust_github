use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::broadcast::RoleBroadcast;

use petgraph::dot::Dot;

// Get roles
use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

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
type StackCInit = RoleB<RoleB<RoleB<RoleEnd>>>;

type StackAEnd = RoleEnd;
type StackAVideo = RoleC<RoleC<RoleB<RoleEnd>>>;
type StackARecurs = RoleB<RoleEnd>;

type StackBRecurs = RoleBroadcast;
type StackBFull = RoleC<RoleC<StackBRecurs>>;

/// Creating the MP sessions

/// For B
type EndpointBEnd = MeshedChannels<End, End, RoleEnd, RoleB<RoleEnd>>;
type EndpointBFull = MeshedChannels<Choose0fromBtoA, InitB, StackBFull, RoleB<RoleEnd>>;

/// For C
type EndpointCFull = MeshedChannels<End, InitC, StackCInit, RoleC<RoleEnd>>;

/// For A
type EndpointARecurs = MeshedChannels<RecursAtoB, End, StackARecurs, RoleA<RoleEnd>>;

/////////////////////////////////////////

pub fn main() {
    let graphs = mpstthree::checker_concat!(
        EndpointARecurs,
        EndpointCFull,
        EndpointBFull
        =>
        [
            EndpointBFull,
            Branches0AtoB, Video,
            Branches0CtoB, Video
        ],
        [
            EndpointBEnd,
            Branches0AtoB, End,
            Branches0CtoB, End
        ]
    )
    .unwrap();

    ////////////// Test graph A
    let graph_a = &graphs["RoleA"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_a)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"0.1\\\"\" ]\n    \
            2 [ label = \"\\\"0.1\\\"\" ]\n    \
            3 [ label = \"\\\"0.2\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 2 [ label = \"\\\"RoleA?RoleC: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleA!RoleC: i32\\\"\" ]\n    \
            3 -> 0 [ label = \"\\\"µ\\\"\" ]\n\
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
            0 -> 1 [ label = \"\\\"RoleB!RoleC: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleB?RoleC: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"0\\\"\" ]\n    \
            2 -> 4 [ label = \"\\\"RoleB!RoleC: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleB?RoleC: i32\\\"\" ]\n    \
            5 -> 2 [ label = \"\\\"µ\\\"\" ]\n\
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
            3 [ label = \"\\\"2.1\\\"\" ]\n    \
            4 [ label = \"\\\"2.1\\\"\" ]\n    \
            5 [ label = \"\\\"2.2\\\"\" ]\n    \
            6 [ label = \"\\\"2.3\\\"\" ]\n    \
            7 [ label = \"\\\"2.4\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleC?RoleB: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleC!RoleB: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"0\\\"\" ]\n    \
            2 -> 4 [ label = \"\\\"RoleC?RoleB: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleC!RoleA: i32\\\"\" ]\n    \
            5 -> 6 [ label = \"\\\"RoleC?RoleA: i32\\\"\" ]\n    \
            6 -> 7 [ label = \"\\\"RoleC!RoleB: i32\\\"\" ]\n    \
            7 -> 2 [ label = \"\\\"µ\\\"\" ]\n\
        }\n"
    );
}
