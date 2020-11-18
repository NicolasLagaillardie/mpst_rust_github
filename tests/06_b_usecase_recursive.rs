// extern crate mpstthree;
// extern crate rand;

// use rand::{thread_rng, Rng};

// use mpstthree::binary::{End, Recv, Send, Session};
// use mpstthree::fork::fork_mpst;
// use mpstthree::role::Role;
// use mpstthree::sessionmpst::SessionMpst;

// use std::any::type_name;
// use std::boxed::Box;
// use std::collections::hash_map::RandomState;
// use std::collections::HashMap;
// use std::error::Error;
// use std::fmt;
// use std::marker;

// use mpstthree::checking::checker;

// use mpstthree::functionmpst::close::close_mpst;

// // Get roles
// use mpstthree::role::a::RoleA;
// use mpstthree::role::b::RoleB;
// use mpstthree::role::c::RoleC;
// use mpstthree::role::end::RoleEnd;

// // Get recv functions
// use mpstthree::functionmpst::recv::recv_mpst_a_to_c;
// use mpstthree::functionmpst::recv::recv_mpst_b_to_c;
// use mpstthree::functionmpst::recv::recv_mpst_c_to_a;
// use mpstthree::functionmpst::recv::recv_mpst_c_to_b;

// // Get send functions
// use mpstthree::functionmpst::send::send_mpst_a_to_c;
// use mpstthree::functionmpst::send::send_mpst_b_to_c;
// use mpstthree::functionmpst::send::send_mpst_c_to_a;
// use mpstthree::functionmpst::send::send_mpst_c_to_b;

// use mpstthree::choose_mpst_b_to_all;
// use mpstthree::offer_mpst_a_to_b;
// use mpstthree::offer_mpst_c_to_b;

// /// Test our usecase
// /// Simple types
// /// Client = B → Y → A
// /// Authenticator = C → Z → B
// /// Server = A → X → C

// type CtoBClose = End;
// type CtoAClose = End;
// type CtoAVideo<N> = Send<N, Recv<N, End>>;
// type CtoBVideo<N> = Recv<N, Send<N, RecursCtoB<N>>>;

// type InitC<N> = Recv<N, Send<N, RecursCtoB<N>>>;

// type AtoCClose = <CtoAClose as Session>::Dual;
// type AtoBClose = End;
// type AtoCVideo<N> = <CtoAVideo<N> as Session>::Dual;

// type RecursCtoB<N> = Recv<BrancheCtoB<N>, End>;
// type RecursAtoB<N> = Recv<BrancheAtoB<N>, End>;

// enum BrancheCtoB<N: marker::Send> {
//     End(SessionMpst<CtoAClose, CtoBClose, QueueCEnd, RoleC<RoleEnd>>),
//     Video(SessionMpst<CtoAVideo<N>, CtoBVideo<N>, QueueCVideo, RoleC<RoleEnd>>),
// }
// enum BrancheAtoB<N: marker::Send> {
//     End(SessionMpst<AtoBClose, AtoCClose, QueueAEnd, RoleA<RoleEnd>>),
//     Video(SessionMpst<RecursAtoB<N>, AtoCVideo<N>, QueueAVideo, RoleA<RoleEnd>>),
// }
// type ChooseBforCtoB<N> = Send<BrancheCtoB<N>, End>;
// type ChooseBforAtoB<N> = Send<BrancheAtoB<N>, End>;

// type InitB<N> = Send<N, Recv<N, ChooseBforCtoB<N>>>;

// /// Queues
// type QueueCEnd = RoleEnd;
// type QueueCVideo = RoleB<RoleA<RoleA<RoleB<RoleB<RoleEnd>>>>>;
// type QueueCRecurs = RoleB<RoleEnd>;
// type QueueCInit = RoleB<RoleB<RoleB<RoleEnd>>>;

// type QueueAEnd = RoleEnd;
// type QueueAVideo = RoleC<RoleC<RoleB<RoleEnd>>>;
// type QueueARecurs = RoleB<RoleEnd>;

// type QueueBRecurs = RoleA<RoleC<RoleEnd>>;
// type QueueBFull = RoleC<RoleC<QueueBRecurs>>;

// /// Creating the MP sessions

// /// For B
// type EndpointBRecurs<N> =
//     SessionMpst<ChooseBforAtoB<N>, ChooseBforCtoB<N>, QueueBRecurs, RoleB<RoleEnd>>;
// type EndpointBFull<N> = SessionMpst<ChooseBforAtoB<N>, InitB<N>, QueueBFull, RoleB<RoleEnd>>;

// /// For C
// type EndpointCRecurs<N> = SessionMpst<End, RecursCtoB<N>, QueueCRecurs, RoleC<RoleEnd>>;
// type EndpointCFull<N> = SessionMpst<End, InitC<N>, QueueCInit, RoleC<RoleEnd>>;

// /// For A
// type EndpointARecurs<N> = SessionMpst<RecursAtoB<N>, End, QueueARecurs, RoleA<RoleEnd>>;

// /// Functions related to endpoints
// fn server(s: EndpointARecurs<i32>) -> Result<(), Box<dyn Error>> {
//     offer_mpst_a_to_b!(s, {
//         BrancheAtoB::End(s) => {
//             close_mpst(s)?;
//             Ok(())
//         },
//         BrancheAtoB::Video(s) => {
//             let (request, s) = recv_mpst_a_to_c(s)?;
//             let s = send_mpst_a_to_c(request + 1, s);
//             server(s)
//         },
//     })?;
//     Ok(())
// }

// fn authenticator(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
//     let (id, s) = recv_mpst_c_to_b(s)?;
//     let s = send_mpst_c_to_b(id + 1, s);

//     let result = authenticator_recurs(s)?;

//     Ok(result)
// }

// fn authenticator_recurs(s: EndpointCRecurs<i32>) -> Result<(), Box<dyn Error>> {
//     offer_mpst_c_to_b!(s, {
//         BrancheCtoB::End(s) => {
//             close_mpst(s)?;
//             Ok(())
//         },
//         BrancheCtoB::Video(s) => {
//             let (request, s) = recv_mpst_c_to_b(s)?;
//             let s = send_mpst_c_to_a(request + 1, s);
//             let (video, s) = recv_mpst_c_to_a(s)?;
//             let s = send_mpst_c_to_b(video + 1, s);
//             authenticator_recurs(s)
//         },
//     })?;
//     Ok(())
// }

// fn client(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
//     let mut rng = thread_rng();
//     let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();

//     let s = send_mpst_b_to_c(0, s);
//     let (_, s) = recv_mpst_b_to_c(s)?;

//     let result = client_recurs(s, xs, 1)?;

//     Ok(result)
// }

// fn client_recurs(
//     s: EndpointBRecurs<i32>,
//     mut xs: Vec<i32>,
//     index: i32,
// ) -> Result<(), Box<dyn Error>> {
//     match xs.pop() {
//         Option::Some(_) => {
//             let s = choose_mpst_b_to_all!(s, BrancheAtoB::Video, BrancheCtoB::Video);

//             let s = send_mpst_b_to_c(1, s);
//             let (_, s) = recv_mpst_b_to_c(s)?;

//             client_recurs(s, xs, index + 1)
//         }
//         Option::None => {
//             let s = choose_mpst_b_to_all!(s, BrancheAtoB::End, BrancheCtoB::End);

//             close_mpst(s)?;

//             assert_eq!(index, 100);

//             Ok(())
//         }
//     }
// }

// ///////////////////////////////////////// Need a refactoring to be included in macro

// fn type_of<T>(_: T) -> &'static str {
//     type_name::<T>()
// }

// impl<N: marker::Send> fmt::Display for BrancheCtoB<N> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             BrancheCtoB::Video(s) => write!(f, "Video:{}", type_of(&s)),
//             BrancheCtoB::End(s) => write!(f, "End:{}", type_of(&s)),
//         }
//     }
// }

// fn hashmap_c_branches_c_to_b() -> Vec<String> {
//     let (s_video, _) = <_ as Session>::new();

//     let video = BrancheCtoB::Video::<i32>(s_video);

//     let (s_end, _) = <_ as Session>::new();

//     let end = BrancheCtoB::End::<i32>(s_end);

//     vec![(&video).to_string(), (&end).to_string()]
// }

// impl<N: marker::Send> fmt::Display for BrancheAtoB<N> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             BrancheAtoB::Video(s) => write!(f, "Video:{}", type_of(&s)),
//             BrancheAtoB::End(s) => write!(f, "End:{}", type_of(&s)),
//         }
//     }
// }

// fn hashmap_c_branches_a_to_b() -> Vec<String> {
//     let (s_video, _) = <_ as Session>::new();

//     let video = BrancheAtoB::Video::<i32>(s_video);

//     let (s_end, _) = <_ as Session>::new();

//     let end = BrancheAtoB::End::<i32>(s_end);

//     println!("Type of end {}", type_of(&end));

//     vec![(&video).to_string(), (&end).to_string()]
// }

// /////////////////////////////////////////

// #[test]
// fn run_b_usecase_recursive() {
//     assert!(|| -> Result<(), Box<dyn Error>> {
//         {
//             let (thread_a, thread_b, thread_c) = fork_mpst(server, client, authenticator);

//             assert!(thread_a.is_ok());
//             assert!(thread_b.is_ok());
//             assert!(thread_c.is_ok());
//         }
//         Ok(())
//     }()
//     .is_ok());
// }

// #[test]
// fn run_b_usecase_recursive_checker() {
//     assert!(|| -> Result<(), Box<dyn Error>> {
//         {
//             let s = RandomState::new();
//             let mut hm: HashMap<String, &Vec<String>> = HashMap::with_hasher(s);

//             let c_branches_c_to_b: Vec<String> = hashmap_c_branches_c_to_b();
//             let c_branches_a_to_b: Vec<String> = hashmap_c_branches_a_to_b();

//             hm.insert(String::from("BrancheCtoB<i32>"), &c_branches_c_to_b);
//             hm.insert(String::from("BrancheAtoB<i32>"), &c_branches_a_to_b);

//             let (s1, _): (EndpointARecurs<i32>, _) = SessionMpst::new();
//             let (s2, _): (EndpointBFull<i32>, _) = SessionMpst::new();
//             let (s3, _): (EndpointCFull<i32>, _) = SessionMpst::new();

//             let (_a, _b, _c) = checker(s1, s2, s3, &hm)?;

//             // assert_eq!(a, "A: A?C.A!C.µX( A?C.A!B.A?B.A!C.X & 0 )");
//             // assert_eq!(b, "B: µX( B?A.B!A.X & 0 )");
//             // assert_eq!(c, "C: C!A.C?A.µX( A?C.A!B.A?B.A!C.X + 0 )");
//         }
//         Ok(())
//     }()
//     .is_ok());
// }
