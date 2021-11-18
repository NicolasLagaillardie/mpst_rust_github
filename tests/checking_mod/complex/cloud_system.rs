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

// The new types

struct Connect;
struct SyncAccess;
struct Access;
struct Logout;
struct SyncLogout;
struct SyncLog;
struct Setup;
struct Log;

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
    Connect(MeshedChannels<Recurs1BfromA, Recv<Setup, End>, End, RoleC<RoleA<RoleEnd>>, NameB>),
}

type Recurs1BfromA = Recv<Branches1BfromA, End>;

enum Branches1BfromA {
    Looping(MeshedChannels<Recv<Access, Recurs1BfromA>, End, End, RoleA<RoleA<RoleEnd>>, NameB>),
    Logging(
        MeshedChannels<
            Recv<SyncLogout, Send<SyncLog, Recurs0BfromA>>,
            End,
            Send<Log, End>,
            RoleA<RoleD<RoleA<RoleA<RoleEnd>>>>,
            NameB,
        >,
    ),
}

// C
type Recurs0CfromA = Recv<Branches0CfromA, End>;

enum Branches0CfromA {
    End(MeshedChannels<End, End, End, RoleEnd, NameC>),
    Connect(MeshedChannels<Recv<Connect, Recurs1CfromA>, End, End, RoleA<RoleA<RoleEnd>>, NameC>),
}

type Recurs1CfromA = Recv<Branches1CfromA, End>;

enum Branches1CfromA {
    Looping(MeshedChannels<Recurs1CfromA, End, End, RoleA<RoleEnd>, NameC>),
    Logging(
        MeshedChannels<
            Send<SyncAccess, Recv<Logout, Recurs0CfromA>>,
            Send<Setup, End>,
            End,
            RoleB<RoleA<RoleA<RoleA<RoleEnd>>>>,
            NameC,
        >,
    ),
}

// D
type Recurs0DfromA = Recv<Branches0DfromA, End>;

enum Branches0DfromA {
    End(MeshedChannels<End, End, End, RoleEnd, NameD>),
    Connect(MeshedChannels<Recurs1DfromA, End, End, RoleA<RoleEnd>, NameD>),
}

type Recurs1DfromA = Recv<Branches1DfromA, End>;

enum Branches1DfromA {
    Looping(MeshedChannels<Recurs1CfromA, End, End, RoleA<RoleEnd>, NameD>),
    Logging(MeshedChannels<Recurs0DfromA, Recv<Log, End>, End, RoleB<RoleA<RoleEnd>>, NameD>),
}

// Creating the MP sessions

// For A
type EndpointAEnd = MeshedChannels<End, End, End, RoleEnd, NameA>;
type EndpointAConnect = MeshedChannels<
    Choose1fromAtoB,
    Send<Connect, Recv<SyncAccess, Choose1fromAtoC>>,
    Choose1fromAtoD,
    RoleC<RoleC<RoleBroadcast>>,
    NameA,
>;
type EndpointALooping = MeshedChannels<
    Send<Access, Choose1fromAtoB>,
    Choose1fromAtoC,
    Choose1fromAtoD,
    RoleB<RoleBroadcast>,
    NameA,
>;
type EndpointALogging = MeshedChannels<
    Send<SyncLogout, Recv<SyncLog, Choose0fromAtoB>>,
    Send<Logout, Choose0fromAtoC>,
    Choose0fromAtoD,
    RoleC<RoleB<RoleB<RoleBroadcast>>>,
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
    let graphs = checker_concat!(
        "cloud_system",
        1,
        2,
        EndpointAFull,
        EndpointBFull,
        EndpointCFull,
        EndpointDFull
        =>
        [
            EndpointAEnd,
            Branches0BfromA, End,
            Branches0CfromA, End,
            Branches0DfromA, End
        ],
        [
            EndpointAConnect,
            Branches0BfromA, Connect,
            Branches0CfromA, Connect,
            Branches0DfromA, Connect
        ],
        [
            EndpointALooping,
            Branches1BfromA, Looping,
            Branches1CfromA, Looping,
            Branches1DfromA, Looping
        ],
        [
            EndpointALogging,
            Branches1BfromA, Logging,
            Branches1CfromA, Logging,
            Branches1DfromA, Logging
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
            3 [ label = \"\\\"0.2.1\\\"\" ]\n    \
            4 [ label = \"\\\"0.2.2\\\"\" ]\n    \
            5 [ label = \"\\\"0.2.3\\\"\" ]\n    \
            6 [ label = \"\\\"0.2.1\\\"\" ]\n    \
            7 [ label = \"\\\"0.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleA!RoleC: Connect\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleA?RoleC: SyncAccess\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleA!RoleC: Logout\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"RoleA!RoleB: SyncLogout\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleA?RoleB: SyncLog\\\"\" ]\n    \
            5 -> 0 [ label = \"\\\"µ\\\"\" ]\n    \
            2 -> 6 [ label = \"\\\"RoleA!RoleB: Access\\\"\" ]\n    \
            6 -> 2 [ label = \"\\\"µ\\\"\" ]\n    \
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
            3 [ label = \"\\\"0.1.2\\\"\" ]\n    \
            4 [ label = \"\\\"0.1.3\\\"\" ]\n    \
            5 [ label = \"\\\"0.1.1\\\"\" ]\n    \
            6 [ label = \"\\\"0.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleB?RoleC: Setup\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleB?RoleA: SyncLogout\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleB!RoleD: Log\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"RoleB!RoleA: SyncLog\\\"\" ]\n    \
            4 -> 0 [ label = \"\\\"µ\\\"\" ]\n    \
            1 -> 5 [ label = \"\\\"RoleB?RoleA: Access\\\"\" ]\n    \
            5 -> 1 [ label = \"\\\"µ\\\"\" ]\n    \
            0 -> 6 [ label = \"\\\"0\\\"\" ]\n\
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
            3 [ label = \"\\\"0.1.2\\\"\" ]\n    \
            4 [ label = \"\\\"0.1.3\\\"\" ]\n    \
            5 [ label = \"\\\"0.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleC?RoleA: Connect\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleC!RoleB: Setup\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleC!RoleA: SyncAccess\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"RoleC?RoleA: Logout\\\"\" ]\n    \
            4 -> 0 [ label = \"\\\"µ\\\"\" ]\n    \
            0 -> 5 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test graph D
    let graph_d = &graphs["RoleD"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_d)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"0.0.1\\\"\" ]\n    \
            2 [ label = \"\\\"0.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleD?RoleB: Log\\\"\" ]\n    \
            1 -> 0 [ label = \"\\\"µ\\\"\" ]\n    \
            0 -> 2 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test KMC output
    assert_eq!(
        "CSA: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 1-OBI: \u{1b}[91mFalse\n\
        \u{1b}[0mreduced 1-SIBI: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 1-exhaustive: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 1-safe: \u{1b}[92mTrue\n\
        \u{1b}[0m\n",
        read_to_string("outputs/cloud_system_1_kmc.txt").unwrap()
    );

    assert_eq!(
        "CSA: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 2-OBI: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 2-SIBI: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 2-exhaustive: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 2-safe: \u{1b}[92mTrue\n\
        \u{1b}[0m\n",
        read_to_string("outputs/cloud_system_2_kmc.txt").unwrap()
    );
}
