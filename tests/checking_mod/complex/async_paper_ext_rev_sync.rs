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
    RoleS, RoleADual |
    RoleM, RoleBDual |
    RoleC, RoleCDual |
);

// Payload types
struct Req;
struct Data;
struct Ko;
struct Error;
struct Okay;
struct Log;

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
    Okay(MeshedChannels<End, Recv<Okay, End>, RoleS<RoleEnd>, NameC>),
    Error(MeshedChannels<End, Recv<Error, End>, RoleS<RoleEnd>, NameC>),
    Ko(MeshedChannels<Choose0fromCtoM, Recv<Ko, Choose0fromCtoS>, RoleS<RoleBroadcast>, NameC>),
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
    Error(MeshedChannels<End, Send<Log, End>, RoleS<RoleEnd>, NameM>),
    Ko(MeshedChannels<Recurs0MfromC, End, RoleC<RoleEnd>, NameM>),
}

// S
type Recurs0SfromC = Recv<Branches0SfromC, End>;

enum Branches0SfromC {
    End(MeshedChannels<End, End, RoleEnd, NameS>),
    Looping(
        MeshedChannels<Recv<Req, Choose1fromStoC>, Choose1fromStoM, RoleC<RoleBroadcast>, NameS>,
    ),
}

type Choose1fromStoC = Send<Branches1CfromS, End>;
type Choose1fromStoM = Send<Branches1MfromS, End>;

// Creating the MP sessions

// For S
type EndpointSOkay = MeshedChannels<
    Send<Okay, Recv<Data, End>>,
    Recv<Log, End>,
    RoleC<RoleC<RoleM<RoleEnd>>>,
    NameS,
>;
type EndpointSError = MeshedChannels<End, End, RoleEnd, NameS>;
type EndpointSKo =
    MeshedChannels<Send<Ko, Recv<Data, Recurs0SfromC>>, End, RoleC<RoleC<RoleC<RoleEnd>>>, NameS>;

type EndpointSFull = MeshedChannels<Recurs0SfromC, End, RoleC<RoleEnd>, NameS>;

// For M
type EndpointMFull = MeshedChannels<Recurs0MfromC, End, RoleC<RoleEnd>, NameM>;

// For C
type EndpointCEnd = MeshedChannels<End, End, RoleEnd, NameC>;
type EndpointCLooping =
    MeshedChannels<End, Send<Req, Send<Data, Recurs1CfromS>>, RoleS<RoleS<RoleS<RoleEnd>>>, NameC>;

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
            EndpointSOkay,
            Branches1CfromS, Okay,
            Branches1MfromS, Okay
        ],
        [
            EndpointSKo,
            Branches1CfromS, Ko,
            Branches1MfromS, Ko
        ],
        [
            EndpointSError,
            Branches1CfromS, Error,
            Branches1MfromS, Error
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
            3 [ label = \"\\\"0.1.1\\\"\" ]\n    \
            4 [ label = \"\\\"0.1.1\\\"\" ]\n    \
            5 [ label = \"\\\"0.1.2\\\"\" ]\n    \
            6 [ label = \"\\\"0.1.1\\\"\" ]\n    \
            7 [ label = \"\\\"0.1.2\\\"\" ]\n    \
            8 [ label = \"\\\"0.1.3\\\"\" ]\n    \
            9 [ label = \"\\\"0.1.4\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 2 [ label = \"\\\"RoleS?RoleC: Req\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"0\\\"\" ]\n    \
            2 -> 4 [ label = \"\\\"RoleS!RoleC: Ko\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleS?RoleC: Data\\\"\" ]\n    \
            5 -> 0 [ label = \"\\\"µ\\\"\" ]\n    \
            2 -> 6 [ label = \"\\\"RoleS!RoleC: Okay\\\"\" ]\n    \
            6 -> 7 [ label = \"\\\"RoleS?RoleC: Data\\\"\" ]\n    \
            7 -> 8 [ label = \"\\\"RoleS?RoleM: Log\\\"\" ]\n    \
            8 -> 9 [ label = \"\\\"0\\\"\" ]\n\
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
            0 -> 2 [ label = \"\\\"RoleM!RoleS: Log\\\"\" ]\n    \
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
            7 [ label = \"\\\"0.2.1\\\"\" ]\n    \
            8 [ label = \"\\\"0.2.2\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 2 [ label = \"\\\"RoleC!RoleS: Req\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleC!RoleS: Data\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"RoleC?RoleS: Error\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"0\\\"\" ]\n    \
            3 -> 6 [ label = \"\\\"RoleC?RoleS: Ko\\\"\" ]\n    \
            6 -> 0 [ label = \"\\\"µ\\\"\" ]\n    \
            3 -> 7 [ label = \"\\\"RoleC?RoleS: Okay\\\"\" ]\n    \
            7 -> 8 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test KMC output
    assert_eq!(
        "CSA: \u{1b}[92mTrue\n\
        \u{1b}[0mBasic: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 1-exhaustive: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 1-safe: \u{1b}[92mTrue\n\
        \u{1b}[0m\n",
        read_to_string("outputs/async_paper_ext_rev_sync_1_kmc.txt").unwrap()
    );

    assert_eq!(
        "CSA: \u{1b}[92mTrue\n\
        \u{1b}[0mBasic: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 2-exhaustive: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 2-safe: \u{1b}[92mTrue\n\
        \u{1b}[0m\n",
        read_to_string("outputs/async_paper_ext_rev_sync_2_kmc.txt").unwrap()
    );
}
