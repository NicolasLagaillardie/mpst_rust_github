// use mpstthree::binary::struct_trait::end::*;

// use mpstthree::role::broadcast::RoleBroadcast;
// use mpstthree::role::end::RoleEnd;

// use mpstthree::binary_timed::struct_trait::recv::RecvTimed;
// use mpstthree::binary_timed::struct_trait::send::SendTimed;

// use mpstthree::bundle_impl_timed_with_enum_and_cancel;

// use rand::{thread_rng, Rng};
// use std::boxed::Box;
// use std::collections::HashMap;
// use std::error::Error;
// use std::thread::sleep;
// use std::time::{Duration, Instant};

// bundle_impl_timed_with_enum_and_cancel!(MeshedChannels, Worker, Server);

// type NegServer = MeshedChannels<
//     RecvTimed<i32, SendTimed<i32, End, 'a', 4, true, 6, true, false>, 'a', 2, true, 4, true, false>,
//     RoleWorker<RoleWorker<RoleEnd>>,
//     NameServer,
// >;
// type NegWoker = MeshedChannels<
//     SendTimed<i32, RecvTimed<i32, End, 'a', 4, true, 6, true, false>, 'a', 2, true, 4, true, false>,
//     RoleServer<RoleServer<RoleEnd>>,
//     NameWorker,
// >;

// type AddServer = MeshedChannels<
//     RecvTimed<
//         i32,
//         RecvTimed<
//             i32,
//             SendTimed<i32, End, 'a', 6, true, 8, true, false>,
//             'a',
//             4,
//             true,
//             6,
//             true,
//             false,
//         >,
//         'a',
//         2,
//         true,
//         4,
//         true,
//         false,
//     >,
//     RoleWorker<RoleWorker<RoleWorker<RoleEnd>>>,
//     NameServer,
// >;
// type AddWoker = MeshedChannels<
//     SendTimed<
//         i32,
//         SendTimed<
//             i32,
//             RecvTimed<i32, End, 'a', 6, true, 8, true, false>,
//             'a',
//             4,
//             true,
//             6,
//             true,
//             false,
//         >,
//         'a',
//         2,
//         true,
//         4,
//         true,
//         false,
//     >,
//     RoleServer<RoleServer<RoleServer<RoleEnd>>>,
//     NameWorker,
// >;

// pub enum BranchesServer {
//     Neg(NegServer),
//     Add(AddServer),
// }

// type SimpleCalcServer = MeshedChannels<
//     RecvTimed<BranchesServer, End, 'a', 1, true, 2, true, false>,
//     RoleWorker<RoleEnd>,
//     NameServer,
// >;
// type SimpleCalcWoker = MeshedChannels<
//     SendTimed<BranchesServer, End, 'a', 1, true, 2, true, false>,
//     RoleBroadcast,
//     NameWorker,
// >;

// fn simple_calc_server(
//     s: SimpleCalcServer,
//     all_clocks: &mut HashMap<char, Instant>,
// ) -> Result<(), Box<dyn Error>> {
//     all_clocks.insert('a', Instant::now());
//     sleep(Duration::from_secs(1));
//     offer_mpst!(
//         all_clocks,
//         s,
//         {
//             BranchesServer::Neg(s) => {
//                 sleep(Duration::from_secs(2));
//                 let (x, s) = s.recv(all_clocks)?;
//                 sleep(Duration::from_secs(2));
//                 let s = s.send(-x, all_clocks)?;
//                 sleep(Duration::from_secs(2));
//                 s.close()
//             },
//             BranchesServer::Add(s) => {
//                 sleep(Duration::from_secs(2));
//                 let (x, s) = s.recv(all_clocks)?;
//                 sleep(Duration::from_secs(2));
//                 let (y, s) = s.recv(all_clocks)?;
//                 sleep(Duration::from_secs(2));
//                 let s = s.send(x.wrapping_add(y), all_clocks)?;
//                 sleep(Duration::from_secs(2));
//                 s.close()
//             },
//         }
//     )
// }

// pub fn simple_calc_works_add(
//     s: SimpleCalcWoker,
//     all_clocks: &mut HashMap<char, Instant>,
// ) -> std::result::Result<(), Box<dyn std::error::Error>> {
//     let x: i32 = thread_rng().gen();

//     all_clocks.insert('a', Instant::now());

//     sleep(Duration::from_secs(1));
//     let s: NegWoker = choose_mpst_worker_to_all!(s, all_clocks, BranchesServer::Neg);
//     sleep(Duration::from_secs(2));
//     let s = s.send(x, all_clocks)?;
//     sleep(Duration::from_secs(2));
//     let (y, s) = s.recv(all_clocks)?;
//     sleep(Duration::from_secs(2));

//     assert_eq!(-x, y);

//     s.close()
// }

// pub fn simple_calc_works_neg(
//     s: SimpleCalcWoker,
//     all_clocks: &mut HashMap<char, Instant>,
// ) -> std::result::Result<(), Box<dyn std::error::Error>> {
//     let x: i32 = thread_rng().gen();
//     let y: i32 = thread_rng().gen();

//     all_clocks.insert('a', Instant::now());

//     sleep(Duration::from_secs(1));
//     let s: AddWoker = choose_mpst_worker_to_all!(s, all_clocks, BranchesServer::Add);
//     sleep(Duration::from_secs(2));
//     let s = s.send(x, all_clocks)?;
//     sleep(Duration::from_secs(2));
//     let s = s.send(y, all_clocks)?;
//     sleep(Duration::from_secs(2));
//     let (z, s) = s.recv(all_clocks)?;
//     sleep(Duration::from_secs(2));

//     assert_eq!(x.wrapping_add(y), z);

//     s.close()
// }

// #[test]
// fn main() {
//     assert!(|| -> Result<(), Box<dyn Error>> {
//         {
//             let (thread_server, thread_worker) =
//                 fork_mpst(simple_calc_server, simple_calc_works_neg);

//             assert!(thread_server.join().is_ok());
//             assert!(thread_worker.join().is_ok());
//         }
//         Ok(())
//     }()
//     .is_ok());

//     assert!(|| -> Result<(), Box<dyn Error>> {
//         {
//             let (thread_server, thread_worker) =
//                 fork_mpst(simple_calc_server, simple_calc_works_add);

//             assert!(thread_server.join().is_ok());
//             assert!(thread_worker.join().is_ok());
//         }
//         Ok(())
//     }()
//     .is_ok());
// }

fn main() {}
