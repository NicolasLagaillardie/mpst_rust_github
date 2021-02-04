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
use mpstthree::functionmpst::recv::recv_mpst_a_to_c;
use mpstthree::functionmpst::recv::recv_mpst_b_to_c;
use mpstthree::functionmpst::recv::recv_mpst_c_to_a;
use mpstthree::functionmpst::recv::recv_mpst_c_to_b;

// Get send functions
use mpstthree::functionmpst::send::send_mpst_a_to_c;
use mpstthree::functionmpst::send::send_mpst_b_to_c;
use mpstthree::functionmpst::send::send_mpst_c_to_a;
use mpstthree::functionmpst::send::send_mpst_c_to_b;

use mpstthree::choose_mpst_b_to_all;
use mpstthree::offer_mpst_a_to_b;
use mpstthree::offer_mpst_c_to_b;

/// Test our usecase
/// Simple types
/// Client = B
/// Authenticator = C
/// Server = A

type CtoBClose = End;
type CtoAClose = End;
type CtoAVideo<N> = Send<N, Recv<N, End>>;
type CtoBVideo<N> = Recv<N, Send<N, RecursCtoB<N>>>;

type InitC<N> = Recv<N, Send<N, RecursCtoB<N>>>;

type AtoCClose = <CtoAClose as Session>::Dual;
type AtoBClose = End;
type AtoCVideo<N> = <CtoAVideo<N> as Session>::Dual;

type RecursCtoB<N> = Recv<Branches0CtoB<N>, End>;
type RecursAtoB<N> = Recv<Branches0AtoB<N>, End>;

enum Branches0CtoB<N: marker::Send> {
    End(SessionMpst<CtoAClose, CtoBClose, QueueCEnd, RoleC<RoleEnd>>),
    Video(SessionMpst<CtoAVideo<N>, CtoBVideo<N>, QueueCVideo, RoleC<RoleEnd>>),
}
enum Branches0AtoB<N: marker::Send> {
    End(SessionMpst<AtoBClose, AtoCClose, QueueAEnd, RoleA<RoleEnd>>),
    Video(SessionMpst<RecursAtoB<N>, AtoCVideo<N>, QueueAVideo, RoleA<RoleEnd>>),
}
type ChooseBforCtoB<N> = Send<Branches0CtoB<N>, End>;
type ChooseBforAtoB<N> = Send<Branches0AtoB<N>, End>;

type InitB<N> = Send<N, Recv<N, ChooseBforCtoB<N>>>;

/// Queues
type QueueCEnd = RoleEnd;
type QueueCVideo = RoleB<RoleA<RoleA<RoleB<RoleB<RoleEnd>>>>>;
type QueueCRecurs = RoleB<RoleEnd>;
type QueueCInit = RoleB<RoleB<RoleB<RoleEnd>>>;

type QueueAEnd = RoleEnd;
type QueueAVideo = RoleC<RoleC<RoleB<RoleEnd>>>;
type QueueARecurs = RoleB<RoleEnd>;

type QueueBRecurs = RoleA<RoleC<RoleEnd>>;
type QueueBFull = RoleC<RoleC<QueueBRecurs>>;

/// Creating the MP sessions

/// For B
type EndpointBRecurs<N> =
    SessionMpst<ChooseBforAtoB<N>, ChooseBforCtoB<N>, QueueBRecurs, RoleB<RoleEnd>>;
type EndpointBFull<N> = SessionMpst<ChooseBforAtoB<N>, InitB<N>, QueueBFull, RoleB<RoleEnd>>;

/// For C
type EndpointCRecurs<N> = SessionMpst<End, RecursCtoB<N>, QueueCRecurs, RoleC<RoleEnd>>;
type EndpointCFull<N> = SessionMpst<End, InitC<N>, QueueCInit, RoleC<RoleEnd>>;

/// For A
type EndpointARecurs<N> = SessionMpst<RecursAtoB<N>, End, QueueARecurs, RoleA<RoleEnd>>;

/// Functions related to endpoints
fn server(s: EndpointARecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_a_to_b!(s, {
        Branches0AtoB::End(s) => {
            close_mpst(s)?;
            Ok(())
        },
        Branches0AtoB::Video(s) => {
            let (request, s) = recv_mpst_a_to_c(s)?;
            let s = send_mpst_a_to_c(request + 1, s);
            server(s)
        },
    })
}

fn authenticator(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_c_to_b(s)?;
    let s = send_mpst_c_to_b(id + 1, s);

    authenticator_recurs(s)
}

fn authenticator_recurs(s: EndpointCRecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_c_to_b!(s, {
        Branches0CtoB::End(s) => {
            close_mpst(s)
        },
        Branches0CtoB::Video(s) => {
            let (request, s) = recv_mpst_c_to_b(s)?;
            let s = send_mpst_c_to_a(request + 1, s);
            let (video, s) = recv_mpst_c_to_a(s)?;
            let s = send_mpst_c_to_b(video + 1, s);
            authenticator_recurs(s)
        },
    })
}

fn client(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();

    let s = send_mpst_b_to_c(0, s);
    let (_, s) = recv_mpst_b_to_c(s)?;

    client_recurs(s, xs, 1)
}

fn client_recurs(
    s: EndpointBRecurs<i32>,
    mut xs: Vec<i32>,
    index: i32,
) -> Result<(), Box<dyn Error>> {
    match xs.pop() {
        Option::Some(_) => {
            let s = choose_mpst_b_to_all!(s, Branches0AtoB::Video, Branches0CtoB::Video);

            let s = send_mpst_b_to_c(1, s);
            let (_, s) = recv_mpst_b_to_c(s)?;

            client_recurs(s, xs, index + 1)
        }
        Option::None => {
            let s = choose_mpst_b_to_all!(s, Branches0AtoB::End, Branches0CtoB::End);

            assert_eq!(index, 100);

            close_mpst(s)
        }
    }
}

///////////////////////////////////////// Need a refactoring to be included in macro

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

impl<N: marker::Send> fmt::Display for Branches0CtoB<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Branches0CtoB::Video(s) => write!(f, "Video:{}", type_of(&s)),
            Branches0CtoB::End(s) => write!(f, "End:{}", type_of(&s)),
        }
    }
}

fn hashmap_branche_0_c_to_b() -> Vec<String> {
    let (s_video, _) = <_ as Session>::new();

    let video = Branches0CtoB::Video::<i32>(s_video);

    let (s_end, _) = <_ as Session>::new();

    let end = Branches0CtoB::End::<i32>(s_end);

    vec![(&video).to_string(), (&end).to_string()]
}

impl<N: marker::Send> fmt::Display for Branches0AtoB<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Branches0AtoB::Video(s) => write!(f, "Video:{}", type_of(&s)),
            Branches0AtoB::End(s) => write!(f, "End:{}", type_of(&s)),
        }
    }
}

fn hashmap_branche_0_a_to_b() -> Vec<String> {
    let (s_video, _) = <_ as Session>::new();

    let video = Branches0AtoB::Video::<i32>(s_video);

    let (s_end, _) = <_ as Session>::new();

    let end = Branches0AtoB::End::<i32>(s_end);

    vec![(&video).to_string(), (&end).to_string()]
}

/////////////////////////////////////////

fn run_a_usecase_recursive() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(server, client, authenticator);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}

type QueueBEnd = RoleEnd;
type QueueBVideo = RoleC<RoleC<RoleA<RoleC<RoleEnd>>>>;

fn run_a_usecase_recursive_checker() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            // Get the new sessionmpst of the passive roles
            let state_branches_receivers = RandomState::new();
            let mut branches_receivers: HashMap<String, &Vec<String>> =
                HashMap::with_hasher(state_branches_receivers);

            let branche_0_a_to_b: Vec<String> = hashmap_branche_0_a_to_b();
            let branche_0_c_to_b: Vec<String> = hashmap_branche_0_c_to_b();

            branches_receivers.insert(String::from("Branches0AtoB<i32>"), &branche_0_a_to_b);
            branches_receivers.insert(String::from("Branches0CtoB<i32>"), &branche_0_c_to_b);

            let (s1, _): (EndpointARecurs<i32>, _) = SessionMpst::new();
            let (s2, _): (EndpointBFull<i32>, _) = SessionMpst::new();
            let (s3, _): (EndpointCFull<i32>, _) = SessionMpst::new();

            // Get the new stack of the active role
            let state_branches_sender = RandomState::new();
            let mut branches_sender: HashMap<String, &Vec<String>> =
                HashMap::with_hasher(state_branches_sender);

            let (stack_video, _): (QueueBVideo, _) = Role::new();
            let (stack_end, _): (QueueBEnd, _) = Role::new();

            let mut stacks: Vec<String> = Vec::new();
            stacks.push(type_of(&stack_video).to_string());
            stacks.push(type_of(&stack_end).to_string());

            branches_sender.insert(String::from("Branches0AtoB<i32>"), &stacks);
            branches_sender.insert(String::from("Branches0CtoB<i32>"), &stacks);

            let (a, b, c) = checker(s1, s2, s3, &branches_receivers, &branches_sender)?;

            assert_eq!(a, "A: µX( A?C.A!C.X & 0 )");
            assert_eq!(b, "B: B!C.B?C.µX( B!C.B?C.X + 0 )");
            assert_eq!(c, "C: C?B.C!B.µX( C?B.C!A.C?A.C!B.X & 0 )");
        }
        Ok(())
    }()
    .is_ok());
}

/////////////////////////////////////////

fn main() {
    run_a_usecase_recursive();
    run_a_usecase_recursive_checker();
}
