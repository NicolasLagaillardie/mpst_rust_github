#![allow(dead_code, clippy::type_complexity)]

use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary::struct_trait::recv::Recv;
use mpstthree::binary::struct_trait::send::Send;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use mpstthree::{
    checker_concat, checker_concat_impl, create_meshedchannels, create_multiple_normal_name,
    create_multiple_normal_role,
};

use petgraph::dot::Dot;

use std::fs::read_to_string;

// Create new MeshedChannels
create_meshedchannels!(MeshedChannels, 2);

// Create new roles
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
);

// Create new names
create_multiple_normal_name!(NameA, NameB);

// Payload types
struct D0 {}
struct D1 {}
struct A0 {}
struct A1 {}

// Types

// A
type Choose0fromAtoB = Send<Branches0BfromA, End>;

type Recurs1BfromA = Recv<Branches1AfromB, End>;

enum Branches1AfromB {
    End(MeshedChannels<End, RoleEnd, NameA>),
    Looping(MeshedChannels<Recv<A1, Send<D0, Recurs1BfromA>>, RoleB<RoleB<RoleB<RoleEnd>>>, NameA>),
    Extend(MeshedChannels<Recv<A0, Send<D1, Recurs2BfromA>>, RoleB<RoleB<RoleB<RoleEnd>>>, NameA>),
}

type Recurs2BfromA = Recv<Branches2AfromB, End>;

enum Branches2AfromB {
    End(MeshedChannels<End, RoleEnd, NameA>),
    Looping(MeshedChannels<Recv<A0, Send<D1, Recurs2BfromA>>, RoleB<RoleB<RoleB<RoleEnd>>>, NameA>),
    Extend(MeshedChannels<Recv<A1, Choose0fromAtoB>, RoleB<RoleBroadcast>, NameA>),
}

// B
type Recurs0BfromA = Recv<Branches0BfromA, End>;

type Choose1fromBtoA = Send<Branches1AfromB, End>;

enum Branches0BfromA {
    End(MeshedChannels<End, RoleEnd, NameB>),
    Start(MeshedChannels<Recv<D0, Send<A0, Choose1fromBtoA>>, RoleA<RoleA<RoleBroadcast>>, NameB>),
    Stop(MeshedChannels<Recv<D1, End>, RoleA<RoleEnd>, NameB>),
}

// Creating the MP sessions

// For A
type EndpointAEnd = MeshedChannels<End, RoleEnd, NameA>;
type EndpointAStart = MeshedChannels<Send<D0, Recurs1BfromA>, RoleB<RoleB<RoleEnd>>, NameA>;
type EndpointAStop = MeshedChannels<End, RoleEnd, NameA>;
type EndpointAFull = MeshedChannels<Choose0fromAtoB, RoleBroadcast, NameA>;

// For B
type EndpointBEnd1 = MeshedChannels<End, RoleEnd, NameB>;
type EndpointBLooping1 =
    MeshedChannels<Recv<D0, Send<A0, Choose1fromBtoA>>, RoleA<RoleA<RoleBroadcast>>, NameB>;
type EndpointBExtend1 =
    MeshedChannels<Recv<D1, Send<A1, Recurs0BfromA>>, RoleA<RoleA<RoleA<RoleEnd>>>, NameB>;
type EndpointBEnd2 = MeshedChannels<End, RoleEnd, NameB>;
type EndpointBLooping2 = MeshedChannels<End, RoleEnd, NameB>;
type EndpointBExtend2 = MeshedChannels<End, RoleEnd, NameB>;
type EndpointBFull = MeshedChannels<Recurs0BfromA, RoleA<RoleEnd>, NameB>;

/////////////////////////////////////////

checker_concat_impl!(
    [Branches0BfromA, End],
    [Branches0BfromA, Start],
    [Branches0BfromA, Stop],
    [Branches1AfromB, End],
    [Branches1AfromB, Looping],
    [Branches1AfromB, Extend],
    [Branches2AfromB, End],
    [Branches2AfromB, Looping],
    [Branches2AfromB, Extend]
);

pub fn main() {
    let (graphs, kmc) = checker_concat!(
        "alternating_bit",
        EndpointAFull,
        EndpointBFull
        =>
        [
            EndpointAEnd,
            Branches0BfromA, End
        ],
        [
            EndpointAStart,
            Branches0BfromA, Start
        ],
        [
            EndpointAStop,
            Branches0BfromA, Stop
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
        ],
        [
            EndpointBEnd2,
            Branches2AfromB, End
        ],
        [
            EndpointBLooping2,
            Branches2AfromB, Looping
        ],
        [
            EndpointBExtend2,
            Branches2AfromB, Extend
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
            6 [ label = \"\\\"0.1.2.1\\\"\" ]\n    \
            7 [ label = \"\\\"0.1.2.1\\\"\" ]\n    \
            8 [ label = \"\\\"0.1.2.1\\\"\" ]\n    \
            9 [ label = \"\\\"0.1.2.2\\\"\" ]\n    \
            10 [ label = \"\\\"0.1.1\\\"\" ]\n    \
            11 [ label = \"\\\"0.1.2\\\"\" ]\n    \
            12 [ label = \"\\\"0.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 2 [ label = \"\\\"RoleA!RoleB: D0\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"0\\\"\" ]\n    \
            2 -> 4 [ label = \"\\\"RoleA?RoleB: A0\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleA!RoleB: D1\\\"\" ]\n    \
            5 -> 6 [ label = \"\\\"0\\\"\" ]\n    \
            5 -> 7 [ label = \"\\\"RoleA?RoleB: A1\\\"\" ]\n    \
            7 -> 0 [ label = \"\\\"µ\\\"\" ]\n    \
            5 -> 8 [ label = \"\\\"RoleA?RoleB: A0\\\"\" ]\n    \
            8 -> 9 [ label = \"\\\"RoleA!RoleB: D1\\\"\" ]\n    \
            9 -> 5 [ label = \"\\\"µ\\\"\" ]\n    \
            2 -> 10 [ label = \"\\\"RoleA?RoleB: A1\\\"\" ]\n    \
            10 -> 11 [ label = \"\\\"RoleA!RoleB: D0\\\"\" ]\n    \
            11 -> 2 [ label = \"\\\"µ\\\"\" ]\n    \
            0 -> 12 [ label = \"\\\"0\\\"\" ]\n\
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
            3 [ label = \"\\\"0.2\\\"\" ]\n    \
            4 [ label = \"\\\"0.2.1\\\"\" ]\n    \
            5 [ label = \"\\\"0.2.1\\\"\" ]\n    \
            6 [ label = \"\\\"0.2.2\\\"\" ]\n    \
            7 [ label = \"\\\"0.2.1\\\"\" ]\n    \
            8 [ label = \"\\\"0.2.2\\\"\" ]\n    \
            9 [ label = \"\\\"0.1\\\"\" ]\n    \
            10 [ label = \"\\\"0.2\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 2 [ label = \"\\\"RoleB?RoleA: D0\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleB!RoleA: A0\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"0\\\"\" ]\n    \
            3 -> 5 [ label = \"\\\"RoleB?RoleA: D1\\\"\" ]\n    \
            5 -> 6 [ label = \"\\\"RoleB!RoleA: A1\\\"\" ]\n    \
            6 -> 0 [ label = \"\\\"µ\\\"\" ]\n    \
            3 -> 7 [ label = \"\\\"RoleB?RoleA: D0\\\"\" ]\n    \
            7 -> 8 [ label = \"\\\"RoleB!RoleA: A0\\\"\" ]\n    \
            8 -> 3 [ label = \"\\\"µ\\\"\" ]\n    \
            0 -> 9 [ label = \"\\\"RoleB?RoleA: D1\\\"\" ]\n    \
            9 -> 10 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test KMC output
    assert_eq!(
        "CSA: \u{1b}[92mTrue\n\
        \u{1b}[0mBasic: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 1-exhaustive: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 1-safe: \u{1b}[92mTrue\n\
        \u{1b}[0m\n",
        read_to_string("outputs/alternating_bit_1_kmc.txt").unwrap()
    );

    ////////////// Test KMC number
    assert_eq!(kmc, Some(1));
}
