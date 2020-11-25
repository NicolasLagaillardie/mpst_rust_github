// Test for parametrisation on the number of roles
use rand::{thread_rng, Rng};

use mpstthree::binary::{End, Recv, Send, Session};
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::{
    choose_mpst_X_to_all, close_mpst, create_broadcast_role, create_choose_mpst_session_multi_left,
    create_choose_mpst_session_multi_right, create_choose_type_multi, create_normal_role,
    create_offer_mpst_session_multi, create_offer_type_multi, create_recv_mpst_all_session,
    create_recv_mpst_session, create_send_mpst_session, create_sessionmpst, fork_mpst_multi,
    fork_simple_multi, offer_mpst,
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
// broadcast
create_recv_mpst_all_session!(
    recv_mpst_b_all_to_d,
    RoleAlltoD,
    next_all_to_d,
    RoleB,
    SessionMpst,
    3,
    2
);
create_recv_mpst_all_session!(
    recv_mpst_a_all_to_d,
    RoleAlltoD,
    next_all_to_d,
    RoleA,
    SessionMpst,
    3,
    2
);

close_mpst!(close_mpst_multi, SessionMpst, 3);

create_offer_type_multi!(OfferMpstMultiThree, SessionMpst, 3);
create_choose_type_multi!(ChooseMpstThree, SessionMpst, 3);

create_offer_mpst_session_multi!(
    offer_mpst_session_a_to_d,
    OfferMpstMultiThree,
    RoleAlltoD,
    recv_mpst_a_all_to_d,
    RoleA,
    SessionMpst,
    3,
    2
);

create_offer_mpst_session_multi!(
    offer_mpst_session_b_to_d,
    OfferMpstMultiThree,
    RoleAlltoD,
    recv_mpst_b_all_to_d,
    RoleB,
    SessionMpst,
    3,
    2
);

create_choose_mpst_session_multi_left!(
    choose_left_mpst_session_d_to_all,
    ChooseMpstThree,
    RoleDtoAll,
    next_d_to_all,
    RoleD,
    SessionMpst,
    3
);

create_choose_mpst_session_multi_right!(
    choose_right_mpst_session_d_to_all,
    ChooseMpstThree,
    RoleDtoAll,
    next_d_to_all,
    RoleD,
    SessionMpst,
    3
);

fork_simple_multi!(fork_simple, SessionMpst, 3);
fork_mpst_multi!(fork_mpst, fork_simple, SessionMpst, 3);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameD = RoleD<RoleEnd>;

/// Test our usecase
/// Simple types
/// Client = C
/// Authenticator = A
/// Server = B

type AtoCClose = End;
type AtoBClose = End;
type AtoBVideo<N> = Send<N, Recv<N, End>>;
type AtoCVideo<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type InitA<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type BtoAClose = <AtoBClose as Session>::Dual;
type BtoCClose = End;
type BtoAVideo<N> = <AtoBVideo<N> as Session>::Dual;

type RecursAtoC<N> = Recv<CBranchesAtoC<N>, End>;
type RecursBtoC<N> = Recv<CBranchesBtoC<N>, End>;

enum CBranchesAtoC<N: marker::Send> {
    End(SessionMpst<AtoBClose, AtoCClose, QueueAEnd, NameA>),
    Video(SessionMpst<AtoBVideo<N>, AtoCVideo<N>, QueueAVideo, NameA>),
}
enum CBranchesBtoC<N: marker::Send> {
    End(SessionMpst<BtoAClose, BtoCClose, QueueBEnd, NameB>),
    Video(SessionMpst<BtoAVideo<N>, RecursBtoC<N>, QueueBVideo, NameB>),
}
type ChooseCforAtoC<N> = Send<CBranchesAtoC<N>, End>;
type ChooseCforBtoC<N> = Send<CBranchesBtoC<N>, End>;

type InitC<N> = Send<N, Recv<N, ChooseCforAtoC<N>>>;

/// Queues
type QueueAEnd = RoleEnd;
type QueueAVideo = RoleD<RoleB<RoleB<RoleD<RoleD<RoleEnd>>>>>;
type QueueARecurs = RoleD<RoleEnd>;
type QueueAInit = RoleD<RoleD<RoleD<RoleEnd>>>;

type QueueBEnd = RoleEnd;
type QueueBVideo = RoleA<RoleA<RoleD<RoleEnd>>>;
type QueueBRecurs = RoleD<RoleEnd>;

type QueueCRecurs = RoleA<RoleB<RoleEnd>>;
type QueueCFull = RoleA<RoleA<QueueCRecurs>>;

/// Creating the MP sessions
/// For C

type EndpointCRecurs<N> = SessionMpst<ChooseCforAtoC<N>, ChooseCforBtoC<N>, QueueCRecurs, NameD>;
type EndpointCFull<N> = SessionMpst<InitC<N>, ChooseCforBtoC<N>, QueueCFull, NameD>;

/// For A
type EndpointARecurs<N> = SessionMpst<End, RecursAtoC<N>, QueueARecurs, NameA>;
type EndpointAFull<N> = SessionMpst<End, InitA<N>, QueueAInit, NameA>;

/// For B
type EndpointBRecurs<N> = SessionMpst<End, RecursBtoC<N>, QueueBRecurs, NameB>;

/// Functions related to endpoints
fn server(s: EndpointBRecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_to_d, {
        CBranchesBtoC::End(s) => {
            close_mpst_multi(s)
        },
        CBranchesBtoC::Video(s) => {
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
        CBranchesAtoC::End(s) => {
            close_mpst_multi(s)
        },
        CBranchesAtoC::Video(s) => {
            let (request, s) = recv_mpst_a_to_d(s)?;
            let s = send_mpst_a_to_b(request + 1, s);
            let (video, s) = recv_mpst_a_to_b(s)?;
            let s = send_mpst_a_to_d(video + 1, s);
            authenticator_recurs(s)
        },
    })
}

fn client(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();

    let s = send_mpst_d_to_a(0, s);
    let (_, s) = recv_mpst_d_to_a(s)?;

    client_recurs(s, xs, 1)
}

fn client_recurs(
    s: EndpointCRecurs<i32>,
    mut xs: Vec<i32>,
    index: i32,
) -> Result<(), Box<dyn Error>> {
    match xs.pop() {
        Option::Some(_) => {
            let s = choose_mpst_X_to_all!(
                s,
                send_mpst_d_to_a,
                send_mpst_d_to_b, =>
                CBranchesAtoC::Video,
                CBranchesBtoC::Video, =>
                RoleA,
                RoleB, =>
                RoleD,
                SessionMpst,
                3
            );

            let s = send_mpst_d_to_a(1, s);
            let (_, s) = recv_mpst_d_to_a(s)?;

            client_recurs(s, xs, index + 1)
        }
        Option::None => {
            let s = choose_mpst_X_to_all!(
                s,
                send_mpst_d_to_a,
                send_mpst_d_to_b, =>
                CBranchesAtoC::End,
                CBranchesBtoC::End, =>
                RoleA,
                RoleB, =>
                RoleD,
                SessionMpst,
                3
            );

            assert_eq!(index, 100);

            close_mpst_multi(s)
        }
    }
}

////////////////////////////////////////

#[test]
fn new_run_usecase_recursive() {
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
