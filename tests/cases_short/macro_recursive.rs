// Test for Macro, exact same as usecase-recursive
use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::fork::fork_mpst;
use mpstthree::functionmpst::close::close_mpst;
use mpstthree::role::end::RoleEnd;
use mpstthree::sessionmpst::SessionMpst;
use std::error::Error;
use std::marker;

use rand::{thread_rng, Rng};

use mpstthree::{
    choose_mpst_to_all, create_multiple_normal_role, create_recv_mpst_session_1,
    create_recv_mpst_session_2, create_send_mpst_session_1, create_send_mpst_session_2, offer_mpst,
};

// Create new roles
create_multiple_normal_role!(
    RoleA, next_a, RoleADual, next_a_dual |
    RoleB, next_b, RoleBDual, next_b_dual |
    RoleC, next_c, RoleCDual, next_c_dual |
);

// Create new send functions
create_send_mpst_session_1!(send_mpst_c_to_a, RoleA, RoleC);
create_send_mpst_session_2!(send_mpst_a_to_c, RoleC, RoleA);
create_send_mpst_session_2!(send_mpst_c_to_b, RoleB, RoleC);
create_send_mpst_session_1!(send_mpst_b_to_a, RoleA, RoleB);
create_send_mpst_session_1!(send_mpst_a_to_b, RoleB, RoleA);

// Create new recv functions and related types
create_recv_mpst_session_1!(recv_mpst_c_from_a, RoleA, RoleC);
create_recv_mpst_session_2!(recv_mpst_a_from_c, RoleC, RoleA);
create_recv_mpst_session_2!(recv_mpst_b_from_c, RoleC, RoleB);
create_recv_mpst_session_1!(recv_mpst_b_from_a, RoleA, RoleB);
create_recv_mpst_session_1!(recv_mpst_a_from_b, RoleB, RoleA);

// Types
type AtoBVideo<N> = Send<N, Recv<N, End>>;
type AtoCVideo<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type InitA<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type BtoAVideo<N> = <AtoBVideo<N> as Session>::Dual;

type RecursAtoC<N> = Recv<Branches0AtoC<N>, End>;
type RecursBtoC<N> = Recv<Branches0BtoC<N>, End>;

enum Branches0AtoC<N: marker::Send> {
    End(SessionMpst<End, End, RoleEnd, RoleA<RoleEnd>>),
    Video(SessionMpst<AtoBVideo<N>, AtoCVideo<N>, QueueAVideo, RoleA<RoleEnd>>),
}
enum Branches0BtoC<N: marker::Send> {
    End(SessionMpst<End, End, RoleEnd, RoleB<RoleEnd>>),
    Video(SessionMpst<BtoAVideo<N>, RecursBtoC<N>, QueueBVideo, RoleB<RoleEnd>>),
}
type Choose0fromCtoA<N> = Send<Branches0AtoC<N>, End>;
type Choose0fromCtoB<N> = Send<Branches0BtoC<N>, End>;

type InitC<N> = Send<N, Recv<N, Choose0fromCtoA<N>>>;

/// Queues
type QueueAVideo = RoleC<RoleB<RoleB<RoleC<RoleC<RoleEnd>>>>>;
type QueueAInit = RoleC<RoleC<RoleC<RoleEnd>>>;

type QueueBVideo = RoleA<RoleA<RoleC<RoleEnd>>>;

type QueueCRecurs = RoleA<RoleB<RoleEnd>>;
type QueueCFull = RoleA<RoleA<QueueCRecurs>>;

/// Creating the MP sessions
/// For C

type EndpointCRecurs<N> =
    SessionMpst<Choose0fromCtoA<N>, Choose0fromCtoB<N>, QueueCRecurs, RoleC<RoleEnd>>;
type EndpointCFull<N> = SessionMpst<InitC<N>, Choose0fromCtoB<N>, QueueCFull, RoleC<RoleEnd>>;

/// For A
type EndpointARecurs<N> = SessionMpst<End, RecursAtoC<N>, RoleC<RoleEnd>, RoleA<RoleEnd>>;
type EndpointAFull<N> = SessionMpst<End, InitA<N>, QueueAInit, RoleA<RoleEnd>>;

/// For B
type EndpointBRecurs<N> = SessionMpst<End, RecursBtoC<N>, RoleC<RoleEnd>, RoleB<RoleEnd>>;

/// Functions related to endpoints
fn server(s: EndpointBRecurs<i32>) -> Result<(), Box<dyn Error>> {
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
    match xs.pop() {
        Option::Some(_) => {
            let s = choose_mpst_to_all!(
                s,
                Branches0AtoC::Video,
                Branches0BtoC::Video, =>
                send_mpst_c_to_a,
                send_mpst_c_to_b, =>
                RoleA,
                RoleB, =>
                RoleC
            );

            let s = send_mpst_c_to_a(1, s);
            let (_, s) = recv_mpst_c_from_a(s)?;

            client_recurs(s, xs, index + 1)
        }
        Option::None => {
            let s = choose_mpst_to_all!(
                s,
                Branches0AtoC::End,
                Branches0BtoC::End, =>
                send_mpst_c_to_a,
                send_mpst_c_to_b, =>
                RoleA,
                RoleB, =>
                RoleC
            );

            assert_eq!(index, 100);

            close_mpst(s)
        }
    }
}

/////////////////////////////////////////

pub fn run_macro_recursive() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}
