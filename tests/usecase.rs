extern crate mpst;

use mpst::binary::{End, Recv, Send};
use mpst::run_processes;
use mpst::sessionmpst::SessionMpst;

use std::boxed::Box;
use std::error::Error;

use mpst::functionmpst::close::close_mpst;

use mpst::role::a_to_b::RoleAtoB;
use mpst::role::a_to_c::RoleAtoC;
use mpst::role::b_to_a::RoleBtoA;
use mpst::role::b_to_c::RoleBtoC;
use mpst::role::c_to_a::RoleCtoA;
use mpst::role::c_to_all::RoleCtoAll;
use mpst::role::end::RoleEnd;

use mpst::functionmpst::recv::recv_mpst_session_one_a_to_b;
use mpst::functionmpst::recv::recv_mpst_session_one_b_to_a;
use mpst::functionmpst::recv::recv_mpst_session_one_c_to_a;
use mpst::functionmpst::recv::recv_mpst_session_two_a_to_c;

use mpst::functionmpst::send::send_mpst_session_one_a_to_b;
use mpst::functionmpst::send::send_mpst_session_one_b_to_a;
use mpst::functionmpst::send::send_mpst_session_one_c_to_a;
use mpst::functionmpst::send::send_mpst_session_two_a_to_c;

use mpst::functionmpst::offer::offer_mpst_session_a_to_c;
use mpst::functionmpst::offer::offer_mpst_session_b_to_c;

use mpst::functionmpst::choose::choose_left_mpst_session_c_to_all;
use mpst::functionmpst::choose::choose_right_mpst_session_c_to_all;

use mpst::functionmpst::ChooseMpst;
use mpst::functionmpst::OfferMpst;

/// Test our usecase from Places 2020
/// Simple types
/// Client = C
/// Authenticator = A
/// Server = B

type CtoBClose = End;
type CtoAVideo<N> = Send<N, Recv<N, End>>;

type AtoCClose = End;
type AtoBClose = End;
type AtoCVideo<N> = Recv<N, Send<N, End>>;
type AtoBVideo<N> = Send<N, Recv<N, End>>;

type BtoAClose = End;
type BtoCClose = End;
type BtoAVideo<N> = Recv<N, Send<N, End>>;

// /// Queues
type QueueCEnd = RoleEnd;
type QueueCVideo = RoleCtoA<RoleCtoA<RoleEnd>>;
type QueueCChoice = RoleCtoAll<QueueCVideo, QueueCEnd>;
type QueueCFull = RoleCtoA<RoleCtoA<QueueCChoice>>;

type QueueBEnd = RoleEnd;
type QueueBVideo = RoleBtoA<RoleBtoA<RoleEnd>>;
type QueueBVideoDual = RoleAtoB<RoleAtoB<RoleEnd>>;
type QueueBFull = RoleBtoC<RoleEnd>;

type QueueAEnd = RoleEnd;
type QueueAVideo = RoleAtoC<RoleAtoB<RoleAtoB<RoleAtoC<RoleEnd>>>>;
type QueueAVideoDual = RoleCtoA<RoleBtoA<RoleBtoA<RoleCtoA<RoleEnd>>>>;
type QueueAFull = RoleAtoC<RoleAtoC<RoleAtoC<RoleEnd>>>;

/// Creating the MP sessions
/// For C
type EndpointCtoAVideo<N> = SessionMpst<BtoAVideo<N>, CtoAVideo<N>, QueueAVideoDual>;
type EndpointCtoAEnd = SessionMpst<AtoCClose, AtoCClose, QueueAEnd>;
type EndpointCtoBVideo<N> = SessionMpst<AtoBVideo<N>, CtoBClose, QueueBVideoDual>;
type EndpointCtoBEnd = SessionMpst<BtoAClose, BtoCClose, QueueBEnd>;

type ChooseCtoB<N> = ChooseMpst<EndpointCtoBVideo<N>, EndpointCtoBEnd>;
type ChooseCtoA<N> = ChooseMpst<EndpointCtoAVideo<N>, EndpointCtoAEnd>;
type InitC<N> = Send<N, Recv<N, ChooseCtoA<N>>>;
type EndpointCFull<N> = SessionMpst<InitC<N>, ChooseCtoB<N>, QueueCFull>;

/// For A
type EndpointAEnd = SessionMpst<End, End, QueueAEnd>;
type EndpointAVideo<N> = SessionMpst<AtoBVideo<N>, AtoCVideo<N>, QueueAVideo>;

type OfferA<N> = OfferMpst<EndpointAVideo<N>, EndpointAEnd>;
type InitA<N> = Recv<N, Send<N, OfferA<N>>>;
type EndpointAFull<N> = SessionMpst<End, InitA<N>, QueueAFull>;

/// For B
type EndpointBEnd = SessionMpst<BtoAClose, BtoCClose, QueueBEnd>;
type EndpointBVideo<N> = SessionMpst<BtoAVideo<N>, BtoCClose, QueueBVideo>;

type OfferB<N> = OfferMpst<EndpointBVideo<N>, EndpointBEnd>;
type EndpointBFull<N> = SessionMpst<End, OfferB<N>, QueueBFull>;

/// Functions related to endpoints
fn server(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_b_to_c(
        s,
        |s: EndpointBVideo<i32>| {
            let (request, s) = recv_mpst_session_one_b_to_a(s)?;
            let s = send_mpst_session_one_b_to_a(request + 1, s);

            close_mpst(s)?;

            assert_eq!(request, 2);
            Ok(())
        },
        |s: EndpointBEnd| {
            close_mpst(s)?;

            Ok(())
        },
    )
}

fn authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_session_two_a_to_c(s)?;
    let s = send_mpst_session_two_a_to_c(id + 1, s);

    assert_eq!(id, 0);

    offer_mpst_session_a_to_c(
        s,
        |s: EndpointAVideo<i32>| {
            let (request, s) = recv_mpst_session_two_a_to_c(s)?;
            let s = send_mpst_session_one_a_to_b(request + 1, s);
            let (video, s) = recv_mpst_session_one_a_to_b(s)?;
            let s = send_mpst_session_two_a_to_c(video + 1, s);

            assert_eq!(request, 1);
            assert_eq!(video, 3);

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
        let s = send_mpst_session_one_c_to_a(0, s);
        let (accept, s) = recv_mpst_session_one_c_to_a(s)?;

        assert_eq!(accept, 1);

        let s = choose_left_mpst_session_c_to_all::<
            BtoAVideo<i32>,
            AtoBClose,
            CtoAVideo<i32>,
            BtoCClose,
            BtoCClose,
            AtoCClose,
            QueueAVideoDual,
            QueueAEnd,
            QueueBVideoDual,
            QueueBEnd,
            QueueCVideo,
            QueueCEnd,
        >(s);

        let s = send_mpst_session_one_c_to_a(1, s);
        let (result, s) = recv_mpst_session_one_c_to_a(s)?;

        assert_eq!(result, 4);

        close_mpst(s)?;
    }
    Ok(())
}

fn client_close(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    {
        let s = send_mpst_session_one_c_to_a(0, s);
        let (accept, s) = recv_mpst_session_one_c_to_a(s)?;

        assert_eq!(accept, 1);

        let s = choose_right_mpst_session_c_to_all::<
            BtoAVideo<i32>,
            AtoBClose,
            CtoAVideo<i32>,
            BtoCClose,
            BtoCClose,
            AtoCClose,
            QueueAVideoDual,
            QueueAEnd,
            QueueBVideoDual,
            QueueBEnd,
            QueueCVideo,
            QueueCEnd,
        >(s);

        close_mpst(s)?;
    }
    Ok(())
}

#[test]
fn run_usecase() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test video branch.
        {
            let (thread_a, thread_b, thread_c) = run_processes(authenticator, server, client_video);

            thread_a.unwrap();
            thread_b.unwrap();
            thread_c.unwrap();
        }

        // Test right branch.
        {
            let (thread_a, thread_b, thread_c) = run_processes(authenticator, server, client_close);

            thread_a.unwrap();
            thread_b.unwrap();
            thread_c.unwrap();
        }

        Ok(())
    }()
    .is_ok());
}
