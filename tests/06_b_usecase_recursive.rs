extern crate mpstthree;
extern crate rand;

use rand::{thread_rng, Rng};

use mpstthree::binary::{End, Recv, Send, Session};
use mpstthree::fork::fork_mpst;
use mpstthree::role::Role;
use mpstthree::sessionmpst::SessionMpst;

use std::any::type_name;
use std::boxed::Box;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::marker;

use mpstthree::checking::checker;

use mpstthree::functionmpst::close::close_mpst;

// Get roles
use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

// Get recv functions
use mpstthree::functionmpst::recv::recv_mpst_a_to_b;
use mpstthree::functionmpst::recv::recv_mpst_b_to_a;
use mpstthree::functionmpst::recv::recv_mpst_b_to_c;
use mpstthree::functionmpst::recv::recv_mpst_c_to_b;

// Get send functions
use mpstthree::functionmpst::send::send_mpst_a_to_b;
use mpstthree::functionmpst::send::send_mpst_b_to_a;
use mpstthree::functionmpst::send::send_mpst_b_to_c;
use mpstthree::functionmpst::send::send_mpst_c_to_b;

use mpstthree::choose_mpst_a_to_all;
use mpstthree::offer_mpst_b_to_a;
use mpstthree::offer_mpst_c_to_a;

/// Test our usecase
/// Simple types
/// Client = B → Y → A
/// Authenticator = C → Z → B
/// Server = A → X → C

type BtoAClose = End;
type BtoCClose = End;
type BtoCVideo<N> = Send<N, Recv<N, End>>;
type BtoAVideo<N> = Recv<N, Send<N, RecursBtoA<N>>>;

type InitB<N> = Recv<N, Send<N, RecursBtoA<N>>>;

type CtoBClose = <BtoCClose as Session>::Dual;
type CtoAClose = End;
type CtoBVideo<N> = <BtoCVideo<N> as Session>::Dual;

type RecursBtoA<N> = Recv<Branche0BtoA<N>, End>;
type RecursCtoA<N> = Recv<Branche0CtoA<N>, End>;

enum Branche0BtoA<N: marker::Send> {
    End(SessionMpst<BtoAClose, BtoCClose, QueueBEnd, RoleB<RoleEnd>>),
    Video(SessionMpst<BtoAVideo<N>, BtoCVideo<N>, QueueBVideo, RoleB<RoleEnd>>),
}
enum Branche0CtoA<N: marker::Send> {
    End(SessionMpst<CtoAClose, CtoBClose, QueueCEnd, RoleC<RoleEnd>>),
    Video(SessionMpst<RecursCtoA<N>, CtoBVideo<N>, QueueCVideo, RoleC<RoleEnd>>),
}
type ChooseAforBtoA<N> = Send<Branche0BtoA<N>, End>;
type ChooseAforCtoA<N> = Send<Branche0CtoA<N>, End>;

type InitA<N> = Send<N, Recv<N, ChooseAforBtoA<N>>>;

/// Queues
type QueueBEnd = RoleEnd;
type QueueBVideo = RoleA<RoleC<RoleC<RoleA<RoleA<RoleEnd>>>>>;
type QueueBRecurs = RoleA<RoleEnd>;
type QueueBInit = RoleA<RoleA<RoleA<RoleEnd>>>;

type QueueCEnd = RoleEnd;
type QueueCVideo = RoleB<RoleB<RoleA<RoleEnd>>>;
type QueueCRecurs = RoleA<RoleEnd>;

type QueueARecurs = RoleB<RoleC<RoleEnd>>;
type QueueAFull = RoleB<RoleB<QueueARecurs>>;

/// Creating the MP sessions

/// For B
type EndpointAEnd = SessionMpst<End, End, RoleEnd, RoleA<RoleEnd>>;
type EndpointAVideo<N> = SessionMpst<
    Send<N, Recv<N, Send<Branche0BtoA<N>, End>>>,
    Send<Branche0CtoA<N>, End>,
    RoleB<RoleB<RoleB<RoleC<RoleEnd>>>>,
    RoleA<RoleEnd>,
>;
type EndpointARecurs<N> =
    SessionMpst<ChooseAforBtoA<N>, ChooseAforCtoA<N>, QueueARecurs, RoleA<RoleEnd>>;
type EndpointAFull<N> = SessionMpst<InitA<N>, ChooseAforCtoA<N>, QueueAFull, RoleA<RoleEnd>>;

/// For C
type EndpointBRecurs<N> = SessionMpst<RecursBtoA<N>, End, QueueBRecurs, RoleB<RoleEnd>>;
type EndpointBFull<N> = SessionMpst<InitB<N>, End, QueueBInit, RoleB<RoleEnd>>;

/// For A
type EndpointCRecurs<N> = SessionMpst<RecursCtoA<N>, End, QueueCRecurs, RoleC<RoleEnd>>;

/// Functions related to endpoints
fn server(s: EndpointCRecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_c_to_a!(s, {
        Branche0CtoA::End(s) => {
            close_mpst(s)?;
            Ok(())
        },
        Branche0CtoA::Video(s) => {
            let (request, s) = recv_mpst_c_to_b(s)?;
            let s = send_mpst_c_to_b(request + 1, s);
            server(s)
        },
    })?;
    Ok(())
}

fn authenticator(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_b_to_a(s)?;
    let s = send_mpst_b_to_a(id + 1, s);

    let result = authenticator_recurs(s)?;

    Ok(result)
}

fn authenticator_recurs(s: EndpointBRecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_b_to_a!(s, {
        Branche0BtoA::End(s) => {
            close_mpst(s)?;
            Ok(())
        },
        Branche0BtoA::Video(s) => {
            let (request, s) = recv_mpst_b_to_a(s)?;
            let s = send_mpst_b_to_c(request + 1, s);
            let (video, s) = recv_mpst_b_to_c(s)?;
            let s = send_mpst_b_to_a(video + 1, s);
            authenticator_recurs(s)
        },
    })?;
    Ok(())
}

fn client(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();

    let s = send_mpst_a_to_b(0, s);
    let (_, s) = recv_mpst_a_to_b(s)?;

    let result = client_recurs(s, xs, 1)?;

    Ok(result)
}

fn client_recurs(
    s: EndpointARecurs<i32>,
    mut xs: Vec<i32>,
    index: i32,
) -> Result<(), Box<dyn Error>> {
    match xs.pop() {
        Option::Some(_) => {
            let s = choose_mpst_a_to_all!(s, Branche0BtoA::Video, Branche0CtoA::Video);

            let s = send_mpst_a_to_b(1, s);
            let (_, s) = recv_mpst_a_to_b(s)?;

            client_recurs(s, xs, index + 1)
        }
        Option::None => {
            let s = choose_mpst_a_to_all!(s, Branche0BtoA::End, Branche0CtoA::End);

            close_mpst(s)?;

            assert_eq!(index, 100);

            Ok(())
        }
    }
}

///////////////////////////////////////// Need a refactoring to be included in macro

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

impl<N: marker::Send> fmt::Display for Branche0BtoA<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Branche0BtoA::Video(s) => write!(f, "Video:{}", type_of(&s)),
            Branche0BtoA::End(s) => write!(f, "End:{}", type_of(&s)),
        }
    }
}

fn hashmap_branche_0_b_to_a() -> Vec<String> {
    let (s_video, _) = <_ as Session>::new();

    let video = Branche0BtoA::Video::<i32>(s_video);

    println!("Type of video for C {}", type_of(&video));

    let (s_end, _) = <_ as Session>::new();

    let end = Branche0BtoA::End::<i32>(s_end);

    println!("Type of end for C {}", type_of(&end));

    vec![(&video).to_string(), (&end).to_string()]
}

impl<N: marker::Send> fmt::Display for Branche0CtoA<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Branche0CtoA::Video(s) => write!(f, "Video:{}", type_of(&s)),
            Branche0CtoA::End(s) => write!(f, "End:{}", type_of(&s)),
        }
    }
}

fn hashmap_branche_0_c_to_a() -> Vec<String> {
    let (s_video, _) = <_ as Session>::new();

    let video = Branche0CtoA::Video::<i32>(s_video);

    println!("Type of video for A {}", type_of(&video));

    let (s_end, _) = <_ as Session>::new();

    let end = Branche0CtoA::End::<i32>(s_end);

    println!("Type of end for A {}", type_of(&end));

    vec![(&video).to_string(), (&end).to_string()]
}

/////////////////////////////////////////

#[test]
fn run_b_usecase_recursive() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(client, authenticator, server);

            assert!(thread_a.is_ok());
            assert!(thread_b.is_ok());
            assert!(thread_c.is_ok());
        }
        Ok(())
    }()
    .is_ok());
}

#[test]
fn run_b_usecase_recursive_checker() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            // Get the new sessionmpst of the passive roles
            let state_branches_receivers = RandomState::new();
            let mut branches_receivers: HashMap<String, &Vec<String>> =
                HashMap::with_hasher(state_branches_receivers);

            let branche_0_c_to_a: Vec<String> = hashmap_branche_0_c_to_a();
            let branche_0_b_to_a: Vec<String> = hashmap_branche_0_b_to_a();

            branches_receivers.insert(String::from("Branche0CtoA<i32>"), &branche_0_c_to_a);
            branches_receivers.insert(String::from("Branche0BtoA<i32>"), &branche_0_b_to_a);

            let (s1, _): (EndpointAFull<i32>, _) = SessionMpst::new();
            let (s2, _): (EndpointBFull<i32>, _) = SessionMpst::new();
            let (s3, _): (EndpointCRecurs<i32>, _) = SessionMpst::new();

            // Get the new stack of the active role
            let state_branches_sender = RandomState::new();
            let mut branches_sender: HashMap<String, &Vec<String>> =
                HashMap::with_hasher(state_branches_sender);

            let (s_video, _): (EndpointARecurs<i32>, _) = SessionMpst::new();
            let s_video: EndpointAVideo<i32> =
                choose_mpst_a_to_all!(s_video, Branche0BtoA::Video, Branche0CtoA::Video);
            let (s_end, _): (EndpointARecurs<i32>, _) = SessionMpst::new();
            let s_end: EndpointAEnd =
                choose_mpst_a_to_all!(s_end, Branche0BtoA::End, Branche0CtoA::End);

            let mut stacks: Vec<String> = Vec::new();
            stacks.push(type_of(&s_video.stack).to_string());
            stacks.push(type_of(&s_end.stack).to_string());

            branches_sender.insert(String::from("Branche0CtoA<i32>"), &stacks);
            branches_sender.insert(String::from("Branche0BtoA<i32>"), &stacks);

            let (a, b, c) = checker(s1, s2, s3, &branches_receivers, &branches_sender)?;

            assert_eq!(a, "A: A!B.A?B.µX( A!B.A?B.X + 0 )");
            assert_eq!(b, "B: B?A.B!A.µX( B?A.B!C.B?C.B!A.X & 0 )");
            assert_eq!(c, "C: µX( C?B.C!B.X & 0 )");
        }
        Ok(())
    }()
    .is_ok());
}
