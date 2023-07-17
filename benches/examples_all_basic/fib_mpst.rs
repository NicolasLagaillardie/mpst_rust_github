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
    create_multiple_normal_role, create_recv_mpst_session_bundle, create_send_mpst_session_bundle,
    offer_mpst,
};

use std::error::Error;
use std::marker;

// use std::time::Duration;

// See the folder scribble_protocols for the related Scribble protocol

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsThree, 3);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
    RoleC, RoleCDual |
);

// Create new roles
// normal
create_multiple_normal_name!(NameA, NameB, NameC);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, 1 |
    send_mpst_a_to_c, RoleC, 2 | =>
    NameA, MeshedChannelsThree, 3
);

// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a, RoleA, 1 | =>
    NameB, MeshedChannelsThree, 3
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 1 | =>
    NameA, MeshedChannelsThree, 3
);

// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 1 | =>
    NameB, MeshedChannelsThree, 3
);

// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a, RoleA, 1 | =>
    NameC, MeshedChannelsThree, 3
);

// Types
// A
type Choose0fromAtoB<N> = <RecursBtoA<N> as Session>::Dual;
type Choose0fromAtoC = <RecursCtoA as Session>::Dual;

// B
enum Branching0fromAtoB<N: marker::Send> {
    More(
        MeshedChannelsThree<
            Recv<N, Send<N, RecursBtoA<N>>>,
            End,
            RoleA<RoleA<RoleA<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsThree<End, End, RoleEnd, NameB>),
}
type RecursBtoA<N> = Recv<Branching0fromAtoB<N>, End>;

// C
enum Branching0fromAtoC {
    More(MeshedChannelsThree<RecursCtoA, End, RoleA<RoleEnd>, NameC>),
    Done(MeshedChannelsThree<End, End, RoleEnd, NameC>),
}
type RecursCtoA = Recv<Branching0fromAtoC, End>;

// Creating the MP sessions
type EndpointA<N> = MeshedChannelsThree<Choose0fromAtoB<N>, Choose0fromAtoC, RoleBroadcast, NameA>;
type EndpointB<N> = MeshedChannelsThree<RecursBtoA<N>, End, RoleA<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsThree<RecursCtoA, End, RoleA<RoleEnd>, NameC>;

// Functions
fn endpoint_a(s: EndpointA<i64>) -> Result<(), Box<dyn Error>> {
    recurs_a(s, LOOPS, 1)
}

fn recurs_a(s: EndpointA<i64>, index: i64, old: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromAtoB::<i64>::Done,
                Branching0fromAtoC::Done, =>
                NameA,
                MeshedChannelsThree,
                1
            );

            close_mpst_multi(s)
        }
        i => {
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromAtoB::<i64>::More,
                Branching0fromAtoC::More, =>
                NameA,
                MeshedChannelsThree,
                1
            );

            let s = send_mpst_a_to_b(old, s);
            let (new, s) = recv_mpst_a_from_b(s)?;

            recurs_a(s, i - 1, new)
        }
    }
}

fn endpoint_b(s: EndpointB<i64>) -> Result<(), Box<dyn Error>> {
    recurs_b(s, 0)
}

fn recurs_b(s: EndpointB<i64>, old: i64) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_a, {
        Branching0fromAtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromAtoB::More(s) => {
            let (new, s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_a(new + old, s);
            recurs_b(s, new + old)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_a, {
        Branching0fromAtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromAtoC::More(s) => {
            endpoint_c(s)
        },
    })
}

fn aux() {
    let (thread_a, thread_b, thread_c) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 20;

pub fn fib(c: &mut Criterion) {
    c.bench_function(&format!("Fibo basic {LOOPS}"), |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(20000);
    targets = fib,
}

criterion_main! {
    bench
}
