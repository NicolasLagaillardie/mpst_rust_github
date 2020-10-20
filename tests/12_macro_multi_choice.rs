// // Test for parametrisation on the number of roles
// extern crate crossbeam_channel;
// extern crate mpstthree;
// use rand::{thread_rng, Rng};

// use mpstthree::binary::{End, Recv, Send, Session};
// use mpstthree::role::end::RoleEnd;
// use mpstthree::role::Role;
// use mpstthree::{
//     close_mpst, create_broadcast_role, create_choose_left_from_3_to_1_and_2,
//     create_choose_right_from_3_to_1_and_2, create_choose_type_multi, create_normal_role,
//     create_offer_mpst_session_multi, create_offer_type_multi, create_recv_mpst_all_session,
//     create_recv_mpst_session, create_send_mpst_session, create_sessionmpst, fork_mpst_multi,
//     fork_simple_multi,
// };
// use std::error::Error;

// // Create new SessionMpst for three participants
// create_sessionmpst!(SessionMpst, 3);

// // Create new roles
// // normal
// create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
// create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
// create_normal_role!(RoleD, next_d, RoleDDual, next_d_dual);
// // broadcast
// create_broadcast_role!(RoleAlltoD, next_all_to_d, RoleDtoAll, next_d_to_all);

// // Create new send functions
// create_send_mpst_session!(send_mpst_d_to_a, RoleA, next_a, RoleD, SessionMpst, 3, 1);
// create_send_mpst_session!(send_mpst_a_to_d, RoleD, next_d, RoleA, SessionMpst, 3, 2);
// create_send_mpst_session!(send_mpst_d_to_b, RoleB, next_b, RoleD, SessionMpst, 3, 2);
// create_send_mpst_session!(send_mpst_b_to_a, RoleA, next_a, RoleB, SessionMpst, 3, 1);
// create_send_mpst_session!(send_mpst_a_to_b, RoleB, next_b, RoleA, SessionMpst, 3, 1);

// // Create new recv functions and related types
// // normal
// create_recv_mpst_session!(recv_mpst_d_to_a, RoleA, next_a, RoleD, SessionMpst, 3, 1);
// create_recv_mpst_session!(recv_mpst_a_to_d, RoleD, next_d, RoleA, SessionMpst, 3, 2);
// create_recv_mpst_session!(recv_mpst_b_to_d, RoleD, next_d, RoleB, SessionMpst, 3, 2);
// create_recv_mpst_session!(recv_mpst_b_to_a, RoleA, next_a, RoleB, SessionMpst, 3, 1);
// create_recv_mpst_session!(recv_mpst_a_to_b, RoleB, next_b, RoleA, SessionMpst, 3, 1);
// // broadcast
// create_recv_mpst_all_session!(
//     recv_mpst_b_all_to_d,
//     RoleAlltoD,
//     next_all_to_d,
//     RoleB,
//     SessionMpst,
//     3,
//     2
// );
// create_recv_mpst_all_session!(
//     recv_mpst_a_all_to_d,
//     RoleAlltoD,
//     next_all_to_d,
//     RoleA,
//     SessionMpst,
//     3,
//     2
// );

// close_mpst!(close_mpst_multi, SessionMpst, 3);

// create_offer_type_multi!(OfferMpstMultiThree, SessionMpst, 3, 2);
// create_choose_type_multi!(ChooseMpstThree, SessionMpst, 3, 2);

// create_offer_mpst_session_multi!(
//     offer_mpst_session_a_to_c,
//     OfferMpstMultiThree,
//     RoleAlltoD,
//     recv_mpst_a_all_to_d,
//     RoleA,
//     SessionMpst,
//     3,
//     2
// );

// create_offer_mpst_session_multi!(
//     offer_mpst_session_b_to_c,
//     OfferMpstMultiThree,
//     RoleAlltoD,
//     recv_mpst_b_all_to_d,
//     RoleB,
//     SessionMpst,
//     3,
//     2
// );

// // Create the choose functions
// create_choose_right_from_3_to_1_and_2!(
//     choose_right_mpst_session_d_to_all,
//     RoleDtoAll,
//     RoleADual,
//     RoleBDual,
//     next_d_to_all,
//     RoleD
// );
// create_choose_left_from_3_to_1_and_2!(
//     choose_left_mpst_session_d_to_all,
//     RoleDtoAll,
//     RoleADual,
//     RoleBDual,
//     next_d_to_all,
//     RoleD
// );

// // Types
// type AtoCClose = End;
// type AtoBClose = End;
// type AtoCVideo<N> = Recv<N, Send<N, End>>;
// type AtoBVideo<N> = Send<N, Recv<N, End>>;

// type BtoAClose = <AtoBClose as Session>::Dual;
// type BtoCClose = End;
// type BtoAVideo<N> = <AtoBVideo<N> as Session>::Dual;

// type CtoBClose = <BtoCClose as Session>::Dual;
// type CtoAClose = <AtoCClose as Session>::Dual;
// type CtoAVideo<N> = <AtoCVideo<N> as Session>::Dual;

// /// Queues
// type QueueAEnd = RoleEnd;
// type QueueAEndDual = <QueueAEnd as Role>::Dual;
// type QueueAVideo = RoleD<RoleB<RoleB<RoleD<RoleEnd>>>>;
// type QueueAVideoDual = <QueueAVideo as Role>::Dual;
// type QueueAFull = RoleD<RoleD<RoleAlltoD<RoleEnd, RoleEnd>>>;

// type QueueBEnd = RoleEnd;
// type QueueBEndDual = <QueueBEnd as Role>::Dual;
// type QueueBVideo = RoleA<RoleA<RoleEnd>>;
// type QueueBVideoDual = <QueueBVideo as Role>::Dual;
// type QueueBFull = RoleAlltoD<RoleEnd, RoleEnd>;

// type QueueCEnd = RoleEnd;
// type QueueCVideo = RoleA<RoleA<RoleEnd>>;
// type QueueCChoice = RoleDtoAll<QueueCVideo, QueueCEnd>;
// type QueueCFull = RoleA<RoleA<QueueCChoice>>;

// /// Creating the MP sessions
// /// For C
// type ChooseCtoA<N> = ChooseMpstThree<
//     BtoAVideo<N>,
//     CtoAVideo<N>,
//     BtoAClose,
//     CtoAClose,
//     QueueAVideoDual,
//     QueueAEnd,
//     RoleADual<RoleEnd>,
// >;
// type ChooseCtoB<N> = ChooseMpstThree<
//     AtoBVideo<N>,
//     CtoBClose,
//     AtoBClose,
//     CtoBClose,
//     QueueBVideoDual,
//     QueueBEnd,
//     RoleBDual<RoleEnd>,
// >;
// type InitC<N> = Send<N, Recv<N, ChooseCtoA<N>>>;
// type EndpointCFull<N> = SessionMpst<InitC<N>, ChooseCtoB<N>, QueueCFull, RoleD<RoleEnd>>;

// /// For A
// type EndpointAVideo<N> = SessionMpst<AtoBVideo<N>, AtoCVideo<N>, QueueAVideo, RoleA<RoleEnd>>;
// type EndpointAEnd = SessionMpst<AtoBClose, AtoCClose, QueueAEnd, RoleA<RoleEnd>>;

// type OfferA<N> = OfferMpstMultiThree<
//     AtoBVideo<N>,
//     AtoCVideo<N>,
//     AtoBClose,
//     AtoCClose,
//     QueueAVideo,
//     QueueAEnd,
//     RoleA<RoleEnd>,
// >;
// type InitA<N> = Recv<N, Send<N, OfferA<N>>>;
// type EndpointAFull<N> = SessionMpst<End, InitA<N>, QueueAFull, RoleA<RoleEnd>>;

// /// For B
// type EndpointBVideo<N> = SessionMpst<BtoAVideo<N>, BtoCClose, QueueBVideo, RoleB<RoleEnd>>;
// type EndpointBEnd = SessionMpst<BtoAClose, BtoCClose, QueueBEnd, RoleB<RoleEnd>>;

// type OfferB<N> = OfferMpstMultiThree<
//     BtoAVideo<N>,
//     BtoCClose,
//     BtoAClose,
//     BtoCClose,
//     QueueBVideo,
//     QueueBEnd,
//     RoleB<RoleEnd>,
// >;
// type EndpointBFull<N> = SessionMpst<End, OfferB<N>, QueueBFull, RoleB<RoleEnd>>;

// /// Functions related to endpoints
// fn server(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
//     offer_mpst_session_b_to_c(
//         s,
//         |s: EndpointBVideo<i32>| {
//             let (request, s) = recv_mpst_b_to_a(s)?;
//             let s = send_mpst_b_to_a(request + 1, s);

//             close_mpst_multi(s)?;

//             Ok(())
//         },
//         |s: EndpointBEnd| {
//             close_mpst_multi(s)?;

//             Ok(())
//         },
//     )
// }

// fn authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
//     let (id, s) = recv_mpst_a_to_d(s)?;
//     let s = send_mpst_a_to_d(id + 1, s);

//     offer_mpst_session_a_to_c(
//         s,
//         |s: EndpointAVideo<i32>| {
//             let (request, s) = recv_mpst_a_to_d(s)?;
//             let s = send_mpst_a_to_b(request + 1, s);
//             let (video, s) = recv_mpst_a_to_b(s)?;
//             let s = send_mpst_a_to_d(video + 1, s);

//             assert_eq!(request, id + 1);
//             assert_eq!(video, id + 3);

//             close_mpst_multi(s)?;
//             Ok(())
//         },
//         |s: EndpointAEnd| {
//             close_mpst_multi(s)?;

//             Ok(())
//         },
//     )
// }

// fn client_video(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
//     {
//         let mut rng = thread_rng();
//         let id: i32 = rng.gen();

//         let s = send_mpst_d_to_a(id, s);
//         let (accept, s) = recv_mpst_d_to_a(s)?;

//         assert_eq!(accept, id + 1);

//         let s = choose_left_mpst_session_d_to_all::<
//             BtoAVideo<i32>,
//             BtoAClose,
//             CtoAVideo<i32>,
//             CtoAClose,
//             BtoCClose,
//             AtoCClose,
//             QueueAVideoDual,
//             QueueAEndDual,
//             QueueBVideoDual,
//             QueueBEndDual,
//             QueueCVideo,
//             QueueCEnd,
//         >(s);

//         let s = send_mpst_d_to_a(accept, s);
//         let (result, s) = recv_mpst_d_to_a(s)?;

//         assert_eq!(result, accept + 3);

//         close_mpst_multi(s)?;
//     }
//     Ok(())
// }

// fn client_close(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
//     {
//         let mut rng = thread_rng();
//         let id: i32 = rng.gen();

//         let s = send_mpst_d_to_a(id, s);
//         let (accept, s) = recv_mpst_d_to_a(s)?;

//         assert_eq!(accept, id + 1);

//         let s = choose_right_mpst_session_d_to_all::<
//             BtoAVideo<i32>,
//             BtoAClose,
//             CtoAVideo<i32>,
//             CtoAClose,
//             BtoCClose,
//             AtoCClose,
//             QueueAVideoDual,
//             QueueAEndDual,
//             QueueBVideoDual,
//             QueueBEndDual,
//             QueueCVideo,
//             QueueCEnd,
//         >(s);

//         close_mpst_multi(s)?;
//     }
//     Ok(())
// }

// ////////////////////////////////////////
// // To be replaced
// // TODO

// fork_simple_multi!(fork_simple_multi, SessionMpst, 3);
// fork_mpst_multi!(fork_mpst_multi, fork_simple_multi, SessionMpst, 3);

// #[doc(hidden)]
// pub fn fork_simple<S1, S2, R, N, P>(
//     p: P,
//     s: SessionMpst<S1, S2, R, N>,
// ) -> std::thread::JoinHandle<()>
// where
//     S1: mpstthree::binary::Session + 'static,
//     S2: mpstthree::binary::Session + 'static,
//     R: mpstthree::role::Role + 'static,
//     N: mpstthree::role::Role + 'static,
//     P: FnOnce(SessionMpst<S1, S2, R, N>) -> Result<(), Box<dyn Error>>
//         + std::marker::Send
//         + 'static,
// {
//     std::thread::spawn(move || {
//         std::panic::set_hook(Box::new(|_info| {
//             // do nothing
//         }));
//         match p(s) {
//             Ok(()) => (),
//             Err(e) => panic!("{:?}", e),
//         }
//     })
// }

// // fork_simple_multi!(SessionMpst, 3);

// /// Creates and returns three child processes for three `SessionMpst` linked together.
// ///
// /// Creates 3 pairs of endpoints, each pair of type `S` and `S::Dual`.
// /// Creates 3 `Role` for each queue.
// /// Creates 3 `SessionMpst`, linked together with the pairs of endpoints, and get the related child processes from `fork_simple`.
// /// Returns the tuple of the 3 child processes.
// pub fn fork_mpst<S1, S2, S3, R1, R2, R3, N1, N2, N3, F1, F2, F3>(
//     f1: F1,
//     f2: F2,
//     f3: F3,
// ) -> (
//     Result<(), Box<(dyn std::any::Any + std::marker::Send + 'static)>>,
//     Result<(), Box<(dyn std::any::Any + std::marker::Send + 'static)>>,
//     Result<(), Box<(dyn std::any::Any + std::marker::Send + 'static)>>,
// )
// where
//     S1: mpstthree::binary::Session + 'static,
//     S2: mpstthree::binary::Session + 'static,
//     S3: mpstthree::binary::Session + 'static,
//     R1: mpstthree::role::Role + 'static,
//     R2: mpstthree::role::Role + 'static,
//     R3: mpstthree::role::Role + 'static,
//     N1: mpstthree::role::Role + 'static,
//     N2: mpstthree::role::Role + 'static,
//     N3: mpstthree::role::Role + 'static,
//     F1: FnOnce(
//             SessionMpst<S1, <S3 as mpstthree::binary::Session>::Dual, R1, N1>,
//         ) -> Result<(), Box<dyn Error>>
//         + std::marker::Send
//         + 'static,
//     F2: FnOnce(
//             SessionMpst<<S1 as mpstthree::binary::Session>::Dual, S2, R2, N2>,
//         ) -> Result<(), Box<dyn Error>>
//         + std::marker::Send
//         + 'static,
//     F3: FnOnce(
//             SessionMpst<S3, <S2 as mpstthree::binary::Session>::Dual, R3, N3>,
//         ) -> Result<(), Box<dyn Error>>
//         + std::marker::Send
//         + 'static,
// {
//     let (channel_ab, channel_ba) = S1::new();
//     let (channel_ca, channel_ac) = S3::new();
//     let (channel_bc, channel_cb) = S2::new();

//     let (role_a, _) = R1::new();
//     let (role_b, _) = R2::new();
//     let (role_c, _) = R3::new();

//     let (name_a, _) = N1::new();
//     let (name_b, _) = N2::new();
//     let (name_c, _) = N3::new();

//     let a = SessionMpst {
//         session1: channel_ab,
//         session2: channel_ac,
//         stack: role_a,
//         name: name_a,
//     };
//     let b = SessionMpst {
//         session1: channel_ba,
//         session2: channel_bc,
//         stack: role_b,
//         name: name_b,
//     };
//     let c = SessionMpst {
//         session1: channel_ca,
//         session2: channel_cb,
//         stack: role_c,
//         name: name_c,
//     };

//     let thread_a = fork_simple(f1, a);
//     let thread_b = fork_simple(f2, b);
//     let thread_c = fork_simple(f3, c);

//     (thread_a.join(), thread_b.join(), thread_c.join())
// }

// ////////////////////////////////////////

// #[test]
// fn test_new_send() {
//     assert!(|| -> Result<(), Box<dyn Error>> {
//         {
//             let (thread_a, thread_pawn, thread_d) = fork_mpst(authenticator, server, client_video);

//             assert!(thread_a.is_ok());
//             assert!(thread_pawn.is_ok());
//             assert!(thread_d.is_ok());
//         }
//         Ok(())
//     }()
//     .is_ok());
// }
