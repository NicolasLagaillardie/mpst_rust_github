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
baker!("rec_and_cancel", MeshedChannelsTwo, A, B);

// Types
// A
enum Branching0fromBtoA {
    Forward(MeshedChannelsTwo<Send<(), RecursAtoB>, RoleB<RoleB<RoleEnd>>, NameA>),
    Backward(MeshedChannelsTwo<Recv<(), RecursAtoB>, RoleB<RoleB<RoleEnd>>, NameA>),
    Done(MeshedChannelsTwo<End, RoleEnd, NameA>),
}
type RecursAtoB = <Choose0fromBtoA as Session>::Dual;

// B
type Choose0fromBtoA = Send<Branching0fromBtoA, End>;
type EndpointForwardB = MeshedChannelsTwo<Recv<(), Choose0fromBtoA>, RoleA<RoleBroadcast>, NameB>;
type EndpointBackwardB = MeshedChannelsTwo<Send<(), Choose0fromBtoA>, RoleA<RoleBroadcast>, NameB>;

// Creating the MP sessions
type EndpointA = MeshedChannelsTwo<RecursAtoB, RoleB<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsTwo<Choose0fromBtoA, RoleBroadcast, NameB>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromBtoA::Done(s) => {
            s.close()
        },
        Branching0fromBtoA::Forward(s) => {
            let s = s.send(())?;
            endpoint_a(s)
        },
        Branching0fromBtoA::Backward(s) => {
            let (_, s) = s.recv()?;
            endpoint_a(s)
        },
    })
}

#[inline]
fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    let mut temp_s = s;

    for i in 1..LOOPS {
        temp_s = recurs_b(temp_s, i)?;
    }

    let s = choose_mpst_b_to_all!(temp_s, Branching0fromBtoA::Done);

    s.close()
}

fn recurs_b(s: EndpointB, index: i64) -> Result<EndpointB, Box<dyn Error>> {
    match index {
        i if i % 2 == 0 => {
            let s: EndpointForwardB = choose_mpst_b_to_all!(s, Branching0fromBtoA::Forward);

            let (_, s) = s.recv()?;

            Ok(s)
        }
        _ => {
            let s: EndpointBackwardB = choose_mpst_b_to_all!(s, Branching0fromBtoA::Backward);

            let s = s.send(())?;

            Ok(s)
        }
    }
}

fn all_mpst() {
    let (thread_a, thread_b) = fork_mpst(black_box(endpoint_a), black_box(endpoint_b));

    thread_a.join().unwrap();
    thread_b.join().unwrap();
}


/////////////////////////

static LOOPS: i64 = 100;

pub fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(
        &format!("ring two baking inline protocol MPST {LOOPS}"),
        |b| b.iter(all_mpst),
    );
}

