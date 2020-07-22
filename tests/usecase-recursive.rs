extern crate mpstthree;
extern crate rand;

use rand::{thread_rng, Rng};

use mpstthree::binary::{cancel, End, Recv, Send, Session};
use mpstthree::fork_mpst;
use mpstthree::role::Role;
use mpstthree::sessionmpst::SessionMpst;

use std::any::type_name;
use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::marker;

use mpstthree::checking::checker;

use mpstthree::functionmpst::close::close_mpst;

use mpstthree::role::a_to_b::RoleAtoB;
use mpstthree::role::a_to_c::RoleAtoC;
use mpstthree::role::b_to_a::RoleBtoA;
use mpstthree::role::b_to_c::RoleBtoC;
use mpstthree::role::c_to_a::RoleCtoA;
use mpstthree::role::c_to_b::RoleCtoB;
use mpstthree::role::end::RoleEnd;

use mpstthree::functionmpst::recv::recv_mpst_a_to_b;
use mpstthree::functionmpst::recv::recv_mpst_a_to_c;
use mpstthree::functionmpst::recv::recv_mpst_b_to_a;
use mpstthree::functionmpst::recv::recv_mpst_b_to_c;
use mpstthree::functionmpst::recv::recv_mpst_c_to_a;

use mpstthree::functionmpst::send::send_mpst_a_to_b;
use mpstthree::functionmpst::send::send_mpst_a_to_c;
use mpstthree::functionmpst::send::send_mpst_b_to_a;
use mpstthree::functionmpst::send::send_mpst_c_to_a;
use mpstthree::functionmpst::send::send_mpst_c_to_b;

use mpstthree::choose_mpst_c_to_all;
use mpstthree::offer_mpst_a_to_c;
use mpstthree::offer_mpst_b_to_c;

/// Test our usecase from Places 2020
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

type RecursAtoC<N> = Recv<CBranchesAtoC<N>, End>;
type RecursBtoC<N> = Recv<CBranchesBtoC<N>, End>;

enum CBranchesAtoC<N: marker::Send> {
    End(SessionMpst<AtoBClose, AtoCClose, QueueAEnd>),
    Video(SessionMpst<AtoBVideo<N>, AtoCVideo<N>, QueueAVideo>),
}
enum CBranchesBtoC<N: marker::Send> {
    End(SessionMpst<BtoAClose, BtoCClose, QueueBEnd>),
    Video(SessionMpst<BtoAVideo<N>, RecursBtoC<N>, QueueBVideo>),
}
type ChooseCforAtoC<N> = Send<CBranchesAtoC<N>, End>;
type ChooseCforBtoC<N> = Send<CBranchesBtoC<N>, End>;

type InitC<N> = Send<N, Recv<N, ChooseCforAtoC<N>>>;

/// Queues
type QueueAEnd = RoleEnd;
type QueueAVideo = RoleAtoC<RoleAtoB<RoleAtoB<RoleAtoC<RoleAtoC<RoleEnd>>>>>;
type QueueARecurs = RoleAtoC<RoleEnd>;
type QueueAInit = RoleAtoC<RoleAtoC<RoleAtoC<RoleEnd>>>;

type QueueBEnd = RoleEnd;
type QueueBVideo = RoleBtoA<RoleBtoA<RoleBtoC<RoleEnd>>>;
type QueueBRecurs = RoleBtoC<RoleEnd>;

type QueueCRecurs = RoleCtoA<RoleCtoB<RoleEnd>>;
type QueueCFull = RoleCtoA<RoleCtoA<QueueCRecurs>>;

/// Creating the MP sessions
/// For C

type EndpointCRecurs<N> = SessionMpst<ChooseCforAtoC<N>, ChooseCforBtoC<N>, QueueCRecurs>;
type EndpointCFull<N> = SessionMpst<InitC<N>, ChooseCforBtoC<N>, QueueCFull>;

/// For A
type EndpointARecurs<N> = SessionMpst<End, RecursAtoC<N>, QueueARecurs>;
type EndpointAFull<N> = SessionMpst<End, InitA<N>, QueueAInit>;

/// For B
type EndpointBRecurs<N> = SessionMpst<End, RecursBtoC<N>, QueueBRecurs>;

/// Functions related to endpoints
fn server(s: EndpointBRecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_b_to_c!(s, {
        CBranchesBtoC::End(s) => {
            close_mpst(s)?;
            Ok(())
        },
        CBranchesBtoC::Video(s) => {
            let (request, s) = recv_mpst_b_to_a(s)?;
            let s = send_mpst_b_to_a(request + 1, s);
            server(s)
        },
    })?;
    Ok(())
}

fn authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_a_to_c(s)?;
    let s = send_mpst_a_to_c(id + 1, s);

    let result = authenticator_recurs(s)?;

    Ok(result)
}

fn authenticator_recurs(s: EndpointARecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_a_to_c!(s, {
        CBranchesAtoC::End(s) => {
            close_mpst(s)?;
            Ok(())
        },
        CBranchesAtoC::Video(s) => {
            let (request, s) = recv_mpst_a_to_c(s)?;
            let s = send_mpst_a_to_b(request + 1, s);
            let (video, s) = recv_mpst_a_to_b(s)?;
            let s = send_mpst_a_to_c(video + 1, s);
            authenticator_recurs(s)
        },
    })?;
    Ok(())
}

fn client(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();

    let s = send_mpst_c_to_a(0, s);
    let (_, s) = recv_mpst_c_to_a(s)?;

    let result = client_recurs(s, xs, 1)?;

    Ok(result)
}

fn client_recurs(
    s: EndpointCRecurs<i32>,
    mut xs: Vec<i32>,
    index: i32,
) -> Result<(), Box<dyn Error>> {
    match xs.pop() {
        Option::Some(_) => {
            let s = choose_mpst_c_to_all!(CBranchesAtoC::Video, CBranchesBtoC::Video, s);

            let s = send_mpst_c_to_a(1, s);
            let (_, s) = recv_mpst_c_to_a(s)?;

            client_recurs(s, xs, index + 1)
        }
        Option::None => {
            let s = choose_mpst_c_to_all!(CBranchesAtoC::End, CBranchesBtoC::End, s);

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

impl<N: marker::Send> fmt::Display for CBranchesAtoC<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CBranchesAtoC::Video(s) => write!(f, "Video:{}", type_of(&s)),
            CBranchesAtoC::End(s) => write!(f, "End:{}", type_of(&s)),
        }
    }
}

impl<N: marker::Send> fmt::Display for CBranchesBtoC<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CBranchesBtoC::Video(s) => write!(f, "Video:{}", type_of(&s)),
            CBranchesBtoC::End(s) => write!(f, "End:{}", type_of(&s)),
        }
    }
}

fn hashmap_c_branches_a_to_c() -> Vec<String> {
    let (channel_1_video, _) = <_ as Session>::new();
    let (channel_2_video, _) = <_ as Session>::new();
    let (role_video, _) = <_ as Role>::new();

    let (channel_1_end, _) = <_ as Session>::new();
    let (channel_2_end, _) = <_ as Session>::new();
    let (role_end, _) = <_ as Role>::new();

    let s_video = SessionMpst {
        session1: channel_1_video,
        session2: channel_2_video,
        stack: role_video,
    };

    let s_end = SessionMpst {
        session1: channel_1_end,
        session2: channel_2_end,
        stack: role_end,
    };

    let video = CBranchesAtoC::<i32>::Video(s_video);
    let end = CBranchesAtoC::<i32>::End(s_end);

    vec![
        String::from(&video.to_string()),
        String::from(&end.to_string()),
    ]
}

fn hashmap_c_branches_b_to_c() -> Vec<String> {
    let (channel_1_video, _) = <_ as Session>::new();
    let (channel_2_video, _) = <_ as Session>::new();
    let (role_video, _) = <_ as Role>::new();

    let (channel_1_end, _) = <_ as Session>::new();
    let (channel_2_end, _) = <_ as Session>::new();
    let (role_end, _) = <_ as Role>::new();

    let s_video = SessionMpst {
        session1: channel_1_video,
        session2: channel_2_video,
        stack: role_video,
    };

    let s_end = SessionMpst {
        session1: channel_1_end,
        session2: channel_2_end,
        stack: role_end,
    };

    let video = CBranchesBtoC::<i32>::Video(s_video);
    let end = CBranchesBtoC::<i32>::End(s_end);

    vec![
        String::from(&video.to_string()),
        String::from(&end.to_string()),
    ]
}

/////////////////////////////////////////

#[test]
fn run_usecase_recursive() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client);

            assert!(thread_a.is_ok());
            assert!(thread_b.is_ok());
            assert!(thread_c.is_ok());
        }
        Ok(())
    }()
    .is_ok());

    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let mut hm: HashMap<String, &Vec<String>> = HashMap::new();

            let c_branches_a_to_c: Vec<String> = hashmap_c_branches_a_to_c();
            let c_branches_b_to_c: Vec<String> = hashmap_c_branches_b_to_c();

            hm.insert(String::from("CBranchesAtoC<i32>"), &c_branches_a_to_c);
            hm.insert(String::from("CBranchesBtoC<i32>"), &c_branches_b_to_c);

            let (s1, _): (EndpointAFull<i32>, _) = SessionMpst::new();
            let (s2, _): (EndpointBRecurs<i32>, _) = SessionMpst::new();
            let (s3, _): (EndpointCFull<i32>, _) = SessionMpst::new();

            let (a, b, c) = checker(s1, s2, s3, &hm)?;

            assert_eq!(a, "A: A?C.A!C.µX( A?C.A!B.A?B.A!C.X & 0 )");
            assert_eq!(b, "B: µX( B?A.B!A.X & 0 )");
            assert_eq!(c, "C: C!A.C?A.µX( A?C.A!B.A?B.A!C.X + 0 )");
        }
        Ok(())
    }()
    .is_ok());
}
