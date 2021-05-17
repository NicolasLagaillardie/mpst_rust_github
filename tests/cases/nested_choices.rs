// use std::boxed::Box;
// use std::error::Error;

// use mpstthree::functionmpst::close::close_mpst;

// use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
// use mpstthree::fork::fork_mpst;
// use mpstthree::role::Role;
// use mpstthree::sessionmpst::SessionMpst;

// use mpstthree::role::a::RoleA;
// use mpstthree::role::a_dual::RoleADual;
// use mpstthree::role::all_to_b::RoleAlltoB;
// use mpstthree::role::b::RoleB;
// use mpstthree::role::b_to_all::RoleBtoAll;
// use mpstthree::role::c::RoleC;
// use mpstthree::role::c_dual::RoleCDual;
// use mpstthree::role::end::RoleEnd;

// use mpstthree::functionmpst::recv::recv_mpst_a_from_b;
// use mpstthree::functionmpst::recv::recv_mpst_c_from_a;

// use mpstthree::functionmpst::send::send_mpst_a_to_c;
// use mpstthree::functionmpst::send::send_mpst_b_to_a;

// use mpstthree::functionmpst::offer::offer_mpst_session_to_a_from_b;
// use mpstthree::functionmpst::offer::offer_mpst_session_to_c_from_b;

// use mpstthree::functionmpst::choose::choose_left_mpst_session_b_to_all;
// use mpstthree::functionmpst::choose::choose_right_mpst_session_b_to_all;

// use mpstthree::functionmpst::ChooseMpst;
// use mpstthree::functionmpst::OfferMpst;

// /// Test a simple storage server, implemented using binary
// /// choice. Simple types
// type AtoBNeg<N> = Recv<N, End>;
// type AtoBAdd<N> = Recv<N, End>;
// type AtoBAddOffer<N> = Recv<N, OfferA1<N>>;

// type AtoCNeg<N> = Send<N, End>;
// type AtoCAdd<N> = Send<N, End>;
// type AtoCAddOffer<N> = Send<N, End>;

// type BtoANeg<N> = <AtoBNeg<N> as Session>::Dual;
// type BtoAAdd<N> = <AtoBAdd<N> as Session>::Dual;
// type BtoAAddOffer<N> = <AtoBAddOffer<N> as Session>::Dual;

// type CtoANeg<N> = <AtoCNeg<N> as Session>::Dual;
// type CtoAAdd<N> = <AtoCAdd<N> as Session>::Dual;
// type CtoAAddOffer<N> = <AtoCAddOffer<N> as Session>::Dual;

// /// Stacks
// type StackOfferA = RoleB<RoleC<RoleEnd>>;
// type StackOfferADual = <StackOfferA as Role>::Dual;
// type StackOfferDoubleA = RoleB<RoleC<RoleAlltoB<RoleEnd, RoleEnd>>>;
// type StackOfferDoubleADual = <StackOfferDoubleA as Role>::Dual;
// type StackFullA = RoleAlltoB<RoleEnd, RoleEnd>;

// type StackChoiceB = RoleA<RoleEnd>;
// type StackFullB = RoleBtoAll<StackChoiceB, StackChoiceB>;
// type StackChoiceDoubleB = RoleA<RoleEnd>;
// type StackFullDoubleB = RoleA<RoleBtoAll<StackChoiceDoubleB, StackChoiceDoubleB>>;

// type StackOfferC = RoleA<RoleEnd>;
// type StackOfferCDual = <StackOfferC as Role>::Dual;
// type StackOfferDoubleC = RoleA<StackFullC>;
// type StackOfferDoubleCDual = <StackOfferDoubleC as Role>::Dual;
// type StackFullC = RoleAlltoB<RoleEnd, RoleEnd>;

// /// Creating the MP sessions
// /// For A
// type EndpointAAdd<N> = SessionMpst<AtoBAdd<N>, AtoCAdd<N>, StackOfferA, RoleA<RoleEnd>>;
// type EndpointAAddOffer<N> =
//     SessionMpst<AtoBAddOffer<N>, AtoCAddOffer<N>, StackOfferDoubleA, RoleA<RoleEnd>>;
// type EndpointANeg<N> = SessionMpst<AtoBNeg<N>, AtoCNeg<N>, StackOfferA, RoleA<RoleEnd>>;

// type OfferA0<N> = OfferMpst<
//     AtoBAddOffer<N>,
//     AtoCAddOffer<N>,
//     AtoBNeg<N>,
//     AtoCNeg<N>,
//     StackOfferDoubleA,
//     StackOfferA,
//     RoleA<RoleEnd>,
// >;
// type OfferA1<N> = OfferMpst<
//     AtoBAdd<N>,
//     AtoCAdd<N>,
//     AtoBNeg<N>,
//     AtoCNeg<N>,
//     StackOfferA,
//     StackOfferA,
//     RoleA<RoleEnd>,
// >;
// type EndpointChoiceA0<N> = SessionMpst<OfferA0<N>, End, StackFullA, RoleA<RoleEnd>>;
// type EndpointChoiceA1<N> = SessionMpst<OfferA1<N>, End, StackFullA, RoleA<RoleEnd>>;

// /// For B
// type ChooseBtoA0<N> = ChooseMpst<
//     BtoAAddOffer<N>,
//     CtoAAddOffer<N>,
//     BtoANeg<N>,
//     CtoANeg<N>,
//     StackOfferADual,
//     StackOfferADual,
//     RoleADual<RoleEnd>,
// >;
// type ChooseBtoA1<N> = ChooseMpst<
//     BtoAAdd<N>,
//     CtoAAdd<N>,
//     BtoANeg<N>,
//     CtoANeg<N>,
//     StackOfferADual,
//     StackOfferADual,
//     RoleADual<RoleEnd>,
// >;
// type ChooseBtoC0<N> = ChooseMpst<
//     AtoCAdd<N>,
//     OfferC1<N>,
//     AtoCNeg<N>,
//     End,
//     StackOfferCDual,
//     StackOfferCDual,
//     RoleCDual<RoleEnd>,
// >;
// type ChooseBtoC1<N> = ChooseMpst<
//     AtoCAdd<N>,
//     End,
//     AtoCNeg<N>,
//     End,
//     StackOfferCDual,
//     StackOfferCDual,
//     RoleCDual<RoleEnd>,
// >;
// type EndpointChoiceB0<N> = SessionMpst<ChooseBtoA0<N>, ChooseBtoC0<N>, StackFullB, RoleB<RoleEnd>>;
// type EndpointChoiceB1<N> = SessionMpst<ChooseBtoA1<N>, ChooseBtoC1<N>, StackFullB, RoleB<RoleEnd>>;

// /// For C
// type EndpointCAddOffer<N> =
//     SessionMpst<CtoAAddOffer<N>, OfferC1<N>, StackOfferDoubleC, RoleC<RoleEnd>>;
// type EndpointCAdd<N> = SessionMpst<CtoAAdd<N>, End, StackOfferC, RoleC<RoleEnd>>;
// type EndpointCNeg<N> = SessionMpst<CtoANeg<N>, End, StackOfferC, RoleC<RoleEnd>>;

// type OfferC0<N> = OfferMpst<
//     CtoAAddOffer<N>,
//     OfferC1<N>,
//     CtoANeg<N>,
//     End,
//     StackOfferDoubleC,
//     StackOfferC,
//     RoleC<RoleEnd>,
// >;
// type OfferC1<N> =
//     OfferMpst<CtoAAdd<N>, End, CtoANeg<N>, End, StackOfferC, StackOfferC, RoleC<RoleEnd>>;
// type EndpointChoiceC0<N> = SessionMpst<End, OfferC0<N>, StackFullC, RoleC<RoleEnd>>;
// type EndpointChoiceC1<N> = SessionMpst<End, OfferC1<N>, StackFullC, RoleC<RoleEnd>>;

// /// Functions related to endpoints
// fn simple_store_server_0(s: EndpointChoiceA0<i32>) -> Result<(), Box<dyn Error>> {
//     offer_mpst_session_to_a_from_b(
//         s,
//         |s: EndpointAAddOffer<i32>| {
//             let (x, s) = recv_mpst_a_from_b(s)?;
//             let s = send_mpst_a_to_c(x + 1, s);

//             simple_store_server_1(s)
//         },
//         |s: EndpointANeg<i32>| {
//             let (x, s) = recv_mpst_a_from_b(s)?;
//             let s = send_mpst_a_to_c(x + 1, s);

//             close_mpst(s)
//         },
//     )
// }

// fn simple_store_server_1(s: EndpointChoiceA1<i32>) -> Result<(), Box<dyn Error>> {
//     offer_mpst_session_to_a_from_b(
//         s,
//         |s: EndpointAAdd<i32>| {
//             let (x, s) = recv_mpst_a_from_b(s)?;
//             let s = send_mpst_a_to_c(x + 1, s);

//             close_mpst(s)
//         },
//         |s: EndpointANeg<i32>| {
//             let (x, s) = recv_mpst_a_from_b(s)?;
//             let s = send_mpst_a_to_c(x + 1, s);

//             close_mpst(s)
//         },
//     )
// }

// fn simple_store_client_left_0(s: EndpointChoiceB0<i32>) -> Result<(), Box<dyn Error>> {
//     let s = choose_left_mpst_session_b_to_all::<
//         CtoAAddOffer<i32>,
//         CtoANeg<i32>,
//         BtoAAddOffer<i32>,
//         OfferC1<i32>,
//         BtoANeg<i32>,
//         End,
//         StackOfferDoubleADual,
//         StackOfferDoubleADual,
//         StackOfferDoubleCDual,
//         StackOfferDoubleCDual,
//         StackChoiceDoubleB,
//         StackChoiceDoubleB,
//     >(s);
//     let s = send_mpst_b_to_a(1, s);
//     simple_store_client_left_1(s)
// }

// fn simple_store_client_left_0_1(s: EndpointChoiceB1<i32>) -> Result<(), Box<dyn Error>> {
//     let s = choose_left_mpst_session_b_to_all::<
//         CtoAAdd<i32>,
//         CtoANeg<i32>,
//         BtoAAdd<i32>,
//         End,
//         BtoANeg<i32>,
//         End,
//         StackOfferADual,
//         StackOfferADual,
//         StackOfferCDual,
//         StackOfferCDual,
//         StackChoiceB,
//         StackChoiceB,
//     >(s);
//     let s = send_mpst_b_to_a(1, s);
//     close_mpst(s)
// }

// fn simple_store_client_right_0_1(s: EndpointChoiceB1<i32>) -> Result<(), Box<dyn Error>> {
//     let s = choose_right_mpst_session_b_to_all::<
//         CtoAAdd<i32>,
//         CtoANeg<i32>,
//         BtoAAdd<i32>,
//         End,
//         BtoANeg<i32>,
//         End,
//         StackOfferADual,
//         StackOfferADual,
//         StackOfferCDual,
//         StackOfferCDual,
//         StackChoiceB,
//         StackChoiceB,
//     >(s);
//     let s = send_mpst_b_to_a(1, s);
//     close_mpst(s)
// }

// fn simple_store_client_right_0(s: EndpointChoiceB0<i32>) -> Result<(), Box<dyn Error>> {
//     let s = choose_right_mpst_session_b_to_all::<
//         CtoAAdd<i32>,
//         CtoANeg<i32>,
//         BtoAAdd<i32>,
//         End,
//         BtoANeg<i32>,
//         End,
//         StackOfferADual,
//         StackOfferADual,
//         StackOfferCDual,
//         StackOfferCDual,
//         StackChoiceB,
//         StackChoiceB,
//     >(s);
//     let s = send_mpst_b_to_a(2, s);
//     close_mpst(s)
// }

// fn simple_store_pawn_0(s: EndpointChoiceC0<i32>) -> Result<(), Box<dyn Error>> {
//     offer_mpst_session_to_c_from_b(
//         s,
//         |s: EndpointCAddOffer<i32>| {
//             let (x, s) = recv_mpst_c_from_a(s)?;

//             simple_store_pawn_1(s)
//         },
//         |s: EndpointCNeg<i32>| {
//             let (x, s) = recv_mpst_c_from_a(s)?;

//             close_mpst(s)
//         },
//     )
// }

// fn simple_store_pawn_1(s: EndpointChoiceC1<i32>) -> Result<(), Box<dyn Error>> {
//     offer_mpst_session_to_c_from_b(
//         s,
//         |s: EndpointCAdd<i32>| {
//             let (x, s) = recv_mpst_c_from_a(s)?;

//             close_mpst(s)
//         },
//         |s: EndpointCNeg<i32>| {
//             let (x, s) = recv_mpst_c_from_a(s)?;

//             close_mpst(s)
//         },
//     )
// }

// /////////////////////////////////////////

// pub fn nested_choice() {
//     assert!(|| -> Result<(), Box<dyn Error>> {
//         // Test the left branch.
//         {
//             let (thread_a, thread_b, thread_c) = fork_mpst(
//                 simple_store_server_0,
//                 simple_store_client_left_0,
//                 simple_store_pawn_0,
//             );

//             assert!(thread_a.join().is_ok());
//             assert!(thread_b.join().is_ok());
//             assert!(thread_c.join().is_ok());
//         }

//         // Test the right branch.
//         {
//             let (thread_a, thread_b, thread_c) = fork_mpst(
//                 simple_store_server_0,
//                 simple_store_client_right_0,
//                 simple_store_pawn_0,
//             );

//             assert!(thread_a.join().is_ok());
//             assert!(thread_b.join().is_ok());
//             assert!(thread_c.join().is_ok());
//         }

//         Ok(())
//     }()
//     .is_ok());
// }

pub fn nested_choice() {}
