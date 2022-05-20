#![allow(dead_code)]

use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary::struct_trait::recv::Recv;
use mpstthree::binary::struct_trait::send::Send;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use mpstthree::{
    checker_concat, create_meshedchannels, create_multiple_normal_name, create_multiple_normal_role,
};

use petgraph::dot::Dot;

use std::fs::read_to_string;

// Create new MeshedChannels
create_meshedchannels!(MeshedChannels, 3);

// Create new roles
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
    RoleC, RoleCDual |
);

// Create new names
create_multiple_normal_name!(NameA, NameB, NameC);

// The new types

struct Haggle;
struct Price;
struct Happy;
struct Info;

// Types

// A
type Choose0fromAtoB = Send<Branches0BfromA, End>;
type Choose0fromAtoC = Send<Branches0CfromA, End>;

// B
type Recurs0BfromA = Recv<Branches0BfromA, End>;

enum Branches0BfromA {
    End(MeshedChannels<End, End, RoleEnd, NameB>),
    Haggle(
        MeshedChannels<
            Recv<Haggle, Send<Price, Recurs0BfromA>>,
            End,
            RoleA<RoleA<RoleA<RoleEnd>>>,
            NameB,
        >,
    ),
    Happy(MeshedChannels<Recv<Happy, End>, End, RoleA<RoleEnd>, NameB>),
}

// C
type Recurs0CfromA = Recv<Branches0CfromA, End>;

enum Branches0CfromA {
    End(MeshedChannels<End, End, RoleEnd, NameC>),
    Haggle(MeshedChannels<Recurs0CfromA, End, RoleA<RoleEnd>, NameC>),
    Happy(MeshedChannels<Recv<Info, End>, End, RoleA<RoleEnd>, NameC>),
}

// Creating the MP sessions

// For A
type EndpointAEnd = MeshedChannels<End, End, RoleEnd, NameA>;
type EndpointAHaggle = MeshedChannels<
    Send<Haggle, Recv<Price, Choose0fromAtoB>>,
    Choose0fromAtoC,
    RoleB<RoleB<RoleBroadcast>>,
    NameA,
>;
type EndpointAHappy =
    MeshedChannels<Send<Happy, End>, Send<Info, End>, RoleB<RoleC<RoleEnd>>, NameA>;
type EndpointAFull = MeshedChannels<Choose0fromAtoB, Choose0fromAtoC, RoleBroadcast, NameA>;

// For B
type EndpointBFull = MeshedChannels<Recurs0BfromA, End, RoleA<RoleEnd>, NameB>;

// For C
type EndpointCFull = MeshedChannels<Recurs0CfromA, End, RoleA<RoleEnd>, NameC>;

/////////////////////////////////////////

pub fn main() {
    let (graphs, kmc) = checker_concat!(
        "bargain",
        EndpointAFull,
        EndpointBFull,
        EndpointCFull,
        =>
        [
            EndpointAHappy,
            Branches0BfromA, Happy,
            Branches0CfromA, Happy
        ],
        [
            EndpointAHaggle,
            Branches0BfromA, Haggle,
            Branches0CfromA, Haggle
        ],
        [
            EndpointAEnd,
            Branches0BfromA, End,
            Branches0CfromA, End
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
            4 [ label = \"\\\"0.1\\\"\" ]\n    \
            5 [ label = \"\\\"0.2\\\"\" ]\n    \
            6 [ label = \"\\\"0.3\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 2 [ label = \"\\\"RoleA!RoleB: Haggle\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleA?RoleB: Price\\\"\" ]\n    \
            3 -> 0 [ label = \"\\\"µ\\\"\" ]\n    \
            0 -> 4 [ label = \"\\\"RoleA!RoleB: Happy\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleA!RoleC: Info\\\"\" ]\n    \
            5 -> 6 [ label = \"\\\"0\\\"\" ]\n\
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
            4 [ label = \"\\\"0.1\\\"\" ]\n    \
            5 [ label = \"\\\"0.2\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 2 [ label = \"\\\"RoleB?RoleA: Haggle\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleB!RoleA: Price\\\"\" ]\n    \
            3 -> 0 [ label = \"\\\"µ\\\"\" ]\n    \
            0 -> 4 [ label = \"\\\"RoleB?RoleA: Happy\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"0\\\"\" ]\n\
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
            0 -> 2 [ label = \"\\\"RoleC?RoleA: Info\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test KMC output
    assert_eq!(
        "CSA: \u{1b}[92mTrue\n\
        \u{1b}[0mBasic: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 1-exhaustive: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 1-safe: \u{1b}[92mTrue\n\
        \u{1b}[0m\n",
        read_to_string("outputs/bargain_1_kmc.txt").unwrap()
    );

    ////////////// Test KMC number
    assert_eq!(kmc, Some(1));
}
