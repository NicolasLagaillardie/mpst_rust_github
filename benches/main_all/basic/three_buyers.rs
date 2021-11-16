// #![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose_mpst_multi_to_all, create_multiple_normal_role_short,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, offer_mpst,
};

use rand::{random, thread_rng, Rng};

use std::error::Error;
use std::marker;
// use std::time::Duration;

// See the folder scribble_protocols for the related Scribble protocol

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsThree, 3);

// Create new roles
// normal
create_multiple_normal_role_short!(A, C, S);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_c, RoleC, 1 |
    send_mpst_a_to_s, RoleS, 2 | =>
    RoleA, MeshedChannelsThree, 3
);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a, RoleA, 1 |
    send_mpst_c_to_s, RoleS, 2 | =>
    RoleC, MeshedChannelsThree, 3
);
// S
create_send_mpst_session_bundle!(
    send_mpst_s_to_a, RoleA, 1 |
    send_mpst_s_to_c, RoleC, 2 | =>
    RoleS, MeshedChannelsThree, 3
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_c, RoleC, 1 |
    recv_mpst_a_from_s, RoleS, 2 | =>
    RoleA, MeshedChannelsThree, 3
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a, RoleA, 1 |
    recv_mpst_c_from_s, RoleS, 2 | =>
    RoleC, MeshedChannelsThree, 3
);
// S
create_recv_mpst_session_bundle!(
    recv_mpst_s_from_a, RoleA, 1 |
    recv_mpst_s_from_c, RoleC, 2 | =>
    RoleS, MeshedChannelsThree, 3
);

// Names
type NameA = RoleA<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameS = RoleS<RoleEnd>;

// Types
// A
type Choose0fromCtoA<N> = Send<Branching0fromCtoA<N>, End>;
type Choose0fromCtoS<N> = Send<Branching0fromCtoS<N>, End>;

// A
enum Branching0fromCtoA<N: marker::Send> {
    Accept(MeshedChannelsThree<Recv<N, End>, End, RoleC<RoleEnd>, NameA>),
    Quit(MeshedChannelsThree<End, End, RoleEnd, NameA>),
}
// S
enum Branching0fromCtoS<N: marker::Send> {
    Accept(MeshedChannelsThree<End, Recv<N, Send<N, End>>, RoleC<RoleC<RoleEnd>>, NameS>),
    Quit(MeshedChannelsThree<End, End, RoleEnd, NameS>),
}

// Creating the MP sessions
// A
type EndpointA<N> = MeshedChannelsThree<
    Send<N, Recv<Branching0fromCtoA<N>, End>>,
    Send<N, Recv<N, End>>,
    RoleS<RoleS<RoleC<RoleC<RoleEnd>>>>,
    NameA,
>;
// C
type EndpointC<N> = MeshedChannelsThree<
    Recv<N, Choose0fromCtoA<N>>,
    Recv<N, Choose0fromCtoS<N>>,
    RoleS<RoleA<RoleBroadcast>>,
    NameC,
>;
// S
type EndpointS<N> = MeshedChannelsThree<
    Recv<N, Send<N, End>>,
    Send<N, Recv<Branching0fromCtoS<N>, End>>,
    RoleA<RoleA<RoleC<RoleC<RoleEnd>>>>,
    NameS,
>;

// Functions
fn endpoint_a(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_a_to_s(random(), s);
    let (_empty2, s) = recv_mpst_a_from_s(s)?;
    let s = send_mpst_a_to_c(random(), s);
    offer_mpst!(s, recv_mpst_a_from_c, {
        Branching0fromCtoA::Accept(s) => {
            let (_ok, s) = recv_mpst_a_from_c(s)?;
            close_mpst_multi(s)
        },
        Branching0fromCtoA::Quit(s) => {
            close_mpst_multi(s)
        },
    })
}

fn endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
    let (_empty3, s) = recv_mpst_c_from_s(s)?;
    let (_empty4, s) = recv_mpst_c_from_a(s)?;

    let choice = thread_rng().gen_range(1..=3);

    if choice != 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching0fromCtoA::<i32>::Accept,
            Branching0fromCtoS::<i32>::Accept, =>
            RoleA,
            RoleS, =>
            RoleC,
            MeshedChannelsThree,
            2
        );

        let s = send_mpst_c_to_a(random(), s);
        let s = send_mpst_c_to_s(random(), s);
        let (_empty5, s) = recv_mpst_c_from_s(s)?;

        close_mpst_multi(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching0fromCtoA::<i32>::Quit,
            Branching0fromCtoS::<i32>::Quit, =>
            RoleA,
            RoleS, =>
            RoleC,
            MeshedChannelsThree,
            2
        );
        close_mpst_multi(s)
    }
}

fn endpoint_s(s: EndpointS<i32>) -> Result<(), Box<dyn Error>> {
    let (_empty1, s) = recv_mpst_s_from_a(s)?;
    let s = send_mpst_s_to_a(random(), s);
    let s = send_mpst_s_to_c(random(), s);
    offer_mpst!(s, recv_mpst_s_from_c, {
        Branching0fromCtoS::Accept(s) => {
            let (_ok, s) = recv_mpst_s_from_c(s)?;
            let s = send_mpst_s_to_c(random(), s);
            close_mpst_multi(s)
        },
        Branching0fromCtoS::Quit(s) => {
            close_mpst_multi(s)
        },
    })
}

fn all_mpst() {
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

fn three_buyers_mpst(c: &mut Criterion) {
    c.bench_function(&format!("Three buyers MPST"), |b| b.iter(|| all_mpst()));
}

// fn long_warmup() -> Criterion {
//     Criterion::default().measurement_time(Duration::new(30, 0))
// }

criterion_group! {
    name = three_buyers;
    // config = long_warmup();
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = three_buyers_mpst
}

criterion_main!(three_buyers);
