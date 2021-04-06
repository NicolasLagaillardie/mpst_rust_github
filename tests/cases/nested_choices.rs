// Unfinished

// use rand::{thread_rng, Rng};

// use mpstthree::checking::checker;

// use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
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

// use mpstthree::functionmpst::recv::recv_mpst_a_from_b;
// use mpstthree::functionmpst::recv::recv_mpst_a_from_c;
// use mpstthree::functionmpst::recv::recv_mpst_b_from_a;
// use mpstthree::functionmpst::recv::recv_mpst_c_from_a;

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

// /// Stacks
// type StackAEnd = RoleEnd;
// type StackAVideo =
// RoleAtoC<RoleAtoB<RoleAtoB<RoleAtoC<RoleEnd>>>>;
// type StackAVideoDual = <StackAVideo as Role>::Dual;
// type StackASecond =
// RoleAlltoC<RoleAtoC<RoleAtoC<RoleEnd>>, RoleEnd>;
// type StackAIniit = RoleAtoC<RoleAtoC<RoleAlltoC<RoleEnd,
// RoleEnd>>>;

// type StackBEnd = RoleEnd;
// type StackBVideo = RoleBtoA<RoleBtoA<RoleEnd>>;
// type StackBVideoDual = <StackBVideo as Role>::Dual;
// type StackBFull = RoleAlltoC<RoleEnd, RoleEnd>;
// type StackBFullDual = <StackBFull as Role>::Dual;

// type StackCEnd = RoleEnd;
// type StackCVideo = RoleCtoA<RoleCtoA<RoleEnd>>;
// type StackCChoice2 = RoleCtoAll<StackCVideo, StackCEnd>;
// type StackCChoice = RoleCtoAll<StackCChoice2, StackCEnd>;
// type StackCFull = RoleCtoA<RoleCtoA<StackCChoice>>;

// /// Creating the MP sessions
// /// For A
// type EndpointAEnd = SessionMpst<AtoBClose, AtoCClose,
// StackAEnd>; type EndpointAVideo<N> =
// SessionMpst<AtoBVideo<N>, AtoCVideo<N>, StackAVideo>;

// type OfferA2<N> =
//     OfferMpst<AtoBVideo<N>, AtoCVideo<N>, AtoBClose,
// AtoCClose, StackAVideo, StackAEnd>; type OfferA<N> =
// OfferMpst<AtoBClose, OfferA2<N>, AtoBClose, AtoCClose,
// StackASecond, StackAEnd>; type InitA<N> = Recv<N, Send<N,
// OfferA<N>>>; type EndpointAFull<N> = SessionMpst<End,
// InitA<N>, StackAIniit>;

// /// For B
// type EndpointBEnd = SessionMpst<BtoAClose, BtoCClose,
// StackBEnd>; type EndpointBVideo<N> =
// SessionMpst<BtoAVideo<N>, BtoCClose, StackBVideo>;

// type OfferB2<N> = OfferMpst<BtoAVideo<N>, BtoCClose,
// BtoAClose, BtoCClose, StackBVideo, StackBEnd>; type
// OfferB<N> = OfferMpst<BtoAClose, OfferB2<N>, BtoAClose,
// BtoCClose, StackBFull, StackBEnd>;

// type EndpointBFull<N> = SessionMpst<End, OfferB<N>,
// StackBFull>;

// /// For C
// type EndpointCtoAVideo2<N> = SessionMpst<BtoAVideo<N>,
// CtoAVideo<N>, StackAVideoDual>; type EndpointCtoAEnd2 =
// SessionMpst<BtoAClose, CtoAClose, StackAEnd>; type
// EndpointCtoBVideo2<N> = SessionMpst<AtoBVideo<N>,
// CtoBClose, StackBVideoDual>; type EndpointCtoBEnd2 =
// SessionMpst<AtoBClose, CtoBClose, StackBEnd>;

// type ChooseCtoA2<N> =
//     ChooseMpst<BtoAVideo<N>, CtoAVideo<N>, BtoAClose,
// CtoAClose, StackAVideoDual, StackAEnd>; type
// ChooseCtoB2<N> =     ChooseMpst<AtoBVideo<N>, CtoBClose,
// AtoBClose, CtoBClose, StackBVideoDual, StackBEnd>;

// type EndpointCtoAVideo<N> = SessionMpst<ChooseCtoA2<N>,
// ChooseCtoB2<N>, StackCChoice2>; type EndpointCtoAEnd =
// SessionMpst<BtoAClose, CtoAClose, StackAEnd>; type
// EndpointCtoBVideo<N> = SessionMpst<AtoBVideo<N>,
// CtoBClose, StackBVideoDual>; type EndpointCtoBEnd =
// SessionMpst<AtoBClose, CtoBClose, StackBEnd>;

// type ChooseCtoA<N> =
//     ChooseMpst<BtoAClose, ChooseCtoA2<N>, CtoAClose,
// ChooseCtoB2<N>, StackCChoice2, StackAEnd>; type
// ChooseCtoB<N> =     ChooseMpst<AtoBClose, ChooseCtoB2<N>,
// AtoBClose, CtoBClose, StackBFullDual, StackBEnd>;

// type InitC<N> = Send<N, Recv<N, ChooseCtoA<N>>>;
// type EndpointCFull<N> = SessionMpst<InitC<N>,
// ChooseCtoB<N>, StackCFull>;

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

// type resultExpected = mpstthree::binary::struct_trait::Recv<
//     _,
//     mpstthree::binary::struct_trait::Send<
//         _,
//         mpstthree::binary::struct_trait::Recv<
//             either::Either<
//                 _,
//                 mpstthree::sessionmpst::SessionMpst<
//                     _,
//                     mpstthree::binary::struct_trait::Recv<
//                         either::Either<
//
// mpstthree::sessionmpst::SessionMpst<
// mpstthree::binary::struct_trait::Recv<
// i32,
// mpstthree::binary::struct_trait::Send<i32, mpstthree::binary::struct_trait::End>,
// >,
// mpstthree::binary::struct_trait::End,
// mpstthree::role::b_to_a::RoleBtoA<
// mpstthree::role::b_to_a::RoleBtoA<
// mpstthree::role::end::RoleEnd,
// >,                                 >,
//                             >,
//
// mpstthree::sessionmpst::SessionMpst<
// mpstthree::binary::struct_trait::End,
// mpstthree::binary::struct_trait::End,
// mpstthree::role::end::RoleEnd,
// >,                         >,
//                         mpstthree::binary::struct_trait::End,
//                     >,
//                     _,
//                 >,
//             >,
//             _,
//         >,
//     >,
// >;

// type resultFound = mpstthree::binary::struct_trait::Recv<
//     _,
//     mpstthree::binary::struct_trait::Send<
//         _,
//         mpstthree::binary::struct_trait::Recv<
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
// //             let (request, s) = recv_mpst_b_from_a(s)?;
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
// recv_mpst_a_from_c(s)?; //     let s = send_mpst_a_to_c(id
// + 1, s);

// //     offer_mpst_session_a_to_c(
// //         s,
// //         |s: EndpointAVideo<i32>| {
// //             let (request, s) = recv_mpst_a_from_c(s)?;
// //             let s = send_mpst_a_to_b(request + 1, s);
// //             let (video, s) = recv_mpst_a_from_b(s)?;
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
// //         let (accept, s) = recv_mpst_c_from_a(s)?;

// //         assert_eq!(accept, id + 1);

// //         let s = choose_left_mpst_session_c_to_all::<
// //             BtoAVideo<i32>,
// //             AtoBClose,
// //             CtoAVideo<i32>,
// //             BtoCClose,
// //             BtoCClose,
// //             AtoCClose,
// //             StackAVideoDual,
// //             StackAEnd,
// //             StackBVideoDual,
// //             StackBEnd,
// //             StackCVideo,
// //             StackCEnd,
// //         >(s);

// //         let s = send_mpst_c_to_a(accept, s);
// //         let (result, s) = recv_mpst_c_from_a(s)?;

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
// //         let (accept, s) = recv_mpst_c_from_a(s)?;

// //         assert_eq!(accept, id + 1);

// //         let s = choose_left_mpst_session_c_to_all::<
// //             BtoAVideo<i32>,
// //             AtoBClose,
// //             CtoAVideo<i32>,
// //             BtoCClose,
// //             BtoCClose,
// //             AtoCClose,
// //             StackAVideoDual,
// //             StackAEnd,
// //             StackBVideoDual,
// //             StackBEnd,
// //             StackCVideo,
// //             StackCEnd,
// //         >(s);

// //         let s = send_mpst_c_to_a(accept, s);
// //         let (result, s) = recv_mpst_c_from_a(s)?;

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
// //         let (accept, s) = recv_mpst_c_from_a(s)?;

// //         assert_eq!(accept, id + 1);

// //         let s = choose_right_mpst_session_c_to_all::<
// //             BtoAVideo<i32>,
// //             AtoBClose,
// //             CtoAVideo<i32>,
// //             BtoCClose,
// //             BtoCClose,
// //             AtoCClose,
// //             StackAVideoDual,
// //             StackAEnd,
// //             StackBVideoDual,
// //             StackBEnd,
// //             StackCVideo,
// //             StackCEnd,
// //         >(s);

// //         let s = send_mpst_c_to_a(accept, s);
// //         let (result, s) = recv_mpst_c_from_a(s)?;

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
