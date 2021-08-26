use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary::struct_trait::recv::Recv;
use mpstthree::binary::struct_trait::send::Send;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use mpstthree::{create_meshedchannels, create_multiple_normal_role};

use petgraph::dot::Dot;

// Create new MeshedChannels
create_meshedchannels!(MeshedChannels, 3);

// Create new roles
create_multiple_normal_role!(
    RoleS, RoleADual |
    RoleM, RoleBDual |
    RoleC, RoleCDual |
);

// Names
type NameC = RoleC<RoleEnd>;
type NameM = RoleM<RoleEnd>;
type NameS = RoleS<RoleEnd>;

// Types

// C
type Choose0fromCtoM = Send<Branches0MfromC, End>;
type Choose0fromCtoS = Send<Branches0SfromC, End>;

type Recurs1CfromS = Recv<Branches1CfromS, End>;

enum Branches1CfromS {
    Okay(MeshedChannels<End, Recv<i32, End>, RoleS<RoleEnd>, NameC>),
    Error(MeshedChannels<End, Recv<i32, End>, RoleS<RoleEnd>, NameC>),
    Ko(MeshedChannels<Choose0fromCtoM, Choose0fromCtoS, RoleBroadcast, NameC>),
}

// M
type Recurs0MfromC = Recv<Branches0MfromC, End>;

enum Branches0MfromC {
    End(MeshedChannels<End, End, RoleEnd, NameM>),
    Looping(MeshedChannels<End, Recurs1MfromS, RoleS<RoleEnd>, NameM>),
}

type Recurs1MfromS = Recv<Branches1MfromS, End>;

enum Branches1MfromS {
    Okay(MeshedChannels<End, End, RoleEnd, NameM>),
    Error(MeshedChannels<End, Recv<i32, End>, RoleS<RoleEnd>, NameM>),
    Ko(MeshedChannels<Recurs0MfromC, End, RoleC<RoleEnd>, NameM>),
}

// S
type Recurs0SfromC = Recv<Branches0SfromC, End>;

enum Branches0SfromC {
    End(MeshedChannels<End, End, RoleEnd, NameS>),
    Looping(
        MeshedChannels<
            Recv<i32, Recv<i32, Choose1fromStoC>>,
            Choose1fromStoM,
            RoleC<RoleC<RoleBroadcast>>,
            NameS,
        >,
    ),
}

type Choose1fromStoC = Send<Branches1CfromS, End>;
type Choose1fromStoM = Send<Branches1MfromS, End>;

// Creating the MP sessions

// For S
type EndpointSOkay = MeshedChannels<Send<i32, End>, End, RoleC<RoleEnd>, NameS>;
type EndpointSError = MeshedChannels<Send<i32, End>, Send<i32, End>, RoleC<RoleM<RoleEnd>>, NameS>;
type EndpointSKo = MeshedChannels<Recurs0SfromC, End, RoleC<RoleEnd>, NameS>;

type EndpointSFull = MeshedChannels<Recurs0SfromC, End, RoleC<RoleEnd>, NameS>;

// For M
type EndpointMFull = MeshedChannels<Recurs0MfromC, End, RoleC<RoleEnd>, NameM>;

// For C
type EndpointCEnd = MeshedChannels<End, End, RoleEnd, NameC>;
type EndpointCLooping =
    MeshedChannels<End, Send<i32, Send<i32, Recurs1CfromS>>, RoleS<RoleS<RoleS<RoleEnd>>>, NameC>;

type EndpointCFull = MeshedChannels<Choose0fromCtoM, Choose0fromCtoS, RoleBroadcast, NameC>;

/////////////////////////////////////////

pub fn main() {
    let graphs = mpstthree::checker_concat!(
        "async_paper_ext_rev_sync",
        EndpointCFull,
        EndpointSFull,
        EndpointMFull
        =>
        [
            EndpointCEnd,
            Branches0MfromC, End,
            Branches0SfromC, End
        ],
        [
            EndpointCLooping,
            Branches0MfromC, Looping,
            Branches0SfromC, Looping
        ],
        [
            EndpointSError,
            Branches1CfromS, Error,
            Branches1MfromS, Error
        ],
        [
            EndpointSOkay,
            Branches1CfromS, Okay,
            Branches1MfromS, Okay
        ],
        [
            EndpointSKo,
            Branches1CfromS, Ko,
            Branches1MfromS, Ko
        ]
    )
    .unwrap();

    ////////////// Test graph S
    let graph_s = &graphs["RoleS"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_s)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"0.1\\\"\" ]\n    \
            2 [ label = \"\\\"0.1\\\"\" ]\n    \
            3 [ label = \"\\\"0.2\\\"\" ]\n    \
            4 [ label = \"\\\"0.2.1\\\"\" ]\n    \
            5 [ label = \"\\\"0.2.2\\\"\" ]\n    \
            6 [ label = \"\\\"0.2.3\\\"\" ]\n    \
            7 [ label = \"\\\"0.2.1\\\"\" ]\n    \
            8 [ label = \"\\\"0.2.2\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 2 [ label = \"\\\"RoleS?RoleC: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleS?RoleC: i32\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"RoleS!RoleC: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleS!RoleM: i32\\\"\" ]\n    \
            5 -> 6 [ label = \"\\\"0\\\"\" ]\n    \
            3 -> 0 [ label = \"\\\"µ\\\"\" ]\n    \
            3 -> 7 [ label = \"\\\"RoleS!RoleC: i32\\\"\" ]\n    \
            7 -> 8 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test graph M
    let graph_m = &graphs["RoleM"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_m)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"0.1\\\"\" ]\n    \
            2 [ label = \"\\\"0.0.1\\\"\" ]\n    \
            3 [ label = \"\\\"0.0.2\\\"\" ]\n    \
            4 [ label = \"\\\"0.0.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 2 [ label = \"\\\"RoleM?RoleS: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 4 [ label = \"\\\"0\\\"\" ]\n\
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
            4 [ label = \"\\\"0.2.1\\\"\" ]\n    \
            5 [ label = \"\\\"0.2.2\\\"\" ]\n    \
            6 [ label = \"\\\"0.2.1\\\"\" ]\n    \
            7 [ label = \"\\\"0.2.2\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 2 [ label = \"\\\"RoleC!RoleS: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleC!RoleS: i32\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"RoleC?RoleS: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"0\\\"\" ]\n    \
            3 -> 0 [ label = \"\\\"µ\\\"\" ]\n    \
            3 -> 6 [ label = \"\\\"RoleC?RoleS: i32\\\"\" ]\n    \
            6 -> 7 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );
}
