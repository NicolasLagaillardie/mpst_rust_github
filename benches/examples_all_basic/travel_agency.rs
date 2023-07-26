#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose_mpst_multi_to_all, create_multiple_normal_name,
    create_multiple_normal_role_short, create_recv_mpst_session_bundle,
    create_send_mpst_session_bundle, offer_mpst,
};

use rand::{random, thread_rng, Rng};

use std::error::Error;
use std::marker;

// See the folder scribble_protocols for the related Scribble protocol

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannels, 3);

// Create new roles
// normal
create_multiple_normal_role_short!(A, C, S);

// Create new names
create_multiple_normal_name!(NameA, NameC, NameS);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_c, RoleC, 1 |
    send_mpst_a_to_s, RoleS, 2 | =>
    NameA, MeshedChannels, 3
);

// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a, RoleA, 1 |
    send_mpst_c_to_s, RoleS, 2 | =>
    NameC, MeshedChannels, 3
);

// S
create_send_mpst_session_bundle!(
    send_mpst_s_to_c, RoleC, 2 | =>
    NameS, MeshedChannels, 3
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_c, RoleC, 1 | =>
    NameA, MeshedChannels, 3
);

// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a, RoleA, 1 |
    recv_mpst_c_from_s, RoleS, 2 | =>
    NameC, MeshedChannels, 3
);

// S
create_recv_mpst_session_bundle!(
    recv_mpst_s_from_a, RoleA, 1 |
    recv_mpst_s_from_c, RoleC, 2 | =>
    NameS, MeshedChannels, 3
);

// Types
// C0
type Choose0fromCtoA<N> = Send<Branching0fromCtoA<N>, End>;
type Choose0fromCtoS<N> = Send<Branching0fromCtoS<N>, End>;

// C1
type Choose1fromCtoA<N> = <Choice1fromCtoA<N> as Session>::Dual;
type Choose1fromCtoS<N> = <Choice1fromCtoS<N> as Session>::Dual;

// A
enum Branching0fromCtoA<N: marker::Send> {
    Select(MeshedChannels<Choice1fromCtoA<N>, End, RoleC<RoleEnd>, NameA>),
    Loop(
        MeshedChannels<
            Recv<N, Send<N, Choice0fromCtoA<N>>>,
            Send<N, End>,
            RoleC<RoleC<RoleS<RoleC<RoleEnd>>>>,
            NameA,
        >,
    ),
}
type Choice0fromCtoA<N> = Recv<Branching0fromCtoA<N>, End>;
enum Branching1fromCtoA<N: marker::Send> {
    Yes(MeshedChannels<Recv<N, End>, Send<N, End>, RoleC<RoleS<RoleEnd>>, NameA>),
    No(MeshedChannels<Recv<N, End>, Send<N, End>, RoleC<RoleS<RoleEnd>>, NameA>),
}
type Choice1fromCtoA<N> = Recv<Branching1fromCtoA<N>, End>;

// S
enum Branching0fromCtoS<N: marker::Send> {
    Select(MeshedChannels<End, Choice1fromCtoS<N>, RoleC<RoleEnd>, NameS>),
    Loop(MeshedChannels<Recv<N, End>, Choice0fromCtoS<N>, RoleA<RoleC<RoleEnd>>, NameS>),
}
type Choice0fromCtoS<N> = Recv<Branching0fromCtoS<N>, End>;
enum Branching1fromCtoS<N: marker::Send> {
    Yes(MeshedChannels<Recv<N, End>, Recv<N, Send<N, End>>, RoleA<RoleC<RoleC<RoleEnd>>>, NameS>),
    No(MeshedChannels<Recv<N, End>, End, RoleA<RoleEnd>, NameS>),
}
type Choice1fromCtoS<N> = Recv<Branching1fromCtoS<N>, End>;

// Creating the MP sessions
// A
type ChoiceA<N> = MeshedChannels<Choice1fromCtoA<N>, End, RoleC<RoleEnd>, NameA>;
type EndpointA<N> = MeshedChannels<Choice0fromCtoA<N>, End, RoleC<RoleEnd>, NameA>;

// C
type ChoiceC<N> = MeshedChannels<Choose1fromCtoA<N>, Choose1fromCtoS<N>, RoleBroadcast, NameC>;
type EndpointC<N> = MeshedChannels<Choose0fromCtoA<N>, Choose0fromCtoS<N>, RoleBroadcast, NameC>;

// S
type ChoiceS<N> = MeshedChannels<End, Choice1fromCtoS<N>, RoleC<RoleEnd>, NameS>;
type EndpointS<N> = MeshedChannels<End, Choice0fromCtoS<N>, RoleC<RoleEnd>, NameS>;

// Functions
// A
fn endpoint_a(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_c, {
        Branching0fromCtoA::Select(s) => {
            choice_a(s)
        },
        Branching0fromCtoA::Loop(s) => {
            let (query, s) = recv_mpst_a_from_c(s)?;
            let s = send_mpst_a_to_c(query, s);
            let s = send_mpst_a_to_s(random(), s);
            endpoint_a(s)
        },
    })
}

fn choice_a(s: ChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_c, {
        Branching1fromCtoA::Yes(s) => {
            let (yes, s) = recv_mpst_a_from_c(s)?;
            let s = send_mpst_a_to_s(yes, s);
            close_mpst_multi(s)
        },
        Branching1fromCtoA::No(s) => {
            let (no, s) = recv_mpst_a_from_c(s)?;
            let s = send_mpst_a_to_s(no, s);
            close_mpst_multi(s)
        },
    })
}

fn endpoint_init(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
    endpoint_c(s, LOOPS)
}

fn endpoint_c(s: EndpointC<i32>, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromCtoA::<i32>::Select,
                Branching0fromCtoS::<i32>::Select, =>
                NameC,
                MeshedChannels,
                2
            );
            choice_c(s)
        }
        _ => {
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromCtoA::<i32>::Loop,
                Branching0fromCtoS::<i32>::Loop, =>
                NameC,
                MeshedChannels,
                2
            );

            let s = send_mpst_c_to_a(random(), s);
            let (_quote, s) = recv_mpst_c_from_a(s)?;
            endpoint_c(s, loops - 1)
        }
    }
}

fn choice_c(s: ChoiceC<i32>) -> Result<(), Box<dyn Error>> {
    let choice: i32 = thread_rng().gen_range(1..3);

    if choice != 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching1fromCtoA::<i32>::Yes,
            Branching1fromCtoS::<i32>::Yes, =>
            NameC,
            MeshedChannels,
            2
        );

        let s = send_mpst_c_to_a(random(), s);
        let s = send_mpst_c_to_s(random(), s);
        let (_ack, s) = recv_mpst_c_from_s(s)?;
        close_mpst_multi(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching1fromCtoA::<i32>::No,
            Branching1fromCtoS::<i32>::No, =>
            NameC,
            MeshedChannels,
            2
        );

        let s = send_mpst_c_to_a(0, s);
        close_mpst_multi(s)
    }
}

fn endpoint_s(s: EndpointS<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_s_from_c, {
        Branching0fromCtoS::Select(s) => {
            choice_s(s)
        },
        Branching0fromCtoS::Loop(s) => {
            let (_dummy, s) = recv_mpst_s_from_a(s)?;
            endpoint_s(s)
        },
    })
}

fn choice_s(s: ChoiceS<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_s_from_c, {
        Branching1fromCtoS::Yes(s) => {
            let (_yes, s) = recv_mpst_s_from_a(s)?;
            let (payment, s) = recv_mpst_s_from_c(s)?;
            let s = send_mpst_s_to_c(payment, s);
            close_mpst_multi(s)
        },
        Branching1fromCtoS::No(s) => {
            let (_no, s) = recv_mpst_s_from_a(s)?;
            close_mpst_multi(s)
        },
    })
}

fn aux() {
    let (thread_a, thread_c, thread_s) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_init),
        black_box(endpoint_s),
    );

    thread_a.join().unwrap();
    thread_c.join().unwrap();
    thread_s.join().unwrap();
}

/////////////////////////

static LOOPS: i32 = 100;

pub fn travel(c: &mut Criterion) {
    c.bench_function("Travel agency basic", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(20000);
    targets = travel,
}

criterion_main! {
    bench
}
