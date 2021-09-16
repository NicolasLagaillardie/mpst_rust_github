// Test for Macro, exact same as usecase-recursive
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::interleaved::close::close_mpst_interleaved;
use mpstthree::interleaved::fork::fork_mpst_interleaved;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::error::Error;
use std::marker;

use rand::{thread_rng, Rng};

use mpstthree::{
    choose_mpst_to_all, create_multiple_normal_role, create_recv_mpst_session_1,
    create_recv_mpst_session_2, create_send_mpst_session_1, create_send_mpst_session_2,
    offer_mpst_interleaved,
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

// Stacks
type StackAVideo = RoleC<RoleB<RoleB<RoleC<RoleC<RoleEnd>>>>>;

type StackBVideo = RoleA<RoleA<RoleC<RoleEnd>>>;

// Creating the MP sessions
// For C
type EndpointCFour<N> = MeshedChannels<
    Send<N, Recv<N, Send<Branches0AtoC<N>, End>>>,
    Send<Branches0BtoC<N>, End>,
    RoleA<RoleA<RoleBroadcast>>,
    RoleC<RoleEnd>,
>;
type EndpointCThree<N> = MeshedChannels<
    Send<Branches0AtoC<N>, End>,
    Send<Branches0BtoC<N>, End>,
    RoleBroadcast,
    RoleC<RoleEnd>,
>;
type EndpointCOne<N> = MeshedChannels<
    Send<N, Recv<N, Send<Branches0AtoC<N>, End>>>,
    Send<Branches0BtoC<N>, End>,
    RoleA<RoleA<RoleBroadcast>>,
    RoleC<RoleEnd>,
>;

// For A
type EndpointAFour<N> = MeshedChannels<
    Send<N, Recv<N, End>>,
    Recv<N, Send<N, RecursAtoC<N>>>,
    RoleC<RoleB<RoleB<RoleC<RoleC<RoleEnd>>>>>,
    RoleA<RoleEnd>,
>;
type EndpointAThree<N> = MeshedChannels<End, RecursAtoC<N>, RoleC<RoleEnd>, RoleA<RoleEnd>>;
type EndpointAOne<N> = MeshedChannels<
    End,
    Recv<N, Send<N, RecursAtoC<N>>>,
    RoleC<RoleC<RoleC<RoleEnd>>>,
    RoleA<RoleEnd>,
>;

// For B
type EndpointBFour<N> = MeshedChannels<
    Recv<N, Send<N, End>>,
    Recv<Branches0BtoC<N>, End>,
    RoleA<RoleA<RoleC<RoleEnd>>>,
    RoleB<RoleEnd>,
>;
type EndpointBThree<N> =
    MeshedChannels<End, Recv<Branches0BtoC<N>, End>, RoleC<RoleEnd>, RoleB<RoleEnd>>;
type EndpointBOne<N> =
    MeshedChannels<End, Recv<Branches0BtoC<N>, End>, RoleC<RoleEnd>, RoleB<RoleEnd>>;

// Functions related to endpoints
fn step_one(
    s_a: EndpointAOne<i32>,
    s_b: EndpointBOne<i32>,
    s_c: EndpointCOne<i32>,
) -> Result<(), Box<dyn Error>> {
    let s_c = send_mpst_c_to_a(0, s_c);
    let (_, s_a) = recv_mpst_a_from_c(s_a)?;
    let s_a = send_mpst_a_to_c(1, s_a);
    let (_, s_c) = recv_mpst_c_from_a(s_c)?;

    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();

    step_two_recurs(s_a, s_b, s_c, xs, 1)
}

fn step_two_recurs(
    s_a: EndpointAThree<i32>,
    s_b: EndpointBThree<i32>,
    s_c: EndpointCThree<i32>,
    mut xs: Vec<i32>,
    index: i32,
) -> Result<(), Box<dyn Error>> {
    match xs.pop() {
        Option::Some(_) => {
            let s_c = choose_mpst_to_all!(
                s_c,
                Branches0AtoC::Video,
                Branches0BtoC::Video, =>
                RoleA,
                RoleB, =>
                RoleC
            );

            let (s_a, s_b) = offer_mpst_interleaved!(
                s_a,
                recv_mpst_a_from_c,
                Branches0AtoC::Video,
                s_b,
                recv_mpst_b_from_c,
                Branches0BtoC::Video
            );

            step_three_recurs(s_a, s_b, s_c, xs, index)
        }
        Option::None => {
            let s_c = choose_mpst_to_all!(
                s_c,
                Branches0AtoC::End,
                Branches0BtoC::End, =>
                RoleA,
                RoleB, =>
                RoleC
            );

            assert_eq!(index, 100);

            let (s_a, s_b) = offer_mpst_interleaved!(
                s_a,
                recv_mpst_a_from_c,
                Branches0AtoC::End,
                s_b,
                recv_mpst_b_from_c,
                Branches0BtoC::End
            );

            close_mpst_interleaved(s_a, s_b, s_c)
        }
    }
}

fn step_three_recurs(
    s_a: EndpointAFour<i32>,
    s_b: EndpointBFour<i32>,
    s_c: EndpointCFour<i32>,
    xs: Vec<i32>,
    index: i32,
) -> Result<(), Box<dyn Error>> {
    let s_c = send_mpst_c_to_a(1, s_c);
    let (request_a, s_a) = recv_mpst_a_from_c(s_a)?;
    let s_a = send_mpst_a_to_b(request_a + 1, s_a);
    let (request_b, s_b) = recv_mpst_b_from_a(s_b)?;
    let s_b = send_mpst_b_to_a(request_b + 1, s_b);
    let (video, s_a) = recv_mpst_a_from_b(s_a)?;
    let s_a = send_mpst_a_to_c(video + 1, s_a);
    let (_, s_c) = recv_mpst_c_from_a(s_c)?;

    step_two_recurs(s_a, s_b, s_c, xs, index + 1)
}

/////////////////////////////////////////

pub fn interleaved_main() {
    assert!(fork_mpst_interleaved(step_one).is_ok());
}
