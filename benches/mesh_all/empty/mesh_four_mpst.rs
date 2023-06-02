use crossbeam_channel::bounded;

use criterion::{black_box, Criterion};

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose, create_fn_choose_mpst_multi_to_all_bundle,
    create_multiple_normal_name_short, create_multiple_normal_role_short,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, offer, offer_mpst,
};

use std::error::Error;
use std::thread::{spawn, JoinHandle};

// use std::time::Duration;

// Create the new MeshedChannels for four participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsFour, 4);

// Create new roles
// normal
create_multiple_normal_role_short!(A, B, C, D);

// Create new names
create_multiple_normal_name_short!(A, B, C, D);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, 1 |
    send_mpst_a_to_c, RoleC, 2 |
    send_mpst_a_to_d, RoleD, 3 | =>
    NameA, MeshedChannelsFour, 4
);

// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a, RoleA, 1 |
    send_mpst_b_to_c, RoleC, 2 |
    send_mpst_b_to_d, RoleD, 3 | =>
    NameB, MeshedChannelsFour, 4
);

// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a, RoleA, 1 |
    send_mpst_c_to_b, RoleB, 2 |
    send_mpst_c_to_d, RoleD, 3 | =>
    NameC, MeshedChannelsFour, 4
);

// D
create_send_mpst_session_bundle!(
    send_mpst_d_to_a, RoleA, 1 |
    send_mpst_d_to_b, RoleB, 2 |
    send_mpst_d_to_c, RoleC, 3 | =>
    NameD, MeshedChannelsFour, 4
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 1 |
    recv_mpst_a_from_c, RoleC, 2 |
    recv_mpst_a_from_d, RoleD, 3 | =>
    NameA, MeshedChannelsFour, 4
);

// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 1 |
    recv_mpst_b_from_c, RoleC, 2 |
    recv_mpst_b_from_d, RoleD, 3 | =>
    NameB, MeshedChannelsFour, 4
);

// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a, RoleA, 1 |
    recv_mpst_c_from_b, RoleB, 2 |
    recv_mpst_c_from_d, RoleD, 3 | =>
    NameC, MeshedChannelsFour, 4
);

// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_a, RoleA, 1 |
    recv_mpst_d_from_b, RoleB, 2 |
    recv_mpst_d_from_c, RoleC, 3 | =>
    NameD, MeshedChannelsFour, 4
);

// Types
// Send/Recv
type RS = Recv<(), Send<(), End>>;
type SR = Send<(), Recv<(), End>>;

// Roles
type R2A<R> = RoleA<RoleA<R>>;
type R2B<R> = RoleB<RoleB<R>>;
type R2C<R> = RoleC<RoleC<R>>;
type R2D<R> = RoleD<RoleD<R>>;

// A
enum Branching0fromDtoA {
    More(
        MeshedChannelsFour<
            RS,
            RS,
            Recv<(), Send<(), RecursAtoD>>,
            R2D<R2B<R2C<RoleD<RoleEnd>>>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsFour<End, End, End, RoleEnd, NameA>),
}
type RecursAtoD = Recv<Branching0fromDtoA, End>;

// B
enum Branching0fromDtoB {
    More(
        MeshedChannelsFour<
            SR,
            RS,
            Recv<(), Send<(), RecursBtoD>>,
            R2D<R2A<R2C<RoleD<RoleEnd>>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsFour<End, End, End, RoleEnd, NameB>),
}
type RecursBtoD = Recv<Branching0fromDtoB, End>;

// C
enum Branching0fromDtoC {
    More(
        MeshedChannelsFour<
            SR,
            SR,
            Recv<(), Send<(), RecursCtoD>>,
            R2D<R2A<R2B<RoleD<RoleEnd>>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsFour<End, End, End, RoleEnd, NameC>),
}
type RecursCtoD = Recv<Branching0fromDtoC, End>;

// D
type Choose0fromDtoA = Send<Branching0fromDtoA, End>;
type Choose0fromDtoB = Send<Branching0fromDtoB, End>;
type Choose0fromDtoC = Send<Branching0fromDtoC, End>;
type EndpointDoneD = MeshedChannelsFour<End, End, End, RoleEnd, NameD>;
type EndpointMoreD = MeshedChannelsFour<
    Send<(), Recv<(), Choose0fromDtoA>>,
    Send<(), Recv<(), Choose0fromDtoB>>,
    Send<(), Recv<(), Choose0fromDtoC>>,
    R2A<R2B<R2C<RoleBroadcast>>>,
    NameD,
>;

// Creating the MP sessions
type EndpointA = MeshedChannelsFour<End, End, RecursAtoD, RoleD<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsFour<End, End, RecursBtoD, RoleD<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsFour<End, End, RecursCtoD, RoleD<RoleEnd>, NameC>;
type EndpointD =
    MeshedChannelsFour<Choose0fromDtoA, Choose0fromDtoB, Choose0fromDtoC, RoleBroadcast, NameD>;

create_fn_choose_mpst_multi_to_all_bundle!(
    done_from_d_to_all, more_from_d_to_all, =>
    Done, More, =>
    EndpointDoneD, EndpointMoreD, =>
    Branching0fromDtoA,
    Branching0fromDtoB,
    Branching0fromDtoC, =>
    NameA,
    NameB,
    NameC, =>
    NameD, MeshedChannelsFour, 4
);

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_d, {
        Branching0fromDtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromDtoA::More(s) => {
            let (_, s) = recv_mpst_a_from_d(s)?;
            let s = send_mpst_a_to_d((), s);
            let (_, s) = recv_mpst_a_from_b(s)?;
            let s = send_mpst_a_to_b((), s);
            let (_, s) = recv_mpst_a_from_c(s)?;
            let s = send_mpst_a_to_c((), s);
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_d, {
        Branching0fromDtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromDtoB::More(s) => {
            let (_, s) = recv_mpst_b_from_d(s)?;
            let s = send_mpst_b_to_d((), s);
            let s = send_mpst_b_to_a((), s);
            let (_, s) = recv_mpst_b_from_a(s)?;
            let (_, s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_c((), s);
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_d, {
        Branching0fromDtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromDtoC::More(s) => {
            let (_, s) = recv_mpst_c_from_d(s)?;
            let s = send_mpst_c_to_d((), s);
            let s = send_mpst_c_to_a((), s);
            let (_, s) = recv_mpst_c_from_a(s)?;
            let s = send_mpst_c_to_b((), s);
            let (_, s) = recv_mpst_c_from_b(s)?;
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    recurs_d(s, LOOPS)
}

fn recurs_d(s: EndpointD, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_d_to_all(s);

            close_mpst_multi(s)
        }
        i => {
            let s = more_from_d_to_all(s);

            let s = send_mpst_d_to_a((), s);
            let (_, s) = recv_mpst_d_from_a(s)?;
            let s = send_mpst_d_to_b((), s);
            let (_, s) = recv_mpst_d_from_b(s)?;
            let s = send_mpst_d_to_c((), s);
            let (_, s) = recv_mpst_d_from_c(s)?;

            recurs_d(s, i - 1)
        }
    }
}

fn all_mpst() {
    let (thread_a, thread_b, thread_c, thread_d) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
        black_box(endpoint_d),
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
}


/////////////////////////

static LOOPS: i64 = 0;

pub fn mesh_protocol_mpst(c: &mut Criterion) {
    c.bench_function(
        &format!("mesh four empty protocol MPST {LOOPS}"),
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


