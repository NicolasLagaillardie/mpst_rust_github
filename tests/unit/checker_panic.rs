use mpstthree::checking::checker;

use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::sessionmpst::SessionMpst;

use std::any::type_name;
use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::marker;

use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

use mpstthree::role::Role;

/// Test our usecase from Places 2020
/// Simple types
/// Client = C
/// Authenticator = A
/// Server = B

type AtoCClose = End;
type AtoBClose = End;
type AtoBVideo<N> = Send<N, Recv<N, End>>;

type InitA<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type BtoAClose = <AtoBClose as Session>::Dual;
type BtoCClose = End;
type BtoAVideo<N> = <AtoBVideo<N> as Session>::Dual;

type RecursAtoC<N> = Recv<Branche0AtoC<N>, End>;
type RecursBtoC<N> = Recv<Branche0BtoC<N>, End>;

enum Branche0AtoC<N: marker::Send> {
    End(SessionMpst<AtoBClose, AtoCClose, StackAEnd, RoleA<RoleEnd>>),
    Video(SessionMpst<AtoBVideo<N>, InitA<N>, StackAVideo, RoleA<RoleEnd>>),
}
enum Branche0BtoC<N: marker::Send> {
    End(SessionMpst<BtoAClose, BtoCClose, StackBEnd, RoleB<RoleEnd>>),
    Video(SessionMpst<BtoAVideo<N>, RecursBtoC<N>, StackBVideo, RoleB<RoleEnd>>),
}
type Choose0fromCtoA<N> = Send<Branche0AtoC<N>, End>;
type Choose0fromCtoB<N> = Send<Branche0BtoC<N>, End>;

type InitC<N> = Send<N, Recv<N, Choose0fromCtoA<N>>>;

/// Stacks
type StackAEnd = RoleEnd;
type StackAVideo = RoleC<RoleB<RoleB<RoleC<RoleC<RoleEnd>>>>>;
type StackAInit = RoleC<RoleC<RoleC<RoleEnd>>>;

type StackBEnd = RoleEnd;
type StackBVideo = RoleA<RoleA<RoleC<RoleEnd>>>;
type StackBRecurs = RoleC<RoleEnd>;

type StackCRecurs = RoleA<RoleB<RoleEnd>>;
type StackCFull = RoleA<RoleA<StackCRecurs>>;

/// Creating the MP sessions
/// For C
type EndpointCFull<N> = SessionMpst<InitC<N>, Choose0fromCtoB<N>, StackCFull, RoleC<RoleEnd>>;

/// For A
type EndpointAFull<N> = SessionMpst<End, InitA<N>, StackAInit, RoleA<RoleEnd>>;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

impl<N: marker::Send> fmt::Display for Branche0AtoC<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Branche0AtoC::Video(s) => {
                write!(f, "Video:{}", type_of(&s))
            }
            Branche0AtoC::End(s) => {
                write!(f, "End:{}", type_of(&s))
            }
        }
    }
}

impl<N: marker::Send> fmt::Display for Branche0BtoC<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Branche0BtoC::Video(s) => {
                write!(f, "Video:{}", type_of(&s))
            }
            Branche0BtoC::End(s) => {
                write!(f, "End:{}", type_of(&s))
            }
        }
    }
}

fn hashmap_c_branches_a_to_c() -> Vec<String> {
    let (channel_1_video, _) = <_ as Session>::new();
    let (channel_2_video, _) = <_ as Session>::new();
    let (role_video, _) = <_ as Role>::new();
    let (name_video, _) = <_ as Role>::new();

    let (channel_1_end, _) = <_ as Session>::new();
    let (channel_2_end, _) = <_ as Session>::new();
    let (role_end, _) = <_ as Role>::new();
    let (name_end, _) = <_ as Role>::new();

    let s_video = SessionMpst {
        session1: channel_1_video,
        session2: channel_2_video,
        stack: role_video,
        name: name_video,
    };

    let s_end = SessionMpst {
        session1: channel_1_end,
        session2: channel_2_end,
        stack: role_end,
        name: name_end,
    };

    let video = Branche0AtoC::<i32>::Video(s_video);
    let end = Branche0AtoC::<i32>::End(s_end);

    vec![
        String::from(&video.to_string()),
        String::from(&end.to_string()),
    ]
}

fn hashmap_c_branches_b_to_c() -> Vec<String> {
    let (channel_1_video, _) = <_ as Session>::new();
    let (channel_2_video, _) = <_ as Session>::new();
    let (role_video, _) = <_ as Role>::new();
    let (name_video, _) = <_ as Role>::new();

    let (channel_1_end, _) = <_ as Session>::new();
    let (channel_2_end, _) = <_ as Session>::new();
    let (role_end, _) = <_ as Role>::new();
    let (name_end, _) = <_ as Role>::new();

    let s_video = SessionMpst {
        session1: channel_1_video,
        session2: channel_2_video,
        stack: role_video,
        name: name_video,
    };

    let s_end = SessionMpst {
        session1: channel_1_end,
        session2: channel_2_end,
        stack: role_end,
        name: name_end,
    };

    let video = Branche0BtoC::<i32>::Video(s_video);
    let end = Branche0BtoC::<i32>::End(s_end);

    vec![
        String::from(&video.to_string()),
        String::from(&end.to_string()),
    ]
}

/////////////////////////////////////////

type EndpointBRecursPanicStack<N> = SessionMpst<End, RecursBtoC<N>, RoleA<RoleEnd>, RoleB<RoleEnd>>;

pub fn test_checker_panic_stack() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let mut hm: HashMap<String, &Vec<String>> = HashMap::new();

            let c_branches_a_to_c: Vec<String> = hashmap_c_branches_a_to_c();
            let c_branches_b_to_c: Vec<String> = hashmap_c_branches_b_to_c();

            hm.insert(String::from("Branche0AtoC<i32>"), &c_branches_a_to_c);
            hm.insert(String::from("Branche0BtoC<i32>"), &c_branches_b_to_c);

            let (s1, _): (EndpointAFull<i32>, _) = SessionMpst::new();
            let (s2, _): (EndpointBRecursPanicStack<i32>, _) = SessionMpst::new();
            let (s3, _): (EndpointCFull<i32>, _) = SessionMpst::new();

            checker(s1, s2, s3, &hm, &HashMap::new())?;
        }
        Ok(())
    }()
    .is_ok());
}

type EndpointBRecursPanicName<N> = SessionMpst<End, RecursBtoC<N>, StackBRecurs, RoleC<RoleEnd>>;

pub fn test_checker_panic_name() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let mut hm: HashMap<String, &Vec<String>> = HashMap::new();

            let c_branches_a_to_c: Vec<String> = hashmap_c_branches_a_to_c();
            let c_branches_b_to_c: Vec<String> = hashmap_c_branches_b_to_c();

            hm.insert(String::from("Branche0AtoC<i32>"), &c_branches_a_to_c);
            hm.insert(String::from("Branche0BtoC<i32>"), &c_branches_b_to_c);

            let (s1, _): (EndpointAFull<i32>, _) = SessionMpst::new();
            let (s2, _): (EndpointBRecursPanicName<i32>, _) = SessionMpst::new();
            let (s3, _): (EndpointCFull<i32>, _) = SessionMpst::new();

            checker(s1, s2, s3, &hm, &HashMap::new())?;
        }
        Ok(())
    }()
    .is_ok());
}
