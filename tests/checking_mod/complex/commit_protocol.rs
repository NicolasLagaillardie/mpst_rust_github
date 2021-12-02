use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary::struct_trait::recv::Recv;
use mpstthree::binary::struct_trait::send::Send;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use mpstthree::{checker_concat, create_meshedchannels, create_multiple_normal_role};

use petgraph::dot::Dot;

use std::fs::read_to_string;

// Create new MeshedChannels
create_meshedchannels!(MeshedChannels, 4);

// Create new roles
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
    RoleC, RoleCDual |
    RoleD, RoleDDual |
);

// Payload types
struct Update;
struct Okay;

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;

// Types

// A
type Choose0fromAtoB = Send<Branches0BfromA, End>;
type Choose0fromAtoC = Send<Branches0CfromA, End>;
type Choose0fromAtoD = Send<Branches0DfromA, End>;

type Choose1fromAtoB = Send<Branches1BfromA, End>;
type Choose1fromAtoC = Send<Branches1CfromA, End>;
type Choose1fromAtoD = Send<Branches1DfromA, End>;

// B
type Recurs0BfromA = Recv<Branches0BfromA, End>;

enum Branches0BfromA {
    End(MeshedChannels<End, End, End, RoleEnd, NameB>),
    CommitForward(
        MeshedChannels<Send<Update, Recurs1BfromA>, End, End, RoleA<RoleA<RoleEnd>>, NameB>,
    ),
}

type Recurs1BfromA = Recv<Branches1BfromA, End>;

enum Branches1BfromA {
    CommitBackward(
        MeshedChannels<Recv<Okay, Recurs0BfromA>, End, End, RoleA<RoleA<RoleEnd>>, NameB>,
    ),
}

// C
type Recurs0CfromA = Recv<Branches0CfromA, End>;

enum Branches0CfromA {
    End(MeshedChannels<End, End, End, RoleEnd, NameC>),
    CommitForward(
        MeshedChannels<Recv<Update, Recurs1CfromA>, End, End, RoleA<RoleA<RoleEnd>>, NameC>,
    ),
}

type Recurs1CfromA = Recv<Branches1CfromA, End>;

enum Branches1CfromA {
    CommitBackward(
        MeshedChannels<Send<Okay, Recurs0CfromA>, End, End, RoleA<RoleA<RoleEnd>>, NameC>,
    ),
}

// D
type Recurs0DfromA = Recv<Branches0DfromA, End>;

enum Branches0DfromA {
    End(MeshedChannels<End, End, End, RoleEnd, NameD>),
    CommitForward(
        MeshedChannels<Recv<Update, Recurs1DfromA>, End, End, RoleA<RoleA<RoleEnd>>, NameD>,
    ),
}

type Recurs1DfromA = Recv<Branches1DfromA, End>;

enum Branches1DfromA {
    CommitBackward(
        MeshedChannels<Send<Okay, Recurs0DfromA>, End, End, RoleA<RoleA<RoleEnd>>, NameD>,
    ),
}

// Creating the MP sessions

// For A
type EndpointAEnd = MeshedChannels<End, End, End, RoleEnd, NameA>;
type EndpointACommitForward = MeshedChannels<
    Recv<Update, Choose1fromAtoB>,
    Send<Update, Choose1fromAtoC>,
    Send<Update, Choose1fromAtoD>,
    RoleB<RoleC<RoleD<RoleBroadcast>>>,
    NameA,
>;
type EndpointACommitBackward = MeshedChannels<
    Send<Okay, Choose0fromAtoB>,
    Recv<Okay, Choose0fromAtoC>,
    Recv<Okay, Choose0fromAtoD>,
    RoleD<RoleC<RoleB<RoleBroadcast>>>,
    NameA,
>;
type EndpointAFull =
    MeshedChannels<Choose0fromAtoB, Choose0fromAtoC, Choose0fromAtoD, RoleBroadcast, NameA>;

// For B
type EndpointBFull = MeshedChannels<Recurs0BfromA, End, End, RoleA<RoleEnd>, NameB>;

// For C
type EndpointCFull = MeshedChannels<Recurs0CfromA, End, End, RoleA<RoleEnd>, NameC>;

// For D
type EndpointDFull = MeshedChannels<Recurs0DfromA, End, End, RoleA<RoleEnd>, NameD>;

/////////////////////////////////////////

pub fn main() {
    let (graphs, kmc) = checker_concat!(
        "commit_protocol",
        EndpointAFull,
        EndpointBFull,
        EndpointCFull,
        EndpointDFull
        =>
        [
            EndpointACommitForward,
            Branches0BfromA, CommitForward,
            Branches0CfromA, CommitForward,
            Branches0DfromA, CommitForward
        ],
        [
            EndpointACommitBackward,
            Branches1BfromA, CommitBackward,
            Branches1CfromA, CommitBackward,
            Branches1DfromA, CommitBackward
        ],
        [
            EndpointAEnd,
            Branches0BfromA, End,
            Branches0CfromA, End,
            Branches0DfromA, End
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
            2 [ label = \"\\\"0.2\\\"\" ]\n    \
            3 [ label = \"\\\"0.3\\\"\" ]\n    \
            4 [ label = \"\\\"0.3.1\\\"\" ]\n    \
            5 [ label = \"\\\"0.3.2\\\"\" ]\n    \
            6 [ label = \"\\\"0.3.3\\\"\" ]\n    \
            7 [ label = \"\\\"0.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleA?RoleB: Update\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleA!RoleC: Update\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleA!RoleD: Update\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"RoleA?RoleD: Okay\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleA?RoleC: Okay\\\"\" ]\n    \
            5 -> 6 [ label = \"\\\"RoleA!RoleB: Okay\\\"\" ]\n    \
            6 -> 0 [ label = \"\\\"µ\\\"\" ]\n    \
            0 -> 7 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test graph B
    let graph_b = &graphs["RoleB"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_b)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"0.1\\\"\" ]\n    \
            2 [ label = \"\\\"0.1.1\\\"\" ]\n    \
            3 [ label = \"\\\"0.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleB!RoleA: Update\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleB?RoleA: Okay\\\"\" ]\n    \
            2 -> 0 [ label = \"\\\"µ\\\"\" ]\n    \
            0 -> 3 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test graph C
    let graph_c = &graphs["RoleC"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_c)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"0.1\\\"\" ]\n    \
            2 [ label = \"\\\"0.1.1\\\"\" ]\n    \
            3 [ label = \"\\\"0.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleC?RoleA: Update\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleC!RoleA: Okay\\\"\" ]\n    \
            2 -> 0 [ label = \"\\\"µ\\\"\" ]\n    \
            0 -> 3 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test graph D
    let graph_d = &graphs["RoleD"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_d)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"0.1\\\"\" ]\n    \
            2 [ label = \"\\\"0.1.1\\\"\" ]\n    \
            3 [ label = \"\\\"0.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleD?RoleA: Update\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleD!RoleA: Okay\\\"\" ]\n    \
            2 -> 0 [ label = \"\\\"µ\\\"\" ]\n    \
            0 -> 3 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test KMC output
    assert_eq!(
        "CSA: \u{1b}[92mTrue\n\
        \u{1b}[0mBasic: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 1-exhaustive: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 1-safe: \u{1b}[92mTrue\n\
        \u{1b}[0m\n",
        read_to_string("outputs/commit_protocol_1_kmc.txt").unwrap()
    );

    ////////////// Test KMC number
    assert_eq!(kmc, Some(1));
}
