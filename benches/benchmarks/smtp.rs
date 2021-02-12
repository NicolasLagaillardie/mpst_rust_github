// #![allow(dead_code)]

// use criterion::{black_box, criterion_group, criterion_main, Criterion};

// use mpstthree::binary::{End, Recv, Send};
// use mpstthree::role::end::RoleEnd;
// use mpstthree::role::Role;
// use mpstthree::{
//     bundle_fork_multi, choose_mpst_multi_to_all, close_mpst, create_normal_role,
//     create_recv_mpst_session, create_recv_mpst_session_bundle, create_send_mpst_session,
//     create_send_mpst_session_bundle, create_sessionmpst, offer_mpst,
// };

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
// //                     // Do TLS handshake here: level below the application level protocol (like regular TCP handshake)
// //                     choice at C // Choice 3
// //                     {
// //                         Ehlo2() from C to S;
// //                         rec X
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
// //                                     choice at C // Choice 5
// //                                     {
// //                                         Auth() from C to S;
// //                                         choice at S // Choice 6
// //                                         {
// //                                             235() from S to C;
// //                                             rec Z1
// //                                             {
// //                                                 choice at C // Choice 7
// //                                                 {
// //                                                     Mail() from C to S; //Mail from:<a@b.com>
// //                                                     choice at S // Choice 8
// //                                                     {
// //                                                         501() from S to C;
// //                                                         continue Z1;
// //                                                     }
// //                                                     or
// //                                                     {
// //                                                         2502() from S to C;
// //
// //                                                         rec Z2
// //                                                         {
// //                                                             choice at C // Choice 9
// //                                                             {
// //                                                                 Rcpt() from C to S; //Rcpt to:<c@d.com>
// //                                                                 choice at S // What is this choice???  // Choice 10
// //                                                                 {
// //                                                                     2503() from S to C;
// //                                                                     continue Z2;
// //                                                                 }
// //                                                             }
// //                                                             or
// //                                                             {
// //                                                                 Data() from C to S;
// //                                                                 354() from S to C;
// //                                                                 //too from C to S; //to:<you>
// //                                                                 //froom from C to S; //from:<me>
// //                                                                 rec Z3
// //                                                                 {
// //                                                                     choice at C // Choice 11
// //                                                                     {
// //                                                                         DataLine() from C to S;
// //                                                                         DataLine() from C to S;
// //                                                                         continue Z3;
// //                                                                     }
// //                                                                     or
// //                                                                     {
// //                                                                         Subject() from C to S; //Subject:<my Subject>
// //                                                                         Subject() from C to S; //Subject:<my Subject>
// //                                                                         continue Z3;
// //                                                                     }
// //                                                                     or
// //                                                                     {
// //                                                                         EndOfData() from C to S; // CRLF.CRLF
// //                                                                         2504() from S to C;
// //                                                                         continue Z1;
// //                                                                     }
// //                                                                 }
// //                                                             }
// //                                                         }
// //                                                     }
// //                                                 }
// //                                                 or
// //                                                 {
// //                                                     Quit5() from C to S;
// //                                                     221() from S to C;
// //                                                 }
// //                                             }
// //                                         }
// //                                         or
// //                                         {
// //                                             535() from S to C;
// //                                             continue Y;
// //                                         }
// //                                         //.. 501 Invalid base64 Data
// //                                     }
// //                                     or
// //                                     {
// //                                         Quit4() from C to S;
// //                                     }
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
// create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
// create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
// create_normal_role!(RoleS, next_s, RoleSDual, next_s_dual);

// // Create new send functions
// // A
// create_send_mpst_session_bundle!(
//     send_mpst_a_to_c,
//     RoleC,
//     next_c,
//     1, |
//     send_mpst_a_to_s,
//     RoleS,
//     next_s,
//     2, | =>
//     RoleA,
//     SessionMpstThree,
//     3
// );
// // C
// create_send_mpst_session_bundle!(
//     send_mpst_c_to_a,
//     RoleA,
//     next_a,
//     1, |
//     send_mpst_c_to_s,
//     RoleS,
//     next_s,
//     2, | =>
//     RoleC,
//     SessionMpstThree,
//     3
// );
// // S
// create_send_mpst_session_bundle!(
//     send_mpst_s_to_c,
//     RoleC,
//     next_c,
//     2, | =>
//     RoleS,
//     SessionMpstThree,
//     3
// );

// // Create new recv functions and related types
// // A
// create_recv_mpst_session_bundle!(
//     recv_mpst_a_to_c,
//     RoleC,
//     next_c,
//     1, | =>
//     RoleA,
//     SessionMpstThree,
//     3
// );
// // C
// create_recv_mpst_session_bundle!(
//     recv_mpst_c_to_a,
//     RoleA,
//     next_a,
//     1, |
//     recv_mpst_c_to_s,
//     RoleS,
//     next_s,
//     2, | =>
//     RoleC,
//     SessionMpstThree,
//     3
// );
// // S
// create_recv_mpst_session_bundle!(
//     recv_mpst_s_to_a,
//     RoleA,
//     next_a,
//     1, |
//     recv_mpst_s_to_c,
//     RoleC,
//     next_c,
//     2, | =>
//     RoleS,
//     SessionMpstThree,
//     3
// );

// // Create close function
// close_mpst!(close_mpst_multi, SessionMpstThree, 3);

// // Create fork function
// bundle_fork_multi!(fork_mpst, fork_simple, SessionMpstThree, 3);

// // Names
// type NameA = RoleA<RoleEnd>;
// type NameC = RoleC<RoleEnd>;
// type NameS = RoleS<RoleEnd>;

// // Types
// // C0
// type Choose0fromCtoA = Send<Branching0fromCtoA, End>;
// type Choose0fromCtoS = Send<Branching0fromCtoS, End>;
// // C2
// type Choose2fromCtoA = Send<Branching2fromCtoA, End>;
// type Choose2fromCtoS = Send<Branching2fromCtoS, End>;
// // C3
// type Choose3fromCtoA = Send<Branching3fromCtoA, End>;
// type Choose3fromCtoS = Send<Branching3fromCtoS, End>;
// // C5
// type Choose5fromCtoA = Send<Branching5fromCtoA, End>;
// type Choose5fromCtoS = Send<Branching5fromCtoS, End>;
// // C7
// type Choose7fromCtoA = Send<Branching7fromCtoA, End>;
// type Choose7fromCtoS = Send<Branching7fromCtoS, End>;
// // C0
// type Choose9fromCtoA = Send<Branching9fromCtoA, End>;
// type Choose9fromCtoS = Send<Branching9fromCtoS, End>;

// // A
// enum Branching0fromCtoA {
//     Continue(SessionMpstThree<Choice1fromStoA, End, RoleC<RoleEnd>, NameA>),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameA>),
// }
// type Choice0fromCtoA = Recv<Branching0fromCtoA, End>;
// enum Branching1fromStoA {
//     Continue(SessionMpstThree<Choice2fromCtoA, End, RoleC<RoleEnd>, NameA>),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameA>),
// }
// type Choice1fromStoA = Recv<Branching1fromStoA, End>;
// enum Branching2fromCtoA {
//     Continue(SessionMpstThree<Choice3fromCtoA, End, RoleC<RoleEnd>, NameA>),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameA>),
// }
// type Choice2fromCtoA = Recv<Branching2fromCtoA, End>;
// enum Branching3fromCtoA {
//     Continue(SessionMpstThree<Choice4fromStoA, End, RoleC<RoleEnd>, NameA>),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameA>),
// }
// type Choice3fromCtoA = Recv<Branching3fromCtoA, End>;
// enum Branching4fromStoA {
//     Continue(SessionMpstThree<Choice5fromCtoA, End, RoleC<RoleEnd>, NameA>),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameA>),
// }
// type Choice4fromStoA = Recv<Branching4fromStoA, End>;
// enum Branching5fromCtoA {
//     Continue(SessionMpstThree<Choice6fromStoA, End, RoleC<RoleEnd>, NameA>),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameA>),
// }
// type Choice5fromCtoA = Recv<Branching5fromCtoA, End>;
// enum Branching6fromStoA {
//     Continue(SessionMpstThree<Choice7fromCtoA, End, RoleC<RoleEnd>, NameA>),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameA>),
// }
// type Choice6fromStoA = Recv<Branching6fromStoA, End>;
// enum Branching7fromCtoA {
//     Continue(SessionMpstThree<Choice8fromStoA, End, RoleC<RoleEnd>, NameA>),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameA>),
// }
// type Choice7fromCtoA = Recv<Branching7fromCtoA, End>;
// enum Branching8fromStoA {
//     Continue(SessionMpstThree<Choice9fromCtoA, End, RoleC<RoleEnd>, NameA>),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameA>),
// }
// type Choice8fromStoA = Recv<Branching8fromStoA, End>;
// enum Branching9fromCtoA {
//     Continue(SessionMpstThree<Choice9fromCtoA, End, RoleC<RoleEnd>, NameA>),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameA>),
// }
// type Choice9fromCtoA = Recv<Branching9fromCtoA, End>;
// enum Branching10fromStoA {
//     Continue(SessionMpstThree<End, End, RoleC<RoleEnd>, NameA>),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameA>),
// }
// type Choice10fromStoA = Recv<Branching10fromStoA, End>;

// // S
// enum Branching0fromCtoS {
//     Continue(SessionMpstThree<Choose1fromStoA, Choose1fromStoC, RoleA<RoleC<RoleEnd>>, NameS>),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameS>),
// }
// type Choice0fromCtoS = Recv<Branching0fromCtoS, End>;

// // S1
// type Choose1fromStoA = Send<Branching1fromStoA, End>;
// type Choose1fromStoC = Send<Branching1fromStoC, End>;

// enum Branching2fromCtoS {
//     Continue(SessionMpstThree<Choice3fromCtoS, End, RoleC<RoleEnd>, NameS>),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameS>),
// }
// type Choice2fromCtoS = Recv<Branching2fromCtoS, End>;
// enum Branching3fromCtoS {
//     Continue(SessionMpstThree<Choose4fromStoA, Choose4fromStoC, RoleA<RoleC<RoleEnd>>, NameS>),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameS>),
// }
// type Choice3fromCtoS = Recv<Branching3fromCtoS, End>;

// // S4
// type Choose4fromStoA = Send<Branching4fromStoA, End>;
// type Choose4fromStoC = Send<Branching4fromStoC, End>;

// enum Branching5fromCtoS {
//     Continue(SessionMpstThree<Choose6fromStoA, Choose6fromStoC, RoleA<RoleC<RoleEnd>>, NameS>),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameS>),
// }
// type Choice5fromCtoS = Recv<Branching5fromCtoS, End>;

// // S6
// type Choose6fromStoA = Send<Branching6fromStoA, End>;
// type Choose6fromStoC = Send<Branching6fromStoC, End>;

// enum Branching7fromCtoS {
//     Continue(SessionMpstThree<Choose8fromStoA, Choose8fromStoC, RoleA<RoleC<RoleEnd>>, NameS>),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameS>),
// }
// type Choice7fromCtoS = Recv<Branching7fromCtoS, End>;

// // S8
// type Choose8fromStoA = Send<Branching8fromStoA, End>;
// type Choose8fromStoC = Send<Branching8fromStoC, End>;

// enum Branching9fromCtoS {
//     Continue(SessionMpstThree<Choice9fromCtoS, End, RoleC<RoleEnd>, NameS>),
//     Quit(SessionMpstThree<End, End, RoleEnd, NameS>),
// }
// type Choice9fromCtoS = Recv<Branching9fromCtoS, End>;

// // Creating the MP sessions
// // A
// type EndpointA = SessionMpstThree<Choice0fromCtoA, End, RoleC<RoleEnd>, NameA>;
// // C
// type EndpointC = SessionMpstThree<Choose0fromCtoA, Choose0fromCtoS, RoleA<RoleS<RoleEnd>>, NameC>;
// // S
// type EndpointS = SessionMpstThree<End, Choice0fromCtoS, RoleC<RoleEnd>, NameS>;

// // None
// type EndpointNoneA = SessionMpstThree<End, End, RoleEnd, NameA>;
// type EndpointNoneC = SessionMpstThree<End, End, RoleEnd, NameC>;
// type EndpointNoneS = SessionMpstThree<End, End, RoleEnd, NameS>;

// // Functions
// // A
// fn simple_five_endpoint_a(s: EndpointNoneA) -> Result<(), Box<dyn Error>> {
//     close_mpst_multi(s)
// }
// // C
// fn simple_five_endpoint_c(s: EndpointNoneC) -> Result<(), Box<dyn Error>> {
//     close_mpst_multi(s)
// }
// // S
// fn simple_five_endpoint_s(s: EndpointNoneS) -> Result<(), Box<dyn Error>> {
//     close_mpst_multi(s)
// }

// fn all_mpst() -> Result<(), Box<dyn Error>> {
//     let (thread_a, thread_c, thread_s) = fork_mpst(
//         black_box(simple_five_endpoint_a),
//         black_box(simple_five_endpoint_c),
//         black_box(simple_five_endpoint_s),
//     );

//     thread_a.join().unwrap();
//     thread_c.join().unwrap();
//     thread_s.join().unwrap();

//     Ok(())
// }

// /////////////////////////

// fn smtp_mpst(c: &mut Criterion) {
//     c.bench_function(&format!("Smtp MPST"), |b| b.iter(|| all_mpst()));
// }

// fn long_warmup() -> Criterion {
//     Criterion::default().measurement_time(Duration::new(30, 0))
// }

// criterion_group! {
//     name = smtp;
//     config = long_warmup();
//     targets = smtp_mpst
// }

// criterion_main!(smtp);
