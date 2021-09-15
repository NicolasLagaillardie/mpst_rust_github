// Test for Macro, exact same as usecase-recursive
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::functionmpst::close::close_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use std::error::Error;
use std::marker;

use rand::{thread_rng, Rng};

use mpstthree::{
    choose_mpst_to_all, create_multiple_normal_role, create_recv_mpst_session_1,
    create_recv_mpst_session_2, create_send_mpst_session_1, create_send_mpst_session_2, offer_mpst,
};

// Create new roles
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
    RoleC, RoleCDual |
);

// Create new send functions
create_send_mpst_session_1!(send_mpst_c_to_a, RoleA, RoleC);
create_send_mpst_session_2!(send_mpst_a_to_c, RoleC, RoleA);
create_send_mpst_session_2!(send_mpst_c_to_b, RoleB, RoleC);
create_send_mpst_session_1!(send_mpst_b_to_a, RoleA, RoleB);
create_send_mpst_session_1!(send_mpst_a_to_b, RoleB, RoleA);

// Create new recv functions and related types
create_recv_mpst_session_1!(recv_mpst_c_from_a, RoleA, RoleC);
create_recv_mpst_session_2!(recv_mpst_a_from_c, RoleC, RoleA);
create_recv_mpst_session_2!(recv_mpst_b_from_c, RoleC, RoleB);
create_recv_mpst_session_1!(recv_mpst_b_from_a, RoleA, RoleB);
create_recv_mpst_session_1!(recv_mpst_a_from_b, RoleB, RoleA);

// Types
type AtoBVideo<N> = Send<N, Recv<N, End>>;
type AtoCVideo<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type InitA<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type BtoAVideo<N> = <AtoBVideo<N> as Session>::Dual;

type RecursAtoC<N> = Recv<Branches0AtoC<N>, End>;
type RecursBtoC<N> = Recv<Branches0BtoC<N>, End>;

enum Branches0AtoC<N: marker::Send> {
    End(MeshedChannels<End, End, RoleEnd, RoleA<RoleEnd>>),
    Video(MeshedChannels<AtoBVideo<N>, AtoCVideo<N>, StackAVideo, RoleA<RoleEnd>>),
}
enum Branches0BtoC<N: marker::Send> {
    End(MeshedChannels<End, End, RoleEnd, RoleB<RoleEnd>>),
    Video(MeshedChannels<BtoAVideo<N>, RecursBtoC<N>, StackBVideo, RoleB<RoleEnd>>),
}
type Choose0fromCtoA<N> = Send<Branches0AtoC<N>, End>;
type Choose0fromCtoB<N> = Send<Branches0BtoC<N>, End>;

type InitC<N> = Send<N, Recv<N, Choose0fromCtoA<N>>>;

/// Stacks
type StackAVideo = RoleC<RoleB<RoleB<RoleC<RoleC<RoleEnd>>>>>;
type StackAInit = RoleC<RoleC<RoleC<RoleEnd>>>;

type StackBVideo = RoleA<RoleA<RoleC<RoleEnd>>>;

type StackCRecurs = RoleBroadcast;
type StackCFull = RoleA<RoleA<StackCRecurs>>;

/// Creating the MP sessions
/// For C

type EndpointCRecurs<N> =
    MeshedChannels<Choose0fromCtoA<N>, Choose0fromCtoB<N>, StackCRecurs, RoleC<RoleEnd>>;
type EndpointCFull<N> = MeshedChannels<InitC<N>, Choose0fromCtoB<N>, StackCFull, RoleC<RoleEnd>>;

/// For A
type EndpointARecurs<N> = MeshedChannels<End, RecursAtoC<N>, RoleC<RoleEnd>, RoleA<RoleEnd>>;
type EndpointAFull<N> = MeshedChannels<End, InitA<N>, StackAInit, RoleA<RoleEnd>>;

/// For B
type EndpointBFull<N> = MeshedChannels<End, RecursBtoC<N>, RoleC<RoleEnd>, RoleB<RoleEnd>>;

/// Functions related to endpoints
fn server(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_c, {
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
    println!("start auth");
    let (id, s) = recv_mpst_a_from_c(s)?;
    println!("recv auth");
    let s = send_mpst_a_to_c(id + 1, s);
    println!("send auth");

    authenticator_recurs(s)
}

fn authenticator_recurs(s: EndpointARecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_c, {
        Branches0AtoC::End(s) => {
            println!("close auth");
            close_mpst(s)
        },
        Branches0AtoC::Video(s) => {
            println!("video auth");
            let (request, s) = recv_mpst_a_from_c(s)?;
            println!("video recv auth");
            let s = send_mpst_a_to_b(request + 1, s);
            println!("video send auth");
            let (video, s) = recv_mpst_a_from_b(s)?;
            println!("video recv auth");
            let s = send_mpst_a_to_c(video + 1, s);
            println!("video send auth");
            authenticator_recurs(s)
        },
    })
}

fn client(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();

    println!("start client");
    let s = send_mpst_c_to_a(0, s);
    println!("send client");
    let (_, s) = recv_mpst_c_from_a(s)?;
    println!("recv client");

    client_recurs(s, xs, 1)
}

fn client_recurs(
    s: EndpointCRecurs<i32>,
    mut xs: Vec<i32>,
    index: i32,
) -> Result<(), Box<dyn Error>> {
    println!("index: {:?}", index);

    match xs.pop() {
        Option::Some(_) => {
            let s = choose_mpst_to_all!(
                s,
                Branches0AtoC::Video,
                Branches0BtoC::Video, =>
                RoleA,
                RoleB, =>
                RoleC
            );

            let s = send_mpst_c_to_a(1, s);
            let (_, s) = recv_mpst_c_from_a(s)?;

            client_recurs(s, xs, index + 1)
        }
        Option::None => {
            let s = choose_mpst_to_all!(
                s,
                Branches0AtoC::End,
                Branches0BtoC::End, =>
                RoleA,
                RoleB, =>
                RoleC
            );

            assert_eq!(index, 100);

            close_mpst(s)
        }
    }
}

async fn async_authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    async { authenticator(s) }.await
}

async fn async_server(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    async { server(s) }.await
}

async fn async_client(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    async { client(s) }.await
}

/////////////////////////////////////////

#[test]
pub fn interleaved_main() {
    let (channel_1_2, channel_2_1) = End::new();
    let (channel_1_3, channel_3_1) = InitA::<i32>::new();
    let (channel_2_3, channel_3_2) = RecursBtoC::<i32>::new();
    let (role_1, _) = StackAInit::new();
    let (role_2, _) = RoleC::<RoleEnd>::new();
    let (role_3, _) = StackCFull::new();
    let (name_1, _) = RoleA::<RoleEnd>::new();
    let (name_2, _) = RoleB::<RoleEnd>::new();
    let (name_3, _) = RoleC::<RoleEnd>::new();
    let meshedchannels_1 = MeshedChannels {
        session1: channel_1_2,
        session2: channel_1_3,
        stack: role_1,
        name: name_1,
    };
    let meshedchannels_2 = MeshedChannels {
        session1: channel_2_1,
        session2: channel_2_3,
        stack: role_2,
        name: name_2,
    };
    let meshedchannels_3 = MeshedChannels {
        session1: channel_3_1,
        session2: channel_3_2,
        stack: role_3,
        name: name_3,
    };

    futures::executor::block_on(async {
        futures::try_join!(
            async_authenticator(meshedchannels_1),
            async_server(meshedchannels_2),
            async_client(meshedchannels_3),
        )
        .unwrap();
    });
}
