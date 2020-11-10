extern crate mpstthree;

use mpstthree::binary::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::sessionmpst::SessionMpst;
use std::marker;

use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;

/////////////////////////////////////////

extern crate rand;

use rand::{thread_rng, Rng};

use std::boxed::Box;
use std::error::Error;

use mpstthree::fork::fork_mpst;
use mpstthree::functionmpst::close::close_mpst;

use mpstthree::role::Role;

// Get recv functions
use mpstthree::functionmpst::recv::recv_mpst_a_to_b;
use mpstthree::functionmpst::recv::recv_mpst_a_to_c;
use mpstthree::functionmpst::recv::recv_mpst_b_to_c;

// Get send functions
use mpstthree::functionmpst::send::send_mpst_b_to_a;
use mpstthree::functionmpst::send::send_mpst_c_to_a;
use mpstthree::functionmpst::send::send_mpst_c_to_b;

use mpstthree::choose_mpst_c_to_all;
use mpstthree::offer_mpst_a_to_c;
use mpstthree::offer_mpst_b_to_c;

/////////////////////////////////////////

type ADDAtoB<N> = Recv<N, End>;

type OrderingA0 = RoleB<RoleEnd>;
type EndpointA1<N> = SessionMpst<ADDAtoB<N>, End, OrderingA0, RoleA<RoleEnd>>;
type BYEAtoB<N> = Recv<N, End>;

type OrderingA2 = RoleB<RoleEnd>;
type EndpointA3<N> = SessionMpst<BYEAtoB<N>, End, OrderingA2, RoleA<RoleEnd>>;

enum CBranchesAtoC<N: marker::Send> {
    ADD(SessionMpst<ADDAtoB<N>, End, OrderingA0, RoleA<RoleEnd>>),
    BYE(SessionMpst<BYEAtoB<N>, End, OrderingA2, RoleA<RoleEnd>>),
}
type ChooseCforAtoC<N> = Send<CBranchesAtoC<N>, End>;

type TestAtoC<N> = Recv<N, End>;

type OrderingA4 = RoleC<RoleEnd>;
type OrderingA5Full = RoleC<RoleC<RoleEnd>>;
type EndpointA6<N> =
    SessionMpst<End, Recv<N, Recv<CBranchesAtoC<N>, End>>, OrderingA5Full, RoleA<RoleEnd>>;

type ADDBtoA<N> = Send<N, End>;
type ADDBtoC<N> = Recv<N, End>;

type OrderingB0 = RoleC<RoleA<RoleEnd>>;
type EndpointB1<N> = SessionMpst<ADDBtoA<N>, ADDBtoC<N>, OrderingB0, RoleB<RoleEnd>>;
type BYEBtoA<N> = Send<N, End>;
type BYEBtoC<N> = Recv<N, End>;

type OrderingB2 = RoleC<RoleA<RoleEnd>>;
type EndpointB3<N> = SessionMpst<BYEBtoA<N>, BYEBtoC<N>, OrderingB2, RoleB<RoleEnd>>;

enum CBranchesBtoC<N: marker::Send> {
    ADD(SessionMpst<ADDBtoA<N>, ADDBtoC<N>, OrderingB0, RoleB<RoleEnd>>),
    BYE(SessionMpst<BYEBtoA<N>, BYEBtoC<N>, OrderingB2, RoleB<RoleEnd>>),
}
type ChooseCforBtoC<N> = Send<CBranchesBtoC<N>, End>;

type OrderingB4 = RoleEnd;
type OrderingB5Full = RoleC<RoleEnd>;
type EndpointB6<N> = SessionMpst<End, Recv<CBranchesBtoC<N>, End>, OrderingB5Full, RoleB<RoleEnd>>;

type TestCtoA<N> = Send<N, ChooseCforAtoC<N>>;

type OrderingC0 = RoleA<RoleA<RoleB<RoleEnd>>>;
type EndpointC1<N> = SessionMpst<TestCtoA<N>, ChooseCforBtoC<N>, OrderingC0, RoleC<RoleEnd>>;

/////////////////////////////////////////

// type ADDAtoB<N> = Recv<N, End>;

// type OrderingA0 = RoleB<RoleEnd>;
// type EndpointA1<N> = SessionMpst<ADDAtoB<N>, End, OrderingA0, RoleA<RoleEnd>>;
// type BYEAtoB<N> = Recv<N, End>;

// type OrderingA2 = RoleB<RoleEnd>;
// type EndpointA3<N> = SessionMpst<BYEAtoB<N>, End, OrderingA2, RoleA<RoleEnd>>;

// enum CBranchesAtoC<N: marker::Send> {
//     ADD(SessionMpst<ADDAtoB<N>, End, OrderingA0, RoleA<RoleEnd>>),
//     BYE(SessionMpst<BYEAtoB<N>, End, OrderingA2, RoleA<RoleEnd>>),
// }
// type ChooseCforAtoC<N> = Send<CBranchesAtoC<N>, End>;

// type TestAtoC<N> = Recv<N, End>;

// type OrderingA4 = RoleC<RoleEnd>;
// type OrderingA5Full = RoleC<RoleEnd>;
// // type EndpointA6<N> = SessionMpst<TestAtoC<N>, Recv<CBranchesAtoC<N>, End>, OrderingA5Full, RoleA<RoleEnd>>;
// type EndpointA6<N> = SessionMpst<TestAtoC<N>, Recv<CBranchesAtoC<N>, End>, OrderingA5Full, RoleA<RoleEnd>>;

// type ADDBtoA<N> = Send<N, End>;
// type ADDBtoC<N> = Recv<N, End>;

// type OrderingB0 = RoleC<RoleA<RoleEnd>>;
// type EndpointB1<N> = SessionMpst<ADDBtoA<N>, ADDBtoC<N>, OrderingB0, RoleB<RoleEnd>>;
// type BYEBtoA<N> = Send<N, End>;
// type BYEBtoC<N> = Recv<N, End>;

// type OrderingB2 = RoleC<RoleA<RoleEnd>>;
// type EndpointB3<N> = SessionMpst<BYEBtoA<N>, BYEBtoC<N>, OrderingB2, RoleB<RoleEnd>>;

// enum CBranchesBtoC<N: marker::Send> {
//     ADD(SessionMpst<ADDBtoA<N>, ADDBtoC<N>, OrderingB0, RoleB<RoleEnd>>),
//     BYE(SessionMpst<BYEBtoA<N>, BYEBtoC<N>, OrderingB2, RoleB<RoleEnd>>),
// }
// type ChooseCforBtoC<N> = Send<CBranchesBtoC<N>, End>;

// type OrderingB4 = RoleEnd;
// type OrderingB5Full = RoleC<RoleEnd>;
// type EndpointB6<N> = SessionMpst<End, Recv<CBranchesBtoC<N>, End>, OrderingB5Full, RoleB<RoleEnd>>;

// // type TestCtoA<N> = Send<N, ChooseCforAtoC<N>>;

// type OrderingC0 = RoleA<RoleEnd>;
// // type EndpointC1<N> = SessionMpst<TestCtoA<N>, ChooseCforBtoC<N>, OrderingC0, RoleC<RoleEnd>>;
// type EndpointC1<N> = SessionMpst<ChooseCforAtoC<N>, ChooseCforBtoC<N>, OrderingC0, RoleC<RoleEnd>>;

// /// Functions related to endpoints
// fn server(s: EndpointB6<i32>) -> Result<(), Box<dyn Error>> {
//     offer_mpst_b_to_c!(s, {
//         CBranchesBtoC::BYE(s) => {
//             let (id, s) = recv_mpst_b_to_c(s)?;
//             let s = send_mpst_b_to_a(id + 1, s);
//             close_mpst(s)?;
//             Ok(())
//         },
//         CBranchesBtoC::ADD(s) => {
//             let (id, s) = recv_mpst_b_to_c(s)?;
//             let s = send_mpst_b_to_a(id + 1, s);
//             close_mpst(s)?;
//             Ok(())
//         },
//     })?;
//     Ok(())
// }

// // fn authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
// //     let (id, s) = recv_mpst_a_to_c(s)?;
// //     let s = send_mpst_a_to_c(id + 1, s);

// //     let result = authenticator_recurs(s)?;

// //     Ok(result)
// // }

// fn authenticator_recurs(s: EndpointA6<i32>) -> Result<(), Box<dyn Error>> {
//     offer_mpst_a_to_c!(s, {
//         CBranchesAtoC::BYE(s) => {
//             let (id, s) = recv_mpst_a_to_b(s)?;
//             close_mpst(s)?;
//             Ok(())
//         },
//         CBranchesAtoC::ADD(s) => {
//             let (id, s) = recv_mpst_a_to_b(s)?;
//             close_mpst(s)?;
//             Ok(())
//         },
//     })?;
//     Ok(())
// }

// fn client(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
//     let mut rng = thread_rng();
//     let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();

//     let s = send_mpst_c_to_a(0, s);
//     let (_, s) = recv_mpst_c_to_a(s)?;

//     let result = client_recurs(s, xs, 1)?;

//     Ok(result)
// }

// fn client_recurs(
//     s: EndpointCRecurs<i32>,
//     mut xs: Vec<i32>,
//     index: i32,
// ) -> Result<(), Box<dyn Error>> {
//     match xs.pop() {
//         Option::Some(_) => {
//             let s = choose_mpst_c_to_all!(s, CBranchesAtoC::ADD, CBranchesBtoC::ADD);

//             let s = send_mpst_c_to_a(1, s);
//             let (_, s) = recv_mpst_c_to_a(s)?;

//             client_recurs(s, xs, index + 1)
//         }
//         Option::None => {
//             let s = choose_mpst_c_to_all!(s, CBranchesAtoC::BYE, CBranchesBtoC::BYE);

//             close_mpst(s)?;

//             assert_eq!(index, 100);

//             Ok(())
//         }
//     }
// }

// /////////////////////////////////////////

// #[test]
// fn run_usecase_recursive() {
//     assert!(|| -> Result<(), Box<dyn Error>> {
//         {
//             let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client);

//             assert!(thread_a.is_ok());
//             assert!(thread_b.is_ok());
//             assert!(thread_c.is_ok());
//         }
//         Ok(())
//     }()
//     .is_ok());
// }
