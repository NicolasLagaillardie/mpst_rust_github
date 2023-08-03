use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose_mpst_multi_to_all, create_multiple_normal_name_short,
    create_multiple_normal_role_short, create_recv_mpst_session_bundle,
    create_send_mpst_session_bundle, offer_mpst,
};

use rand::{thread_rng, Rng};

use std::error::Error;
use std::marker;

// See the folder scribble_protocols for the related Scribble protocol

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannels, 3);

// Create new roles
// normal
create_multiple_normal_role_short!(A, C, S);

// Create new names
create_multiple_normal_name_short!(A, C, S);

// Create new send functions
// C
create_send_mpst_session_bundle!(
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
    recv_mpst_c_from_s, RoleS, 2 | =>
    NameC, MeshedChannels, 3
);

// S
create_recv_mpst_session_bundle!(
    recv_mpst_s_from_c, RoleC, 2 | =>
    NameS, MeshedChannels, 3
);

// Types
// A
type Choose0fromCtoA = Send<Branching0fromCtoA, End>;
type Choose0fromCtoS<N> = Send<Branching0fromCtoS<N>, End>;

// A
enum Branching0fromCtoA {
    Sum(MeshedChannels<End, End, RoleEnd, NameA>),
    Diff(MeshedChannels<End, End, RoleEnd, NameA>),
}

// S
enum Branching0fromCtoS<N: marker::Send> {
    Sum(MeshedChannels<End, Send<N, End>, RoleC<RoleEnd>, NameS>),
    Diff(MeshedChannels<End, Send<N, End>, RoleC<RoleEnd>, NameS>),
}

// Creating the MP sessions
// A
type EndpointA = MeshedChannels<Recv<Branching0fromCtoA, End>, End, RoleC<RoleEnd>, NameA>;

// C
type EndpointC<N> = MeshedChannels<
    Choose0fromCtoA,
    Send<N, Send<N, Choose0fromCtoS<N>>>,
    RoleS<RoleS<RoleBroadcast>>,
    NameC,
>;

// S
type EndpointS<N> = MeshedChannels<
    End,
    Recv<N, Recv<N, Recv<Branching0fromCtoS<N>, End>>>,
    RoleC<RoleC<RoleC<RoleEnd>>>,
    NameS,
>;

// Functions
fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_c, {
        Branching0fromCtoA::Sum(s) => {
            close_mpst_multi(s)
        },
        Branching0fromCtoA::Diff(s) => {
            close_mpst_multi(s)
        },
    })
}

fn endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_c_to_s(thread_rng().gen_range(1..=100), s);
    let s = send_mpst_c_to_s(thread_rng().gen_range(1..=100), s);

    let choice: i32 = thread_rng().gen_range(1..=2);

    if choice != 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching0fromCtoA::Sum,
            Branching0fromCtoS::<i32>::Sum, =>
            NameC,
            MeshedChannels,
            2
        );

        let (_sum, s) = recv_mpst_c_from_s(s)?;

        close_mpst_multi(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching0fromCtoA::Diff,
            Branching0fromCtoS::<i32>::Diff, =>
            NameC,
            MeshedChannels,
            2
        );

        let (_diff, s) = recv_mpst_c_from_s(s)?;

        close_mpst_multi(s)
    }
}

fn endpoint_s(s: EndpointS<i32>) -> Result<(), Box<dyn Error>> {
    let (elt_1, s) = recv_mpst_s_from_c(s)?;
    let (elt_2, s) = recv_mpst_s_from_c(s)?;

    offer_mpst!(s, recv_mpst_s_from_c, {
        Branching0fromCtoS::Sum(s) => {
            let s = send_mpst_s_to_c(elt_1 + elt_2, s);
            close_mpst_multi(s)
        },
        Branching0fromCtoS::Diff(s) => {
            let s = send_mpst_s_to_c(elt_1 - elt_2, s);
            close_mpst_multi(s)
        },
    })
}

fn aux() {
    let (thread_a, thread_c, thread_s) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_c),
        black_box(endpoint_s),
    );

    thread_a.join().unwrap();
    thread_c.join().unwrap();
    thread_s.join().unwrap();
}

/////////////////////////

pub fn calculator(c: &mut Criterion) {
    c.bench_function("Calculator basic", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(10000);
    targets = calculator,
}

criterion_main! {
    bench
}
