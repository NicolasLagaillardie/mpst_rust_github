use rand::{thread_rng, Rng};

use mpstthree::checking::checker;

use mpstthree::binary::{End, Recv, Send, Session};
use mpstthree::fork::fork_mpst;
use mpstthree::role::Role;
use mpstthree::sessionmpst::SessionMpst;

use std::boxed::Box;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;

use mpstthree::functionmpst::close::close_mpst;

use mpstthree::role::a::RoleA;
use mpstthree::role::a_dual::RoleADual;
use mpstthree::role::all_to_b::RoleAlltoB;
use mpstthree::role::b::RoleB;
use mpstthree::role::b_to_all::RoleBtoAll;
use mpstthree::role::c::RoleC;
use mpstthree::role::c_dual::RoleCDual;
use mpstthree::role::end::RoleEnd;

use mpstthree::functionmpst::recv::recv_mpst_a_to_c;
use mpstthree::functionmpst::recv::recv_mpst_b_to_c;
use mpstthree::functionmpst::recv::recv_mpst_c_to_a;
use mpstthree::functionmpst::recv::recv_mpst_c_to_b;

use mpstthree::functionmpst::send::send_mpst_a_to_c;
use mpstthree::functionmpst::send::send_mpst_b_to_c;
use mpstthree::functionmpst::send::send_mpst_c_to_a;
use mpstthree::functionmpst::send::send_mpst_c_to_b;

use mpstthree::functionmpst::offer::offer_mpst_session_to_a_from_b;
use mpstthree::functionmpst::offer::offer_mpst_session_to_c_from_b;

use mpstthree::functionmpst::choose::choose_left_mpst_session_b_to_all;
use mpstthree::functionmpst::choose::choose_right_mpst_session_b_to_all;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

/// Test our usecase
/// Simple types
/// Authenticator = C
/// Server = A
/// Client = B

type CtoBClose = End;
type CtoAClose = End;
type CtoBVideo<N> = Recv<N, Send<N, End>>;
type CtoAVideo<N> = Send<N, Recv<N, End>>;

type AtoCClose = <CtoAClose as Session>::Dual;
type AtoBClose = End;
type AtoCVideo<N> = <CtoAVideo<N> as Session>::Dual;

type BtoAClose = <AtoBClose as Session>::Dual;
type BtoCClose = <CtoBClose as Session>::Dual;
type BtoCVideo<N> = <CtoBVideo<N> as Session>::Dual;

/// Queues
type QueueCEnd = RoleEnd;
type QueueCEndDual = <QueueCEnd as Role>::Dual;
type QueueCVideo = RoleB<RoleA<RoleA<RoleB<RoleEnd>>>>;
type QueueCVideoDual = <QueueCVideo as Role>::Dual;
type QueueCFull = RoleB<RoleB<RoleAlltoB<RoleEnd, RoleEnd>>>;

type QueueAEnd = RoleEnd;
type QueueAEndDual = <QueueAEnd as Role>::Dual;
type QueueAVideo = RoleC<RoleC<RoleEnd>>;
type QueueAVideoDual = <QueueAVideo as Role>::Dual;
type QueueAFull = RoleAlltoB<RoleEnd, RoleEnd>;

type QueueBEnd = RoleEnd;
type QueueBVideo = RoleC<RoleC<RoleEnd>>;
type QueueBChoice = RoleBtoAll<QueueBVideo, QueueBEnd>;
type QueueBFull = RoleC<RoleC<QueueBChoice>>;

/// Creating the MP sessions
/// For C
type ChooseBtoC<N> = ChooseMpst<
    AtoCVideo<N>,
    BtoCVideo<N>,
    AtoCClose,
    BtoCClose,
    QueueCVideoDual,
    QueueCEnd,
    RoleCDual<RoleEnd>,
>;
type ChooseBtoA<N> = ChooseMpst<
    BtoAClose,
    CtoAVideo<N>,
    BtoAClose,
    CtoAClose,
    QueueAVideoDual,
    QueueAEnd,
    RoleADual<RoleEnd>,
>;
type InitB<N> = Send<N, Recv<N, ChooseBtoC<N>>>;
type EndpointBFull<N> = SessionMpst<ChooseBtoA<N>, InitB<N>, QueueBFull, RoleB<RoleEnd>>;

/// For A
type EndpointCVideo<N> = SessionMpst<CtoAVideo<N>, CtoBVideo<N>, QueueCVideo, RoleC<RoleEnd>>;
type EndpointCEnd = SessionMpst<CtoAClose, CtoBClose, QueueCEnd, RoleC<RoleEnd>>;

type OfferC<N> = OfferMpst<
    CtoAVideo<N>,
    CtoBVideo<N>,
    CtoAClose,
    CtoBClose,
    QueueCVideo,
    QueueCEnd,
    RoleC<RoleEnd>,
>;
type InitC<N> = Recv<N, Send<N, OfferC<N>>>;
type EndpointCFull<N> = SessionMpst<End, InitC<N>, QueueCFull, RoleC<RoleEnd>>;

/// For B
type EndpointAVideo<N> = SessionMpst<AtoBClose, AtoCVideo<N>, QueueAVideo, RoleA<RoleEnd>>;
type EndpointAEnd = SessionMpst<AtoBClose, AtoCClose, QueueAEnd, RoleA<RoleEnd>>;

type OfferA<N> = OfferMpst<
    AtoBClose,
    AtoCVideo<N>,
    AtoBClose,
    AtoCClose,
    QueueAVideo,
    QueueAEnd,
    RoleA<RoleEnd>,
>;
type EndpointAFull<N> = SessionMpst<OfferA<N>, End, QueueAFull, RoleA<RoleEnd>>;

/// Functions related to endpoints
fn server(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_to_a_from_b(
        s,
        |s: EndpointAVideo<i32>| {
            let (request, s) = recv_mpst_a_to_c(s)?;
            let s = send_mpst_a_to_c(request + 1, s);

            close_mpst(s)
        },
        |s: EndpointAEnd| close_mpst(s),
    )
}

fn authenticator(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_c_to_b(s)?;
    let s = send_mpst_c_to_b(id + 1, s);

    offer_mpst_session_to_c_from_b(
        s,
        |s: EndpointCVideo<i32>| {
            let (request, s) = recv_mpst_c_to_b(s)?;
            let s = send_mpst_c_to_a(request + 1, s);
            let (video, s) = recv_mpst_c_to_a(s)?;
            let s = send_mpst_c_to_b(video + 1, s);

            assert_eq!(request, id + 1);
            assert_eq!(video, id + 3);

            close_mpst(s)
        },
        |s: EndpointCEnd| close_mpst(s),
    )
}

fn client_video(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let id: i32 = rng.gen();

    let s = send_mpst_b_to_c(id, s);
    let (accept, s) = recv_mpst_b_to_c(s)?;

    assert_eq!(accept, id + 1);

    let s = choose_left_mpst_session_b_to_all::<
        CtoAVideo<i32>,
        CtoAClose,
        BtoAClose,
        BtoCVideo<i32>,
        BtoAClose,
        CtoBClose,
        QueueAVideoDual,
        QueueAEndDual,
        QueueCVideoDual,
        QueueCEndDual,
        QueueBVideo,
        QueueBEnd,
    >(s);

    let s = send_mpst_b_to_c(accept, s);
    let (result, s) = recv_mpst_b_to_c(s)?;

    assert_eq!(result, accept + 3);

    close_mpst(s)
}

fn client_close(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let id: i32 = rng.gen();

    let s = send_mpst_b_to_c(id, s);
    let (accept, s) = recv_mpst_b_to_c(s)?;

    assert_eq!(accept, id + 1);

    let s = choose_right_mpst_session_b_to_all::<
        CtoAVideo<i32>,
        CtoAClose,
        BtoAClose,
        BtoCVideo<i32>,
        BtoAClose,
        CtoBClose,
        QueueAVideoDual,
        QueueAEndDual,
        QueueCVideoDual,
        QueueCEndDual,
        QueueBVideo,
        QueueBEnd,
    >(s);

    close_mpst(s)
}

#[test]
pub fn run_b_usecase_left() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test video branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(server, client_video, authenticator);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        Ok(())
    }()
    .is_ok());
}

#[test]
pub fn run_b_usecase_right() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test end branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(server, client_close, authenticator);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        Ok(())
    }()
    .is_ok());
}

#[test]
pub fn run_b_usecase_checker() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let s = RandomState::new();
            let hm: HashMap<String, &Vec<String>> = HashMap::with_hasher(s);

            let (s1, _): (EndpointAFull<i32>, _) = SessionMpst::new();
            let (s2, _): (EndpointBFull<i32>, _) = SessionMpst::new();
            let (s3, _): (EndpointCFull<i32>, _) = SessionMpst::new();

            let (a, b, c) = checker(s1, s2, s3, &hm, &HashMap::new())?;

            assert_eq!(a, "A: ( A?C.A!C.0 & 0 )");
            assert_eq!(b, "B: B!C.B?C.( B!C.B?C.0 + 0 )");
            assert_eq!(c, "C: C?B.C!B.( C?B.C!A.C?A.C!B.0 & 0 )");
        }
        Ok(())
    }()
    .is_ok());
}
