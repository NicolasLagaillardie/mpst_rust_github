// Test for parametrisation on the number of roles
use rand::{thread_rng, Rng};

use mpstthree::binary::{End, Recv, Send, Session};
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::{
    bundle_fork_multi, choose_mpst_multi_to_all, close_mpst, create_broadcast_role,
    create_normal_role, create_recv_mpst_session, create_send_mpst_session, create_sessionmpst,
    offer_mpst,
};
use std::error::Error;
use std::marker;

// Create new SessionMpst for three participants
create_sessionmpst!(SessionMpst, 3);

// Create new roles
// normal
create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
create_normal_role!(RoleD, next_d, RoleDDual, next_d_dual);
// broadcast
create_broadcast_role!(RoleAlltoD, next_all_to_d, RoleDtoAll, next_d_to_all);

// Create new send functions
create_send_mpst_session!(send_mpst_d_to_a, RoleA, next_a, RoleD, SessionMpst, 3, 1);
create_send_mpst_session!(send_mpst_a_to_d, RoleD, next_d, RoleA, SessionMpst, 3, 2);
create_send_mpst_session!(send_mpst_d_to_b, RoleB, next_b, RoleD, SessionMpst, 3, 2);
create_send_mpst_session!(send_mpst_b_to_a, RoleA, next_a, RoleB, SessionMpst, 3, 1);
create_send_mpst_session!(send_mpst_a_to_b, RoleB, next_b, RoleA, SessionMpst, 3, 1);

// Create new recv functions and related types
// normal
create_recv_mpst_session!(recv_mpst_d_to_a, RoleA, next_a, RoleD, SessionMpst, 3, 1);
create_recv_mpst_session!(recv_mpst_a_to_d, RoleD, next_d, RoleA, SessionMpst, 3, 2);
create_recv_mpst_session!(recv_mpst_b_to_d, RoleD, next_d, RoleB, SessionMpst, 3, 2);
create_recv_mpst_session!(recv_mpst_b_to_a, RoleA, next_a, RoleB, SessionMpst, 3, 1);
create_recv_mpst_session!(recv_mpst_a_to_b, RoleB, next_b, RoleA, SessionMpst, 3, 1);

close_mpst!(close_mpst_multi, SessionMpst, 3);

bundle_fork_multi!(fork_mpst, fork_simple, SessionMpst, 3);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameD = RoleD<RoleEnd>;

/// Test our usecase
/// Simple types
/// Client = C
/// Authenticator = A
/// Server = B

type AtoDClose = End;
type AtoBClose = End;
type AtoBVideo<N> = Send<N, Recv<N, End>>;
type AtoDVideo<N> = Recv<N, Send<N, RecursAtoD<N>>>;

type InitA<N> = Recv<N, Send<N, RecursAtoD<N>>>;

type BtoAClose = <AtoBClose as Session>::Dual;
type BtoDClose = End;
type BtoAVideo<N> = <AtoBVideo<N> as Session>::Dual;

type RecursAtoD<N> = Recv<Branches0AtoD<N>, End>;
type RecursBtoD<N> = Recv<Branches0BtoD<N>, End>;

enum Branches0AtoD<N: marker::Send> {
    End(SessionMpst<AtoBClose, AtoDClose, QueueAEnd, NameA>),
    Video(SessionMpst<AtoBVideo<N>, AtoDVideo<N>, QueueAVideo, NameA>),
}
enum Branches0BtoD<N: marker::Send> {
    End(SessionMpst<BtoAClose, BtoDClose, QueueBEnd, NameB>),
    Video(SessionMpst<BtoAVideo<N>, RecursBtoD<N>, QueueBVideo, NameB>),
}
type ChooseCforAtoD<N> = Send<Branches0AtoD<N>, End>;
type ChooseCforBtoD<N> = Send<Branches0BtoD<N>, End>;

type InitD<N> = Send<N, Recv<N, ChooseCforAtoD<N>>>;

/// Queues
type QueueAEnd = RoleEnd;
type QueueAVideo = RoleD<RoleB<RoleB<RoleD<RoleD<RoleEnd>>>>>;
type QueueARecurs = RoleD<RoleEnd>;
type QueueAInit = RoleD<RoleD<RoleD<RoleEnd>>>;

type QueueBEnd = RoleEnd;
type QueueBVideo = RoleA<RoleA<RoleD<RoleEnd>>>;
type QueueBRecurs = RoleD<RoleEnd>;

type QueueDRecurs = RoleA<RoleB<RoleEnd>>;
type QueueDFull = RoleA<RoleA<QueueDRecurs>>;

/// Creating the MP sessions
/// For C

type EndpointDRecurs<N> = SessionMpst<ChooseCforAtoD<N>, ChooseCforBtoD<N>, QueueDRecurs, NameD>;
type EndpointDFull<N> = SessionMpst<InitD<N>, ChooseCforBtoD<N>, QueueDFull, NameD>;

/// For A
type EndpointARecurs<N> = SessionMpst<End, RecursAtoD<N>, QueueARecurs, NameA>;
type EndpointAFull<N> = SessionMpst<End, InitA<N>, QueueAInit, NameA>;

/// For B
type EndpointBRecurs<N> = SessionMpst<End, RecursBtoD<N>, QueueBRecurs, NameB>;

/// Functions related to endpoints
fn server(s: EndpointBRecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_to_d, {
        Branches0BtoD::End(s) => {
            close_mpst_multi(s)
        },
        Branches0BtoD::Video(s) => {
            let (request, s) = recv_mpst_b_to_a(s)?;
            let s = send_mpst_b_to_a(request + 1, s);
            server(s)
        },
    })
}

fn authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_a_to_d(s)?;
    let s = send_mpst_a_to_d(id + 1, s);

    authenticator_recurs(s)
}

fn authenticator_recurs(s: EndpointARecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_to_d, {
        Branches0AtoD::End(s) => {
            close_mpst_multi(s)
        },
        Branches0AtoD::Video(s) => {
            let (request, s) = recv_mpst_a_to_d(s)?;
            let s = send_mpst_a_to_b(request + 1, s);
            let (video, s) = recv_mpst_a_to_b(s)?;
            let s = send_mpst_a_to_d(video + 1, s);
            authenticator_recurs(s)
        },
    })
}

fn client(s: EndpointDFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();

    let s = send_mpst_d_to_a(0, s);
    let (_, s) = recv_mpst_d_to_a(s)?;

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
                send_mpst_d_to_a,
                send_mpst_d_to_b, =>
                Branches0AtoD::Video,
                Branches0BtoD::Video, =>
                RoleA,
                RoleB, =>
                RoleD,
                SessionMpst,
                3,
                3
            );

            let s = send_mpst_d_to_a(1, s);
            let (_, s) = recv_mpst_d_to_a(s)?;

            client_recurs(s, xs, index + 1)
        }
        Option::None => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_d_to_a,
                send_mpst_d_to_b, =>
                Branches0AtoD::End,
                Branches0BtoD::End, =>
                RoleA,
                RoleB, =>
                RoleD,
                SessionMpst,
                3,
                3
            );

            assert_eq!(index, 100);

            close_mpst_multi(s)
        }
    }
}

////////////////////////////////////////

fn new_run_usecase_recursive() {
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

/////////////////////////////////////////

fn main() {
    new_run_usecase_recursive();
}
