use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, create_fn_choose_mpst_multi_to_all_bundle,
    create_multiple_normal_role_short, create_recv_http_session_bundle,
    create_send_mpst_http_bundle, offer_http_mpst,
};

use hyper::Request;

use rand::{thread_rng, Rng};

use std::error::Error;
use std::marker;

// See the folder scribble_protocols for the related Scribble protocol

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsThree, 3);

// Create new roles
// normal
create_multiple_normal_role_short!(A, C, S);

// Create new send functions
// A
create_send_mpst_http_bundle!(
    send_http_a_to_c, RoleC, 1 |
    send_http_a_to_s, RoleS, 2 | =>
    RoleA, MeshedChannelsThree, 3
);
// C
create_send_mpst_http_bundle!(
    send_http_c_to_a, RoleA, 1 |
    send_http_c_to_s, RoleS, 2 | =>
    RoleC, MeshedChannelsThree, 3
);
// S
create_send_mpst_http_bundle!(
    send_http_s_to_a, RoleA, 1 |
    send_http_s_to_c, RoleC, 2 | =>
    RoleS, MeshedChannelsThree, 3
);

// Create new recv functions and related types
// A
create_recv_http_session_bundle!(
    recv_http_a_to_c, RoleC, 1 |
    recv_http_a_to_s, RoleS, 2 | =>
    RoleA, MeshedChannelsThree, 3
);
// C
create_recv_http_session_bundle!(
    recv_http_c_to_a, RoleA, 1 |
    recv_http_c_to_s, RoleS, 2 | =>
    RoleC, MeshedChannelsThree, 3
);
// S
create_recv_http_session_bundle!(
    recv_http_s_to_a, RoleA, 1 |
    recv_http_s_to_c, RoleC, 2 | =>
    RoleS, MeshedChannelsThree, 3
);

// Names
type NameA = RoleA<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameS = RoleS<RoleEnd>;

// Types
// S
type Choose2fromStoA<N> = Send<Branching2fromStoA<N>, End>;
type Choose2fromStoC<N> = Send<Branching2fromStoC<N>, End>;

type Choice0fromAtoS<N> = <Choose0fromAtoS<N> as Session>::Dual;
type Choice1fromCtoS<N> = <Choose1fromCtoS<N> as Session>::Dual;
// C
type Choose1fromCtoA<N> = Send<Branching1fromCtoA<N>, End>;
type Choose1fromCtoS<N> = Send<Branching1fromCtoS<N>, End>;

type Choice0fromAtoC<N> = <Choose0fromAtoC<N> as Session>::Dual;
type Choice2fromStoC<N> = <Choose2fromStoC<N> as Session>::Dual;
// A
type Choose0fromAtoC<N> = Send<Branching0fromAtoC<N>, End>;
type Choose0fromAtoS<N> = Send<Branching0fromAtoS<N>, End>;

type Choice1fromCtoA<N> = <Choose1fromCtoA<N> as Session>::Dual;
type Choice2fromStoA<N> = <Choose2fromStoA<N> as Session>::Dual;

// A

type EndpointAAuth<N> =
    MeshedChannelsThree<Send<N, Choice1fromCtoA<N>>, End, RoleC<RoleC<RoleEnd>>, NameA>;

type EndpointAAuthLoop<N> = MeshedChannelsThree<Choice1fromCtoA<N>, End, RoleC<RoleEnd>, NameA>;

type EndpointADone<N> = MeshedChannelsThree<Send<N, End>, End, RoleC<RoleEnd>, NameA>;

enum Branching1fromCtoA<N: marker::Send> {
    Continue(
        MeshedChannelsThree<
            End,
            Recv<N, Send<N, Choice2fromStoA<N>>>,
            RoleS<RoleS<RoleS<RoleEnd>>>,
            NameA,
        >,
    ),
    Close(MeshedChannelsThree<End, Recv<N, End>, RoleS<RoleEnd>, NameA>),
}

type EndpointAContinue<N> = MeshedChannelsThree<End, Choice2fromStoA<N>, RoleS<RoleEnd>, NameA>;

enum Branching2fromStoA<N: marker::Send> {
    Picture(MeshedChannelsThree<Choice1fromCtoA<N>, End, RoleC<RoleEnd>, NameA>),
    Refusal(MeshedChannelsThree<Choice1fromCtoA<N>, End, RoleC<RoleEnd>, NameA>),
}

// C
enum Branching0fromAtoC<N: marker::Send> {
    Auth(
        MeshedChannelsThree<
            Recv<N, Choose1fromCtoA<N>>,
            Choose1fromCtoS<N>,
            RoleA<RoleBroadcast>,
            NameC,
        >,
    ),
    Done(MeshedChannelsThree<Recv<N, End>, Send<N, End>, RoleA<RoleS<RoleEnd>>, NameC>),
}

type EndpointCContinue<N> =
    MeshedChannelsThree<End, Send<N, Choice2fromStoC<N>>, RoleS<RoleS<RoleEnd>>, NameC>;

type EndpointCContinueLoop<N> =
    MeshedChannelsThree<Choose1fromCtoA<N>, Choose1fromCtoS<N>, RoleBroadcast, NameC>;

type EndpointCDone<N> = MeshedChannelsThree<End, Send<N, End>, RoleS<RoleEnd>, NameC>;

enum Branching2fromStoC<N: marker::Send> {
    Picture(
        MeshedChannelsThree<
            Choose1fromCtoA<N>,
            Recv<N, Choose1fromCtoS<N>>,
            RoleS<RoleBroadcast>,
            NameC,
        >,
    ),
    Refusal(
        MeshedChannelsThree<
            Choose1fromCtoA<N>,
            Recv<N, Choose1fromCtoS<N>>,
            RoleS<RoleBroadcast>,
            NameC,
        >,
    ),
}

type EndpointCPicture<N> = MeshedChannelsThree<End, Choice2fromStoC<N>, RoleS<RoleEnd>, NameC>;

// S
enum Branching0fromAtoS<N: marker::Send> {
    Auth(MeshedChannelsThree<End, Choice1fromCtoS<N>, RoleC<RoleEnd>, NameS>),
    Done(MeshedChannelsThree<End, Recv<N, End>, RoleC<RoleEnd>, NameS>),
}

type EndpointSContinue<N> = MeshedChannelsThree<End, Choice1fromCtoS<N>, RoleC<RoleEnd>, NameS>;

enum Branching1fromCtoS<N: marker::Send> {
    Continue(
        MeshedChannelsThree<
            Send<N, Recv<N, Choose2fromStoA<N>>>,
            Recv<N, Choose2fromStoC<N>>,
            RoleC<RoleA<RoleA<RoleBroadcast>>>,
            NameS,
        >,
    ),
    Close(MeshedChannelsThree<Send<N, End>, Recv<N, End>, RoleC<RoleA<RoleEnd>>, NameS>),
}

type EndpointSContinueLoop<N> =
    MeshedChannelsThree<Choose2fromStoA<N>, Choose2fromStoC<N>, RoleBroadcast, NameS>;

type EndpointSPicture<N> =
    MeshedChannelsThree<End, Send<N, Choice1fromCtoS<N>>, RoleC<RoleC<RoleEnd>>, NameS>;

type EndpointSRefusal<N> =
    MeshedChannelsThree<End, Send<N, Choice1fromCtoS<N>>, RoleC<RoleC<RoleEnd>>, NameS>;

// Creating the MP sessions
// A
type EndpointA<N> = MeshedChannelsThree<
    Recv<N, Choose0fromAtoC<N>>,
    Choose0fromAtoS<N>,
    RoleC<RoleBroadcast>,
    NameA,
>;
// C
type EndpointC<N> =
    MeshedChannelsThree<Send<N, Choice0fromAtoC<N>>, End, RoleA<RoleA<RoleEnd>>, NameC>;
// S
type EndpointS<N> = MeshedChannelsThree<Choice0fromAtoS<N>, End, RoleA<RoleEnd>, NameS>;

create_fn_choose_mpst_multi_to_all_bundle!(
    auth_from_a_to_all, again_from_a_to_all, =>
    Auth, Done, =>
    EndpointAAuth<i32>, EndpointADone<i32>, =>
    Branching0fromAtoC::<i32>, Branching0fromAtoS::<i32>, =>
    RoleC, RoleS, =>
    RoleA, MeshedChannelsThree, 1
);

create_fn_choose_mpst_multi_to_all_bundle!(
    continue_from_c_to_all, close_from_c_to_all, =>
    Continue, Close, =>
    EndpointCContinue<i32>, EndpointCDone<i32>, =>
    Branching1fromCtoA::<i32>, Branching1fromCtoS::<i32>, =>
    RoleA, RoleS, =>
    RoleC, MeshedChannelsThree, 2
);

create_fn_choose_mpst_multi_to_all_bundle!(
    picture_from_s_to_all, refusal_from_s_to_all, =>
    Picture, Refusal, =>
    EndpointSPicture<i32>, EndpointSRefusal<i32>, =>
    Branching2fromStoA::<i32>, Branching2fromStoC::<i32>, =>
    RoleA, RoleC, =>
    RoleS, MeshedChannelsThree, 3
);

// Functions
fn endpoint_a(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
    let (pwd, s, _resp) = recv_http_a_to_c(s, true, Vec::new())?; // Should fail because true but Vec::new()
    let expected = thread_rng().gen_range(1..=3);

    if pwd == expected {
        let s = auth_from_a_to_all(s);

        let (s, _req) = send_http_a_to_c(0, s, false, Request::default())?;

        auth_a(s)
    } else {
        let s = again_from_a_to_all(s);

        let (s, _req) = send_http_a_to_c(1, s, false, Request::default())?;

        close_mpst_multi(s)
    }
}

fn auth_a(s: EndpointAAuthLoop<i32>) -> Result<(), Box<dyn Error>> {
    offer_http_mpst!(s, recv_http_a_to_c, {
        Branching1fromCtoA::Continue(s) => {
            let (_, s, _resp) = recv_http_a_to_s(s, false, Vec::new())?;
            let (s, _req) = send_http_a_to_s(0, s, false, Request::default())?;

            continue_a(s)
        },
        Branching1fromCtoA::Close(s) => {
            let (_, s, _resp) = recv_http_a_to_s(s, false, Vec::new())?;

            close_mpst_multi(s)

        },
    })
}

fn continue_a(s: EndpointAContinue<i32>) -> Result<(), Box<dyn Error>> {
    offer_http_mpst!(s, recv_http_a_to_s, {
        Branching2fromStoA::Picture(s) => {
            auth_a(s)
        },
        Branching2fromStoA::Refusal(s) => {
            auth_a(s)
        },
    })
}

fn endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
    let (s, _req) = send_http_c_to_a(0, s, false, Request::default())?;

    offer_http_mpst!(s, recv_http_c_to_a, {
        Branching0fromAtoC::<i32>::Done(s) => {
            let (_quit, s, _resp) = recv_http_c_to_a(s, false, Vec::new())?;
            let (s, _req) = send_http_c_to_s(0, s, false, Request::default())?;
            close_mpst_multi(s)
        },
        Branching0fromAtoC::<i32>::Auth(s) => {
            let (_quit, s, _resp) = recv_http_c_to_a(s, false, Vec::new())?;

            continue_c(s)
        },
    })
}

fn continue_c(s: EndpointCContinueLoop<i32>) -> Result<(), Box<dyn Error>> {
    let choice = thread_rng().gen_range(1..=6);

    if choice == 1 {
        let s = close_from_c_to_all(s);

        let (s, _req) = send_http_c_to_s(0, s, false, Request::default())?;

        close_mpst_multi(s)
    } else {
        let s = continue_from_c_to_all(s);

        let (s, _req) = send_http_c_to_s(0, s, false, Request::default())?;

        picture_c(s)
    }
}

fn picture_c(s: EndpointCPicture<i32>) -> Result<(), Box<dyn Error>> {
    offer_http_mpst!(s, recv_http_c_to_s, {
        Branching2fromStoC::<i32>::Picture(s) => {
            let (_quit, s, _resp) = recv_http_c_to_s(s, false, Vec::new())?;

            continue_c(s)
        },
        Branching2fromStoC::<i32>::Refusal(s) => {
            let (_quit, s, _resp) = recv_http_c_to_s(s, false, Vec::new())?;

            continue_c(s)
        },
    })
}

fn endpoint_s(s: EndpointS<i32>) -> Result<(), Box<dyn Error>> {
    offer_http_mpst!(s, recv_http_s_to_a, {
        Branching0fromAtoS::<i32>::Done(s) => {
            let (_quit, s, _resp) = recv_http_s_to_c(s, false, Vec::new())?;

            close_mpst_multi(s)
        },
        Branching0fromAtoS::<i32>::Auth(s) => {
            continue_s(s)
        },
    })
}

fn continue_s(s: EndpointSContinue<i32>) -> Result<(), Box<dyn Error>> {
    offer_http_mpst!(s, recv_http_s_to_c, {
        Branching1fromCtoS::<i32>::Continue(s) => {
            let (_quit, s, _resp) = recv_http_s_to_c(s, false, Vec::new())?;
            let (s, _req) = send_http_s_to_a(0, s, false, Request::default())?;
            let (_quit, s, _resp) = recv_http_s_to_a(s, false, Vec::new())?;

            picture_s(s)
        },
        Branching1fromCtoS::<i32>::Close(s) => {
            let (_quit, s, _resp) = recv_http_s_to_c(s, false, Vec::new())?;
            let (s, _req) = send_http_s_to_a(0, s, false, Request::default())?;

            close_mpst_multi(s)
        },
    })
}

fn picture_s(s: EndpointSContinueLoop<i32>) -> Result<(), Box<dyn Error>> {
    let choice = thread_rng().gen_range(1..=6);

    if choice == 1 {
        let s = refusal_from_s_to_all(s);

        let (s, _req) = send_http_s_to_c(0, s, false, Request::default())?;

        continue_s(s)
    } else {
        let s = picture_from_s_to_all(s);

        let (s, _req) = send_http_s_to_c(0, s, false, Request::default())?;

        continue_s(s)
    }
}

/////////////////////////

pub fn main() {
    let (thread_a, thread_c, thread_s) = fork_mpst(endpoint_a, endpoint_c, endpoint_s);

    assert!(thread_a.join().is_err());
    assert!(thread_c.join().is_err());
    assert!(thread_s.join().is_err());
}
