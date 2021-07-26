use rand::{thread_rng, Rng};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::Role;

use std::boxed::Box;
use std::error::Error;

use mpstthree::functionmpst::close::close_mpst;

use mpstthree::role::a::RoleA;
use mpstthree::role::a_dual::RoleADual;
use mpstthree::role::all_to_c::RoleAlltoC;
use mpstthree::role::b::RoleB;
use mpstthree::role::b_dual::RoleBDual;
use mpstthree::role::c::RoleC;
use mpstthree::role::c_to_all::RoleCtoAll;
use mpstthree::role::end::RoleEnd;

use mpstthree::functionmpst::recv::recv_mpst_a_from_b;
use mpstthree::functionmpst::recv::recv_mpst_a_from_c;
use mpstthree::functionmpst::recv::recv_mpst_b_from_a;
use mpstthree::functionmpst::recv::recv_mpst_c_from_a;

use mpstthree::functionmpst::send::send_mpst_a_to_b;
use mpstthree::functionmpst::send::send_mpst_a_to_c;
use mpstthree::functionmpst::send::send_mpst_b_to_a;
use mpstthree::functionmpst::send::send_mpst_c_to_a;

use mpstthree::functionmpst::offer::offer_mpst_session_to_a_from_c;
use mpstthree::functionmpst::offer::offer_mpst_session_to_b_from_c;

use mpstthree::functionmpst::choose::choose_left_mpst_session_c_to_all;
use mpstthree::functionmpst::choose::choose_right_mpst_session_c_to_all;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

/// Test our usecase
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

/// Stacks
type StackAEnd = RoleEnd;
type StackAEndDual = <StackAEnd as Role>::Dual;
type StackAVideo = RoleC<RoleB<RoleB<RoleC<RoleEnd>>>>;
type StackAVideoDual = <StackAVideo as Role>::Dual;
type StackAFull = RoleC<RoleC<RoleAlltoC<RoleEnd, RoleEnd>>>;

type StackBEnd = RoleEnd;
type StackBEndDual = <StackBEnd as Role>::Dual;
type StackBVideo = RoleA<RoleA<RoleEnd>>;
type StackBVideoDual = <StackBVideo as Role>::Dual;
type StackBFull = RoleAlltoC<RoleEnd, RoleEnd>;

type StackCEnd = RoleEnd;
type StackCVideo = RoleA<RoleA<RoleEnd>>;
type StackCChoice = RoleCtoAll<StackCVideo, StackCEnd>;
type StackCFull = RoleA<RoleA<StackCChoice>>;

/// Creating the MP sessions
/// For C
type ChooseCtoA<N> = ChooseMpst<
    BtoAVideo<N>,
    CtoAVideo<N>,
    BtoAClose,
    CtoAClose,
    StackAVideoDual,
    StackAEnd,
    RoleADual<RoleEnd>,
>;
type ChooseCtoB<N> = ChooseMpst<
    AtoBVideo<N>,
    CtoBClose,
    AtoBClose,
    CtoBClose,
    StackBVideoDual,
    StackBEnd,
    RoleBDual<RoleEnd>,
>;
type InitC<N> = Send<N, Recv<N, ChooseCtoA<N>>>;
type EndpointCFull<N> = MeshedChannels<InitC<N>, ChooseCtoB<N>, StackCFull, RoleC<RoleEnd>>;

/// For A
type EndpointAVideo<N> = MeshedChannels<AtoBVideo<N>, AtoCVideo<N>, StackAVideo, RoleA<RoleEnd>>;
type EndpointAEnd = MeshedChannels<AtoBClose, AtoCClose, StackAEnd, RoleA<RoleEnd>>;

type OfferA<N> = OfferMpst<
    AtoBVideo<N>,
    AtoCVideo<N>,
    AtoBClose,
    AtoCClose,
    StackAVideo,
    StackAEnd,
    RoleA<RoleEnd>,
>;
type InitA<N> = Recv<N, Send<N, OfferA<N>>>;
type EndpointAFull<N> = MeshedChannels<End, InitA<N>, StackAFull, RoleA<RoleEnd>>;

/// For B
type EndpointBVideo<N> = MeshedChannels<BtoAVideo<N>, BtoCClose, StackBVideo, RoleB<RoleEnd>>;
type EndpointBEnd = MeshedChannels<BtoAClose, BtoCClose, StackBEnd, RoleB<RoleEnd>>;

type OfferB<N> = OfferMpst<
    BtoAVideo<N>,
    BtoCClose,
    BtoAClose,
    BtoCClose,
    StackBVideo,
    StackBEnd,
    RoleB<RoleEnd>,
>;
type EndpointBFull<N> = MeshedChannels<End, OfferB<N>, StackBFull, RoleB<RoleEnd>>;

/// Functions related to endpoints
fn server(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_to_b_from_c(
        s,
        |s: EndpointBVideo<i32>| {
            let (request, s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_a(request + 1, s);

            close_mpst(s)
        },
        |s: EndpointBEnd| close_mpst(s),
    )
}

fn authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_a_from_c(s)?;
    let s = send_mpst_a_to_c(id + 1, s);

    offer_mpst_session_to_a_from_c(
        s,
        |s: EndpointAVideo<i32>| {
            let (request, s) = recv_mpst_a_from_c(s)?;
            let s = send_mpst_a_to_b(request + 1, s);
            let (video, s) = recv_mpst_a_from_b(s)?;
            let s = send_mpst_a_to_c(video + 1, s);

            assert_eq!(request, id + 1);
            assert_eq!(video, id + 3);

            close_mpst(s)
        },
        |s: EndpointAEnd| close_mpst(s),
    )
}

fn client_video(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let id: i32 = rng.gen();

    let s = send_mpst_c_to_a(id, s);
    let (accept, s) = recv_mpst_c_from_a(s)?;

    assert_eq!(accept, id + 1);

    let s = choose_left_mpst_session_c_to_all::<
        BtoAVideo<i32>,
        BtoAClose,
        CtoAVideo<i32>,
        CtoAClose,
        BtoCClose,
        AtoCClose,
        StackAVideoDual,
        StackAEndDual,
        StackBVideoDual,
        StackBEndDual,
        StackCVideo,
        StackCEnd,
    >(s);

    let s = send_mpst_c_to_a(accept, s);
    let (result, s) = recv_mpst_c_from_a(s)?;

    assert_eq!(result, accept + 3);

    close_mpst(s)
}

fn client_close(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let id: i32 = rng.gen();

    let s = send_mpst_c_to_a(id, s);
    let (accept, s) = recv_mpst_c_from_a(s)?;

    assert_eq!(accept, id + 1);

    let s = choose_right_mpst_session_c_to_all::<
        BtoAVideo<i32>,
        BtoAClose,
        CtoAVideo<i32>,
        CtoAClose,
        BtoCClose,
        AtoCClose,
        StackAVideoDual,
        StackAEndDual,
        StackBVideoDual,
        StackBEndDual,
        StackCVideo,
        StackCEnd,
    >(s);

    close_mpst(s)
}

/////////////////////////////////////////

pub fn run_c_usecase_left() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test video branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client_video);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        Ok(())
    }()
    .is_ok());
}

pub fn run_c_usecase_right() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test end branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client_close);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        Ok(())
    }()
    .is_ok());
}

pub fn run_c_usecase_checker() {
    let graphs =
        mpstthree::checker_concat!(EndpointBFull<i32>, EndpointAFull<i32>, EndpointCFull<i32>)
            .unwrap();

    ////////////// Test graph A
    let graph_a = &graphs["RoleA"];

    assert_eq!(graph_a.node_count(), 10);
    assert_eq!(graph_a.edge_count(), 9);

    ////////////// Test graph B
    let graph_b = &graphs["RoleB"];

    assert_eq!(graph_b.node_count(), 6);
    assert_eq!(graph_b.edge_count(), 5);

    ////////////// Test graph C
    let graph_c = &graphs["RoleC"];

    assert_eq!(graph_c.node_count(), 8);
    assert_eq!(graph_c.edge_count(), 7);
}
