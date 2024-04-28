use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::generate_atmp;
use mpstthree::binary::struct_trait::{end::End, session::Session};
use mpstthree::binary_atmp::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// Create new roles
generate_atmp!(MeshedChannels, A, B);

// Types
// A
type Choose0fromAtoB = <RecursBtoA as Session>::Dual;

// B
enum Branching0fromAtoB {
    More(
        MeshedChannels<
            RecvTimed<
                (),
                'a',
                0,
                true,
                10,
                true,
                ' ',
                SendTimed<(), 'a', 0, true, 10, true, ' ', RecursBtoA>,
            >,
            RoleA<RoleA<RoleA<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannels<End, RoleEnd, NameB>),
}
type RecursBtoA = RecvTimed<Branching0fromAtoB, 'a', 0, true, 10, true, ' ', End>;

// Creating the MP sessions
type EndpointA = MeshedChannels<Choose0fromAtoB, RoleBroadcast, NameA>;
type EndpointB = MeshedChannels<RecursBtoA, RoleA<RoleEnd>, NameB>;

// Functions
fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    recurs_a(s, all_clocks, LOOPS)
}

fn recurs_a(
    s: EndpointA,
    all_clocks: &mut HashMap<char, Instant>,
    index: i64,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    match index {
        0 => {
            let s = choose_mpst_a_to_all!(s, all_clocks, Branching0fromAtoB::Done);

            s.close()
        }
        i => {
            let s = choose_mpst_a_to_all!(s, all_clocks, Branching0fromAtoB::More);

            let s = s.send((), all_clocks)?;
            let ((), s) = s.recv(all_clocks)?;

            recurs_a(s, all_clocks, i - 1)
        }
    }
}
fn endpoint_b(s: EndpointB, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    recurs_b(s, all_clocks)
}

fn recurs_b(s: EndpointB, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branching0fromAtoB::Done(s) => {
            s.close()
        },
        Branching0fromAtoB::More(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            recurs_b(s, all_clocks)
        },
    })
}

fn aux() {
    let (thread_a, thread_b) = fork_mpst(black_box(endpoint_a), black_box(endpoint_b));

    thread_a.join().unwrap();
    thread_b.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 1;

pub fn ping_pong(c: &mut Criterion) {
    c.bench_function(&format!("ping pong baking ATMP {LOOPS}"), |b| {
        b.iter(aux)
    });
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(100000);
    targets = ping_pong
}

criterion_main! {
    bench
}
