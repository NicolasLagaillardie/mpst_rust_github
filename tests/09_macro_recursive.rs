// Test for Macro, exact same as usecase-recursive
use mpstthree::binary::{End, Recv, Send, Session};
use mpstthree::fork::fork_mpst;
use mpstthree::functionmpst::close::close_mpst;
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::sessionmpst::SessionMpst;
use std::error::Error;
use std::marker;

use rand::{thread_rng, Rng};

use mpstthree::{
    choose_mpst_to_all, create_normal_role, create_recv_mpst_session_1, create_recv_mpst_session_2,
    create_send_mpst_session_1, create_send_mpst_session_2, offer_mpst,
};

// Create new roles
create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);

// Create new send functions
create_send_mpst_session_1!(send_mpst_c_to_a, RoleA, next_a, RoleC);
create_send_mpst_session_2!(send_mpst_a_to_c, RoleC, next_c, RoleA);
create_send_mpst_session_2!(send_mpst_c_to_b, RoleB, next_b, RoleC);
create_send_mpst_session_1!(send_mpst_b_to_a, RoleA, next_a, RoleB);
create_send_mpst_session_1!(send_mpst_a_to_b, RoleB, next_b, RoleA);

// Create new recv functions and related types
create_recv_mpst_session_1!(recv_mpst_c_to_a, RoleA, next_a, RoleC);
create_recv_mpst_session_2!(recv_mpst_a_to_c, RoleC, next_c, RoleA);
create_recv_mpst_session_2!(recv_mpst_b_to_c, RoleC, next_c, RoleB);
create_recv_mpst_session_1!(recv_mpst_b_to_a, RoleA, next_a, RoleB);
create_recv_mpst_session_1!(recv_mpst_a_to_b, RoleB, next_b, RoleA);

// Types
type AtoBVideo<N> = Send<N, Recv<N, End>>;
type AtoCVideo<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type InitA<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type BtoAVideo<N> = <AtoBVideo<N> as Session>::Dual;

type RecursAtoC<N> = Recv<CBranchesAtoC<N>, End>;
type RecursBtoC<N> = Recv<CBranchesBtoC<N>, End>;

enum CBranchesAtoC<N: marker::Send> {
    End(SessionMpst<End, End, RoleEnd, RoleA<RoleEnd>>),
    Video(SessionMpst<AtoBVideo<N>, AtoCVideo<N>, QueueAVideo, RoleA<RoleEnd>>),
}
enum CBranchesBtoC<N: marker::Send> {
    End(SessionMpst<End, End, RoleEnd, RoleB<RoleEnd>>),
    Video(SessionMpst<BtoAVideo<N>, RecursBtoC<N>, QueueBVideo, RoleB<RoleEnd>>),
}
type ChooseCforAtoC<N> = Send<CBranchesAtoC<N>, End>;
type ChooseCforBtoC<N> = Send<CBranchesBtoC<N>, End>;

type InitC<N> = Send<N, Recv<N, ChooseCforAtoC<N>>>;

/// Queues
type QueueAVideo = RoleC<RoleB<RoleB<RoleC<RoleC<RoleEnd>>>>>;
type QueueAInit = RoleC<RoleC<RoleC<RoleEnd>>>;

type QueueBVideo = RoleA<RoleA<RoleC<RoleEnd>>>;

type QueueCRecurs = RoleA<RoleB<RoleEnd>>;
type QueueCFull = RoleA<RoleA<QueueCRecurs>>;

/// Creating the MP sessions
/// For C

type EndpointCRecurs<N> =
    SessionMpst<ChooseCforAtoC<N>, ChooseCforBtoC<N>, QueueCRecurs, RoleC<RoleEnd>>;
type EndpointCFull<N> = SessionMpst<InitC<N>, ChooseCforBtoC<N>, QueueCFull, RoleC<RoleEnd>>;

/// For A
type EndpointARecurs<N> = SessionMpst<End, RecursAtoC<N>, RoleC<RoleEnd>, RoleA<RoleEnd>>;
type EndpointAFull<N> = SessionMpst<End, InitA<N>, QueueAInit, RoleA<RoleEnd>>;

/// For B
type EndpointBRecurs<N> = SessionMpst<End, RecursBtoC<N>, RoleC<RoleEnd>, RoleB<RoleEnd>>;

/// Functions related to endpoints
fn server(s: EndpointBRecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_to_c, {
        CBranchesBtoC::End(s) => {
            close_mpst(s)
        },
        CBranchesBtoC::Video(s) => {
            let (request, s) = recv_mpst_b_to_a(s)?;
            let s = send_mpst_b_to_a(request + 1, s);
            server(s)
        },
    })
}

fn authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_a_to_c(s)?;
    let s = send_mpst_a_to_c(id + 1, s);

    authenticator_recurs(s)
}

fn authenticator_recurs(s: EndpointARecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_to_c, {
        CBranchesAtoC::End(s) => {
            close_mpst(s)
        },
        CBranchesAtoC::Video(s) => {
            let (request, s) = recv_mpst_a_to_c(s)?;
            let s = send_mpst_a_to_b(request + 1, s);
            let (video, s) = recv_mpst_a_to_b(s)?;
            let s = send_mpst_a_to_c(video + 1, s);
            authenticator_recurs(s)
        },
    })
}

fn client(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();

    let s = send_mpst_c_to_a(0, s);
    let (_, s) = recv_mpst_c_to_a(s)?;

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
                CBranchesAtoC::Video,
                CBranchesBtoC::Video,
                send_mpst_c_to_a,
                send_mpst_c_to_b,
                RoleA,
                RoleB,
                RoleC
            );

            let s = send_mpst_c_to_a(1, s);
            let (_, s) = recv_mpst_c_to_a(s)?;

            client_recurs(s, xs, index + 1)
        }
        Option::None => {
            let s = choose_mpst_to_all!(
                s,
                CBranchesAtoC::End,
                CBranchesBtoC::End,
                send_mpst_c_to_a,
                send_mpst_c_to_b,
                RoleA,
                RoleB,
                RoleC
            );

            assert_eq!(index, 100);

            close_mpst(s)
        }
    }
}

#[test]
fn run_macro_recursive() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client);

            assert!(thread_a.is_ok());
            assert!(thread_b.is_ok());
            assert!(thread_c.is_ok());
        }
        Ok(())
    }()
    .is_ok());
}
