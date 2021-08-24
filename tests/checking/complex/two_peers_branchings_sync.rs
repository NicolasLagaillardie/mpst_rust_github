use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary::struct_trait::recv::Recv;
use mpstthree::binary::struct_trait::send::Send;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use mpstthree::{create_meshedchannels, create_multiple_normal_role};

use petgraph::dot::Dot;

// Create new MeshedChannels
create_meshedchannels!(MeshedChannels, 2);

// Create new roles
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;

// Types

// A
type Choose0fromAtoB = Send<Branches0BfromA, End>;

type Choose1fromAtoB = Send<Branches1BfromA, End>;

type Choose2fromAtoB = Send<Branches2BfromA, End>;

type Offer3BfromA = Recv<Branches3AfromB, End>;

enum Branches3AfromB {
    B(MeshedChannels<End, RoleEnd, NameA>),
    C(MeshedChannels<End, RoleEnd, NameA>),
    D(MeshedChannels<End, RoleEnd, NameA>),
    E(MeshedChannels<End, RoleEnd, NameA>),
}

// B
type Offer0BfromA = Recv<Branches0BfromA, End>;

enum Branches0BfromA {
    A(MeshedChannels<Offer2BfromA, RoleA<RoleEnd>, NameB>),
    B(MeshedChannels<Offer1BfromA, RoleA<RoleEnd>, NameB>),
}

type Offer1BfromA = Recv<Branches1BfromA, End>;

enum Branches1BfromA {
    C(MeshedChannels<End, RoleEnd, NameB>),
    E(MeshedChannels<Choose3fromBtoA, RoleBroadcast, NameB>),
}

type Offer2BfromA = Recv<Branches2BfromA, End>;

enum Branches2BfromA {
    F(MeshedChannels<End, RoleEnd, NameB>),
    G(MeshedChannels<End, RoleEnd, NameB>),
}

type Choose3fromBtoA = Send<Branches3AfromB, End>;

// Creating the MP sessions

// For A
type EndpointAEnd = MeshedChannels<End, RoleEnd, NameA>;

type EndpointAOfferBCDE = MeshedChannels<Offer3BfromA, RoleB<RoleEnd>, NameA>;

type EndpointAChoiceFG = MeshedChannels<Choose2fromAtoB, RoleBroadcast, NameA>;

type EndpointAChoiceCE = MeshedChannels<Choose1fromAtoB, RoleBroadcast, NameA>;

type EndpointAFull = MeshedChannels<Choose0fromAtoB, RoleBroadcast, NameA>;

// For B
type EndpointBEnd = MeshedChannels<End, RoleEnd, NameB>;

type EndpointBFull = MeshedChannels<Offer0BfromA, RoleA<RoleEnd>, NameB>;

/////////////////////////////////////////

pub fn main() {
    let graphs = mpstthree::checker_concat!(
        "two_peers_branchings_sync",
        EndpointAFull,
        EndpointBFull
        =>
        [
            EndpointAChoiceFG,
            Branches0BfromA, A
        ],
        [
            EndpointAChoiceCE,
            Branches0BfromA, B
        ],
        [
            EndpointAEnd,
            Branches1BfromA, C
        ],
        [
            EndpointAOfferBCDE,
            Branches1BfromA, E
        ],
        [
            EndpointAEnd,
            Branches2BfromA, F
        ],
        [
            EndpointAEnd,
            Branches2BfromA, G
        ],
        [
            EndpointBEnd,
            Branches3AfromB, B
        ],
        [
            EndpointBEnd,
            Branches3AfromB, C
        ],
        [
            EndpointBEnd,
            Branches3AfromB, D
        ],
        [
            EndpointBEnd,
            Branches3AfromB, E
        ]
    )
    .unwrap();

    ////////////// Test graph A
    let graph_a = &graphs["RoleA"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_a)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"0.0.1\\\"\" ]\n    \
            2 [ label = \"\\\"0.0.1\\\"\" ]\n    \
            3 [ label = \"\\\"0.0.1\\\"\" ]\n    \
            4 [ label = \"\\\"0.0.0.1\\\"\" ]\n    \
            5 [ label = \"\\\"0.0.0.1\\\"\" ]\n    \
            6 [ label = \"\\\"0.0.0.1\\\"\" ]\n    \
            7 [ label = \"\\\"0.0.0.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 2 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 3 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 4 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 5 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 6 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 7 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test graph B
    let graph_b = &graphs["RoleB"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_b)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"0.0.1\\\"\" ]\n    \
            2 [ label = \"\\\"0.0.1\\\"\" ]\n    \
            3 [ label = \"\\\"0.0.1\\\"\" ]\n    \
            4 [ label = \"\\\"0.0.0.1\\\"\" ]\n    \
            5 [ label = \"\\\"0.0.0.1\\\"\" ]\n    \
            6 [ label = \"\\\"0.0.0.1\\\"\" ]\n    \
            7 [ label = \"\\\"0.0.0.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 2 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 3 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 4 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 5 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 6 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 7 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );
}
