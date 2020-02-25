extern crate mpst;

use mpst::binary::{cancel, End, Recv, Send, Session};
use mpst::role::Role;
use mpst::run_processes;
use mpst::sessionmpst::SessionMpst;

use std::boxed::Box;
use std::error::Error;
use std::marker;

use mpst::functionmpst::close::close_mpst;

use mpst::role::a_to_b::RoleAtoB;
use mpst::role::a_to_c::RoleAtoC;
use mpst::role::b_to_a::RoleBtoA;
use mpst::role::b_to_c::RoleBtoC;
use mpst::role::c_to_a::RoleCtoA;
use mpst::role::c_to_b::RoleCtoB;
use mpst::role::end::RoleEnd;

use mpst::functionmpst::recv::recv_mpst_a_to_b;
use mpst::functionmpst::recv::recv_mpst_a_to_c;
use mpst::functionmpst::recv::recv_mpst_b_to_a;
use mpst::functionmpst::recv::recv_mpst_b_to_c;
use mpst::functionmpst::recv::recv_mpst_c_to_a;

use mpst::functionmpst::send::send_mpst_a_to_b;
use mpst::functionmpst::send::send_mpst_a_to_c;
use mpst::functionmpst::send::send_mpst_b_to_a;
use mpst::functionmpst::send::send_mpst_c_to_a;
use mpst::functionmpst::send::send_mpst_c_to_b;

/// Test our usecase from Places 2020
/// Simple types
/// Client = C
/// Authenticator = A
/// Server = B

type AtoCClose = End;
type AtoBClose = End;
type AtoBVideo<N> = Send<N, Recv<N, End>>;

type InitA<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type BtoAClose = <AtoBClose as Session>::Dual;
type BtoCClose = End;
type BtoAVideo<N> = <AtoBVideo<N> as Session>::Dual;

type RecursAtoC<N> = Recv<CBranchesAtoC<N>, End>;
type RecursBtoC<N> = Recv<CBranchesBtoC<N>, End>;

enum CBranchesAtoC<N: marker::Send> {
    End((AtoBClose, AtoCClose, QueueAEnd)),
    Video((AtoBVideo<N>, Recv<N, Send<N, RecursAtoC<N>>>, QueueAVideo)),
}
enum CBranchesBtoC<N: marker::Send> {
    End((BtoAClose, BtoCClose, QueueBEnd)),
    Video((BtoAVideo<N>, RecursBtoC<N>, QueueBVideo)),
}
type ChooseCforAtoC<N> = Send<CBranchesAtoC<N>, End>;
type ChooseCforBtoC<N> = Send<CBranchesBtoC<N>, End>;

type InitC<N> = Send<N, Recv<N, ChooseCforAtoC<N>>>;

// /// Queues
type QueueAEnd = RoleEnd;
type QueueAVideo = RoleAtoC<RoleAtoB<RoleAtoB<RoleAtoC<RoleAtoC<RoleEnd>>>>>;
type QueueARecurs = RoleAtoC<RoleEnd>;
type QueueAInit = RoleAtoC<RoleAtoC<RoleAtoC<RoleEnd>>>;

type QueueBEnd = RoleEnd;
type QueueBVideo = RoleBtoA<RoleBtoA<RoleBtoC<RoleEnd>>>;
type QueueBRecurs = RoleBtoC<RoleEnd>;

type QueueCRecurs = RoleCtoA<RoleCtoB<RoleEnd>>;
type QueueCFull = RoleCtoA<RoleCtoA<QueueCRecurs>>;

/// Creating the MP sessions
/// For C

type EndpointCRecurs<N> = SessionMpst<ChooseCforAtoC<N>, ChooseCforBtoC<N>, QueueCRecurs>;
type EndpointCFull<N> = SessionMpst<InitC<N>, ChooseCforBtoC<N>, QueueCFull>;

/// For A
type EndpointARecurs<N> = SessionMpst<End, RecursAtoC<N>, QueueARecurs>;
type EndpointAFull<N> = SessionMpst<End, InitA<N>, QueueAInit>;

/// For B
type EndpointBRecurs<N> = SessionMpst<End, RecursBtoC<N>, QueueBRecurs>;

/// Functions related to endpoints
fn server(s: EndpointBRecurs<i32>) -> Result<(), Box<dyn Error>> {
    let (l, s) = recv_mpst_b_to_c(s)?;
    cancel(s);

    match l {
        CBranchesBtoC::End((session_ba, session_bc, queue)) => {
            let s = SessionMpst {
                session1: session_ba,
                session2: session_bc,
                queue: queue,
            };

            close_mpst(s)?;
            Ok(())
        }
        CBranchesBtoC::Video((session_ba, session_bc, queue)) => {
            let s = SessionMpst {
                session1: session_ba,
                session2: session_bc,
                queue: queue,
            };

            let (request, s) = recv_mpst_b_to_a(s)?;
            let s = send_mpst_b_to_a(request + 1, s);
            server(s)
        }
    }
}

fn authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_a_to_c(s)?;
    let s = send_mpst_a_to_c(id + 1, s);

    let result = authenticator_recurs(s)?;

    Ok(result)
}

fn authenticator_recurs(s: EndpointARecurs<i32>) -> Result<(), Box<dyn Error>> {
    let (l, s) = recv_mpst_a_to_c(s)?;
    cancel(s);

    match l {
        CBranchesAtoC::End((session_ab, session_ac, queue)) => {
            let s = SessionMpst {
                session1: session_ab,
                session2: session_ac,
                queue: queue,
            };

            close_mpst(s)?;
            Ok(())
        }
        CBranchesAtoC::Video((session_ab, session_ac, queue)) => {
            let s = SessionMpst {
                session1: session_ab,
                session2: session_ac,
                queue: queue,
            };

            let (request, s) = recv_mpst_a_to_c(s)?;
            let s = send_mpst_a_to_b(request + 1, s);
            let (video, s) = recv_mpst_a_to_b(s)?;
            let s = send_mpst_a_to_c(video + 1, s);
            authenticator_recurs(s)
        }
    }
}

fn client(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let xs: Vec<i32> = (1..10).map(|_| 1).collect();

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
            let (session_ac, session_ca) = <_ as Session>::new();
            let (session_bc, session_cb) = <_ as Session>::new();
            let (session_ab, session_ba) = <_ as Session>::new();
            let (queue_a, _) = <_ as Role>::new();
            let (queue_b, _) = <_ as Role>::new();
            let (queue_c, _) = <_ as Role>::new();

            let s = send_mpst_c_to_a(CBranchesAtoC::Video((session_ab, session_ac, queue_a)), s);
            let s = send_mpst_c_to_b(CBranchesBtoC::Video((session_ba, session_bc, queue_b)), s);

            cancel(s);

            let ns = SessionMpst {
                session1: session_ca,
                session2: session_cb,
                queue: queue_c,
            };

            let ns = send_mpst_c_to_a(1, ns);
            let (_, ns) = recv_mpst_c_to_a(ns)?;

            client_recurs(ns, xs, index + 1)
        }
        Option::None => {
            let (session_ac, session_ca) = <_ as Session>::new();
            let (session_bc, session_cb) = <_ as Session>::new();
            let (session_ab, session_ba) = <_ as Session>::new();
            let (queue_a, _) = <_ as Role>::new();
            let (queue_b, _) = <_ as Role>::new();
            let (queue_c, _) = <_ as Role>::new();

            let s = send_mpst_c_to_a(CBranchesAtoC::End((session_ab, session_ac, queue_a)), s);
            let s = send_mpst_c_to_b(CBranchesBtoC::End((session_ba, session_bc, queue_b)), s);

            cancel(s);

            let ns = SessionMpst {
                session1: session_ca,
                session2: session_cb,
                queue: queue_c,
            };

            close_mpst(ns)?;

            assert_eq!(index, 10);

            Ok(())
        }
    }
}

#[test]
fn run_usecase() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test whole usecase.
        {
            let (thread_a, thread_b, thread_c) = run_processes(authenticator, server, client);

            assert!(thread_a.is_ok());
            assert!(thread_b.is_ok());
            assert!(thread_c.is_ok());
        }
        Ok(())
    }()
    .is_ok());
}
