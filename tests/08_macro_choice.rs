// Test for Macro, exact same as usecase
extern crate mpstthree;
extern crate rand;

use mpstthree::binary::{End, Recv, Send, Session};
use mpstthree::fork::fork_mpst;
use mpstthree::functionmpst::close::close_mpst;
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::sessionmpst::SessionMpst;
use std::error::Error;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

use rand::{thread_rng, Rng};

use mpstthree::{
    create_broadcast_role, create_choose_left_from_3_to_1_and_2,
    create_choose_right_from_3_to_1_and_2, create_normal_role, create_offer_mpst_session_2,
    create_recv_mpst_all_session_2, create_recv_mpst_session_1, create_recv_mpst_session_2,
    create_send_mpst_session_1, create_send_mpst_session_2,
};

// Create new roles
// normal
create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
// broadcast
create_broadcast_role!(RoleAlltoC, next_all_to_c, RoleCtoAll, next_c_to_all);

// Create new send functions
create_send_mpst_session_1!(send_mpst_c_to_a, RoleA, next_a, RoleC);
create_send_mpst_session_2!(send_mpst_a_to_c, RoleC, next_c, RoleA);
create_send_mpst_session_2!(send_mpst_c_to_b, RoleB, next_b, RoleC);
create_send_mpst_session_1!(send_mpst_b_to_a, RoleA, next_a, RoleB);
create_send_mpst_session_1!(send_mpst_a_to_b, RoleB, next_b, RoleA);

// Create new recv functions and related types
// normal
create_recv_mpst_session_1!(recv_mpst_c_to_a, RoleA, next_a, RoleC);
create_recv_mpst_session_2!(recv_mpst_a_to_c, RoleC, next_c, RoleA);
create_recv_mpst_session_2!(recv_mpst_b_to_c, RoleC, next_c, RoleB);
create_recv_mpst_session_1!(recv_mpst_b_to_a, RoleA, next_a, RoleB);
create_recv_mpst_session_1!(recv_mpst_a_to_b, RoleB, next_b, RoleA);
// broadcast
create_recv_mpst_all_session_2!(recv_mpst_b_all_to_c, RoleAlltoC, next_all_to_c, RoleB);
create_recv_mpst_all_session_2!(recv_mpst_a_all_to_c, RoleAlltoC, next_all_to_c, RoleA);

// Create the offer functions
create_offer_mpst_session_2!(
    offer_mpst_session_b_to_c,
    RoleAlltoC,
    recv_mpst_b_all_to_c,
    RoleB
);
create_offer_mpst_session_2!(
    offer_mpst_session_a_to_c,
    RoleAlltoC,
    recv_mpst_a_all_to_c,
    RoleA
);

// Create the choose functions
create_choose_right_from_3_to_1_and_2!(
    choose_right_mpst_session_c_to_all,
    RoleCtoAll,
    RoleADual,
    RoleBDual,
    next_c_to_all,
    RoleC
);
create_choose_left_from_3_to_1_and_2!(
    choose_left_mpst_session_c_to_all,
    RoleCtoAll,
    RoleADual,
    RoleBDual,
    next_c_to_all,
    RoleC
);

// Types
type AtoCClose = End;
type AtoBClose = End;
type AtoCVideo<N> = Recv<N, Send<N, End>>;
type AtoBVideo<N> = Send<N, Recv<N, End>>;

type BtoAClose = <AtoBClose as Session>::Dual;
type BtoCClose = End;
type BtoAVideo<N> = <AtoBVideo<N> as Session>::Dual;

type CtoBClose = <BtoCClose as Session>::Dual;
type CtoAClose = <AtoCClose as Session>::Dual;
type CtoAVideo<N> = <AtoCVideo<N> as Session>::Dual;

/// Queues
type QueueAEnd = RoleEnd;
type QueueAEndDual = <QueueAEnd as Role>::Dual;
type QueueAVideo = RoleC<RoleB<RoleB<RoleC<RoleEnd>>>>;
type QueueAVideoDual = <QueueAVideo as Role>::Dual;
type QueueAFull = RoleC<RoleC<RoleAlltoC<RoleEnd, RoleEnd>>>;

type QueueBEnd = RoleEnd;
type QueueBEndDual = <QueueBEnd as Role>::Dual;
type QueueBVideo = RoleA<RoleA<RoleEnd>>;
type QueueBVideoDual = <QueueBVideo as Role>::Dual;
type QueueBFull = RoleAlltoC<RoleEnd, RoleEnd>;

type QueueCEnd = RoleEnd;
type QueueCVideo = RoleA<RoleA<RoleEnd>>;
type QueueCChoice = RoleCtoAll<QueueCVideo, QueueCEnd>;
type QueueCFull = RoleA<RoleA<QueueCChoice>>;

/// Creating the MP sessions
/// For C
type ChooseCtoA<N> = ChooseMpst<
    BtoAVideo<N>,
    CtoAVideo<N>,
    BtoAClose,
    CtoAClose,
    QueueAVideoDual,
    QueueAEnd,
    RoleADual<RoleEnd>,
>;
type ChooseCtoB<N> = ChooseMpst<
    AtoBVideo<N>,
    CtoBClose,
    AtoBClose,
    CtoBClose,
    QueueBVideoDual,
    QueueBEnd,
    RoleBDual<RoleEnd>,
>;
type InitC<N> = Send<N, Recv<N, ChooseCtoA<N>>>;
type EndpointCFull<N> = SessionMpst<InitC<N>, ChooseCtoB<N>, QueueCFull, RoleC<RoleEnd>>;

/// For A
type EndpointAVideo<N> = SessionMpst<AtoBVideo<N>, AtoCVideo<N>, QueueAVideo, RoleA<RoleEnd>>;
type EndpointAEnd = SessionMpst<AtoBClose, AtoCClose, QueueAEnd, RoleA<RoleEnd>>;

type OfferA<N> = OfferMpst<
    AtoBVideo<N>,
    AtoCVideo<N>,
    AtoBClose,
    AtoCClose,
    QueueAVideo,
    QueueAEnd,
    RoleA<RoleEnd>,
>;
type InitA<N> = Recv<N, Send<N, OfferA<N>>>;
type EndpointAFull<N> = SessionMpst<End, InitA<N>, QueueAFull, RoleA<RoleEnd>>;

/// For B
type EndpointBVideo<N> = SessionMpst<BtoAVideo<N>, BtoCClose, QueueBVideo, RoleB<RoleEnd>>;
type EndpointBEnd = SessionMpst<BtoAClose, BtoCClose, QueueBEnd, RoleB<RoleEnd>>;

type OfferB<N> = OfferMpst<
    BtoAVideo<N>,
    BtoCClose,
    BtoAClose,
    BtoCClose,
    QueueBVideo,
    QueueBEnd,
    RoleB<RoleEnd>,
>;
type EndpointBFull<N> = SessionMpst<End, OfferB<N>, QueueBFull, RoleB<RoleEnd>>;

/// Functions related to endpoints
fn server(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_b_to_c(
        s,
        |s: EndpointBVideo<i32>| {
            let (request, s) = recv_mpst_b_to_a(s)?;
            let s = send_mpst_b_to_a(request + 1, s);

            close_mpst(s)?;

            Ok(())
        },
        |s: EndpointBEnd| {
            close_mpst(s)?;

            Ok(())
        },
    )
}

fn authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_a_to_c(s)?;
    let s = send_mpst_a_to_c(id + 1, s);

    offer_mpst_session_a_to_c(
        s,
        |s: EndpointAVideo<i32>| {
            let (request, s) = recv_mpst_a_to_c(s)?;
            let s = send_mpst_a_to_b(request + 1, s);
            let (video, s) = recv_mpst_a_to_b(s)?;
            let s = send_mpst_a_to_c(video + 1, s);

            assert_eq!(request, id + 1);
            assert_eq!(video, id + 3);

            close_mpst(s)?;
            Ok(())
        },
        |s: EndpointAEnd| {
            close_mpst(s)?;

            Ok(())
        },
    )
}

fn client_video(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    {
        let mut rng = thread_rng();
        let id: i32 = rng.gen();

        let s = send_mpst_c_to_a(id, s);
        let (accept, s) = recv_mpst_c_to_a(s)?;

        assert_eq!(accept, id + 1);

        let s = choose_left_mpst_session_c_to_all::<
            BtoAVideo<i32>,
            BtoAClose,
            CtoAVideo<i32>,
            CtoAClose,
            BtoCClose,
            AtoCClose,
            QueueAVideoDual,
            QueueAEndDual,
            QueueBVideoDual,
            QueueBEndDual,
            QueueCVideo,
            QueueCEnd,
        >(s);

        let s = send_mpst_c_to_a(accept, s);
        let (result, s) = recv_mpst_c_to_a(s)?;

        assert_eq!(result, accept + 3);

        close_mpst(s)?;
    }
    Ok(())
}

fn client_close(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    {
        let mut rng = thread_rng();
        let id: i32 = rng.gen();

        let s = send_mpst_c_to_a(id, s);
        let (accept, s) = recv_mpst_c_to_a(s)?;

        assert_eq!(accept, id + 1);

        let s = choose_right_mpst_session_c_to_all::<
            BtoAVideo<i32>,
            BtoAClose,
            CtoAVideo<i32>,
            CtoAClose,
            BtoCClose,
            AtoCClose,
            QueueAVideoDual,
            QueueAEndDual,
            QueueBVideoDual,
            QueueBEndDual,
            QueueCVideo,
            QueueCEnd,
        >(s);

        close_mpst(s)?;
    }
    Ok(())
}

#[test]
fn run_usecase_right() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test end branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client_close);

            assert!(thread_a.is_ok());
            assert!(thread_b.is_ok());
            assert!(thread_c.is_ok());
        }

        Ok(())
    }()
    .is_ok());
}

#[test]
fn run_usecase_left() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test video branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client_video);

            assert!(thread_a.is_ok());
            assert!(thread_b.is_ok());
            assert!(thread_c.is_ok());
        }

        Ok(())
    }()
    .is_ok());
}
