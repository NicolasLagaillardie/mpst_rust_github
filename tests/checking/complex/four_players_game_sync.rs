use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary::struct_trait::recv::Recv;
use mpstthree::binary::struct_trait::send::Send;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use mpstthree::{create_meshedchannels, create_multiple_normal_role};

use petgraph::dot::Dot;

// Create new MeshedChannels
create_meshedchannels!(MeshedChannels, 4);

// Create new roles
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
    RoleC, RoleCDual |
    RoleD, RoleDDual |
);

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

// B
type Recurs0BfromA = Recv<Branches0BfromA, End>;

enum Branches0BfromA {
    End(MeshedChannels<End, End, End, RoleEnd, NameB>),
    Win(
        MeshedChannels<
            Send<(), Recurs0BfromA>,
            Recv<(), End>,
            End,
            RoleC<RoleA<RoleA<RoleEnd>>>,
            NameB,
        >,
    ),
    Lose(
        MeshedChannels<
            Send<(), Recurs0BfromA>,
            Recv<(), End>,
            End,
            RoleC<RoleA<RoleA<RoleEnd>>>,
            NameB,
        >,
    ),
}

// C
type Recurs0CfromA = Recv<Branches0CfromA, End>;

enum Branches0CfromA {
    End(MeshedChannels<End, End, End, RoleEnd, NameC>),
    Win(
        MeshedChannels<
            Recv<(), Send<(), Recurs0CfromA>>,
            Send<(), End>,
            Recv<(), End>,
            RoleA<RoleB<RoleA<RoleD<RoleA<RoleEnd>>>>>,
            NameC,
        >,
    ),
    Lose(
        MeshedChannels<
            Recv<(), Send<(), Recurs0CfromA>>,
            Send<(), End>,
            Recv<(), End>,
            RoleA<RoleB<RoleA<RoleD<RoleA<RoleEnd>>>>>,
            NameC,
        >,
    ),
}

// D
type Recurs0DfromA = Recv<Branches0DfromA, End>;

enum Branches0DfromA {
    End(MeshedChannels<End, End, End, RoleEnd, NameD>),
    Win(
        MeshedChannels<
            Send<(), Recurs0DfromA>,
            End,
            Recv<(), End>,
            RoleC<RoleA<RoleA<RoleEnd>>>,
            NameD,
        >,
    ),
    Lose(
        MeshedChannels<
            Send<(), Recurs0DfromA>,
            End,
            Recv<(), End>,
            RoleC<RoleA<RoleA<RoleEnd>>>,
            NameD,
        >,
    ),
}

// Creating the MP sessions

// For A
type EndpointAEnd = MeshedChannels<End, End, End, RoleEnd, NameA>;
type EndpointALose = MeshedChannels<
    Recv<(), Choose0fromAtoB>,
    Send<(), Send<(), Choose0fromAtoC>>,
    Send<(), Choose0fromAtoD>,
    RoleC<RoleB<RoleC<RoleD<RoleBroadcast>>>>,
    NameA,
>;
type EndpointAWin = MeshedChannels<
    Recv<(), Choose0fromAtoB>,
    Send<(), Send<(), Choose0fromAtoC>>,
    Send<(), Choose0fromAtoD>,
    RoleC<RoleB<RoleC<RoleD<RoleBroadcast>>>>,
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
    let graphs = mpstthree::checker_concat!(
        EndpointAFull,
        EndpointBFull,
        EndpointCFull,
        EndpointDFull
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
            1 [ label = \"\\\"0.0\\\"\" ]\n    \
            2 [ label = \"\\\"0.1\\\"\" ]\n    \
            3 [ label = \"\\\"0.1\\\"\" ]\n    \
            4 [ label = \"\\\"0.2\\\"\" ]\n    \
            5 [ label = \"\\\"0.3\\\"\" ]\n    \
            6 [ label = \"\\\"0.4\\\"\" ]\n    \
            7 [ label = \"\\\"0.1\\\"\" ]\n    \
            8 [ label = \"\\\"0.2\\\"\" ]\n    \
            9 [ label = \"\\\"0.3\\\"\" ]\n    \
            10 [ label = \"\\\"0.4\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"+ RoleA\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"0\\\"\" ]\n    \
            1 -> 3 [ label = \"\\\"RoleA!RoleC: ()\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"RoleA?RoleB: ()\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleA!RoleC: ()\\\"\" ]\n    \
            5 -> 6 [ label = \"\\\"RoleA!RoleD: ()\\\"\" ]\n    \
            6 -> 1 [ label = \"\\\"µ\\\"\" ]\n    \
            1 -> 7 [ label = \"\\\"RoleA!RoleC: ()\\\"\" ]\n    \
            7 -> 8 [ label = \"\\\"RoleA?RoleB: ()\\\"\" ]\n    \
            8 -> 9 [ label = \"\\\"RoleA!RoleC: ()\\\"\" ]\n    \
            9 -> 10 [ label = \"\\\"RoleA!RoleD: ()\\\"\" ]\n    \
            10 -> 1 [ label = \"\\\"µ\\\"\" ]\n\
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
            5 [ label = \"\\\"0.1\\\"\" ]\n    \
            6 [ label = \"\\\"0.2\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"& RoleA\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"0\\\"\" ]\n    \
            1 -> 3 [ label = \"\\\"RoleB?RoleC: ()\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"RoleB!RoleA: ()\\\"\" ]\n    \
            4 -> 1 [ label = \"\\\"µ\\\"\" ]\n    \
            1 -> 5 [ label = \"\\\"RoleB?RoleC: ()\\\"\" ]\n    \
            5 -> 6 [ label = \"\\\"RoleB!RoleA: ()\\\"\" ]\n    \
            6 -> 1 [ label = \"\\\"µ\\\"\" ]\n\
        }\n"
    );

    ////////////// Test graph C
    let graph_c = &graphs["RoleC"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_c)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"0.0\\\"\" ]\n    \
            2 [ label = \"\\\"0.1\\\"\" ]\n    \
            3 [ label = \"\\\"0.1\\\"\" ]\n    \
            4 [ label = \"\\\"0.2\\\"\" ]\n    \
            5 [ label = \"\\\"0.3\\\"\" ]\n    \
            6 [ label = \"\\\"0.4\\\"\" ]\n    \
            7 [ label = \"\\\"0.1\\\"\" ]\n    \
            8 [ label = \"\\\"0.2\\\"\" ]\n    \
            9 [ label = \"\\\"0.3\\\"\" ]\n    \
            10 [ label = \"\\\"0.4\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"& RoleA\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"0\\\"\" ]\n    \
            1 -> 3 [ label = \"\\\"RoleC?RoleA: ()\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"RoleC!RoleB: ()\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleC!RoleA: ()\\\"\" ]\n    \
            5 -> 6 [ label = \"\\\"RoleC?RoleD: ()\\\"\" ]\n    \
            6 -> 1 [ label = \"\\\"µ\\\"\" ]\n    \
            1 -> 7 [ label = \"\\\"RoleC?RoleA: ()\\\"\" ]\n    \
            7 -> 8 [ label = \"\\\"RoleC!RoleB: ()\\\"\" ]\n    \
            8 -> 9 [ label = \"\\\"RoleC!RoleA: ()\\\"\" ]\n    \
            9 -> 10 [ label = \"\\\"RoleC?RoleD: ()\\\"\" ]\n    \
            10 -> 1 [ label = \"\\\"µ\\\"\" ]\n\
        }\n"
    );

    ////////////// Test graph D
    let graph_d = &graphs["RoleD"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_d)),
        "digraph {\n    \
    0 [ label = \"\\\"0\\\"\" ]\n    \
    1 [ label = \"\\\"0.0\\\"\" ]\n    \
    2 [ label = \"\\\"0.1\\\"\" ]\n    \
    3 [ label = \"\\\"0.1\\\"\" ]\n    \
    4 [ label = \"\\\"0.2\\\"\" ]\n    \
    5 [ label = \"\\\"0.1\\\"\" ]\n    \
    6 [ label = \"\\\"0.2\\\"\" ]\n    \
    0 -> 1 [ label = \"\\\"& RoleA\\\"\" ]\n    \
    1 -> 2 [ label = \"\\\"0\\\"\" ]\n    \
    1 -> 3 [ label = \"\\\"RoleD?RoleC: ()\\\"\" ]\n    \
    3 -> 4 [ label = \"\\\"RoleD!RoleA: ()\\\"\" ]\n    \
    4 -> 1 [ label = \"\\\"µ\\\"\" ]\n    \
    1 -> 5 [ label = \"\\\"RoleD?RoleC: ()\\\"\" ]\n    \
    5 -> 6 [ label = \"\\\"RoleD!RoleA: ()\\\"\" ]\n    \
    6 -> 1 [ label = \"\\\"µ\\\"\" ]\n}\n"
    );
}
