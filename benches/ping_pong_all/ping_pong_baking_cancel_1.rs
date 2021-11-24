#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{bundle_impl_with_enum_and_cancel, offer_mpst};

use std::error::Error;
// use std::time::Duration;

// global protocol ping_pong(role A, role B)
// {
//     rec PP
//     {
//         choice at A
//         {
//             ping(()) from A to B;
//             pong(()) from B to A;
//             continue PP;
//         }
//         or
//         {
//             stop() from A to B;
//         }
//     }
// }

// Create new roles
bundle_impl_with_enum_and_cancel!(MeshedChannelsTwo, A, B);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;

// Types
// A
type Choose0fromAtoB = <RecursBtoA as Session>::Dual;
// B
enum Branching0fromAtoB {
    More(MeshedChannelsTwo<Recv<(), Send<(), RecursBtoA>>, RoleA<RoleA<RoleA<RoleEnd>>>, NameB>),
    Done(MeshedChannelsTwo<End, RoleEnd, NameB>),
}
type RecursBtoA = Recv<Branching0fromAtoB, End>;

// Creating the MP sessions
type EndpointA = MeshedChannelsTwo<Choose0fromAtoB, RoleBroadcast, NameA>;
type EndpointB = MeshedChannelsTwo<RecursBtoA, RoleA<RoleEnd>, NameB>;

// Functions
fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    recurs_a(s, LOOPS)
}

fn recurs_a(s: EndpointA, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_a_to_all!(s, Branching0fromAtoB::Done);

            s.close()
        }
        i => {
            let s = choose_mpst_a_to_all!(s, Branching0fromAtoB::More);

            let s = s.send(())?;
            let ((), s) = s.recv()?;

            recurs_a(s, i - 1)
        }
    }
}

fn recurs_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromAtoB::Done(s) => {
            s.close()
        },
        Branching0fromAtoB::More(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            recurs_b(s)
        },
    })
}

fn all_mpst() {
    let (thread_a, thread_b) = fork_mpst(black_box(endpoint_a), black_box(recurs_b));

    thread_a.join().unwrap();
    thread_b.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 1;

fn ping_pong_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("ping pong protocol MPST {}", LOOPS), |b| {
        b.iter(all_mpst)
    });
}

// fn long_warmup() -> Criterion {
//     Criterion::default().measurement_time(Duration::new(1800, 0))
// }

criterion_group! {
    name = ping_pong;
    // config = long_warmup();
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = ping_pong_protocol_mpst
}

criterion_main!(ping_pong);
