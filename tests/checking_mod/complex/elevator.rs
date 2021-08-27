use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary::struct_trait::recv::Recv;
use mpstthree::binary::struct_trait::send::Send;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use mpstthree::{create_meshedchannels, create_multiple_normal_role};

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

// The new types

struct OpenDoor {}
struct CloseDoor {}
struct Reset {}
struct Stop {}
struct Open {}
struct Close {}
struct DoorStopped {}
struct DoorClosed {}
struct DoorOpened {}

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;

// Types

// A
type Choose0fromAtoB = Send<Branches0BfromA, End>;
type Choose0fromAtoC = Send<Branches0CfromA, End>;

// B
type Recurs0BfromA = Recv<Branches0BfromA, End>;

enum Branches0BfromA {
    End(MeshedChannels<End, End, RoleEnd, NameB>),
    Open(
        MeshedChannels<
            End,
            Recv<Open, Send<DoorOpened, Recurs1BfromA>>,
            RoleC<RoleC<RoleC<RoleEnd>>>,
            NameB,
        >,
    ),
    Close(
        MeshedChannels<
            End,
            Recv<Close, Send<DoorClosed, Recurs1BfromA>>,
            RoleC<RoleC<RoleC<RoleEnd>>>,
            NameB,
        >,
    ),
}

type Recurs1BfromA = Recv<Branches1BfromA, End>;

enum Branches1BfromA {
    Open(MeshedChannels<End, Recv<Open, Recurs1BfromA>, RoleC<RoleC<RoleEnd>>, NameB>),
    Close(MeshedChannels<End, Recv<Close, Recurs1BfromA>, RoleC<RoleC<RoleEnd>>, NameB>),
    Stop(MeshedChannels<End, Recv<Stop, Recurs1BfromA>, RoleC<RoleC<RoleEnd>>, NameB>),
    Reset(MeshedChannels<End, Recv<Reset, End>, RoleC<RoleC<RoleEnd>>, NameB>),
}

// C
type Recurs0CfromA = Recv<Branches0CfromA, End>;

enum Branches0CfromA {
    End(MeshedChannels<End, End, End, RoleEnd, NameC>),
    Win(
        MeshedChannels<
            Recv<i32, Send<i32, Recurs0CfromA>>,
            Send<i32, End>,
            Send<i32, End>,
            RoleA<RoleB<RoleD<RoleA<RoleA<RoleEnd>>>>>,
            NameC,
        >,
    ),
    Lose(
        MeshedChannels<
            Recv<i32, Send<i32, Recurs0CfromA>>,
            Send<i32, End>,
            Send<i32, End>,
            RoleA<RoleB<RoleD<RoleA<RoleA<RoleEnd>>>>>,
            NameC,
        >,
    ),
}

// Creating the MP sessions

// For A
type EndpointAEnd = MeshedChannels<End, End, RoleEnd, NameA>;
type EndpointAOpenDoor =
    MeshedChannels<Choose0fromAtoB, Send<OpenDoor, Choose0fromAtoC>, RoleC<RoleBroadcast>, NameA>;
type EndpointACLoseDoor =
    MeshedChannels<Choose0fromAtoB, Send<CloseDoor, Choose0fromAtoC>, RoleC<RoleBroadcast>, NameA>;
type EndpointAFull = MeshedChannels<Choose0fromAtoB, Choose0fromAtoC, RoleBroadcast, NameA>;

// For B
type EndpointBFull = MeshedChannels<Recurs0BfromA, End, RoleA<RoleEnd>, NameB>;

// For C
type EndpointCFull = MeshedChannels<Recurs0CfromA, End, RoleA<RoleEnd>, NameC>;

/////////////////////////////////////////

pub fn main() {
    let graphs = mpstthree::checker_concat!(
        "elevator",
        EndpointAFull,
        EndpointBFull,
        EndpointCFull,
        =>
        [
            EndpointAWin,
            Branches0BfromA, Win,
            Branches0CfromA, Win,
            Branches0DfromA, Win
        ],
        [
            EndpointALose,
            Branches0BfromA, Lose,
            Branches0CfromA, Lose,
            Branches0DfromA, Lose
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
            2 [ label = \"\\\"0.1\\\"\" ]\n    \
            3 [ label = \"\\\"0.2\\\"\" ]\n    \
            4 [ label = \"\\\"0.3\\\"\" ]\n    \
            5 [ label = \"\\\"0.4\\\"\" ]\n    \
            6 [ label = \"\\\"0.1\\\"\" ]\n    \
            7 [ label = \"\\\"0.2\\\"\" ]\n    \
            8 [ label = \"\\\"0.3\\\"\" ]\n    \
            9 [ label = \"\\\"0.4\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 2 [ label = \"\\\"RoleA!RoleC: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleA?RoleB: i32\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"RoleA?RoleC: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleA!RoleD: i32\\\"\" ]\n    \
            5 -> 0 [ label = \"\\\"µ\\\"\" ]\n    \
            0 -> 6 [ label = \"\\\"RoleA!RoleC: i32\\\"\" ]\n    \
            6 -> 7 [ label = \"\\\"RoleA?RoleB: i32\\\"\" ]\n    \
            7 -> 8 [ label = \"\\\"RoleA?RoleC: i32\\\"\" ]\n    \
            8 -> 9 [ label = \"\\\"RoleA!RoleD: i32\\\"\" ]\n    \
            9 -> 0 [ label = \"\\\"µ\\\"\" ]\n\
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
            0 -> 2 [ label = \"\\\"RoleB?RoleC: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleB!RoleA: i32\\\"\" ]\n    \
            3 -> 0 [ label = \"\\\"µ\\\"\" ]\n    \
            0 -> 4 [ label = \"\\\"RoleB?RoleC: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleB!RoleA: i32\\\"\" ]\n    \
            5 -> 0 [ label = \"\\\"µ\\\"\" ]\n\
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
            4 [ label = \"\\\"0.3\\\"\" ]\n    \
            5 [ label = \"\\\"0.4\\\"\" ]\n    \
            6 [ label = \"\\\"0.1\\\"\" ]\n    \
            7 [ label = \"\\\"0.2\\\"\" ]\n    \
            8 [ label = \"\\\"0.3\\\"\" ]\n    \
            9 [ label = \"\\\"0.4\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 2 [ label = \"\\\"RoleC?RoleA: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleC!RoleB: i32\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"RoleC!RoleD: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleC!RoleA: i32\\\"\" ]\n    \
            5 -> 0 [ label = \"\\\"µ\\\"\" ]\n    \
            0 -> 6 [ label = \"\\\"RoleC?RoleA: i32\\\"\" ]\n    \
            6 -> 7 [ label = \"\\\"RoleC!RoleB: i32\\\"\" ]\n    \
            7 -> 8 [ label = \"\\\"RoleC!RoleD: i32\\\"\" ]\n    \
            8 -> 9 [ label = \"\\\"RoleC!RoleA: i32\\\"\" ]\n    \
            9 -> 0 [ label = \"\\\"µ\\\"\" ]\n\
        }\n"
    );

    ////////////// Test KMC output
    assert_eq!(
        "CSA: \u{1b}[92mTrue\n\u{1b}[0mBasic: \
        \u{1b}[92mTrue\n\u{1b}[0mreduced 1-exhaustive: \
        \u{1b}[92mTrue\n\u{1b}[0mreduced 1-safe: \
        \u{1b}[92mTrue\n\u{1b}[0m\n",
        read_to_string("outputs/elevator_1_kmc.txt").unwrap()
    );

    assert_eq!(
        "CSA: \u{1b}[92mTrue\n\u{1b}[0mBasic: \
        \u{1b}[92mTrue\n\u{1b}[0mreduced 2-exhaustive: \
        \u{1b}[92mTrue\n\u{1b}[0mreduced 2-safe: \
        \u{1b}[92mTrue\n\u{1b}[0m\n",
        read_to_string("outputs/elevator_2_kmc.txt").unwrap()
    );
}
