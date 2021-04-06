use rand::{thread_rng, Rng};

use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::fork::fork_mpst;
use mpstthree::role::broadcast::RoleBroadcast;
// use mpstthree::role::Role;
use mpstthree::sessionmpst::SessionMpst;

// use std::any::type_name;
use std::boxed::Box;
// use std::collections::hash_map::RandomState;
// use std::collections::HashMap;
use std::error::Error;
// use std::fmt;
use std::marker;

// use mpstthree::checking::checker;

use mpstthree::functionmpst::close::close_mpst;

// Get roles
use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

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

/// Test our usecase
/// Simple types
/// Client = C
/// Authenticator = A
/// Server = B

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
    End(SessionMpst<AtoBClose, AtoCClose, StackAEnd, RoleA<RoleEnd>>),
    Video(SessionMpst<AtoBVideo<N>, AtoCVideo<N>, StackAVideo, RoleA<RoleEnd>>),
}
enum Branches0BtoC<N: marker::Send> {
    End(SessionMpst<BtoAClose, BtoCClose, StackBEnd, RoleB<RoleEnd>>),
    Video(SessionMpst<BtoAVideo<N>, RecursBtoC<N>, StackBVideo, RoleB<RoleEnd>>),
}
type Choose0fromCtoA<N> = Send<Branches0AtoC<N>, End>;
type Choose0fromCtoB<N> = Send<Branches0BtoC<N>, End>;

type InitC<N> = Send<N, Recv<N, Choose0fromCtoA<N>>>;

/// Stacks
type StackAEnd = RoleEnd;
type StackAVideo = RoleC<RoleB<RoleB<RoleC<RoleC<RoleEnd>>>>>;
type StackARecurs = RoleC<RoleEnd>;
type StackAInit = RoleC<RoleC<RoleC<RoleEnd>>>;

type StackBEnd = RoleEnd;
type StackBVideo = RoleA<RoleA<RoleC<RoleEnd>>>;
type StackBRecurs = RoleC<RoleEnd>;

type StackCRecurs = RoleBroadcast;
type StackCFull = RoleA<RoleA<StackCRecurs>>;

/// Creating the MP sessions

/// For C
type EndpointCRecurs<N> =
    SessionMpst<Choose0fromCtoA<N>, Choose0fromCtoB<N>, StackCRecurs, RoleC<RoleEnd>>;
type EndpointCFull<N> = SessionMpst<InitC<N>, Choose0fromCtoB<N>, StackCFull, RoleC<RoleEnd>>;

/// For A
type EndpointARecurs<N> = SessionMpst<End, RecursAtoC<N>, StackARecurs, RoleA<RoleEnd>>;
type EndpointAFull<N> = SessionMpst<End, InitA<N>, StackAInit, RoleA<RoleEnd>>;

/// For B
type EndpointBRecurs<N> = SessionMpst<End, RecursBtoC<N>, StackBRecurs, RoleB<RoleEnd>>;

/// Functions related to endpoints
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

pub fn run_c_usecase_recursive() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}

///////////////////////////////////////// Need a refactoring
///////////////////////////////////////// to be included in
///////////////////////////////////////// macro

// fn type_of<T>(_: T) -> &'static str {
//     type_name::<T>()
// }

// impl<N: marker::Send> fmt::Display for Branches0AtoC<N> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Branches0AtoC::Video(s) => {
//                 write!(f, "Video:{}", type_of(&s))
//             }
//             Branches0AtoC::End(s) => {
//                 write!(f, "End:{}", type_of(&s))
//             }
//         }
//     }
// }

// fn hashmap_c_branches_a_to_c() -> Vec<String> {
//     let (s_video, _) = <_ as Session>::new();

//     let video = Branches0AtoC::Video::<i32>(s_video);

//     let (s_end, _) = <_ as Session>::new();

//     let end = Branches0AtoC::End::<i32>(s_end);

//     vec![(&video).to_string(), (&end).to_string()]
// }

// impl<N: marker::Send> fmt::Display for Branches0BtoC<N> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Branches0BtoC::Video(s) => {
//                 write!(f, "Video:{}", type_of(&s))
//             }
//             Branches0BtoC::End(s) => {
//                 write!(f, "End:{}", type_of(&s))
//             }
//         }
//     }
// }

// fn hashmap_c_branches_b_to_c() -> Vec<String> {
//     let (s_video, _) = <_ as Session>::new();

//     let video = Branches0BtoC::Video::<i32>(s_video);

//     let (s_end, _) = <_ as Session>::new();

//     let end = Branches0BtoC::End::<i32>(s_end);

//     vec![(&video).to_string(), (&end).to_string()]
// }

// type StackCEnd = RoleEnd;
// type StackCVideo = RoleA<RoleA<RoleA<RoleB<RoleEnd>>>>;

// pub fn run_c_usecase_recursive_checker() {
//     assert!(|| -> Result<(), Box<dyn Error>> {
//         {
//             // Get the new sessionmpst of the passive roles
//             let state_branches_receivers = RandomState::new();
//             let mut branches_receivers: HashMap<String, &Vec<String>> =
//                 HashMap::with_hasher(state_branches_receivers);

//             let c_branches_a_to_c: Vec<String> = hashmap_c_branches_a_to_c();
//             let c_branches_b_to_c: Vec<String> = hashmap_c_branches_b_to_c();

//             branches_receivers.insert(String::from("Branches0AtoC<i32>"), &c_branches_a_to_c);
//             branches_receivers.insert(String::from("Branches0BtoC<i32>"), &c_branches_b_to_c);

//             let (s1, _): (EndpointAFull<i32>, _) = SessionMpst::new();
//             let (s2, _): (EndpointBRecurs<i32>, _) = SessionMpst::new();
//             let (s3, _): (EndpointCFull<i32>, _) = SessionMpst::new();

//             // Get the new stack of the active role
//             let state_branches_sender = RandomState::new();
//             let mut branches_sender: HashMap<String, &Vec<String>> =
//                 HashMap::with_hasher(state_branches_sender);

//             let (stack_video, _): (StackCVideo, _) = Role::new();
//             let (stack_end, _): (StackCEnd, _) = Role::new();

//             let mut stacks: Vec<String> = Vec::new();
//             stacks.push(type_of(&stack_video).to_string());
//             stacks.push(type_of(&stack_end).to_string());

//             branches_sender.insert(String::from("Branches0AtoC<i32>"), &stacks);
//             branches_sender.insert(String::from("Branches0BtoC<i32>"), &stacks);

//             let (a, b, c) = checker(s1, s2, s3, &branches_receivers, &branches_sender)?;

//             assert_eq!(a, "A: A?C.A!C.µX( A?C.A!B.A?B.A!C.X & 0 )");
//             assert_eq!(b, "B: µX( B?A.B!A.X & 0 )");
//             assert_eq!(c, "C: C!A.C?A.µX( C!A.C?A.X + 0 )");
//         }
//         Ok(())
//     }()
//     .is_ok());
// }
