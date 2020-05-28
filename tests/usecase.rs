extern crate mpstthree;
extern crate rand;

use rand::{thread_rng, Rng};

use mpstthree::checking::checker;

use mpstthree::binary::{End, Recv, Send, Session};
use mpstthree::fork_mpst;
use mpstthree::role::Role;
use mpstthree::sessionmpst::SessionMpst;

use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;

use mpstthree::functionmpst::close::close_mpst;

use mpstthree::role::a_to_b::RoleAtoB;
use mpstthree::role::a_to_c::RoleAtoC;
use mpstthree::role::b_to_a::RoleBtoA;
use mpstthree::role::b_to_c::RoleBtoC;
use mpstthree::role::c_to_a::RoleCtoA;
use mpstthree::role::c_to_all::RoleCtoAll;
use mpstthree::role::end::RoleEnd;

use mpstthree::functionmpst::recv::recv_mpst_a_to_b;
use mpstthree::functionmpst::recv::recv_mpst_a_to_c;
use mpstthree::functionmpst::recv::recv_mpst_b_to_a;
use mpstthree::functionmpst::recv::recv_mpst_c_to_a;

use mpstthree::functionmpst::send::send_mpst_a_to_b;
use mpstthree::functionmpst::send::send_mpst_a_to_c;
use mpstthree::functionmpst::send::send_mpst_b_to_a;
use mpstthree::functionmpst::send::send_mpst_c_to_a;

use mpstthree::functionmpst::offer::offer_mpst_session_a_to_c;
use mpstthree::functionmpst::offer::offer_mpst_session_b_to_c;

use mpstthree::functionmpst::choose::choose_left_mpst_session_c_to_all;
use mpstthree::functionmpst::choose::choose_right_mpst_session_c_to_all;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

/// Test our usecase from Places 2020
/// Simple types
/// Client = C
/// Authenticator = A
/// Server = B

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
type QueueAVideo = RoleAtoC<RoleAtoB<RoleAtoB<RoleAtoC<RoleEnd>>>>;
type QueueAVideoDual = <QueueAVideo as Role>::Dual;
type QueueAFull = RoleAtoC<RoleAtoC<RoleAtoC<RoleEnd>>>;

type QueueBEnd = RoleEnd;
type QueueBEndDual = <QueueBEnd as Role>::Dual;
type QueueBVideo = RoleBtoA<RoleBtoA<RoleEnd>>;
type QueueBVideoDual = <QueueBVideo as Role>::Dual;
type QueueBFull = RoleBtoC<RoleEnd>;

type QueueCEnd = RoleEnd;
type QueueCVideo = RoleCtoA<RoleCtoA<RoleEnd>>;
type QueueCChoice = RoleCtoAll<QueueCVideo, QueueCEnd>;
type QueueCFull = RoleCtoA<RoleCtoA<QueueCChoice>>;

/// Creating the MP sessions
/// For C
type ChooseCtoA<N> =
    ChooseMpst<BtoAVideo<N>, CtoAVideo<N>, BtoAClose, CtoAClose, QueueAVideoDual, QueueAEnd>;
type ChooseCtoB<N> =
    ChooseMpst<AtoBVideo<N>, CtoBClose, AtoBClose, CtoBClose, QueueBVideoDual, QueueBEnd>;
type InitC<N> = Send<N, Recv<N, ChooseCtoA<N>>>;
type EndpointCFull<N> = SessionMpst<InitC<N>, ChooseCtoB<N>, QueueCFull>;

/// For A
type EndpointAVideo<N> = SessionMpst<AtoBVideo<N>, AtoCVideo<N>, QueueAVideo>;
type EndpointAEnd = SessionMpst<AtoBClose, AtoCClose, QueueAEnd>;

type OfferA<N> =
    OfferMpst<AtoBVideo<N>, AtoCVideo<N>, AtoBClose, AtoCClose, QueueAVideo, QueueAEnd>;
type InitA<N> = Recv<N, Send<N, OfferA<N>>>;
type EndpointAFull<N> = SessionMpst<End, InitA<N>, QueueAFull>;

/// For B
type EndpointBVideo<N> = SessionMpst<BtoAVideo<N>, BtoCClose, QueueBVideo>;
type EndpointBEnd = SessionMpst<BtoAClose, BtoCClose, QueueBEnd>;

type OfferB<N> = OfferMpst<BtoAVideo<N>, BtoCClose, BtoAClose, BtoCClose, QueueBVideo, QueueBEnd>;
type EndpointBFull<N> = SessionMpst<End, OfferB<N>, QueueBFull>;

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
fn run_usecase() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test video branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client_video);

            assert!(thread_a.is_ok());
            assert!(thread_b.is_ok());
            assert!(thread_c.is_ok());
        }

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

    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let hm: HashMap<String, &Vec<String>> = HashMap::new();

            let (s1, _): (EndpointAFull<i32>, _) = SessionMpst::new();
            let (s2, _): (EndpointBFull<i32>, _) = SessionMpst::new();
            let (s3, _): (EndpointCFull<i32>, _) = SessionMpst::new();

            let (a, b, c) = checker(s1, s2, s3, &hm)?;

            assert_eq!(a, "A: A?C.A!C.( A?C.A!B.A?B.A!C.0 & 0 )");
            assert_eq!(b, "B: ( B?A.B!A.0 & 0 )");
            assert_eq!(c, "C: C!A.C?A.( C!A.C?A.0 + 0 )");
        }
        Ok(())
    }()
    .is_ok());
}
