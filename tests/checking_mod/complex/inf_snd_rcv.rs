use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary::struct_trait::recv::Recv;
use mpstthree::binary::struct_trait::send::Send;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use mpstthree::{checker_concat, create_meshedchannels, create_multiple_normal_role};

use petgraph::dot::Dot;

use std::fs::read_to_string;

// Create new MeshedChannels
create_meshedchannels!(MeshedChannels, 2);

// Create new roles
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
);

// Payload names
type A = i32;
type B = i64;
type C = i32;
type D = i64;

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;

// Types

// A
type Recurs0BfromA = Recv<Branches0AfromB, End>;

enum Branches0AfromB {
    End(MeshedChannels<End, RoleEnd, NameA>),
    Looping(MeshedChannels<Send<A, Recurs0BfromA>, RoleB<RoleB<RoleEnd>>, NameA>),
    Extend(MeshedChannels<Send<B, Recurs1BfromA>, RoleB<RoleB<RoleEnd>>, NameA>),
}

type Recurs1BfromA = Recv<Branches1AfromB, End>;

enum Branches1AfromB {
    End(MeshedChannels<End, RoleEnd, NameA>),
    Looping(MeshedChannels<Recv<C, Recurs1BfromA>, RoleB<RoleB<RoleEnd>>, NameA>),
    Extend(MeshedChannels<Recv<D, End>, RoleB<RoleEnd>, NameA>),
}

// B
type Choose0fromBtoA = Send<Branches0AfromB, End>;

type Choose1fromBtoA = Send<Branches1AfromB, End>;

// Creating the MP sessions

// For A
type EndpointAFull = MeshedChannels<Recurs0BfromA, RoleB<RoleEnd>, NameA>;

// For B
type EndpointBEnd0 = MeshedChannels<End, RoleEnd, NameB>;
type EndpointBLooping0 = MeshedChannels<Send<C, Choose0fromBtoA>, RoleA<RoleBroadcast>, NameB>;
type EndpointBExtend0 = MeshedChannels<Send<D, Choose1fromBtoA>, RoleA<RoleBroadcast>, NameB>;
type EndpointBEnd1 = MeshedChannels<End, RoleEnd, NameB>;
type EndpointBLooping1 = MeshedChannels<Recv<A, Choose1fromBtoA>, RoleA<RoleBroadcast>, NameB>;
type EndpointBExtend1 = MeshedChannels<Recv<B, End>, RoleA<RoleEnd>, NameB>;
type EndpointBFull = MeshedChannels<Choose0fromBtoA, RoleBroadcast, NameB>;

/////////////////////////////////////////

pub fn main() {
    let graphs = checker_concat!(
        "inf_snd_rcv",
        1,
        2,
        EndpointAFull,
        EndpointBFull
        =>
        [
            EndpointBEnd0,
            Branches0AfromB, End
        ],
        [
            EndpointBLooping0,
            Branches0AfromB, Looping
        ],
        [
            EndpointBExtend0,
            Branches0AfromB, Extend
        ],
        [
            EndpointBEnd1,
            Branches1AfromB, End
        ],
        [
            EndpointBLooping1,
            Branches1AfromB, Looping
        ],
        [
            EndpointBExtend1,
            Branches1AfromB, Extend
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
            3 [ label = \"\\\"0.1.1\\\"\" ]\n    \
            4 [ label = \"\\\"0.1.1\\\"\" ]\n    \
            5 [ label = \"\\\"0.1.2\\\"\" ]\n    \
            6 [ label = \"\\\"0.1.1\\\"\" ]\n    \
            7 [ label = \"\\\"0.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 2 [ label = \"\\\"RoleA!RoleB: i64\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"0\\\"\" ]\n    \
            2 -> 4 [ label = \"\\\"RoleA?RoleB: i64\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"0\\\"\" ]\n    \
            2 -> 6 [ label = \"\\\"RoleA?RoleB: i32\\\"\" ]\n    \
            6 -> 2 [ label = \"\\\"µ\\\"\" ]\n    \
            0 -> 7 [ label = \"\\\"RoleA!RoleB: i32\\\"\" ]\n    \
            7 -> 0 [ label = \"\\\"µ\\\"\" ]\n\
        }\n"
    );

    ////////////// Test graph B
    let graph_b = &graphs["RoleB"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_b)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"0.1\\\"\" ]\n    \
            2 [ label = \"\\\"0.1\\\"\" ]\n    \
            3 [ label = \"\\\"0.1.1\\\"\" ]\n    \
            4 [ label = \"\\\"0.1.1\\\"\" ]\n    \
            5 [ label = \"\\\"0.1.2\\\"\" ]\n    \
            6 [ label = \"\\\"0.1.1\\\"\" ]\n    \
            7 [ label = \"\\\"0.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 2 [ label = \"\\\"RoleB!RoleA: i64\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"0\\\"\" ]\n    \
            2 -> 4 [ label = \"\\\"RoleB?RoleA: i64\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"0\\\"\" ]\n    \
            2 -> 6 [ label = \"\\\"RoleB?RoleA: i32\\\"\" ]\n    \
            6 -> 2 [ label = \"\\\"µ\\\"\" ]\n    \
            0 -> 7 [ label = \"\\\"RoleB!RoleA: i32\\\"\" ]\n    \
            7 -> 0 [ label = \"\\\"µ\\\"\" ]\n\
        }\n"
    );

    ////////////// Test KMC output
    assert_eq!(
        "CSA: \u{1b}[92mTrue\n\
        \u{1b}[0mBasic: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 1-exhaustive: \u{1b}[91mFalse\n\
        \u{1b}[0mreduced 1-safe: \u{1b}[91mFalse\n\
        \u{1b}[0mTraces violating progress: []\n\
        Traces violating eventual reception: \
        [0->1!i32<>; 1->0!i32<>, 0->1!i32<>; 1->0!i32<>]\n\n",
        read_to_string("outputs/inf_snd_rcv_1_kmc.txt").unwrap()
    );

    assert_eq!(
        "CSA: \u{1b}[92mTrue\n\
        \u{1b}[0mBasic: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 2-exhaustive: \u{1b}[91mFalse\n\
        \u{1b}[0mreduced 2-safe: \u{1b}[91mFalse\n\
        \u{1b}[0mTraces violating progress: []\n\
        Traces violating eventual reception: \
        [0->1!i32<>; 1->0!i32<>; 0->1!i32<>; 1->0!i32<>, \
        0->1!i32<>; 1->0!i32<>; 0->1!i32<>; 1->0!i32<>]\n\n",
        read_to_string("outputs/inf_snd_rcv_2_kmc.txt").unwrap()
    );
}
