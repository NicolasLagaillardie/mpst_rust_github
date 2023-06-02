use crossbeam_channel::bounded;

use criterion::{black_box, Criterion};

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{baker, choose, offer};

use std::error::Error;
use std::thread::{spawn, JoinHandle};

// use std::time::Duration;

// Create new roles
baker!("recursive", MeshedChannelsThree, A, B, C);

// Types
// A
enum Branching0fromCtoA {
    Forward(MeshedChannelsThree<Send<(), End>, RecursAtoC, RoleB<RoleC<RoleEnd>>, NameA>),
    Backward(MeshedChannelsThree<Recv<(), End>, RecursAtoC, RoleB<RoleC<RoleEnd>>, NameA>),
    Done(MeshedChannelsThree<End, End, RoleEnd, NameA>),
}
type RecursAtoC = Recv<Branching0fromCtoA, End>;

// B
enum Branching0fromCtoB {
    Forward(
        MeshedChannelsThree<
            Recv<(), End>,
            Send<(), RecursBtoC>,
            RoleA<RoleC<RoleC<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannelsThree<
            Send<(), End>,
            Recv<(), RecursBtoC>,
            RoleC<RoleA<RoleC<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsThree<End, End, RoleEnd, NameB>),
}
type RecursBtoC = Recv<Branching0fromCtoB, End>;

// C
type Choose0fromCtoA = Send<Branching0fromCtoA, End>;
type Choose0fromCtoB = Send<Branching0fromCtoB, End>;
type EndpointForwardC =
    MeshedChannelsThree<Choose0fromCtoA, Recv<(), Choose0fromCtoB>, RoleB<RoleBroadcast>, NameC>;
type EndpointBackwardC =
    MeshedChannelsThree<Choose0fromCtoA, Send<(), Choose0fromCtoB>, RoleB<RoleBroadcast>, NameC>;

// Creating the MP sessions
type EndpointA = MeshedChannelsThree<End, RecursAtoC, RoleC<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsThree<End, RecursBtoC, RoleC<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsThree<Choose0fromCtoA, Choose0fromCtoB, RoleBroadcast, NameC>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromCtoA::Done(s) => {
            s.close()
        },
        Branching0fromCtoA::Forward(s) => {
            let s = s.send(());
            endpoint_a(s)
        },
        Branching0fromCtoA::Backward(s) => {
            let (_, s) = s.recv();
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromCtoB::Done(s) => {
            s.close()
        },
        Branching0fromCtoB::Forward(s) => {
            let ((), s) = s.recv();
            let s = s.send(());
            endpoint_b(s)
        },
        Branching0fromCtoB::Backward(s) => {
            let ((), s) = s.recv();
            let s = s.send(());
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    recurs_c(s, LOOPS)
}

fn recurs_c(s: EndpointC, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_c_to_all!(s, Branching0fromCtoA::Done, Branching0fromCtoB::Done);

            s.close()
        }
        i if i % 2 == 0 => {
            let s: EndpointForwardC =
                choose_mpst_c_to_all!(s, Branching0fromCtoA::Forward, Branching0fromCtoB::Forward);

            let (_, s) = s.recv();

            recurs_c(s, i - 1)
        }
        i => {
            let s: EndpointBackwardC = choose_mpst_c_to_all!(
                s,
                Branching0fromCtoA::Backward,
                Branching0fromCtoB::Backward
            );

            let s = s.send(());

            recurs_c(s, i - 1)
        }
    }
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

static LOOPS: i64 = 100;

pub fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(
        &format!("ring three baking protocol MPST {LOOPS}"),
        |b| b.iter(all_mpst)
    );
}




/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.1).sample_size(10000);
    targets = ring_protocol_mpst,
}

criterion_main! {
    bench
}


