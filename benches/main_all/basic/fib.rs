use criterion::{black_box, Criterion};

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose, choose_mpst_multi_to_all, create_multiple_normal_name,
    create_multiple_normal_role, create_recv_mpst_session_bundle, create_send_mpst_session_bundle,
    offer, offer_mpst,
};

use std::error::Error;
use std::marker;
use std::thread::{spawn, JoinHandle};
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

fn all_mpst() {
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
// A
enum BinaryA<N: marker::Send> {
    More(Recv<N, Send<N, RecursA<N>>>),
    Done(End),
}
type RecursA<N> = Recv<BinaryA<N>, End>;

fn binary_a_to_b(s: RecursA<i64>) -> Result<(), Box<dyn Error>> {
    recurs_a_binary(s, 1)
}
fn recurs_a_binary(s: RecursA<i64>, old: i64) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        BinaryA::Done(s) => {
            close(s)
        },
        BinaryA::More(s) => {
            let (new, s) = recv(s)?;
            let s = send(old + new, s);
            recurs_a_binary(s, old + new)
        },
    })
}

// B
type RecursB<N> = <RecursA<N> as Session>::Dual;
fn binary_b_to_a(s: Send<i64, Recv<i64, RecursB<i64>>>) -> Result<RecursB<i64>, Box<dyn Error>> {
    recurs_b_binary(s, 0)
}
fn recurs_b_binary(
    s: Send<i64, Recv<i64, RecursB<i64>>>,
    old: i64,
) -> Result<RecursB<i64>, Box<dyn Error>> {
    let s = send(old, s);
    let (_, s) = recv(s)?;
    Ok(s)
}

fn all_binaries() {
    let mut threads = Vec::new();
    let mut sessions = Vec::new();

    for _ in 0..3 {
        let (thread, s): (JoinHandle<()>, RecursB<i64>) =
            fork_with_thread_id(black_box(binary_a_to_b));

        threads.push(thread);
        sessions.push(s);
    }

    let main = spawn(move || {
        for _ in 0..LOOPS {
            sessions = sessions
                .into_iter()
                .map(|s| binary_b_to_a(choose!(BinaryA::<i64>::More, s)).unwrap())
                .collect::<Vec<_>>();
        }

        sessions
            .into_iter()
            .for_each(|s| close(choose!(BinaryA::<i64>::Done, s)).unwrap());

        threads.into_iter().for_each(|elt| elt.join().unwrap());
    });

    main.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 20;

pub fn fibo_mpst(c: &mut Criterion) {
    c.bench_function(&format!("Fibo MPST {}", LOOPS), |b| b.iter(all_mpst));
}

pub fn fibo_binary(c: &mut Criterion) {
    c.bench_function(&format!("Fibo binary {}", LOOPS), |b| b.iter(all_binaries));
}
