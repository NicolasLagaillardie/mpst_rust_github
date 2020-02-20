extern crate mpst;

use binary::*;
use mpst::*;
use role::*;
use sessionmpst::SessionMpst;
use std::boxed::Box;
use std::error::Error;

use mpst::functionmpst::close::close_mpst;

use mpst::role::a_to_all::RoleAtoAll;
use mpst::role::a_to_b::RoleAtoB;
use mpst::role::a_to_c::RoleAtoC;
use mpst::role::b_to_a::RoleBtoA;
use mpst::role::b_to_all::RoleBtoAll;
use mpst::role::b_to_c::RoleBtoC;
use mpst::role::c_to_a::RoleCtoA;
use mpst::role::c_to_all::RoleCtoAll;
use mpst::role::c_to_b::RoleCtoB;
use mpst::role::end::RoleEnd;

use mpst::functionmpst::recv::recv_mpst_session_one_a_to_b;
use mpst::functionmpst::recv::recv_mpst_session_one_b_to_a;
use mpst::functionmpst::recv::recv_mpst_session_one_c_to_a;
use mpst::functionmpst::recv::recv_mpst_session_two_a_to_c;

use mpst::functionmpst::send::send_mpst_session_one_a_to_b;
use mpst::functionmpst::send::send_mpst_session_one_b_to_a;
use mpst::functionmpst::send::send_mpst_session_one_c_to_a;
use mpst::functionmpst::send::send_mpst_session_two_a_to_c;

use mpst::functionmpst::offer::offer_mpst_session_a_to_b;
use mpst::functionmpst::offer::offer_mpst_session_a_to_c;
use mpst::functionmpst::offer::offer_mpst_session_b_to_a;
use mpst::functionmpst::offer::offer_mpst_session_b_to_c;
use mpst::functionmpst::offer::offer_mpst_session_c_to_b;

use mpst::functionmpst::choose::choose_left_mpst_session_c_to_all;
use mpst::functionmpst::choose::choose_right_mpst_session_c_to_all;

use mpst::functionmpst::ChooseMpst;
use mpst::functionmpst::OfferMpst;

/// Test our usecase from Places 2020
/// Simple types
/// Client = C
/// Authenticator = A
/// Server = B

type CtoAClose = End;
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
type QueueCVideo = RoleCtoA<RoleCtoB<RoleEnd>>;
type QueueCChoice = RoleCtoAll<QueueCVideo, QueueCEnd>;
type QueueCFull = RoleCtoA<RoleCtoA<QueueCChoice>>;

type QueueBEnd = RoleEnd;
type QueueBVideo = RoleBtoA<RoleBtoA<RoleEnd>>;
type QueueBFull = RoleBtoA<RoleEnd>;

type QueueAEnd = RoleEnd;
type QueueAVideo = RoleAtoC<RoleAtoB<RoleAtoB<RoleAtoC<RoleEnd>>>>;
type QueueAFull = RoleAtoC<RoleAtoC<RoleAtoC<RoleEnd>>>;

/// Creating the MP sessions
/// For C
type EndpointCVideo<N> = SessionMpst<CtoAVideo<N>, CtoBClose, QueueCVideo>;
type EndpointCEnd = SessionMpst<CtoAClose, CtoBClose, QueueCEnd>;

type ChooseC<N> = ChooseMpst<EndpointCVideo<N>, EndpointCEnd>;
type InitC<N> = Send<N, Recv<N, ChooseC<N>>>;
type EndpointCFull<N> = SessionMpst<InitC<N>, End, QueueCFull>;

/// For A
type EndpointAEnd = SessionMpst<End, End, QueueAEnd>;
type EndpointAVideo<N> = SessionMpst<AtoBVideo<N>, AtoCVideo<N>, QueueAVideo>;

type OfferA<N> = OfferMpst<EndpointAVideo<N>, EndpointAEnd>;
type InitA<N> = Recv<N, Send<N, OfferA<N>>>;
type EndpointAFull<N> = SessionMpst<End, InitA<N>, QueueAFull>;

/// For B
type EndpointBEnd = SessionMpst<End, End, QueueBEnd>;
type EndpointBVideo<N> = SessionMpst<BtoAVideo<N>, End, QueueBVideo>;

type OfferB<N> = OfferMpst<EndpointBVideo<N>, EndpointBEnd>;
type EndpointBFull<N> = SessionMpst<OfferB<N>, End, QueueBFull>;

/// Functions related to endpoints
fn server(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_b_to_a(
        s,
        |s: EndpointBVideo<i32>| {
            let (request, s) = recv_mpst_session_one_b_to_a(s)?;
            let s = send_mpst_session_one_b_to_a(request + 1, s);

            close_mpst(s)?;

            assert_eq!(request, 1);
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

            close_mpst(s)?;

            assert_eq!(video, 2);
            Ok(())
        },
        |s: EndpointAEnd| {
            close_mpst(s)?;

            Ok(())
        },
    )
}

// fn client_video(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
//     {
//         let s = send_mpst_session_one_c_to_a(0, s);
//         let (accept, s) = recv_mpst_session_one_c_to_a(s)?;

//         let s = choose_left_mpst_session_c_to_all::<
//             AtoBVideo<i32>,
//             AtoBClose,
//             AtoCVideo<i32>,
//             BtoCClose,
//             BtoCClose,
//             AtoCClose,
//             QueueAVideo,
//             QueueAEnd,
//             QueueBVideo,
//             QueueBEnd,
//             QueueCVideo,
//             QueueCEnd,
//         >(s);

//         let s = send_mpst_session_one_c_to_a(1, s);
//         let (result, s) = recv_mpst_session_one_c_to_a(s)?;

//         assert_eq!(result, 2);

//         close_mpst(s)?;
//     }
//     Ok(())
// }

// fn simple_calc_pawn(s: EndpointChoiceC) -> Result<(), Box<dyn Error>> {
//     offer_mpst_session_c_to_b(
//         s,
//         |s: EndpointCAdd| {
//             close_mpst(s)?;
//             Ok(())
//         },
//         |s: EndpointCNeg| {
//             close_mpst(s)?;
//             Ok(())
//         },
//     )
// }

// #[test]
// fn simple_calc_works() {
//     assert!(|| -> Result<(), Box<dyn Error>> {
//         // Test the left branch.
//         {
//             let (channel_ab, channel_ba) = Session::new();
//             let (channel_ac, channel_ca) = Session::new();
//             let (channel_bc, channel_cb) = Session::new();

//             let (role_a, _) = Role::new();
//             let (role_b, _) = Role::new();
//             let (role_c, _) = Role::new();

//             let a: EndpointChoiceA<i32> = SessionMpst {
//                 session1: channel_ab,
//                 session2: channel_ac,
//                 queue: role_a,
//             };
//             let b: EndpointChoiceB<i32> = SessionMpst {
//                 session1: channel_ba,
//                 session2: channel_bc,
//                 queue: role_b,
//             };
//             let c: EndpointChoiceC = SessionMpst {
//                 session1: channel_ca,
//                 session2: channel_cb,
//                 queue: role_c,
//             };

//             let thread_a = fork_simple(simple_calc_server, a);
//             let thread_b = fork_simple(simple_calc_client_left, b);
//             let thread_c = fork_simple(simple_calc_pawn, c);

//             thread_a.join().unwrap();
//             thread_b.join().unwrap();
//             thread_c.join().unwrap();
//         }

//         // Test the right branch.
//         {
//             let (channel_ab, channel_ba) = Session::new();
//             let (channel_ac, channel_ca) = Session::new();
//             let (channel_bc, channel_cb) = Session::new();

//             let (role_a, _) = Role::new();
//             let (role_b, _) = Role::new();
//             let (role_c, _) = Role::new();

//             let a: EndpointChoiceA<i32> = SessionMpst {
//                 session1: channel_ab,
//                 session2: channel_ac,
//                 queue: role_a,
//             };
//             let b: EndpointChoiceB<i32> = SessionMpst {
//                 session1: channel_ba,
//                 session2: channel_bc,
//                 queue: role_b,
//             };
//             let c: EndpointChoiceC = SessionMpst {
//                 session1: channel_ca,
//                 session2: channel_cb,
//                 queue: role_c,
//             };

//             let thread_a = fork_simple(simple_calc_server, a);
//             let thread_b = fork_simple(simple_calc_client_right, b);
//             let thread_c = fork_simple(simple_calc_pawn, c);

//             thread_a.join().unwrap();
//             thread_b.join().unwrap();
//             thread_c.join().unwrap();
//         }

//         Ok(())
//     }()
//     .is_ok());
// }
