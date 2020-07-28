extern crate crossbeam_channel;
extern crate mpstthree;
extern crate rand;

use crossbeam_channel::{bounded, Sender};
use mpstthree::binary::{cancel, recv, send, End, Recv, Send, Session};
use mpstthree::fork_mpst;
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
create_normal_role!(RoleAtoB, next_a_to_b, RoleBtoA, next_b_to_a);
create_normal_role!(RoleAtoC, next_a_to_c, RoleCtoA, next_c_to_a);
create_normal_role!(RoleCtoB, next_c_to_b, RoleBtoC, next_b_to_c);

// Create new send functions
create_send_mpst_session_1!(send_mpst_c_to_a, RoleCtoA, next_c_to_a);
create_send_mpst_session_2!(send_mpst_a_to_c, RoleAtoC, next_a_to_c);
create_send_mpst_session_2!(send_mpst_c_to_b, RoleCtoB, next_c_to_b);
create_send_mpst_session_1!(send_mpst_b_to_a, RoleBtoA, next_b_to_a);
create_send_mpst_session_1!(send_mpst_a_to_b, RoleAtoB, next_a_to_b);

// Create new recv functions and related types
create_recv_mpst_session_1!(recv_mpst_c_to_a, RoleCtoA, next_c_to_a);
create_recv_mpst_session_2!(recv_mpst_a_to_c, RoleAtoC, next_a_to_c);
create_recv_mpst_session_2!(recv_mpst_b_to_c, RoleBtoC, next_b_to_c);
create_recv_mpst_session_1!(recv_mpst_b_to_a, RoleBtoA, next_b_to_a);
create_recv_mpst_session_1!(recv_mpst_a_to_b, RoleAtoB, next_a_to_b);

// Types
type AtoBVideo<N> = Send<N, Recv<N, End>>;
type AtoCVideo<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type InitA<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type BtoAVideo<N> = <AtoBVideo<N> as Session>::Dual;

type RecursAtoC<N> = Recv<CBranchesAtoC<N>, End>;
type RecursBtoC<N> = Recv<CBranchesBtoC<N>, End>;

enum CBranchesAtoC<N: marker::Send> {
    End(SessionMpst<End, End, RoleEnd>),
    Video(SessionMpst<AtoBVideo<N>, AtoCVideo<N>, QueueAVideo>),
}
enum CBranchesBtoC<N: marker::Send> {
    End(SessionMpst<End, End, RoleEnd>),
    Video(SessionMpst<BtoAVideo<N>, RecursBtoC<N>, QueueBVideo>),
}
type ChooseCforAtoC<N> = Send<CBranchesAtoC<N>, End>;
type ChooseCforBtoC<N> = Send<CBranchesBtoC<N>, End>;

type InitC<N> = Send<N, Recv<N, ChooseCforAtoC<N>>>;

/// Queues
type QueueAVideo = RoleAtoC<RoleAtoB<RoleAtoB<RoleAtoC<RoleAtoC<RoleEnd>>>>>;
type QueueAInit = RoleAtoC<RoleAtoC<RoleAtoC<RoleEnd>>>;

type QueueBVideo = RoleBtoA<RoleBtoA<RoleBtoC<RoleEnd>>>;

type QueueCRecurs = RoleCtoA<RoleCtoB<RoleEnd>>;
type QueueCFull = RoleCtoA<RoleCtoA<QueueCRecurs>>;

/// Creating the MP sessions
/// For C

type EndpointCRecurs<N> = SessionMpst<ChooseCforAtoC<N>, ChooseCforBtoC<N>, QueueCRecurs>;
type EndpointCFull<N> = SessionMpst<InitC<N>, ChooseCforBtoC<N>, QueueCFull>;

/// For A
type EndpointARecurs<N> = SessionMpst<End, RecursAtoC<N>, RoleAtoC<RoleEnd>>;
type EndpointAFull<N> = SessionMpst<End, InitA<N>, QueueAInit>;

/// For B
type EndpointBRecurs<N> = SessionMpst<End, RecursBtoC<N>, RoleBtoC<RoleEnd>>;

/// Functions related to endpoints
fn server(s: EndpointBRecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_to_c, {
        CBranchesBtoC::End(s) => {
            close_mpst(s)?;
            Ok(())
        },
        CBranchesBtoC::Video(s) => {
            let (request, s) = recv_mpst_b_to_a(s)?;
            let s = send_mpst_b_to_a(request + 1, s);
            server(s)
        },
    })?;
    Ok(())
}

fn authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_a_to_c(s)?;
    let s = send_mpst_a_to_c(id + 1, s);

    let result = authenticator_recurs(s)?;

    Ok(result)
}

fn authenticator_recurs(s: EndpointARecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_to_c, {
        CBranchesAtoC::End(s) => {
            close_mpst(s)?;
            Ok(())
        },
        CBranchesAtoC::Video(s) => {
            let (request, s) = recv_mpst_a_to_c(s)?;
            let s = send_mpst_a_to_b(request + 1, s);
            let (video, s) = recv_mpst_a_to_b(s)?;
            let s = send_mpst_a_to_c(video + 1, s);
            authenticator_recurs(s)
        },
    })?;
    Ok(())
}

fn client(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();

    let s = send_mpst_c_to_a(0, s);
    let (_, s) = recv_mpst_c_to_a(s)?;

    let result = client_recurs(s, xs, 1)?;

    Ok(result)
}

fn client_recurs(
    s: EndpointCRecurs<i32>,
    mut xs: Vec<i32>,
    index: i32,
) -> Result<(), Box<dyn Error>> {
    match xs.pop() {
        Option::Some(_) => {
            let s = choose_mpst_to_all!(
                CBranchesAtoC::Video,
                CBranchesBtoC::Video,
                s,
                send_mpst_c_to_a,
                send_mpst_c_to_b
            );

            let s = send_mpst_c_to_a(1, s);
            let (_, s) = recv_mpst_c_to_a(s)?;

            client_recurs(s, xs, index + 1)
        }
        Option::None => {
            let s = choose_mpst_to_all!(
                CBranchesAtoC::End,
                CBranchesBtoC::End,
                s,
                send_mpst_c_to_a,
                send_mpst_c_to_b
            );

            close_mpst(s)?;

            assert_eq!(index, 100);

            Ok(())
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
