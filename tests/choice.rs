// extern crate mpst;

// use binary::*;
// use mpst::*;
// use role::*;
// use sessionmpst::SessionMpst;
// use std::boxed::Box;
// use std::error::Error;

// use mpst::functionmpst::close::close_mpst;

// use role::a_receives_from_b::RoleAReceiveFromB;
// use role::b_sends_to_a::RoleBSendToA;
// use mpst::role::end::RoleEnd;

// use mpst::functionmpst::recv::recv_mpst_session_one_a_to_b;
// use mpst::functionmpst::send::send_mpst_session_one_b_to_a;

// use mpst::functionmpst::offer::offer_mpst_session_a_to_b;

// use mpst::functionmpst::choose::choose_left_mpst_session_b_to_a;
// use mpst::functionmpst::choose::choose_right_mpst_session_b_to_a;

// use mpst::functionmpst::ChooseMpst;
// use mpst::functionmpst::OfferMpst;

// /// Test a simple calculator server, implemented using binary choice.
// /// Simple types
// type AtoBNeg<N> = Recv<N, End>;
// type AtoBAdd<N> = Recv<N, End>;

// type BtoANeg<N> = Send<N, End>;
// type BtoAAdd<N> = Send<N, End>;

// /// Queues
// type QueueOfferA = RoleAReceiveFromB<RoleEnd>;
// type QueueFullA = RoleAReceiveFromB<QueueOfferA>;

// type QueueChoiceB = RoleBSendToA<RoleEnd>;
// type QueueFullB = RoleBSendToA<QueueChoiceB>;

// type QueueChoiceC = RoleEnd;

// /// Creating the MP sessions
// /// For A
// type EndpointAAdd<N> = SessionMpst<AtoBNeg<N>, End, QueueOfferA>;
// type EndpointANeg<N> = SessionMpst<AtoBAdd<N>, End, QueueOfferA>;

// type OfferA<N> = OfferMpst<EndpointAAdd<N>, EndpointANeg<N>>;
// type EndpointChoiceA<N> = SessionMpst<OfferA<N>, End, QueueFullA>;

// /// For B
// type EndpointBAdd<N> = SessionMpst<BtoAAdd<N>, End, QueueChoiceB>;
// type EndpointBNeg<N> = SessionMpst<BtoANeg<N>, End, QueueChoiceB>;

// type ChooseB<N> = ChooseMpst<EndpointBAdd<N>, EndpointBNeg<N>>;
// type EndpointChoiceB<N> = SessionMpst<ChooseB<N>, End, QueueFullB>;

// /// For C
// type EndpointChoiceC = SessionMpst<End, End, QueueChoiceC>;

// fn simple_calc_server(s: EndpointChoiceA<i32>) -> Result<(), Box<dyn Error>> {
//     offer_mpst_session_a_to_b(
//         s,
//         |s: EndpointAAdd<i32>| {
//             let (x, s) = recv_mpst_session_one_a_to_b(s)?;
//             close_mpst(s)?;

//             assert_eq!(x, 1);
//             Ok(())
//         },
//         |s: EndpointANeg<i32>| {
//             let (x, s) = recv_mpst_session_one_a_to_b(s)?;
//             close_mpst(s)?;

//             assert_eq!(x, 2);
//             Ok(())
//         },
//     )
// }

// fn simple_calc_client_left(s: EndpointChoiceB<i32>) -> Result<(), Box<dyn Error>> {
//     {
//         let s =
//             choose_left_mpst_session_b_to_a::<_, _, _, BtoANeg<i32>, End, QueueChoiceB, _, _>(s);
//         let s = send_mpst_session_one_b_to_a(1, s);
//         close_mpst(s)?;
//     }
//     Ok(())
// }

// fn simple_calc_client_right(s: EndpointChoiceB<i32>) -> Result<(), Box<dyn Error>> {
//     {
//         let s =
//             choose_right_mpst_session_b_to_a::<BtoANeg<i32>, End, QueueChoiceB, _, _, _, _, _>(s);
//         let s = send_mpst_session_one_b_to_a(2, s);
//         close_mpst(s)?;
//     }
//     Ok(())
// }

// fn simple_calc_pawn(s: EndpointChoiceC) -> Result<(), Box<dyn Error>> {
//     {
//         //close_mpst(s)?;
//     }
//     Ok(())
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