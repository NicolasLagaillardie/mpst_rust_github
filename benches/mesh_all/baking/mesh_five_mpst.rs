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
baker!("recursive", MeshedChannelsFive, A, B, C, D, E);

// Types
// Send/Recv
type RS = Recv<(), Send<(), End>>;
type SR = Send<(), Recv<(), End>>;

// Roles
type R2A<R> = RoleA<RoleA<R>>;
type R2B<R> = RoleB<RoleB<R>>;
type R2C<R> = RoleC<RoleC<R>>;
type R2D<R> = RoleD<RoleD<R>>;
type R2E<R> = RoleE<RoleE<R>>;

// A
enum Branching0fromEtoA {
    More(
        MeshedChannelsFive<
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursAtoE>>,
            R2E<R2B<R2C<R2D<RoleE<RoleEnd>>>>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoE = Recv<Branching0fromEtoA, End>;

// B
enum Branching0fromEtoB {
    More(
        MeshedChannelsFive<
            SR,
            RS,
            RS,
            Recv<(), Send<(), RecursBtoE>>,
            R2E<R2A<R2C<R2D<RoleE<RoleEnd>>>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoE = Recv<Branching0fromEtoB, End>;

// C
enum Branching0fromEtoC {
    More(
        MeshedChannelsFive<
            SR,
            SR,
            RS,
            Recv<(), Send<(), RecursCtoE>>,
            R2E<R2A<R2B<R2D<RoleE<RoleEnd>>>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoE = Recv<Branching0fromEtoC, End>;

// D
enum Branching0fromEtoD {
    More(
        MeshedChannelsFive<
            SR,
            SR,
            SR,
            Recv<(), Send<(), RecursDtoE>>,
            R2E<R2A<R2B<R2C<RoleE<RoleEnd>>>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoE = Recv<Branching0fromEtoD, End>;

// E
type Choose0fromEtoA = Send<Branching0fromEtoA, End>;
type Choose0fromEtoB = Send<Branching0fromEtoB, End>;
type Choose0fromEtoC = Send<Branching0fromEtoC, End>;
type Choose0fromEtoD = Send<Branching0fromEtoD, End>;
type EndpointMoreE = MeshedChannelsFive<
    Send<(), Recv<(), Choose0fromEtoA>>,
    Send<(), Recv<(), Choose0fromEtoB>>,
    Send<(), Recv<(), Choose0fromEtoC>>,
    Send<(), Recv<(), Choose0fromEtoD>>,
    R2A<R2B<R2C<R2D<RoleBroadcast>>>>,
    NameE,
>;

// Creating the MP sessions
type EndpointA = MeshedChannelsFive<End, End, End, RecursAtoE, RoleE<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsFive<End, End, End, RecursBtoE, RoleE<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsFive<End, End, End, RecursCtoE, RoleE<RoleEnd>, NameC>;
type EndpointD = MeshedChannelsFive<End, End, End, RecursDtoE, RoleE<RoleEnd>, NameD>;
type EndpointE = MeshedChannelsFive<
    Choose0fromEtoA,
    Choose0fromEtoB,
    Choose0fromEtoC,
    Choose0fromEtoD,
    RoleBroadcast,
    NameE,
>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromEtoA::Done(s) => {
            s.close()
        },
        Branching0fromEtoA::More(s) => {
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromEtoB::Done(s) => {
            s.close()
        },
        Branching0fromEtoB::More(s) => {
            let (_, s) = s.recv();
            let s = s.send(());
            let s = s.send(());
            let (_, s) = s.recv();
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromEtoC::Done(s) => {
            s.close()
        },
        Branching0fromEtoC::More(s) => {
            let (_, s) = s.recv();
            let s = s.send(());
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let (_, s) = s.recv();
            let s = s.send(());
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromEtoD::Done(s) => {
            s.close()
        },
        Branching0fromEtoD::More(s) => {
            let (_, s) = s.recv();
            let s = s.send(());
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    recurs_e(s, LOOPS)
}

fn recurs_e(s: EndpointE, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_e_to_all!(
                s,
                Branching0fromEtoA::Done,
                Branching0fromEtoB::Done,
                Branching0fromEtoC::Done,
                Branching0fromEtoD::Done,
            );

            s.close()
        }
        i => {
            let s: EndpointMoreE = choose_mpst_e_to_all!(
                s,
                Branching0fromEtoA::More,
                Branching0fromEtoB::More,
                Branching0fromEtoC::More,
                Branching0fromEtoD::More,
            );

            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();

            recurs_e(s, i - 1)
        }
    }
}

fn all_mpst() {
    let (thread_a, thread_b, thread_c, thread_d, thread_e) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
        black_box(endpoint_d),
        black_box(endpoint_e),
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn mesh_protocol_mpst(c: &mut Criterion) {
    c.bench_function(
        &format!("mesh five baking protocol MPST {LOOPS}"),
        |b| b.iter(all_mpst)
    );
}




/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.1).sample_size(10000);
    targets = mesh_protocol_mpst,
}

criterion_main! {
    bench
}

