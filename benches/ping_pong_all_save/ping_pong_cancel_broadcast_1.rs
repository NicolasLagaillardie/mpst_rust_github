#![allow(clippy::type_complexity)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    broadcast_cancel, bundle_struct_fork_close_multi,
    create_fn_choose_mpst_cancel_multi_to_all_bundle, create_multiple_normal_name,
    create_multiple_normal_role, create_recv_mpst_session_bundle, create_send_check_cancel_bundle,
    offer_cancel_mpst,
};

use std::error::Error;

// use std::time::Duration;

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsThree, 3);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
    RoleCentral, RoleCentralDual |
);

// Create new names
create_multiple_normal_name!(NameA, NameB, NameCentral);

// Create new send functions
// A
create_send_check_cancel_bundle!(
    send_mpst_a_to_b, RoleB, 2 | =>
    NameA, MeshedChannelsThree, 3
);

// B
create_send_check_cancel_bundle!(
    send_mpst_b_to_a, RoleA, 2 | =>
    NameB, MeshedChannelsThree, 3
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 2 | =>
    NameA, MeshedChannelsThree, 3
);

// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 2 | =>
    NameB, MeshedChannelsThree, 3
);

// Types
// A
type Choose0fromAtoB = <RecursBtoA as Session>::Dual;

// B
enum Branching0fromAtoB {
    More(
        MeshedChannelsThree<
            End,
            Recv<(), Send<(), RecursBtoA>>,
            RoleA<RoleA<RoleA<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsThree<End, End, RoleEnd, NameB>),
}
type RecursBtoA = Recv<(End, Branching0fromAtoB), End>;

// Creating the MP sessions

type EndpointDoneA = MeshedChannelsThree<End, End, RoleEnd, NameA>;
type EndpointForwardA = MeshedChannelsThree<
    End,
    Send<(), Recv<(), Choose0fromAtoB>>,
    RoleB<RoleB<RoleBroadcast>>,
    NameA,
>;

type EndpointCentral = MeshedChannelsThree<End, End, RoleEnd, NameCentral>;
type EndpointA = MeshedChannelsThree<End, Choose0fromAtoB, RoleBroadcast, NameA>;
type EndpointB = MeshedChannelsThree<End, RecursBtoA, RoleA<RoleEnd>, NameB>;

create_fn_choose_mpst_cancel_multi_to_all_bundle!(
    done_from_a_to_all, more_from_a_to_all, =>
    Done, More, =>
    EndpointDoneA, EndpointForwardA, =>
    Branching0fromAtoB, =>
    NameB, =>
    NameCentral, NameA, MeshedChannelsThree, 2
);

// Functions
fn endpoint_central(s: EndpointCentral) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, 3)
}

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    recurs_a(s, LOOPS)
}

fn recurs_a(s: EndpointA, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_a_to_all(s)?;

            close_mpst_multi(s)
        }
        i => {
            let s = more_from_a_to_all(s)?;

            let s = send_mpst_a_to_b((), s)?;
            let ((), s) = recv_mpst_a_from_b(s)?;

            recurs_a(s, i - 1)
        }
    }
}

fn recurs_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_b_from_a, {
        Branching0fromAtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromAtoB::More(s) => {
            let ((), s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_a((), s)?;
            recurs_b(s)
        },
    })
}

fn aux() {
    let (thread_central, thread_a, thread_b) = fork_mpst(
        black_box(endpoint_central),
        black_box(endpoint_a),
        black_box(recurs_b),
    );

    thread_central.join().unwrap();
    thread_a.join().unwrap();
    thread_b.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 1;

pub fn ping_pong_protocol_mpst_cancel_broadcast(c: &mut Criterion) {
    c.bench_function(
        &format!("ping pong cancel broadcast protocol {LOOPS}"),
        |b| b.iter(aux),
    );
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(20000);
    targets = ping_pong_protocol_mpst_cancel_broadcast
}

criterion_main! {
    bench
}
