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

#![allow(clippy::type_complexity)]

use rand::{thread_rng, Rng};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::functionmpst::close::close_mpst;
use mpstthree::functionmpst::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::broadcast::RoleBroadcast;

use std::boxed::Box;
use std::error::Error;
use std::marker;

// Get roles
use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

// Get names
use mpstthree::name::a::NameA;
use mpstthree::name::b::NameB;
use mpstthree::name::c::NameC;

// Get recv functions
use mpstthree::functionmpst::recv::recv_mpst_a_from_b;
use mpstthree::functionmpst::recv::recv_mpst_a_from_c;
use mpstthree::functionmpst::recv::recv_mpst_b_from_a;
use mpstthree::functionmpst::recv::recv_mpst_c_from_a;

// Get send functions
use mpstthree::functionmpst::send::send_mpst_a_to_b;
use mpstthree::functionmpst::send::send_mpst_a_to_c;
use mpstthree::functionmpst::send::send_mpst_b_to_a;
use mpstthree::functionmpst::send::send_mpst_c_to_a;

use mpstthree::choose_mpst_c_to_all;
use mpstthree::offer_mpst_a_to_c;
use mpstthree::offer_mpst_b_to_c;

// See the folder scribble_protocols for the related Scribble protocol

// Test our usecase Video Stream
// Simple types
// Client = C
// Authenticator = A
// Server = B

type AtoCClose = End;
type AtoBClose = End;
type AtoBVideo<N> = Send<N, Recv<N, End>>;
type AtoCVideo<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type InitA<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type BtoAClose = <AtoBClose as Session>::Dual;
type BtoCClose = End;
type BtoAVideo<N> = <AtoBVideo<N> as Session>::Dual;

type RecursAtoC<N> = Recv<Branches0AtoC<N>, End>;
type RecursBtoC<N> = Recv<Branches0BtoC<N>, End>;

enum Branches0AtoC<N: marker::Send> {
    End(MeshedChannels<AtoBClose, AtoCClose, StackAEnd, NameA>),
    Video(MeshedChannels<AtoBVideo<N>, AtoCVideo<N>, StackAVideo, NameA>),
}
enum Branches0BtoC<N: marker::Send> {
    End(MeshedChannels<BtoAClose, BtoCClose, StackBEnd, NameB>),
    Video(MeshedChannels<BtoAVideo<N>, RecursBtoC<N>, StackBVideo, NameB>),
}
type Choose0fromCtoA<N> = Send<Branches0AtoC<N>, End>;
type Choose0fromCtoB<N> = Send<Branches0BtoC<N>, End>;

type InitC<N> = Send<N, Recv<N, Choose0fromCtoA<N>>>;

// Stacks
type StackAEnd = RoleEnd;
type StackAVideo = RoleC<RoleB<RoleB<RoleC<RoleC<RoleEnd>>>>>;
type StackARecurs = RoleC<RoleEnd>;
type StackAInit = RoleC<RoleC<RoleC<RoleEnd>>>;

type StackBEnd = RoleEnd;
type StackBVideo = RoleA<RoleA<RoleC<RoleEnd>>>;
type StackBRecurs = RoleC<RoleEnd>;

type StackCRecurs = RoleBroadcast;
type StackCFull = RoleA<RoleA<StackCRecurs>>;

// Creating the MP sessions

// For C
type EndpointCRecurs<N> =
    MeshedChannels<Choose0fromCtoA<N>, Choose0fromCtoB<N>, StackCRecurs, NameC>;
type EndpointCFull<N> = MeshedChannels<InitC<N>, Choose0fromCtoB<N>, StackCFull, NameC>;

// For A
type EndpointARecurs<N> = MeshedChannels<End, RecursAtoC<N>, StackARecurs, NameA>;
type EndpointAFull<N> = MeshedChannels<End, InitA<N>, StackAInit, NameA>;

// For B
type EndpointBRecurs<N> = MeshedChannels<End, RecursBtoC<N>, StackBRecurs, NameB>;

// Functions related to endpoints
fn server(s: EndpointBRecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_b_to_c!(s, {
        Branches0BtoC::End(s) => {
            close_mpst(s)
        },
        Branches0BtoC::Video(s) => {
            let (request, s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_a(request + 1, s);
            server(s)
        },
    })
}

fn authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_a_from_c(s)?;
    let s = send_mpst_a_to_c(id + 1, s);

    authenticator_recurs(s)
}

fn authenticator_recurs(s: EndpointARecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_a_to_c!(s, {
        Branches0AtoC::End(s) => {
            close_mpst(s)
        },
        Branches0AtoC::Video(s) => {
            let (request, s) = recv_mpst_a_from_c(s)?;
            let s = send_mpst_a_to_b(request + 1, s);
            let (video, s) = recv_mpst_a_from_b(s)?;
            let s = send_mpst_a_to_c(video + 1, s);
            authenticator_recurs(s)
        },
    })
}

fn client(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();

    let s = send_mpst_c_to_a(0, s);
    let (_, s) = recv_mpst_c_from_a(s)?;

    client_recurs(s, xs, 1)
}

fn client_recurs(
    s: EndpointCRecurs<i32>,
    mut xs: Vec<i32>,
    index: i32,
) -> Result<(), Box<dyn Error>> {
    match xs.pop() {
        Option::Some(_) => {
            let s = choose_mpst_c_to_all!(s, Branches0AtoC::Video, Branches0BtoC::Video);

            let s = send_mpst_c_to_a(1, s);
            let (_, s) = recv_mpst_c_from_a(s)?;

            client_recurs(s, xs, index + 1)
        }
        Option::None => {
            let s = choose_mpst_c_to_all!(s, Branches0AtoC::End, Branches0BtoC::End);

            assert_eq!(index, 100);

            close_mpst(s)
        }
    }
}

/////////////////////////////////////////

fn main() {
    let (thread_a, thread_s, thread_c) = fork_mpst(authenticator, server, client);

    assert!(thread_a.join().is_ok());
    assert!(thread_s.join().is_ok());
    assert!(thread_c.join().is_ok());
}
