// Test for parametrisation on the number of roles
use rand::{thread_rng, Rng};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    choose_mpst_multi_to_all, close_mpst, create_broadcast_role, create_meshedchannels,
    create_multiple_normal_name, create_multiple_normal_role, create_recv_mpst_session,
    create_send_mpst_session, fork_mpst_multi, offer_mpst,
};
use std::error::Error;
use std::marker;

// Create new MeshedChannels for three participants
create_meshedchannels!(MeshedChannels, 3);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
    RoleD, RoleDDual |
);

// broadcast
create_broadcast_role!(RoleAlltoD, RoleDtoAll);

// Create new names
create_multiple_normal_name!(NameA, NameB, NameD);

// Create new send functions
create_send_mpst_session!(send_mpst_d_to_a, RoleA, NameD, MeshedChannels, 3, 1);
create_send_mpst_session!(send_mpst_a_to_d, RoleD, NameA, MeshedChannels, 3, 2);
create_send_mpst_session!(send_mpst_d_to_b, RoleB, NameD, MeshedChannels, 3, 2);
create_send_mpst_session!(send_mpst_b_to_a, RoleA, NameB, MeshedChannels, 3, 1);
create_send_mpst_session!(send_mpst_a_to_b, RoleB, NameA, MeshedChannels, 3, 1);

// Create new recv functions and related types
// normal
create_recv_mpst_session!(recv_mpst_d_from_a, RoleA, NameD, MeshedChannels, 3, 1);
create_recv_mpst_session!(recv_mpst_a_from_d, RoleD, NameA, MeshedChannels, 3, 2);
create_recv_mpst_session!(recv_mpst_b_from_d, RoleD, NameB, MeshedChannels, 3, 2);
create_recv_mpst_session!(recv_mpst_b_from_a, RoleA, NameB, MeshedChannels, 3, 1);
create_recv_mpst_session!(recv_mpst_a_from_b, RoleB, NameA, MeshedChannels, 3, 1);

close_mpst!(close_mpst_multi, MeshedChannels, 3);

fork_mpst_multi!(fork_mpst, MeshedChannels, 3);

// Test our usecase
// Simple types
// Client = C
// Authenticator = A
// Server = B

type AtoDClose = End;
type AtoBClose = End;
type AtoBVideo<N> = Send<N, Recv<N, End>>;
type AtoDVideo<N> = Recv<N, Send<N, RecursAtoD<N>>>;

type InitA<N> = Recv<N, Send<N, RecursAtoD<N>>>;

type BtoAClose = End;
type BtoDClose = End;
type BtoAVideo<N> = Recv<N, Send<N, End>>;

type RecursAtoD<N> = Recv<Branches0AtoD<N>, End>;
type RecursBtoD<N> = Recv<Branches0BtoD<N>, End>;

enum Branches0AtoD<N: marker::Send> {
    End(MeshedChannels<AtoBClose, AtoDClose, StackAEnd, NameA>),
    Video(MeshedChannels<AtoBVideo<N>, AtoDVideo<N>, StackAVideo, NameA>),
}
enum Branches0BtoD<N: marker::Send> {
    End(MeshedChannels<BtoAClose, BtoDClose, StackBEnd, NameB>),
    Video(MeshedChannels<BtoAVideo<N>, RecursBtoD<N>, StackBVideo, NameB>),
}
type Choose0fromCtoA<N> = Send<Branches0AtoD<N>, End>;
type Choose0fromCtoB<N> = Send<Branches0BtoD<N>, End>;

type InitD<N> = Send<N, Recv<N, Choose0fromCtoA<N>>>;

// Stacks
type StackAEnd = RoleEnd;
type StackAVideo = RoleD<RoleB<RoleB<RoleD<RoleD<RoleEnd>>>>>;
type StackARecurs = RoleD<RoleEnd>;
type StackAInit = RoleD<RoleD<RoleD<RoleEnd>>>;

type StackBEnd = RoleEnd;
type StackBVideo = RoleA<RoleA<RoleD<RoleEnd>>>;
type StackBRecurs = RoleD<RoleEnd>;

type StackDRecurs = RoleBroadcast;
type StackDFull = RoleA<RoleA<StackDRecurs>>;

// Creating the MP sessions
// For C

type EndpointDRecurs<N> =
    MeshedChannels<Choose0fromCtoA<N>, Choose0fromCtoB<N>, StackDRecurs, NameD>;
type EndpointDFull<N> = MeshedChannels<InitD<N>, Choose0fromCtoB<N>, StackDFull, NameD>;

// For A
type EndpointARecurs<N> = MeshedChannels<End, RecursAtoD<N>, StackARecurs, NameA>;
type EndpointAFull<N> = MeshedChannels<End, InitA<N>, StackAInit, NameA>;

// For B
type EndpointBRecurs<N> = MeshedChannels<End, RecursBtoD<N>, StackBRecurs, NameB>;

// Functions related to endpoints
fn server(s: EndpointBRecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_d, {
        Branches0BtoD::End(s) => {
            close_mpst_multi(s)
        },
        Branches0BtoD::Video(s) => {
            let (request, s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_a(request + 1, s);
            server(s)
        },
    })
}

fn authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_a_from_d(s)?;
    let s = send_mpst_a_to_d(id + 1, s);

    authenticator_recurs(s)
}

fn authenticator_recurs(s: EndpointARecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_d, {
        Branches0AtoD::End(s) => {
            close_mpst_multi(s)
        },
        Branches0AtoD::Video(s) => {
            let (request, s) = recv_mpst_a_from_d(s)?;
            let s = send_mpst_a_to_b(request + 1, s);
            let (video, s) = recv_mpst_a_from_b(s)?;
            let s = send_mpst_a_to_d(video + 1, s);
            authenticator_recurs(s)
        },
    })
}

fn client(s: EndpointDFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();

    let s = send_mpst_d_to_a(0, s);
    let (_, s) = recv_mpst_d_from_a(s)?;

    client_recurs(s, xs, 1)
}

fn client_recurs(
    s: EndpointDRecurs<i32>,
    mut xs: Vec<i32>,
    index: i32,
) -> Result<(), Box<dyn Error>> {
    match xs.pop() {
        Option::Some(_) => {
            let s = choose_mpst_multi_to_all!(
                s,
                Branches0AtoD::Video,
                Branches0BtoD::Video, =>
                NameD,
                MeshedChannels,
                3
            );

            let s = send_mpst_d_to_a(1, s);
            let (_, s) = recv_mpst_d_from_a(s)?;

            client_recurs(s, xs, index + 1)
        }
        Option::None => {
            let s = choose_mpst_multi_to_all!(
                s,
                Branches0AtoD::End,
                Branches0BtoD::End, =>
                NameD,
                MeshedChannels,
                3
            );

            assert_eq!(index, 100);

            close_mpst_multi(s)
        }
    }
}

////////////////////////////////////////

pub fn new_run_usecase_recursive() {
    let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client);

    assert!(thread_a.join().is_ok());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_ok());
}
