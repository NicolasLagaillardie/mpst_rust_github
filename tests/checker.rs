extern crate mpstthree;
use mpstthree::checking::checker;

use mpstthree::binary::{End, Recv, Send, Session};
use mpstthree::sessionmpst::SessionMpst;

use std::any::type_name;
use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::marker;

use mpstthree::role::a_to_b::RoleAtoB;
use mpstthree::role::a_to_c::RoleAtoC;
use mpstthree::role::b_to_a::RoleBtoA;
use mpstthree::role::b_to_c::RoleBtoC;
use mpstthree::role::c_to_a::RoleCtoA;
use mpstthree::role::c_to_b::RoleCtoB;
use mpstthree::role::end::RoleEnd;

use mpstthree::role::Role;

/// Test our usecase from Places 2020
/// Simple types
/// Client = C
/// Authenticator = A
/// Server = B

/// A: A?C.A!C.μX.( 0 & A?C.A?C.A!B.A?B.A!C.X )
/// B: μX.( 0 & B?A.B!A.X )
/// C: C!A.C?A.μX.( 0 + C!A.C?A.X )

type AtoCClose = End;
type AtoBClose = End;
type AtoBVideo<N> = Send<N, Recv<N, End>>;

type InitA<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type BtoAClose = <AtoBClose as Session>::Dual;
type BtoCClose = End;
type BtoAVideo<N> = <AtoBVideo<N> as Session>::Dual;

type RecursAtoC<N> = Recv<CBranchesAtoC<N>, End>;
type RecursBtoC<N> = Recv<CBranchesBtoC<N>, End>;

enum CBranchesAtoC<N: marker::Send> {
    End(SessionMpst<AtoBClose, AtoCClose, QueueAEnd>),
    Video(SessionMpst<AtoBVideo<N>, InitA<N>, QueueAVideo>),
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
type QueueAInit = RoleAtoC<RoleAtoC<RoleAtoC<RoleEnd>>>;

type QueueBEnd = RoleEnd;
type QueueBVideo = RoleBtoA<RoleBtoA<RoleBtoC<RoleEnd>>>;
type QueueBRecurs = RoleBtoC<RoleEnd>;

type QueueCRecurs = RoleCtoA<RoleCtoB<RoleEnd>>;
type QueueCFull = RoleCtoA<RoleCtoA<QueueCRecurs>>;

/// Creating the MP sessions
/// For C
type EndpointCFull<N> = SessionMpst<InitC<N>, ChooseCforBtoC<N>, QueueCFull>;

/// For A
type EndpointAFull<N> = SessionMpst<End, InitA<N>, QueueAInit>;

/// For B
type EndpointBRecurs<N> = SessionMpst<End, RecursBtoC<N>, QueueBRecurs>;

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

#[test]
fn test_checker() {
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
