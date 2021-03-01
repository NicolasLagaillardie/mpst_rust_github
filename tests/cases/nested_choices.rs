// Unfinished

// use rand::{thread_rng, Rng};

// use mpstthree::checking::checker;

// use mpstthree::binary::{End, Recv, Send, Session};
// use mpstthree::fork_mpst;
// use mpstthree::role::Role;
// use mpstthree::sessionmpst::SessionMpst;

// use std::boxed::Box;
// use std::collections::HashMap;
// use std::error::Error;

// use mpstthree::functionmpst::close::close_mpst;

// use mpstthree::role::a_to_b::RoleAtoB;
// use mpstthree::role::a_to_c::RoleAtoC;
// use mpstthree::role::all_to_c::RoleAlltoC;
// use mpstthree::role::b_to_a::RoleBtoA;
// use mpstthree::role::b_to_c::RoleBtoC;
// use mpstthree::role::c_to_a::RoleCtoA;
// use mpstthree::role::c_to_all::RoleCtoAll;
// use mpstthree::role::end::RoleEnd;

// use mpstthree::functionmpst::recv::recv_mpst_a_to_b;
// use mpstthree::functionmpst::recv::recv_mpst_a_to_c;
// use mpstthree::functionmpst::recv::recv_mpst_b_to_a;
// use mpstthree::functionmpst::recv::recv_mpst_c_to_a;

// use mpstthree::functionmpst::send::send_mpst_a_to_b;
// use mpstthree::functionmpst::send::send_mpst_a_to_c;
// use mpstthree::functionmpst::send::send_mpst_b_to_a;
// use mpstthree::functionmpst::send::send_mpst_c_to_a;

// use mpstthree::functionmpst::offer::
// offer_mpst_session_a_to_c; use mpstthree::functionmpst::
// offer::offer_mpst_session_b_to_c;

// use mpstthree::functionmpst::choose::
// choose_left_mpst_session_c_to_all; use
// mpstthree::functionmpst::choose::choose_right_mpst_session_c_to_all;

// use mpstthree::functionmpst::ChooseMpst;
// use mpstthree::functionmpst::OfferMpst;

// /// Test for nested choices
// ///
// /// A: A?C.A!C.( ( A?C.A!B.A?B.A!C.0 & 0 ) & 0 )
// /// B: ( ( B?A.B!A.0 & 0 ) & 0 )
// /// C: C!A.C?A.( ( C!A.C?A.0 + 0 ) + 0 )

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
// type QueueAVideo =
// RoleAtoC<RoleAtoB<RoleAtoB<RoleAtoC<RoleEnd>>>>;
// type QueueAVideoDual = <QueueAVideo as Role>::Dual;
// type QueueASecond =
// RoleAlltoC<RoleAtoC<RoleAtoC<RoleEnd>>, RoleEnd>;
// type QueueAIniit = RoleAtoC<RoleAtoC<RoleAlltoC<RoleEnd,
// RoleEnd>>>;

// type QueueBEnd = RoleEnd;
// type QueueBVideo = RoleBtoA<RoleBtoA<RoleEnd>>;
// type QueueBVideoDual = <QueueBVideo as Role>::Dual;
// type QueueBFull = RoleAlltoC<RoleEnd, RoleEnd>;
// type QueueBFullDual = <QueueBFull as Role>::Dual;

// type QueueCEnd = RoleEnd;
// type QueueCVideo = RoleCtoA<RoleCtoA<RoleEnd>>;
// type QueueCChoice2 = RoleCtoAll<QueueCVideo, QueueCEnd>;
// type QueueCChoice = RoleCtoAll<QueueCChoice2, QueueCEnd>;
// type QueueCFull = RoleCtoA<RoleCtoA<QueueCChoice>>;

// /// Creating the MP sessions
// /// For A
// type EndpointAEnd = SessionMpst<AtoBClose, AtoCClose,
// QueueAEnd>; type EndpointAVideo<N> =
// SessionMpst<AtoBVideo<N>, AtoCVideo<N>, QueueAVideo>;

// type OfferA2<N> =
//     OfferMpst<AtoBVideo<N>, AtoCVideo<N>, AtoBClose,
// AtoCClose, QueueAVideo, QueueAEnd>; type OfferA<N> =
// OfferMpst<AtoBClose, OfferA2<N>, AtoBClose, AtoCClose,
// QueueASecond, QueueAEnd>; type InitA<N> = Recv<N, Send<N,
// OfferA<N>>>; type EndpointAFull<N> = SessionMpst<End,
// InitA<N>, QueueAIniit>;

// /// For B
// type EndpointBEnd = SessionMpst<BtoAClose, BtoCClose,
// QueueBEnd>; type EndpointBVideo<N> =
// SessionMpst<BtoAVideo<N>, BtoCClose, QueueBVideo>;

// type OfferB2<N> = OfferMpst<BtoAVideo<N>, BtoCClose,
// BtoAClose, BtoCClose, QueueBVideo, QueueBEnd>; type
// OfferB<N> = OfferMpst<BtoAClose, OfferB2<N>, BtoAClose,
// BtoCClose, QueueBFull, QueueBEnd>;

// type EndpointBFull<N> = SessionMpst<End, OfferB<N>,
// QueueBFull>;

// /// For C
// type EndpointCtoAVideo2<N> = SessionMpst<BtoAVideo<N>,
// CtoAVideo<N>, QueueAVideoDual>; type EndpointCtoAEnd2 =
// SessionMpst<BtoAClose, CtoAClose, QueueAEnd>; type
// EndpointCtoBVideo2<N> = SessionMpst<AtoBVideo<N>,
// CtoBClose, QueueBVideoDual>; type EndpointCtoBEnd2 =
// SessionMpst<AtoBClose, CtoBClose, QueueBEnd>;

// type ChooseCtoA2<N> =
//     ChooseMpst<BtoAVideo<N>, CtoAVideo<N>, BtoAClose,
// CtoAClose, QueueAVideoDual, QueueAEnd>; type
// ChooseCtoB2<N> =     ChooseMpst<AtoBVideo<N>, CtoBClose,
// AtoBClose, CtoBClose, QueueBVideoDual, QueueBEnd>;

// type EndpointCtoAVideo<N> = SessionMpst<ChooseCtoA2<N>,
// ChooseCtoB2<N>, QueueCChoice2>; type EndpointCtoAEnd =
// SessionMpst<BtoAClose, CtoAClose, QueueAEnd>; type
// EndpointCtoBVideo<N> = SessionMpst<AtoBVideo<N>,
// CtoBClose, QueueBVideoDual>; type EndpointCtoBEnd =
// SessionMpst<AtoBClose, CtoBClose, QueueBEnd>;

// type ChooseCtoA<N> =
//     ChooseMpst<BtoAClose, ChooseCtoA2<N>, CtoAClose,
// ChooseCtoB2<N>, QueueCChoice2, QueueAEnd>; type
// ChooseCtoB<N> =     ChooseMpst<AtoBClose, ChooseCtoB2<N>,
// AtoBClose, CtoBClose, QueueBFullDual, QueueBEnd>;

// type InitC<N> = Send<N, Recv<N, ChooseCtoA<N>>>;
// type EndpointCFull<N> = SessionMpst<InitC<N>,
// ChooseCtoB<N>, QueueCFull>;

// // type testA = SessionMpst<
// //     End,
// //     Recv<
// //         i32,
// //         Send<
// //             i32,
// //             Recv<
// //                 Either<
// //                     SessionMpst<
// //                         End,
// //                         Recv<
// //                             Either<
// //                                 SessionMpst<
// //                                     Send<i32,
// Recv<i32, End>>, //
// Recv<i32, Send<i32, End>>, //
// RoleAtoC<RoleAtoB<RoleAtoB<RoleAtoC<RoleEnd>>>>, //
// >, //                                 SessionMpst<End,
// End, RoleEnd>, //                             >,
// //                             End,
// //                         >,
// //                         RoleAtoC<RoleEnd>,
// //                     >,
// //                     SessionMpst<End, End, RoleEnd>,
// //                 >,
// //                 End,
// //             >,
// //         >,
// //     >,
// //     RoleAtoC<RoleAtoC<RoleAtoC<RoleEnd>>>,
// // >;
// // type testB = SessionMpst<
// //     End,
// //     Recv<
// //         Either<
// //             SessionMpst<
// //                 End,
// //                 Recv<
// //                     Either<
// //                         SessionMpst<Recv<i32,
// Send<i32, End>>, End, RoleBtoA<RoleBtoA<RoleEnd>>>, //
// SessionMpst<End, End, RoleEnd>, //                     >,
// //                     End,
// //                 >,
// //                 RoleBtoC<RoleEnd>,
// //             >,
// //             SessionMpst<End, End, RoleEnd>,
// //         >,
// //         End,
// //     >,
// //     RoleBtoC<RoleEnd>,
// // >;
// // type testC = SessionMpst<
// //     Send<
// //         i32,
// //         Recv<
// //             i32,
// //             Send<
// //                 Either<
// //                     SessionMpst<
// //                         End,
// //                         Send<
// //                             Either<
// //                                 SessionMpst<
// //                                     Send<i32,
// Recv<i32, End>>, //
// Recv<i32, Send<i32, End>>, //
// RoleAtoC<RoleAtoB<RoleAtoB<RoleAtoC<RoleEnd>>>>, //
// >, //                                 SessionMpst<End,
// End, RoleEnd>, //                             >,
// //                             End,
// //                         >,
// //
// RoleAlltoC<RoleAtoC<RoleAtoC<RoleEnd>>, RoleEnd>, //
// >, //                     SessionMpst<
// //                         End,
// //                         Send<
// //                             Either<
// //                                 SessionMpst<
// //                                     Recv<i32,
// Send<i32, End>>, //
// End, //
// RoleBtoA<RoleBtoA<RoleEnd>>, //
// >, //                                 SessionMpst<End,
// End, RoleEnd>, //                             >,
// //                             End,
// //                         >,
// //                         RoleEnd,
// //                     >,
// //                 >,
// //                 End,
// //             >,
// //         >,
// //     >,
// //     Send<
// //         Either<
// //             SessionMpst<
// //                 End,
// //                 Send<
// //                     Either<
// //                         SessionMpst<Recv<i32,
// Send<i32, End>>, End, RoleBtoA<RoleBtoA<RoleEnd>>>, //
// SessionMpst<End, End, RoleEnd>, //                     >,
// //                     End,
// //                 >,
// //                 RoleBtoC<RoleEnd>,
// //             >,
// //             SessionMpst<End, End, RoleEnd>,
// //         >,
// //         End,
// //     >,
// //     RoleCtoA<RoleCtoA<RoleCtoAll<RoleCtoAll<RoleCtoA<RoleCtoA<RoleEnd>>,
// RoleEnd>, RoleEnd>>>, // >;

// type resultExpected = mpstthree::binary::Recv<
//     _,
//     mpstthree::binary::Send<
//         _,
//         mpstthree::binary::Recv<
//             either::Either<
//                 _,
//                 mpstthree::sessionmpst::SessionMpst<
//                     _,
//                     mpstthree::binary::Recv<
//                         either::Either<
//
// mpstthree::sessionmpst::SessionMpst<
// mpstthree::binary::Recv<
// i32,
// mpstthree::binary::Send<i32, mpstthree::binary::End>,
// >,
// mpstthree::binary::End,
// mpstthree::role::b_to_a::RoleBtoA<
// mpstthree::role::b_to_a::RoleBtoA<
// mpstthree::role::end::RoleEnd,
// >,                                 >,
//                             >,
//
// mpstthree::sessionmpst::SessionMpst<
// mpstthree::binary::End,
// mpstthree::binary::End,
// mpstthree::role::end::RoleEnd,
// >,                         >,
//                         mpstthree::binary::End,
//                     >,
//                     _,
//                 >,
//             >,
//             _,
//         >,
//     >,
// >;

// type resultFound = mpstthree::binary::Recv<
//     _,
//     mpstthree::binary::Send<
//         _,
//         mpstthree::binary::Recv<
//             either::Either<_,
// mpstthree::sessionmpst::SessionMpst<_, mpstthree::binary:
// :End, _>>,             _,         >,
//     >,
// >;

// // /// Functions related to endpoints
// // fn server(s: EndpointBFull<i32>) -> Result<(), Box<dyn
// Error>> { //     offer_mpst_session_b_to_c(
// //         s,
// //         |s: EndpointBVideo<i32>| {
// //             let (request, s) = recv_mpst_b_to_a(s)?;
// //             let s = send_mpst_b_to_a(request + 1, s);

// //             close_mpst(s)?;

// //             Ok(())
// //         },
// //         |s: EndpointBEnd| {
// //             close_mpst(s)?;

// //             Ok(())
// //         },
// //     )
// // }

// // fn authenticator(s: EndpointAFull<i32>) -> Result<(),
// Box<dyn Error>> { //     let (id, s) =
// recv_mpst_a_to_c(s)?; //     let s = send_mpst_a_to_c(id
// + 1, s);

// //     offer_mpst_session_a_to_c(
// //         s,
// //         |s: EndpointAVideo<i32>| {
// //             let (request, s) = recv_mpst_a_to_c(s)?;
// //             let s = send_mpst_a_to_b(request + 1, s);
// //             let (video, s) = recv_mpst_a_to_b(s)?;
// //             let s = send_mpst_a_to_c(video + 1, s);

// //             assert_eq!(request, id + 1);
// //             assert_eq!(video, id + 3);

// //             close_mpst(s)?;
// //             Ok(())
// //         },
// //         |s: EndpointAEnd| {
// //             close_mpst(s)?;

// //             Ok(())
// //         },
// //     )
// // }

// // fn client_video_video(s: EndpointCFull<i32>) ->
// Result<(), Box<dyn Error>> { //     {
// //         let mut rng = thread_rng();
// //         let id: i32 = rng.gen();

// //         let s = send_mpst_c_to_a(id, s);
// //         let (accept, s) = recv_mpst_c_to_a(s)?;

// //         assert_eq!(accept, id + 1);

// //         let s = choose_left_mpst_session_c_to_all::<
// //             BtoAVideo<i32>,
// //             AtoBClose,
// //             CtoAVideo<i32>,
// //             BtoCClose,
// //             BtoCClose,
// //             AtoCClose,
// //             QueueAVideoDual,
// //             QueueAEnd,
// //             QueueBVideoDual,
// //             QueueBEnd,
// //             QueueCVideo,
// //             QueueCEnd,
// //         >(s);

// //         let s = send_mpst_c_to_a(accept, s);
// //         let (result, s) = recv_mpst_c_to_a(s)?;

// //         assert_eq!(result, accept + 3);

// //         close_mpst(s)?;
// //     }
// //     Ok(())
// // }

// // fn client_video_close(s: EndpointCFull<i32>) ->
// Result<(), Box<dyn Error>> { //     {
// //         let mut rng = thread_rng();
// //         let id: i32 = rng.gen();

// //         let s = send_mpst_c_to_a(id, s);
// //         let (accept, s) = recv_mpst_c_to_a(s)?;

// //         assert_eq!(accept, id + 1);

// //         let s = choose_left_mpst_session_c_to_all::<
// //             BtoAVideo<i32>,
// //             AtoBClose,
// //             CtoAVideo<i32>,
// //             BtoCClose,
// //             BtoCClose,
// //             AtoCClose,
// //             QueueAVideoDual,
// //             QueueAEnd,
// //             QueueBVideoDual,
// //             QueueBEnd,
// //             QueueCVideo,
// //             QueueCEnd,
// //         >(s);

// //         let s = send_mpst_c_to_a(accept, s);
// //         let (result, s) = recv_mpst_c_to_a(s)?;

// //         assert_eq!(result, accept + 3);

// //         close_mpst(s)?;
// //     }
// //     Ok(())
// // }

// // fn client_close(s: EndpointCFull<i32>) -> Result<(),
// Box<dyn Error>> { //     {
// //         let mut rng = thread_rng();
// //         let id: i32 = rng.gen();

// //         let s = send_mpst_c_to_a(id, s);
// //         let (accept, s) = recv_mpst_c_to_a(s)?;

// //         assert_eq!(accept, id + 1);

// //         let s = choose_right_mpst_session_c_to_all::<
// //             BtoAVideo<i32>,
// //             AtoBClose,
// //             CtoAVideo<i32>,
// //             BtoCClose,
// //             BtoCClose,
// //             AtoCClose,
// //             QueueAVideoDual,
// //             QueueAEnd,
// //             QueueBVideoDual,
// //             QueueBEnd,
// //             QueueCVideo,
// //             QueueCEnd,
// //         >(s);

// //         let s = send_mpst_c_to_a(accept, s);
// //         let (result, s) = recv_mpst_c_to_a(s)?;

// //         assert_eq!(result, accept + 3);

// //         close_mpst(s)?;
// //     }
// //     Ok(())
// // }

// #[test]
// fn run_nested_choices() {
//     // assert!(|| -> Result<(), Box<dyn Error>> {
//     //     // Test video branch.
//     //     {
//     //         let (thread_a, thread_b, thread_c) =
//     //             fork_mpst(authenticator, server,
// client_video_video);

//     //         assert!(thread_a.is_ok());
//     //         assert!(thread_b.is_ok());
//     //         assert!(thread_c.is_ok());
//     //     }

//     //     // Test end branch.
//     //     {
//     //         let (thread_a, thread_b, thread_c) =
// fork_mpst(authenticator, server, client_close);

//     //         assert!(thread_a.is_ok());
//     //         assert!(thread_b.is_ok());
//     //         assert!(thread_c.is_ok());
//     //     }

//     //     Ok(())
//     // }()
//     // .is_ok());

//     assert!(|| -> Result<(), Box<dyn Error>> {
//         {
//             let hm: HashMap<String, &Vec<String>> =
// HashMap::new();

//             let (s1, _): (EndpointAFull<i32>, _) =
// SessionMpst::new();             let (s2, _):
// (EndpointBFull<i32>, _) = SessionMpst::new();
// let (s3, _): (EndpointCFull<i32>, _) =
// SessionMpst::new();

//             let (a, b, c) = checker(s1, s2, s3, &hm)?;

//             assert_eq!(a, "A: A?C.A!C.( (
// A?C.A!B.A?B.A!C.0 & 0 ) & 0 )");
// assert_eq!(b, "B: ( ( B?A.B!A.0 & 0 ) & 0 )");
//             assert_eq!(c, "C: C!A.C?A.( ( C?A.C!A.0 + 0 )
// + 0 )");         }
//         Ok(())
//     }()
//     .is_ok());
// }
