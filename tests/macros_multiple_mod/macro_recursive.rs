// Test for Macro, exact same as usecase-recursive
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::functionmpst::close::close_mpst;
use mpstthree::functionmpst::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use std::error::Error;
use std::marker;

use rand::{thread_rng, Rng};

use mpstthree::{
    choose_mpst_to_all, create_multiple_normal_name, create_multiple_normal_role,
    create_recv_mpst_session_1, create_recv_mpst_session_2, create_send_mpst_session_1,
    create_send_mpst_session_2, offer_mpst,
};

// Create new roles
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
    RoleC, RoleCDual |
);

// Create new names
create_multiple_normal_name!(NameA, NameB, NameC);

// Create new send functions
create_send_mpst_session_1!(send_mpst_c_to_a, RoleA, NameC);
create_send_mpst_session_2!(send_mpst_a_to_c, RoleC, NameA);
create_send_mpst_session_2!(send_mpst_c_to_b, RoleB, NameC);
create_send_mpst_session_1!(send_mpst_b_to_a, RoleA, NameB);
create_send_mpst_session_1!(send_mpst_a_to_b, RoleB, NameA);

// Create new recv functions and related types
create_recv_mpst_session_1!(recv_mpst_c_from_a, RoleA, NameC);
create_recv_mpst_session_2!(recv_mpst_a_from_c, RoleC, NameA);
create_recv_mpst_session_2!(recv_mpst_b_from_c, RoleC, NameB);
create_recv_mpst_session_1!(recv_mpst_b_from_a, RoleA, NameB);
create_recv_mpst_session_1!(recv_mpst_a_from_b, RoleB, NameA);

// Types
type AtoBVideo<N> = Send<N, Recv<N, End>>;
type AtoCVideo<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type InitA<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type BtoAVideo<N> = Recv<N, Send<N, End>>;

type RecursAtoC<N> = Recv<Branches0AtoC<N>, End>;
type RecursBtoC<N> = Recv<Branches0BtoC<N>, End>;

enum Branches0AtoC<N: marker::Send> {
    End(MeshedChannels<End, End, RoleEnd, NameA>),
    Video(MeshedChannels<AtoBVideo<N>, AtoCVideo<N>, StackAVideo, NameA>),
}
enum Branches0BtoC<N: marker::Send> {
    End(MeshedChannels<End, End, RoleEnd, NameB>),
    Video(MeshedChannels<BtoAVideo<N>, RecursBtoC<N>, StackBVideo, NameB>),
}
type Choose0fromCtoA<N> = Send<Branches0AtoC<N>, End>;
type Choose0fromCtoB<N> = Send<Branches0BtoC<N>, End>;

type InitC<N> = Send<N, Recv<N, Choose0fromCtoA<N>>>;

// Stacks
type StackAVideo = RoleC<RoleB<RoleB<RoleC<RoleC<RoleEnd>>>>>;
type StackAInit = RoleC<RoleC<RoleC<RoleEnd>>>;

type StackBVideo = RoleA<RoleA<RoleC<RoleEnd>>>;

type StackCRecurs = RoleBroadcast;
type StackCFull = RoleA<RoleA<StackCRecurs>>;

// Creating the MP sessions
// For C

type EndpointCRecurs<N> =
    MeshedChannels<Choose0fromCtoA<N>, Choose0fromCtoB<N>, StackCRecurs, NameC>;
type EndpointCFull<N> = MeshedChannels<InitC<N>, Choose0fromCtoB<N>, StackCFull, NameC>;

// For A
type EndpointARecurs<N> = MeshedChannels<End, RecursAtoC<N>, RoleC<RoleEnd>, NameA>;
type EndpointAFull<N> = MeshedChannels<End, InitA<N>, StackAInit, NameA>;

// For B
type EndpointBFull<N> = MeshedChannels<End, RecursBtoC<N>, RoleC<RoleEnd>, NameB>;

// Functions related to endpoints
fn server(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_c, {
        Branches0BtoC::End(s) => {
            close_mpst(s)
        },
        Branches0BtoC::Video(s) => {
            let (request, s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_a(request + 1, s);
            server(s)
        },
    })
}

fn authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_a_from_c(s)?;
    let s = send_mpst_a_to_c(id + 1, s);

    authenticator_recurs(s)
}

fn authenticator_recurs(s: EndpointARecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_c, {
        Branches0AtoC::End(s) => {
            close_mpst(s)
        },
        Branches0AtoC::Video(s) => {
            let (request, s) = recv_mpst_a_from_c(s)?;
            let s = send_mpst_a_to_b(request + 1, s);
            let (video, s) = recv_mpst_a_from_b(s)?;
            let s = send_mpst_a_to_c(video + 1, s);
            authenticator_recurs(s)
        },
    })
}

fn client(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();

    let s = send_mpst_c_to_a(0, s);
    let (_, s) = recv_mpst_c_from_a(s)?;

    client_recurs(s, xs, 1)
}

fn client_recurs(
    s: EndpointCRecurs<i32>,
    mut xs: Vec<i32>,
    index: i32,
) -> Result<(), Box<dyn Error>> {
    match xs.pop()
    {
        Option::Some(_) =>
        {
            let s = choose_mpst_to_all!(
                s,
                Branches0AtoC::Video,
                Branches0BtoC::Video, =>
                NameC
            );

            let s = send_mpst_c_to_a(1, s);
            let (_, s) = recv_mpst_c_from_a(s)?;

            client_recurs(s, xs, index + 1)
        }
        Option::None =>
        {
            let s = choose_mpst_to_all!(
                s,
                Branches0AtoC::End,
                Branches0BtoC::End, =>
                NameC
            );

            assert_eq!(index, 100);

            close_mpst(s)
        }
    }
}

/////////////////////////////////////////

pub fn run_macro_recursive() {
    let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client);

    assert!(thread_a.join().is_ok());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_ok());
}
