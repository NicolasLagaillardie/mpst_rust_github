#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use criterion::{black_box, Criterion};

use mpstthree::baker_timed;
use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// See the folder scribble_protocols for the related Scribble protocol

// Create the new MeshedChannels for three participants and the close and fork functions
baker_timed!(MeshedChannels, A, B);

// Types
// A
type Choose0fromAtoB = SendTimed<Branching0fromAtoB, End, 'a', 0, true, 1, true, false>;
// B
enum Branching0fromAtoB {
    More(MeshedChannels<RSRecursBtoA, ThreeRoleA, NameB>),
    Done(MeshedChannels<End, RoleEnd, NameB>),
}
type RSRecursBtoA = RecvTimed<
    i32,
    SendTimed<i32, RecursBtoA, 'a', 0, true, 1, true, false>,
    'a',
    0,
    true,
    1,
    true,
    false,
>;
type ThreeRoleA = RoleA<RoleA<RoleA<RoleEnd>>>;
type RecursBtoA = RecvTimed<Branching0fromAtoB, End, 'a', 0, true, 1, true, false>;

// Creating the MP sessions
type EndpointA = MeshedChannels<Choose0fromAtoB, RoleBroadcast, NameA>;
type EndpointB = MeshedChannels<RecursBtoA, RoleA<RoleEnd>, NameB>;

// Functions
fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    recurs_a(s, LOOPS, 1, all_clocks)
}

fn recurs_a(
    s: EndpointA,
    index: i32,
    old: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_a_to_all!(s, all_clocks, Branching0fromAtoB::Done);

            s.close()
        }
        i => {
            let s = choose_mpst_a_to_all!(s, all_clocks, Branching0fromAtoB::More);

            let s = s.send(old, all_clocks)?;
            let (new, s) = s.recv(all_clocks)?;

            recurs_a(s, i - 1, new, all_clocks)
        }
    }
}

fn endpoint_b(s: EndpointB, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    recurs_b(s, 0, all_clocks)
}

fn recurs_b(
    s: EndpointB,
    old: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    offer_mpst!(s, all_clocks, {
        Branching0fromAtoB::Done(s) => {
            s.close()
        },
        Branching0fromAtoB::More(s) => {
            let (new, s) = s.recv(all_clocks)?;
            let s = s.send(new + old, all_clocks)?;
            recurs_b(s, new + old, all_clocks)
        },
    })
}

fn all_mpst() {
    let (thread_a, thread_b) = fork_mpst(black_box(endpoint_a), black_box(endpoint_b));

    thread_a.join().unwrap();
    thread_b.join().unwrap();
}

/////////////////////////

static LOOPS: i32 = 20;

pub fn fibo_mpst(c: &mut Criterion) {
    c.bench_function(&format!("Timed Fibo MPST baking {}", LOOPS), |b| {
        b.iter(all_mpst)
    });
}
