#![allow(dead_code)]

// // Test for parametrisation on the number of roles
// use rand::{thread_rng, Rng};

// use mpstthree::binary::{End, Recv, Send, Session};
// use mpstthree::role::end::RoleEnd;
// use mpstthree::role::Role;
// use mpstthree::{
//     fork_mpst_multi, close_mpst, create_broadcast_role,
// create_choose_mpst_session_multi_both,
// create_choose_type_multi, create_normal_role,
// create_offer_mpst_session_multi,
//     create_offer_type_multi,
// create_recv_mpst_all_session, create_recv_mpst_session,
// create_send_mpst_session, create_sessionmpst, };
// use std::error::Error;

// // Create new SessionMpst for three participants
// create_sessionmpst!(SessionMpstFive, 5);

// // Create new roles
// // normal
// create_normal_role!(RoleA, next_a, RoleADual,
// next_a_dual); create_normal_role!(RoleB, next_b,
// RoleBDual, next_b_dual); create_normal_role!(RoleC,
// next_c, RoleCDual, next_c_dual); create_normal_role!
// (RoleD, next_d, RoleDDual, next_d_dual);
// create_normal_role!(RoleE, next_e, RoleEDual,
// next_e_dual); // broadcast
// create_broadcast_role!(RoleAlltoA, next_all_to_a,
// RoleAtoAll, next_a_to_all); create_broadcast_role!
// (RoleAlltoB, next_all_to_b, RoleBtoAll, next_b_to_all);
// create_broadcast_role!(RoleAlltoC, next_all_to_c,
// RoleCtoAll, next_c_to_all); create_broadcast_role!
// (RoleAlltoD, next_all_to_d, RoleDtoAll, next_d_to_all);
// create_broadcast_role!(RoleAlltoE, next_all_to_e,
// RoleEtoAll, next_e_to_all);

// // Create new send functions
// // A
// create_send_mpst_session!(
//     send_mpst_b_to_a,
//     RoleA,
//     next_a,
//     RoleB,
//     SessionMpstFive,
//     5,
//     1
// );
// create_send_mpst_session!(
//     send_mpst_c_to_a,
//     RoleA,
//     next_a,
//     RoleC,
//     SessionMpstFive,
//     5,
//     1
// );
// create_send_mpst_session!(
//     send_mpst_d_to_a,
//     RoleA,
//     next_a,
//     RoleD,
//     SessionMpstFive,
//     5,
//     1
// );
// create_send_mpst_session!(
//     send_mpst_e_to_a,
//     RoleA,
//     next_a,
//     RoleE,
//     SessionMpstFive,
//     5,
//     1
// );
// // B
// create_send_mpst_session!(
//     send_mpst_a_to_b,
//     RoleB,
//     next_b,
//     RoleA,
//     SessionMpstFive,
//     5,
//     1
// );
// create_send_mpst_session!(
//     send_mpst_c_to_b,
//     RoleB,
//     next_b,
//     RoleC,
//     SessionMpstFive,
//     5,
//     2
// );
// create_send_mpst_session!(
//     send_mpst_d_to_b,
//     RoleB,
//     next_b,
//     RoleD,
//     SessionMpstFive,
//     5,
//     2
// );
// create_send_mpst_session!(
//     send_mpst_e_to_b,
//     RoleB,
//     next_b,
//     RoleE,
//     SessionMpstFive,
//     5,
//     2
// );
// // C
// create_send_mpst_session!(
//     send_mpst_a_to_c,
//     RoleC,
//     next_c,
//     RoleA,
//     SessionMpstFive,
//     5,
//     2
// );
// create_send_mpst_session!(
//     send_mpst_b_to_c,
//     RoleC,
//     next_c,
//     RoleB,
//     SessionMpstFive,
//     5,
//     2
// );
// create_send_mpst_session!(
//     send_mpst_d_to_c,
//     RoleC,
//     next_c,
//     RoleD,
//     SessionMpstFive,
//     5,
//     3
// );
// create_send_mpst_session!(
//     send_mpst_e_to_c,
//     RoleC,
//     next_c,
//     RoleE,
//     SessionMpstFive,
//     5,
//     3
// );
// // D
// create_send_mpst_session!(
//     send_mpst_a_to_d,
//     RoleD,
//     next_d,
//     RoleA,
//     SessionMpstFive,
//     5,
//     3
// );
// create_send_mpst_session!(
//     send_mpst_b_to_d,
//     RoleD,
//     next_d,
//     RoleB,
//     SessionMpstFive,
//     5,
//     3
// );
// create_send_mpst_session!(
//     send_mpst_c_to_d,
//     RoleD,
//     next_d,
//     RoleC,
//     SessionMpstFive,
//     5,
//     3
// );
// create_send_mpst_session!(
//     send_mpst_e_to_d,
//     RoleD,
//     next_d,
//     RoleE,
//     SessionMpstFive,
//     5,
//     4
// );
// // E
// create_send_mpst_session!(
//     send_mpst_a_to_e,
//     RoleE,
//     next_e,
//     RoleA,
//     SessionMpstFive,
//     5,
//     4
// );
// create_send_mpst_session!(
//     send_mpst_b_to_e,
//     RoleE,
//     next_e,
//     RoleB,
//     SessionMpstFive,
//     5,
//     4
// );
// create_send_mpst_session!(
//     send_mpst_c_to_e,
//     RoleE,
//     next_e,
//     RoleC,
//     SessionMpstFive,
//     5,
//     4
// );
// create_send_mpst_session!(
//     send_mpst_d_to_e,
//     RoleE,
//     next_e,
//     RoleD,
//     SessionMpstFive,
//     5,
//     4
// );

// // Create new recv functions and related types
// // normal
// // A
// create_recv_mpst_session!(
//     recv_mpst_b_to_a,
//     RoleA,
//     next_a,
//     RoleB,
//     SessionMpstFive,
//     5,
//     1
// );
// create_recv_mpst_session!(
//     recv_mpst_c_to_a,
//     RoleA,
//     next_a,
//     RoleC,
//     SessionMpstFive,
//     5,
//     1
// );
// create_recv_mpst_session!(
//     recv_mpst_d_to_a,
//     RoleA,
//     next_a,
//     RoleD,
//     SessionMpstFive,
//     5,
//     1
// );
// create_recv_mpst_session!(
//     recv_mpst_e_to_a,
//     RoleA,
//     next_a,
//     RoleE,
//     SessionMpstFive,
//     5,
//     1
// );
// // B
// create_recv_mpst_session!(
//     recv_mpst_a_to_b,
//     RoleB,
//     next_b,
//     RoleA,
//     SessionMpstFive,
//     5,
//     1
// );
// create_recv_mpst_session!(
//     recv_mpst_c_to_b,
//     RoleB,
//     next_b,
//     RoleC,
//     SessionMpstFive,
//     5,
//     2
// );
// create_recv_mpst_session!(
//     recv_mpst_d_to_b,
//     RoleB,
//     next_b,
//     RoleD,
//     SessionMpstFive,
//     5,
//     2
// );
// create_recv_mpst_session!(
//     recv_mpst_e_to_b,
//     RoleB,
//     next_b,
//     RoleE,
//     SessionMpstFive,
//     5,
//     2
// );
// // C
// create_recv_mpst_session!(
//     recv_mpst_a_to_c,
//     RoleC,
//     next_c,
//     RoleA,
//     SessionMpstFive,
//     5,
//     2
// );
// create_recv_mpst_session!(
//     recv_mpst_b_to_c,
//     RoleC,
//     next_c,
//     RoleB,
//     SessionMpstFive,
//     5,
//     2
// );
// create_recv_mpst_session!(
//     recv_mpst_d_to_c,
//     RoleC,
//     next_c,
//     RoleD,
//     SessionMpstFive,
//     5,
//     3
// );
// create_recv_mpst_session!(
//     recv_mpst_e_to_c,
//     RoleC,
//     next_c,
//     RoleE,
//     SessionMpstFive,
//     5,
//     3
// );
// // D
// create_recv_mpst_session!(
//     recv_mpst_a_to_d,
//     RoleD,
//     next_d,
//     RoleA,
//     SessionMpstFive,
//     5,
//     3
// );
// create_recv_mpst_session!(
//     recv_mpst_b_to_d,
//     RoleD,
//     next_d,
//     RoleB,
//     SessionMpstFive,
//     5,
//     3
// );
// create_recv_mpst_session!(
//     recv_mpst_c_to_d,
//     RoleD,
//     next_d,
//     RoleC,
//     SessionMpstFive,
//     5,
//     3
// );
// create_recv_mpst_session!(
//     recv_mpst_e_to_d,
//     RoleD,
//     next_d,
//     RoleE,
//     SessionMpstFive,
//     5,
//     4
// );
// // E
// create_recv_mpst_session!(
//     recv_mpst_a_to_e,
//     RoleE,
//     next_e,
//     RoleA,
//     SessionMpstFive,
//     5,
//     4
// );
// create_recv_mpst_session!(
//     recv_mpst_b_to_e,
//     RoleE,
//     next_e,
//     RoleB,
//     SessionMpstFive,
//     5,
//     4
// );
// create_recv_mpst_session!(
//     recv_mpst_c_to_e,
//     RoleE,
//     next_e,
//     RoleC,
//     SessionMpstFive,
//     5,
//     4
// );
// create_recv_mpst_session!(
//     recv_mpst_d_to_e,
//     RoleE,
//     next_e,
//     RoleD,
//     SessionMpstFive,
//     5,
//     4
// );

// // broadcast
// // A
// create_recv_mpst_all_session!(
//     recv_mpst_a_all_to_b,
//     RoleAlltoB,
//     next_all_to_b,
//     RoleA,
//     SessionMpstFive,
//     5,
//     1
// );
// create_recv_mpst_all_session!(
//     recv_mpst_a_all_to_c,
//     RoleAlltoC,
//     next_all_to_c,
//     RoleA,
//     SessionMpstFive,
//     5,
//     2
// );
// create_recv_mpst_all_session!(
//     recv_mpst_a_all_to_d,
//     RoleAlltoD,
//     next_all_to_d,
//     RoleA,
//     SessionMpstFive,
//     5,
//     3
// );
// create_recv_mpst_all_session!(
//     recv_mpst_a_all_to_e,
//     RoleAlltoE,
//     next_all_to_e,
//     RoleA,
//     SessionMpstFive,
//     5,
//     4
// );
// // B
// create_recv_mpst_all_session!(
//     recv_mpst_b_all_to_a,
//     RoleAlltoA,
//     next_all_to_a,
//     RoleB,
//     SessionMpstFive,
//     5,
//     1
// );
// create_recv_mpst_all_session!(
//     recv_mpst_b_all_to_c,
//     RoleAlltoC,
//     next_all_to_c,
//     RoleB,
//     SessionMpstFive,
//     5,
//     2
// );
// create_recv_mpst_all_session!(
//     recv_mpst_b_all_to_d,
//     RoleAlltoD,
//     next_all_to_d,
//     RoleB,
//     SessionMpstFive,
//     5,
//     3
// );
// create_recv_mpst_all_session!(
//     recv_mpst_b_all_to_e,
//     RoleAlltoE,
//     next_all_to_e,
//     RoleB,
//     SessionMpstFive,
//     5,
//     4
// );
// // C
// create_recv_mpst_all_session!(
//     recv_mpst_c_all_to_a,
//     RoleAlltoA,
//     next_all_to_a,
//     RoleC,
//     SessionMpstFive,
//     5,
//     1
// );
// create_recv_mpst_all_session!(
//     recv_mpst_c_all_to_b,
//     RoleAlltoB,
//     next_all_to_b,
//     RoleC,
//     SessionMpstFive,
//     5,
//     2
// );
// create_recv_mpst_all_session!(
//     recv_mpst_c_all_to_d,
//     RoleAlltoD,
//     next_all_to_d,
//     RoleC,
//     SessionMpstFive,
//     5,
//     3
// );
// create_recv_mpst_all_session!(
//     recv_mpst_c_all_to_e,
//     RoleAlltoE,
//     next_all_to_e,
//     RoleC,
//     SessionMpstFive,
//     5,
//     4
// );
// // D
// create_recv_mpst_all_session!(
//     recv_mpst_d_all_to_a,
//     RoleAlltoA,
//     next_all_to_a,
//     RoleD,
//     SessionMpstFive,
//     5,
//     1
// );
// create_recv_mpst_all_session!(
//     recv_mpst_d_all_to_b,
//     RoleAlltoB,
//     next_all_to_b,
//     RoleD,
//     SessionMpstFive,
//     5,
//     2
// );
// create_recv_mpst_all_session!(
//     recv_mpst_d_all_to_c,
//     RoleAlltoC,
//     next_all_to_c,
//     RoleD,
//     SessionMpstFive,
//     5,
//     3
// );
// create_recv_mpst_all_session!(
//     recv_mpst_d_all_to_e,
//     RoleAlltoE,
//     next_all_to_e,
//     RoleD,
//     SessionMpstFive,
//     5,
//     4
// );
// // E
// create_recv_mpst_all_session!(
//     recv_mpst_e_all_to_a,
//     RoleAlltoA,
//     next_all_to_a,
//     RoleE,
//     SessionMpstFive,
//     5,
//     1
// );
// create_recv_mpst_all_session!(
//     recv_mpst_e_all_to_b,
//     RoleAlltoB,
//     next_all_to_b,
//     RoleE,
//     SessionMpstFive,
//     5,
//     2
// );
// create_recv_mpst_all_session!(
//     recv_mpst_e_all_to_c,
//     RoleAlltoC,
//     next_all_to_c,
//     RoleE,
//     SessionMpstFive,
//     5,
//     3
// );
// create_recv_mpst_all_session!(
//     recv_mpst_e_all_to_d,
//     RoleAlltoD,
//     next_all_to_d,
//     RoleE,
//     SessionMpstFive,
//     5,
//     4
// );

// // Create Offer and Choose types
// create_offer_type_multi!(OfferMpstMultiFive,
// SessionMpstFive, 5); create_choose_type_multi!
// (ChooseMpstFive, SessionMpstFive, 5);

// // Create offer functions
// // A
// create_offer_mpst_session_multi!(
//     offer_mpst_session_a_to_b,
//     OfferMpstMultiFive,
//     RoleAlltoB,
//     recv_mpst_a_all_to_b,
//     RoleA,
//     SessionMpstFive,
//     5,
//     1
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_a_to_c,
//     OfferMpstMultiFive,
//     RoleAlltoC,
//     recv_mpst_a_all_to_c,
//     RoleA,
//     SessionMpstFive,
//     5,
//     2
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_a_to_d,
//     OfferMpstMultiFive,
//     RoleAlltoD,
//     recv_mpst_a_all_to_d,
//     RoleA,
//     SessionMpstFive,
//     5,
//     3
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_a_to_e,
//     OfferMpstMultiFive,
//     RoleAlltoE,
//     recv_mpst_a_all_to_e,
//     RoleA,
//     SessionMpstFive,
//     5,
//     4
// );
// // B
// create_offer_mpst_session_multi!(
//     offer_mpst_session_b_to_a,
//     OfferMpstMultiFive,
//     RoleAlltoA,
//     recv_mpst_b_all_to_a,
//     RoleB,
//     SessionMpstFive,
//     5,
//     1
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_b_to_c,
//     OfferMpstMultiFive,
//     RoleAlltoC,
//     recv_mpst_b_all_to_c,
//     RoleB,
//     SessionMpstFive,
//     5,
//     2
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_b_to_d,
//     OfferMpstMultiFive,
//     RoleAlltoD,
//     recv_mpst_b_all_to_d,
//     RoleB,
//     SessionMpstFive,
//     5,
//     3
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_b_to_e,
//     OfferMpstMultiFive,
//     RoleAlltoE,
//     recv_mpst_b_all_to_e,
//     RoleB,
//     SessionMpstFive,
//     5,
//     4
// );
// // C
// create_offer_mpst_session_multi!(
//     offer_mpst_session_c_to_a,
//     OfferMpstMultiFive,
//     RoleAlltoA,
//     recv_mpst_c_all_to_a,
//     RoleC,
//     SessionMpstFive,
//     5,
//     1
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_c_to_b,
//     OfferMpstMultiFive,
//     RoleAlltoB,
//     recv_mpst_c_all_to_b,
//     RoleC,
//     SessionMpstFive,
//     5,
//     2
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_c_to_d,
//     OfferMpstMultiFive,
//     RoleAlltoD,
//     recv_mpst_c_all_to_d,
//     RoleC,
//     SessionMpstFive,
//     5,
//     3
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_c_to_e,
//     OfferMpstMultiFive,
//     RoleAlltoE,
//     recv_mpst_c_all_to_e,
//     RoleC,
//     SessionMpstFive,
//     5,
//     4
// );
// // D
// create_offer_mpst_session_multi!(
//     offer_mpst_session_d_to_a,
//     OfferMpstMultiFive,
//     RoleAlltoA,
//     recv_mpst_d_all_to_a,
//     RoleD,
//     SessionMpstFive,
//     5,
//     1
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_d_to_b,
//     OfferMpstMultiFive,
//     RoleAlltoB,
//     recv_mpst_d_all_to_b,
//     RoleD,
//     SessionMpstFive,
//     5,
//     2
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_d_to_c,
//     OfferMpstMultiFive,
//     RoleAlltoC,
//     recv_mpst_d_all_to_c,
//     RoleD,
//     SessionMpstFive,
//     5,
//     3
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_d_to_e,
//     OfferMpstMultiFive,
//     RoleAlltoE,
//     recv_mpst_d_all_to_e,
//     RoleD,
//     SessionMpstFive,
//     5,
//     4
// );
// // E
// create_offer_mpst_session_multi!(
//     offer_mpst_session_e_to_a,
//     OfferMpstMultiFive,
//     RoleAlltoA,
//     recv_mpst_e_all_to_a,
//     RoleE,
//     SessionMpstFive,
//     5,
//     1
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_e_to_b,
//     OfferMpstMultiFive,
//     RoleAlltoB,
//     recv_mpst_e_all_to_b,
//     RoleE,
//     SessionMpstFive,
//     5,
//     2
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_e_to_c,
//     OfferMpstMultiFive,
//     RoleAlltoC,
//     recv_mpst_e_all_to_c,
//     RoleE,
//     SessionMpstFive,
//     5,
//     3
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_e_to_d,
//     OfferMpstMultiFive,
//     RoleAlltoD,
//     recv_mpst_e_all_to_d,
//     RoleE,
//     SessionMpstFive,
//     5,
//     4
// );

// // Create choose functions
// // A
// create_choose_mpst_session_multi_both!(
//     choose_left_mpst_session_a_to_all,
//     choose_right_mpst_session_a_to_all,
//     ChooseMpstFive,
//     RoleAtoAll,
//     next_a_to_all,
//     RoleA,
//     SessionMpstFive,
//     5
// );
// // B
// create_choose_mpst_session_multi_both!(
//     choose_left_mpst_session_b_to_all,
//     choose_right_mpst_session_b_to_all,
//     ChooseMpstFive,
//     RoleBtoAll,
//     next_b_to_all,
//     RoleB,
//     SessionMpstFive,
//     5
// );
// // C
// create_choose_mpst_session_multi_both!(
//     choose_left_mpst_session_c_to_all,
//     choose_right_mpst_session_c_to_all,
//     ChooseMpstFive,
//     RoleCtoAll,
//     next_c_to_all,
//     RoleC,
//     SessionMpstFive,
//     5
// );
// // D
// create_choose_mpst_session_multi_both!(
//     choose_left_mpst_session_d_to_all,
//     choose_right_mpst_session_d_to_all,
//     ChooseMpstFive,
//     RoleDtoAll,
//     next_d_to_all,
//     RoleD,
//     SessionMpstFive,
//     5
// );
// // E
// create_choose_mpst_session_multi_both!(
//     choose_left_mpst_session_e_to_all,
//     choose_right_mpst_session_e_to_all,
//     ChooseMpstFive,
//     RoleEtoAll,
//     next_e_to_all,
//     RoleE,
//     SessionMpstFive,
//     5
// );

// // Create close function
// close_mpst!(close_mpst_multi, SessionMpstFive, 5);

// // Create fork function
// fork_mpst_multi!(fork_mpst,
// SessionMpstFive, 5);

// // Names
// type NameA = RoleA<RoleEnd>;
// type NameB = RoleB<RoleEnd>;
// type NameC = RoleC<RoleEnd>;
// type NameD = RoleD<RoleEnd>;
// type NameE = RoleE<RoleEnd>;

// // Queues
// type QueueAinE = RoleE<RoleEnd>;
// type QueueAinD = RoleD<RoleAlltoE<RoleE<RoleEnd>,
// RoleE<RoleEnd>>>; type QueueAinC =
// RoleC<RoleAlltoD<QueueAinD, QueueAinD>>; type QueueAinB =
// RoleB<RoleAlltoC<QueueAinC, QueueAinC>>; type QueueA =
// RoleB<RoleC<RoleD<RoleE<RoleAlltoB<QueueAinB,
// QueueAinB>>>>>;

// type QueueBinE = RoleE<RoleEnd>;
// type QueueBinD = RoleD<RoleAlltoE<QueueBinE, QueueBinE>>;
// type QueueBinC = RoleC<RoleAlltoD<QueueBinD, QueueBinD>>;
// type QueueBinA =
// RoleA<RoleC<RoleD<RoleE<RoleAlltoC<QueueBinC,
// QueueBinC>>>>>; type QueueB = RoleA<RoleAlltoA<QueueBinA,
// QueueBinA>>;

// type QueueCinE = RoleE<RoleEnd>;
// type QueueCinD = RoleD<RoleAlltoE<QueueCinE, QueueCinE>>;
// type QueueCinB =
// RoleA<RoleB<RoleD<RoleE<RoleAlltoD<QueueCinD,
// QueueCinD>>>>>; type QueueCinA =
// RoleB<RoleAlltoB<QueueCinB, QueueCinB>>; type QueueC =
// RoleA<RoleAlltoA<QueueCinA, QueueCinA>>;

// type QueueDinE = RoleE<RoleEnd>;
// type QueueDinC =
// RoleA<RoleB<RoleC<RoleE<RoleAlltoE<QueueDinE,
// QueueDinE>>>>>; type QueueDinB =
// RoleC<RoleAlltoC<QueueDinC, QueueDinC>>; type QueueDinA =
// RoleB<RoleAlltoB<QueueDinB, QueueDinB>>; type QueueD =
// RoleA<RoleAlltoA<QueueDinA, QueueDinA>>;

// type QueueEinD = RoleA<RoleB<RoleC<RoleD<RoleEnd>>>>;
// type QueueEinC = RoleD<RoleAlltoD<QueueEinD, QueueEinD>>;
// type QueueEinB = RoleC<RoleAlltoC<QueueEinC, QueueEinC>>;
// type QueueEinA = RoleB<RoleAlltoB<QueueEinB, QueueEinB>>;
// type QueueE = RoleA<RoleAlltoA<QueueEinA, QueueEinA>>;

// // Types
// // Binary
// // A
// type AtoB = Send<(), End>;
// type AtoC = Send<(), End>;
// type AtoD = Send<(), End>;
// type AtoE = Send<(), End>;
// // B
// type BtoA = Recv<(), End>;
// type BtoC = Send<(), End>;
// type BtoD = Send<(), End>;
// type BtoE = Send<(), End>;
// // C
// type CtoA = Recv<(), End>;
// type CtoB = Recv<(), End>;
// type CtoD = Send<(), End>;
// type CtoE = Send<(), End>;
// // D
// type DtoA = Recv<(), End>;
// type DtoB = Recv<(), End>;
// type DtoC = Recv<(), End>;
// type DtoE = Send<(), End>;
// // E
// type EtoA = Recv<(), End>;
// type EtoB = Recv<(), End>;
// type EtoC = Recv<(), End>;
// type EtoD = Recv<(), End>;

// // Choice
// // A
// type ChooseAtoB = ChooseMpstFive<
//     Send<(), ChooseBtoA>,
//     Send<(), ChooseBtoC>,
//     Send<(), ChooseBtoD>,
//     Send<(), ChooseBtoE>,
//     Send<(), ChooseBtoA>,
//     Send<(), ChooseBtoC>,
//     Send<(), ChooseBtoD>,
//     Send<(), ChooseBtoE>,
//     <QueueAinB as Role>::Dual,
//     <QueueAinB as Role>::Dual,
//     RoleADual<RoleEnd>,
// >;
// type ChooseAtoC = ChooseMpstFive<
//     End,
//     Recv<(), OfferBtoC>,
//     End,
//     End,
//     End,
//     Recv<(), OfferBtoC>,
//     End,
//     End,
//     <QueueAinC as Role>::Dual,
//     <QueueAinC as Role>::Dual,
//     RoleADual<RoleEnd>,
// >;
// type ChooseAtoD = ChooseMpstFive<
//     End,
//     End,
//     Recv<(), OfferBtoD>,
//     End,
//     End,
//     End,
//     Recv<(), OfferBtoD>,
//     End,
//     <QueueAinD as Role>::Dual,
//     <QueueAinD as Role>::Dual,
//     RoleADual<RoleEnd>,
// >;
// type ChooseAtoE = ChooseMpstFive<
//     End,
//     End,
//     End,
//     Recv<(), OfferBtoE>,
//     End,
//     End,
//     End,
//     Recv<(), OfferBtoE>,
//     <QueueAinE as Role>::Dual,
//     <QueueAinE as Role>::Dual,
//     RoleADual<RoleEnd>,
// >;
// // B
// type ChooseBtoA = ChooseMpstFive<
//     End,
//     Recv<(), OfferCtoA>,
//     End,
//     End,
//     End,
//     Recv<(), OfferCtoA>,
//     End,
//     End,
//     <QueueBinA as Role>::Dual,
//     <QueueBinA as Role>::Dual,
//     RoleBDual<RoleEnd>,
// >;
// type ChooseBtoC = ChooseMpstFive<
//     Send<(), ChooseCtoA>,
//     Send<(), ChooseCtoB>,
//     Send<(), ChooseCtoD>,
//     Send<(), ChooseCtoE>,
//     Send<(), ChooseCtoA>,
//     Send<(), ChooseCtoB>,
//     Send<(), ChooseCtoD>,
//     Send<(), ChooseCtoE>,
//     <QueueBinC as Role>::Dual,
//     <QueueBinC as Role>::Dual,
//     RoleBDual<RoleEnd>,
// >;
// type ChooseBtoD = ChooseMpstFive<
//     End,
//     End,
//     Recv<(), OfferCtoD>,
//     End,
//     End,
//     End,
//     Recv<(), OfferCtoD>,
//     End,
//     <QueueBinD as Role>::Dual,
//     <QueueBinD as Role>::Dual,
//     RoleBDual<RoleEnd>,
// >;
// type ChooseBtoE = ChooseMpstFive<
//     End,
//     End,
//     End,
//     Recv<(), OfferCtoE>,
//     End,
//     End,
//     End,
//     Recv<(), OfferCtoE>,
//     <QueueBinE as Role>::Dual,
//     <QueueBinE as Role>::Dual,
//     RoleBDual<RoleEnd>,
// >;
// // C
// type ChooseCtoA = ChooseMpstFive<
//     End,
//     End,
//     Recv<(), OfferDtoA>,
//     End,
//     End,
//     End,
//     Recv<(), OfferDtoA>,
//     End,
//     <QueueCinA as Role>::Dual,
//     <QueueCinA as Role>::Dual,
//     RoleCDual<RoleEnd>,
// >;
// type ChooseCtoB = ChooseMpstFive<
//     End,
//     End,
//     Recv<(), OfferDtoB>,
//     End,
//     End,
//     End,
//     Recv<(), OfferDtoB>,
//     End,
//     <QueueCinB as Role>::Dual,
//     <QueueCinB as Role>::Dual,
//     RoleCDual<RoleEnd>,
// >;
// type ChooseCtoD = ChooseMpstFive<
//     Send<(), ChooseDtoA>,
//     Send<(), ChooseDtoB>,
//     Send<(), ChooseDtoC>,
//     Send<(), ChooseDtoE>,
//     Send<(), ChooseDtoA>,
//     Send<(), ChooseDtoB>,
//     Send<(), ChooseDtoC>,
//     Send<(), ChooseDtoE>,
//     <QueueCinD as Role>::Dual,
//     <QueueCinD as Role>::Dual,
//     RoleCDual<RoleEnd>,
// >;
// type ChooseCtoE = ChooseMpstFive<
//     End,
//     End,
//     End,
//     Recv<(), OfferDtoE>,
//     End,
//     End,
//     End,
//     Recv<(), OfferDtoE>,
//     <QueueCinE as Role>::Dual,
//     <QueueCinE as Role>::Dual,
//     RoleCDual<RoleEnd>,
// >;
// // D
// type ChooseDtoA = ChooseMpstFive<
//     End,
//     End,
//     End,
//     Recv<(), OfferEtoA>,
//     End,
//     End,
//     End,
//     Recv<(), OfferEtoA>,
//     <QueueDinA as Role>::Dual,
//     <QueueDinA as Role>::Dual,
//     RoleDDual<RoleEnd>,
// >;
// type ChooseDtoB = ChooseMpstFive<
//     End,
//     End,
//     End,
//     Recv<(), OfferEtoB>,
//     End,
//     End,
//     End,
//     Recv<(), OfferEtoB>,
//     <QueueDinB as Role>::Dual,
//     <QueueDinB as Role>::Dual,
//     RoleDDual<RoleEnd>,
// >;
// type ChooseDtoC = ChooseMpstFive<
//     End,
//     End,
//     End,
//     Recv<(), OfferEtoC>,
//     End,
//     End,
//     End,
//     Recv<(), OfferEtoC>,
//     <QueueDinC as Role>::Dual,
//     <QueueDinC as Role>::Dual,
//     RoleDDual<RoleEnd>,
// >;
// type ChooseDtoE = ChooseMpstFive<
//     Send<(), End>,
//     Send<(), End>,
//     Send<(), End>,
//     Send<(), End>,
//     Send<(), End>,
//     Send<(), End>,
//     Send<(), End>,
//     Send<(), End>,
//     <QueueDinE as Role>::Dual,
//     <QueueDinE as Role>::Dual,
//     RoleDDual<RoleEnd>,
// >;
// // Offer
// // A
// type OfferCtoA = OfferMpstMultiFive<
//     End,
//     Recv<(), ChooseCtoA>,
//     End,
//     End,
//     End,
//     Recv<(), ChooseCtoA>,
//     End,
//     End,
//     QueueBinA,
//     QueueBinA,
//     NameA,
// >;
// type OfferDtoA = OfferMpstMultiFive<
//     End,
//     End,
//     Recv<(), ChooseDtoA>,
//     End,
//     End,
//     End,
//     Recv<(), ChooseDtoA>,
//     End,
//     QueueCinA,
//     QueueCinA,
//     NameA,
// >;
// type OfferEtoA = OfferMpstMultiFive<
//     End,
//     End,
//     End,
//     Recv<(), End>,
//     End,
//     End,
//     End,
//     Recv<(), End>,
//     QueueDinA,
//     QueueDinA,
//     NameA,
// >;
// // B
// type OfferCtoB = OfferMpstMultiFive<
//     End,
//     Recv<(), ChooseCtoB>,
//     End,
//     End,
//     End,
//     Recv<(), ChooseCtoB>,
//     End,
//     End,
//     QueueBinA,
//     QueueBinA,
//     NameA,
// >;

// // Creating the MP sessions
