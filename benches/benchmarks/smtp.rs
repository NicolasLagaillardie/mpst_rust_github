// Unfinished, still in process of adding send_tcp and
// recv_tcp

// #![allow(dead_code)]

// use criterion::{black_box, criterion_group,
// criterion_main, Criterion};

// use mpstthree::binary::struct_trait::{End, Recv, Send};
// use mpstthree::role::end::RoleEnd;
// use mpstthree::role::Role;
// use mpstthree::{
//     fork_mpst_multi, choose_mpst_multi_to_all,
// close_mpst, create_normal_role,
// create_recv_mpst_session_bundle,
//     create_send_mpst_session_bundle, create_sessionmpst,
// offer_mpst, };

// use rand::{random, thread_rng, Rng};
// use std::error::Error;
// use std::marker;
// use std::time::Duration;

// // global protocol Smtp(role S, role C)
// // {
// //     220() from S to C;
// //     choice at C // Choice 0
// //     {
// //         Ehlo1() from C to S;
// //         rec X
// //         {
// //             choice at S // Choice 1
// //             {
// //                 250d() from S to C;
// //                 continue X;
// //             }
// //             or
// //             {
// //                 250() from S to C;
// //                 choice at C // Choice 2
// //                 {
// //                     StartTls() from C to S;
// //                     220() from S to C;
// //                     // Do TLS handshake here: level
// below the application level protocol (like regular TCP
// handshake) //                     choice at C // Choice 3
// //                     { //
// Ehlo2() from C to S; //                         rec X
// //                         {
// //                             choice at S // Choice 4
// //                             {
// //                                 250d1() from S to C;
// //                                 continue X;
// //                             }
// //                             or
// //                             {
// //                                 2501() from S to C;
// //                                 rec Y
// //                                 {
// //                                     choice at C //
// Choice 5 //                                     {
// //                                         Auth() from C
// to S; //                                         choice
// at S // Choice 6 //
// { //                                             235()
// from S to C; //
// rec Z1 //                                             {
// //                                                 choice
// at C // Choice 7 //
// { //
// Mail() from C to S; //Mail from:<a@b.com> //
// choice at S // Choice 8 //
// { //
// 501() from S to C; //
// continue Z1; //
// } //
// or //
// { //
// 2502() from S to C;

// //
// rec Z2 //
// { //
// choice at C // Choice 9 //
// { //
// Rcpt() from C to S; //Rcpt to:<c@d.com> //
// choice at S // What is this choice???  // Choice 10 //
// { //
// 2503() from S to C; //
// continue Z2; //
// } //
// } //
// or //
// { //
// Data() from C to S; //
// 354() from S to C; //
// //too from C to S; //to:<you> //
// //froom from C to S; //from:<me> //
// rec Z3 //
// { //
// choice at C // Choice 11 //
// { //
// DataLine() from C to S; //
// DataLine() from C to S; //
// continue Z3; //
// } //
// or //
// { //
// Subject() from C to S; //Subject:<my Subject> //
// Subject() from C to S; //Subject:<my Subject> //
// continue Z3; //
// } //
// or //
// { //
// EndOfData() from C to S; // CRLF.CRLF //
// 2504() from S to C; //
// continue Z1; //
// } //
// } //
// } //
// } //
// } //                                                 }
// //                                                 or
// //                                                 {
// //
// Quit5() from C to S; //
// 221() from S to C; //
// } //                                             }
// //                                         }
// //                                         or
// //                                         {
// //                                             535() from
// S to C; //
// continue Y; //                                         }
// //                                         //.. 501
// Invalid base64 Data //
// } //                                     or
// //                                     {
// //                                         Quit4() from C
// to S; //                                     }
// //                                 }
// //                             }
// //                         }
// //                     }
// //                     or
// //                     {
// //                         Quit3() from C to S;
// //                     }
// //                 }
// //                 or
// //                 {
// //                     Quit2() from C to S;
// //                 }
// //             }
// //         }
// //     }
// //     or
// //     {
// //         Quit1() from C to S;
// //     }
// // }

// // Create new SessionMpst for three participants
// create_sessionmpst!(SessionMpstThree, 3);

// // Create new roles
// // normal
// create_normal_role!(RoleA, next_a, RoleADual,
// next_a_dual); create_normal_role!(RoleC, next_c,
// RoleCDual, next_c_dual); create_normal_role!(RoleS,
// next_s, RoleSDual, next_s_dual);

// // Create new send functions
// // A
// create_send_mpst_session_bundle!(
//     send_mpst_a_to_c,
//     RoleC,
//     next_c,
//     1 |
//     send_mpst_a_to_s,
//     RoleS,
//     next_s,
//     2 | =>
//     RoleA,
//     SessionMpstThree,
//     3
// );
// // C
// create_send_mpst_session_bundle!(
//     send_mpst_c_to_a,
//     RoleA,
//     next_a,
//     1 |
//     send_mpst_c_to_s,
//     RoleS,
//     next_s,
//     2 | =>
//     RoleC,
//     SessionMpstThree,
//     3
// );
// // S
// create_send_mpst_session_bundle!(
//     send_mpst_s_to_c,
//     RoleC,
//     next_c,
//     2 | =>
//     RoleS,
//     SessionMpstThree,
//     3
// );

// // Create new recv functions and related types
// // A
// create_recv_mpst_session_bundle!(
//     recv_mpst_a_from_c,
//     RoleC,
//     next_c,
//     1 | =>
//     RoleA,
//     SessionMpstThree,
//     3
// );
// // C
// create_recv_mpst_session_bundle!(
//     recv_mpst_c_from_a,
//     RoleA,
//     next_a,
//     1 |
//     recv_mpst_c_from_s,
//     RoleS,
//     next_s,
//     2 | =>
//     RoleC,
//     SessionMpstThree,
//     3
// );
// // S
// create_recv_mpst_session_bundle!(
//     recv_mpst_s_from_a,
//     RoleA,
//     next_a,
//     1 |
//     recv_mpst_s_from_c,
//     RoleC,
//     next_c,
//     2 | =>
//     RoleS,
//     SessionMpstThree,
//     3
// );

// // Create close function
// close_mpst!(close_mpst_multi, SessionMpstThree, 3);

// // Create fork function
// fork_mpst_multi!(fork_mpst,
// SessionMpstThree, 3);

// // Names
// type NameA = RoleA<RoleEnd>;
// type NameC = RoleC<RoleEnd>;
// type NameS = RoleS<RoleEnd>;

// // Types
// // C
// type Choose0fromCtoA = Send<Branching0fromCtoA, End>;
// type Choose0fromCtoS<N> = Send<Branching0fromCtoS<N>,
// End>;

// enum Branching1fromStoC<N: marker::Send> {
//     Continue(
//         SessionMpstThree<
//             Choose2fromCtoA,
//             Recv<N, Choose2fromCtoS<N>>,
//             RoleS<RoleA<RoleS<RoleEnd>>>,
//             NameC,
//         >,
//     ),
//     Quit(SessionMpstThree<End, Recv<N,
// Choice1fromStoC<N>>, RoleS<RoleS<RoleEnd>>, NameC>), }
// type Choice1fromStoC<N> = Recv<Branching1fromStoC<N>,
// End>;

// type Choose2fromCtoA = Send<Branching2fromCtoA, End>;
// type Choose2fromCtoS<N> = Send<Branching2fromCtoS<N>,
// End>;

// type Choose3fromCtoA = Send<Branching3fromCtoA, End>;
// type Choose3fromCtoS<N> = Send<Branching3fromCtoS<N>,
// End>;

// enum Branching4fromStoC<N: marker::Send> {
//     Continue(
//         SessionMpstThree<
//             Choose5fromCtoA,
//             Recv<N, Choose5fromCtoS<N>>,
//             RoleS<RoleA<RoleS<RoleEnd>>>,
//             NameC,
//         >,
//     ),
//     Loop(SessionMpstThree<End, Recv<N,
// Choice4fromStoC<N>>, RoleS<RoleS<RoleEnd>>, NameC>), }
// type Choice4fromStoC<N> = Recv<Branching4fromStoC<N>,
// End>;

// type Choose5fromCtoA = Send<Branching5fromCtoA, End>;
// type Choose5fromCtoS<N> = Send<Branching5fromCtoS<N>,
// End>;

// enum Branching6fromStoC<N: marker::Send> {
//     Continue(
//         SessionMpstThree<
//             Choose7fromCtoA,
//             Recv<N, Choose7fromCtoS<N>>,
//             RoleS<RoleA<RoleS<RoleEnd>>>,
//             NameC,
//         >,
//     ),
//     Loop(
//         SessionMpstThree<
//             Choose5fromCtoA,
//             Recv<N, Choose5fromCtoS<N>>,
//             RoleS<RoleA<RoleS<RoleEnd>>>,
//             NameC,
//         >,
//     ),
// }
// type Choice6fromStoC<N> = Recv<Branching6fromStoC<N>,
// End>;

// type Choose7fromCtoA = Send<Branching7fromCtoA, End>;
// type Choose7fromCtoS<N> = Send<Branching7fromCtoS<N>,
// End>;

// enum Branching8fromStoC<N: marker::Send> {
//     Continue(
//         SessionMpstThree<
//             Choose9fromCtoA,
//             Recv<N, Choose9fromCtoS<N>>,
//             RoleS<RoleA<RoleS<RoleEnd>>>,
//             NameC,
//         >,
//     ),
//     Loop(
//         SessionMpstThree<
//             Choose7fromCtoA,
//             Recv<N, Choose7fromCtoS<N>>,
//             RoleS<RoleA<RoleS<RoleEnd>>>,
//             NameC,
//         >,
//     ),
// }
// type Choice8fromStoC<N> = Recv<Branching8fromStoC<N>,
// End>;

// type Choose9fromCtoA = Send<Branching9fromCtoA, End>;
// type Choose9fromCtoS<N> = Send<Branching9fromCtoS<N>,
// End>;

// enum Branching10fromStoC {
//     Continue(SessionMpstThree<End, End, RoleEnd, NameC>),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameC>),
// }
// type Choice10fromStoC = Recv<Branching10fromStoC, End>;

// type Choose11fromCtoA = Send<Branching11fromCtoA, End>;
// type Choose11fromCtoS = Send<Branching11fromCtoS, End>;

// // A
// enum Branching0fromCtoA {
//     Continue(SessionMpstThree<End, Choice1fromStoA,
// RoleS<RoleEnd>, NameA>),     Quit(SessionMpstThree<End,
// End, RoleEnd, NameA>), }
// type Choice0fromCtoA = Recv<Branching0fromCtoA, End>;

// enum Branching1fromStoA {
//     Continue(SessionMpstThree<Choice2fromCtoA, End,
// RoleC<RoleEnd>, NameA>),     Loop(SessionMpstThree<End,
// Choice1fromStoA, RoleS<RoleEnd>, NameA>), }
// type Choice1fromStoA = Recv<Branching1fromStoA, End>;

// enum Branching2fromCtoA {
//     Continue(SessionMpstThree<Choice3fromCtoA, End,
// RoleC<RoleEnd>, NameA>),     Quit(SessionMpstThree<End,
// End, RoleEnd, NameA>), }
// type Choice2fromCtoA = Recv<Branching2fromCtoA, End>;

// enum Branching3fromCtoA {
//     Continue(SessionMpstThree<End, Choice4fromStoA,
// RoleS<RoleEnd>, NameA>),     Quit(SessionMpstThree<End,
// End, RoleEnd, NameA>), }
// type Choice3fromCtoA = Recv<Branching3fromCtoA, End>;

// enum Branching4fromStoA {
//     Continue(SessionMpstThree<Choice5fromCtoA, End,
// RoleC<RoleEnd>, NameA>),     Loop(SessionMpstThree<End,
// Choice4fromStoA, RoleS<RoleEnd>, NameA>), }
// type Choice4fromStoA = Recv<Branching4fromStoA, End>;

// enum Branching5fromCtoA {
//     Continue(SessionMpstThree<End, Choice6fromStoA,
// RoleS<RoleEnd>, NameA>),     Quit(SessionMpstThree<End,
// End, RoleEnd, NameA>), }
// type Choice5fromCtoA = Recv<Branching5fromCtoA, End>;

// enum Branching6fromStoA {
//     Continue(SessionMpstThree<Choice7fromCtoA, End,
// RoleC<RoleEnd>, NameA>),
//     Loop(SessionMpstThree<Choice5fromCtoA, End,
// RoleC<RoleEnd>, NameA>), }
// type Choice6fromStoA = Recv<Branching6fromStoA, End>;

// enum Branching7fromCtoA {
//     Continue(SessionMpstThree<End, Choice8fromStoA,
// RoleS<RoleEnd>, NameA>),     Quit(SessionMpstThree<End,
// End, RoleEnd, NameA>), }
// type Choice7fromCtoA = Recv<Branching7fromCtoA, End>;

// enum Branching8fromStoA {
//     Continue(SessionMpstThree<Choice9fromCtoA, End,
// RoleC<RoleEnd>, NameA>),
//     Loop(SessionMpstThree<Choice7fromCtoA, End,
// RoleC<RoleEnd>, NameA>), }
// type Choice8fromStoA = Recv<Branching8fromStoA, End>;

// enum Branching9fromCtoA {
//     Continue(SessionMpstThree<End, Choice10fromStoA,
// RoleS<RoleEnd>, NameA>),     Loop(SessionMpstThree<End,
// End, RoleEnd, NameA>), }
// type Choice9fromCtoA = Recv<Branching9fromCtoA, End>;

// enum Branching10fromStoA {
//     Continue(SessionMpstThree<End, End, RoleEnd, NameA>),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameA>),
// }
// type Choice10fromStoA = Recv<Branching10fromStoA, End>;

// enum Branching11fromCtoA {
//     Data(SessionMpstThree<End, End, RoleEnd, NameA>),
//     Subject(SessionMpstThree<End, End, RoleEnd, NameA>),
//     End(SessionMpstThree<End, End, RoleEnd, NameA>),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameA>),
// }
// type Choice11fromCtoA = Recv<Branching11fromCtoA, End>;

// // S
// enum Branching0fromCtoS<N: marker::Send> {
//     Continue(SessionMpstThree<Choose1fromStoA,
// Choose1fromStoC<N>, RoleA<RoleC<RoleEnd>>, NameS>),
// Quit(SessionMpstThree<End, End, RoleEnd, NameS>), }
// type Choice0fromCtoS<N> = Recv<Branching0fromCtoS<N>,
// End>;

// type Choose1fromStoA = Send<Branching1fromStoA, End>;
// type Choose1fromStoC<N> = Send<Branching1fromStoC<N>,
// End>;

// enum Branching2fromCtoS<N: marker::Send> {
//     Continue(
//         SessionMpstThree<
//             Recv<N, Send<N, Choice3fromCtoS<N>>>,
//             End,
//             RoleC<RoleC<RoleC<RoleEnd>>>,
//             NameS,
//         >,
//     ),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameS>),
// }
// type Choice2fromCtoS<N> = Recv<Branching2fromCtoS<N>,
// End>;

// enum Branching3fromCtoS<N: marker::Send> {
//     Continue(
//         SessionMpstThree<
//             Choose4fromStoA,
//             Recv<N, Choose4fromStoC<N>>,
//             RoleC<RoleA<RoleC<RoleEnd>>>,
//             NameS,
//         >,
//     ),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameS>),
// }
// type Choice3fromCtoS<N> = Recv<Branching3fromCtoS<N>,
// End>;

// type Choose4fromStoA = Send<Branching4fromStoA, End>;
// type Choose4fromStoC<N> = Send<Branching4fromStoC<N>,
// End>;

// enum Branching5fromCtoS<N: marker::Send> {
//     Continue(
//         SessionMpstThree<
//             Choose6fromStoA,
//             Recv<N, Choose6fromStoC<N>>,
//             RoleC<RoleA<RoleC<RoleEnd>>>,
//             NameS,
//         >,
//     ),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameS>),
// }
// type Choice5fromCtoS<N> = Recv<Branching5fromCtoS<N>,
// End>;

// type Choose6fromStoA = Send<Branching6fromStoA, End>;
// type Choose6fromStoC<N> = Send<Branching6fromStoC<N>,
// End>;

// enum Branching7fromCtoS<N: marker::Send> {
//     Continue(
//         SessionMpstThree<
//             Choose8fromStoA,
//             Recv<N, Choose8fromStoC<N>>,
//             RoleC<RoleA<RoleC<RoleEnd>>>,
//             NameS,
//         >,
//     ),
//     Quit(SessionMpstThree<End, Recv<N, End>,
// RoleS<RoleEnd>, NameS>), }
// type Choice7fromCtoS<N> = Recv<Branching7fromCtoS<N>,
// End>;

// type Choose8fromStoA = Send<Branching8fromStoA, End>;
// type Choose8fromStoC<N> = Send<Branching8fromStoC<N>,
// End>;

// enum Branching9fromCtoS<N: marker::Send> {
//     Loop(
//         SessionMpstThree<Choose10fromStoA, Recv<N,
// Choose10fromStoC>, RoleC<RoleC<RoleEnd>>, NameS>,     ),
//     Continue(
//         SessionMpstThree<
//             End,
//             Recv<N, Send<N, Choice11fromCtoS>>,
//             RoleC<RoleC<RoleC<RoleEnd>>>,
//             NameS,
//         >,
//     ),
// }
// type Choice9fromCtoS<N> = Recv<Branching9fromCtoS<N>,
// End>;

// type Choose10fromStoA = Send<Branching10fromStoA, End>;
// type Choose10fromStoC = Send<Branching10fromStoC, End>;

// enum Branching11fromCtoS {
//     Data(SessionMpstThree<End, End, RoleEnd, NameS>),
//     Subject(SessionMpstThree<End, End, RoleEnd, NameS>),
//     End(SessionMpstThree<End, End, RoleEnd, NameS>),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameS>),
// }
// type Choice11fromCtoS = Recv<Branching11fromCtoS, End>;

// // Creating the MP sessions

// // A
// type EndpointA0 = SessionMpstThree<Choice0fromCtoA, End,
// RoleC<RoleEnd>, NameA>; type EndpointA1 =
// SessionMpstThree<End, Choice1fromStoA, RoleS<RoleEnd>,
// NameA>; type EndpointA2 =
// SessionMpstThree<Choice2fromCtoA, End, RoleC<RoleEnd>,
// NameA>; type EndpointA3 =
// SessionMpstThree<Choice3fromCtoA, End, RoleC<RoleEnd>,
// NameA>; type EndpointA4 = SessionMpstThree<End,
// Choice4fromStoA, RoleS<RoleEnd>, NameA>; type EndpointA5
// = SessionMpstThree<Choice5fromCtoA, End, RoleC<RoleEnd>,
// NameA>; type EndpointA6 = SessionMpstThree<End,
// Choice6fromStoA, RoleS<RoleEnd>, NameA>; type EndpointA7
// = SessionMpstThree<Choice7fromCtoA, End, RoleC<RoleEnd>,
// NameA>; type EndpointA8 = SessionMpstThree<End,
// Choice8fromStoA, RoleS<RoleEnd>, NameA>; type EndpointA9
// = SessionMpstThree<Choice9fromCtoA, End, RoleC<RoleEnd>,
// NameA>; type EndpointA10 = SessionMpstThree<End,
// Choice10fromStoA, RoleS<RoleEnd>, NameA>; type
// EndpointA11 = SessionMpstThree<Choice11fromCtoA, End,
// RoleS<RoleEnd>, NameA>;

// // C
// type EndpointC0<N> = SessionMpstThree<
//     Choose0fromCtoA,
//     Recv<N, Choose0fromCtoS<N>>,
//     RoleS<RoleA<RoleS<RoleEnd>>>,
//     NameC,
// >;
// type EndpointC1<N> =
//     SessionMpstThree<End, Send<N, Choice1fromStoC<N>>,
// RoleS<RoleS<RoleEnd>>, NameC>; type EndpointC2<N> =
//     SessionMpstThree<Choose2fromCtoA, Choose2fromCtoS<N>,
// RoleA<RoleS<RoleEnd>>, NameC>; type EndpointC3<N> =
//     SessionMpstThree<Choose3fromCtoA, Choose3fromCtoS<N>,
// RoleA<RoleS<RoleEnd>>, NameC>; type EndpointC4<N> =
// SessionMpstThree<End, Choice4fromStoC<N>, RoleS<RoleEnd>,
// NameC>; type EndpointC5<N> =
//     SessionMpstThree<Choose5fromCtoA, Choose5fromCtoS<N>,
// RoleA<RoleS<RoleEnd>>, NameC>; type EndpointC6<N> =
// SessionMpstThree<End, Choice6fromStoC<N>, RoleS<RoleEnd>,
// NameC>; type EndpointC7<N> =
//     SessionMpstThree<Choose7fromCtoA, Choose7fromCtoS<N>,
// RoleA<RoleS<RoleEnd>>, NameC>; type EndpointC8<N> =
// SessionMpstThree<End, Choice8fromStoC<N>, RoleS<RoleEnd>,
// NameC>; type EndpointC9<N> =
//     SessionMpstThree<Choose9fromCtoA, Choose9fromCtoS<N>,
// RoleA<RoleS<RoleEnd>>, NameC>; type EndpointC10 =
// SessionMpstThree<End, Choice10fromStoC, RoleS<RoleEnd>,
// NameC>; type EndpointC11 =
//     SessionMpstThree<Choose11fromCtoA, Choose11fromCtoS,
// RoleA<RoleS<RoleEnd>>, NameC>;

// // S
// type EndpointS0<N> =
//     SessionMpstThree<End, Send<N, Choice0fromCtoS<N>>,
// RoleC<RoleC<RoleEnd>>, NameS>; type EndpointS1<N> =
// SessionMpstThree<     Choose1fromStoA,
//     Recv<N, Choose1fromStoC<N>>,
//     RoleC<RoleA<RoleC<RoleEnd>>>,
//     NameS,
// >;
// type EndpointS2<N> = SessionMpstThree<End,
// Choice2fromCtoS<N>, RoleC<RoleEnd>, NameS>; type
// EndpointS3<N> = SessionMpstThree<End, Choice3fromCtoS<N>,
// RoleC<RoleEnd>, NameS>; type EndpointS4<N> =
//     SessionMpstThree<Choose4fromStoA, Choose4fromStoC<N>,
// RoleA<RoleC<RoleEnd>>, NameS>; type EndpointS5<N> =
// SessionMpstThree<End, Choice5fromCtoS<N>, RoleC<RoleEnd>,
// NameS>; type EndpointS6<N> =
//     SessionMpstThree<Choose6fromStoA, Choose6fromStoC<N>,
// RoleA<RoleC<RoleEnd>>, NameS>; type EndpointS7<N> =
// SessionMpstThree<End, Choice7fromCtoS<N>, RoleC<RoleEnd>,
// NameS>; type EndpointS8<N> =
//     SessionMpstThree<Choose8fromStoA, Choose8fromStoC<N>,
// RoleA<RoleC<RoleEnd>>, NameS>; type EndpointS9<N> =
// SessionMpstThree<End, Choice9fromCtoS<N>, RoleC<RoleEnd>,
// NameS>; type EndpointS10 =
//     SessionMpstThree<Choose10fromStoA, Choose10fromStoC,
// RoleA<RoleC<RoleEnd>>, NameS>; type EndpointS11 =
// SessionMpstThree<End, Choice11fromCtoS, RoleC<RoleEnd>,
// NameS>;

// // None
// type EndpointNoneA = SessionMpstThree<End, End, RoleEnd,
// NameA>; type EndpointNoneC = SessionMpstThree<End, End,
// RoleEnd, NameC>; type EndpointNoneS =
// SessionMpstThree<End, End, RoleEnd, NameS>;

// // Functions
// // A
// fn simple_three_endpoint_a(s: EndpointNoneA) -> Result<(),
// Box<dyn Error>> {     close_mpst_multi(s)
// }
// // C
// fn simple_three_endpoint_c(s: EndpointNoneC) -> Result<(),
// Box<dyn Error>> {     close_mpst_multi(s)
// }
// // S
// fn simple_three_endpoint_s(s: EndpointNoneS) -> Result<(),
// Box<dyn Error>> {     close_mpst_multi(s)
// }

// fn all_mpst() -> Result<(), Box<dyn Error>> {
//     let (thread_a, thread_c, thread_s) = fork_mpst(
//         black_box(simple_three_endpoint_a),
//         black_box(simple_three_endpoint_c),
//         black_box(simple_three_endpoint_s),
//     );

//     thread_a.join().unwrap();
//     thread_c.join().unwrap();
//     thread_s.join().unwrap();

//     Ok(())
// }

// /////////////////////////

// fn smtp_mpst(c: &mut Criterion) {
//     c.bench_function(&format!("Smtp MPST"), |b| b.iter(||
// all_mpst())); }

// fn long_warmup() -> Criterion {
//     Criterion::default().measurement_time(Duration::
// new(30, 0)) }

// criterion_group! {
//     name = smtp;
//     config = long_warmup();
//     targets = smtp_mpst
// }

// criterion_main!(smtp);
