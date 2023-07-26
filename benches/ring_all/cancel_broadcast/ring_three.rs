#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    broadcast_cancel, bundle_struct_fork_close_multi,
    create_fn_choose_mpst_cancel_multi_to_all_bundle, create_multiple_normal_name_short,
    create_multiple_normal_role_short, create_recv_mpst_session_bundle,
    create_send_check_cancel_bundle, offer_cancel_mpst,
};

use std::error::Error;

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannels, 4);

// Create new roles
// normal
create_multiple_normal_role_short!(Central, A, B, C);

// Create new names
create_multiple_normal_name_short!(Central, A, B, C);

// Create new send functions
// A
create_send_check_cancel_bundle!(
    send_mpst_a_to_b, RoleB, 2 | =>
    NameA, MeshedChannels, 4
);

// B
create_send_check_cancel_bundle!(
    send_mpst_b_to_a, RoleA, 2 |
    send_mpst_b_to_c, RoleC, 3 | =>
    NameB, MeshedChannels, 4
);

// C
create_send_check_cancel_bundle!(
    send_mpst_c_to_b, RoleB, 3 | =>
    NameC, MeshedChannels, 4
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 2 |
    recv_mpst_a_from_c, RoleC, 3 | =>
    NameA, MeshedChannels, 4
);

// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 2 |
    recv_mpst_b_from_c, RoleC, 3 | =>
    NameB, MeshedChannels, 4
);

// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_b, RoleB, 3 | =>
    NameC, MeshedChannels, 4
);

// Types
// A
enum Branching0fromCtoA {
    Forward(MeshedChannels<End, Send<(), End>, RecursAtoC, RoleB<RoleC<RoleEnd>>, NameA>),
    Backward(MeshedChannels<End, Recv<(), End>, RecursAtoC, RoleB<RoleC<RoleEnd>>, NameA>),
    Done(MeshedChannels<End, End, End, RoleEnd, NameA>),
}
type RecursAtoC = <Choose0fromCtoA as Session>::Dual;

// B
enum Branching0fromCtoB {
    Forward(
        MeshedChannels<
            End,
            Recv<(), End>,
            Send<(), RecursBtoC>,
            RoleA<RoleC<RoleC<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            Send<(), End>,
            Recv<(), RecursBtoC>,
            RoleC<RoleA<RoleC<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannels<End, End, End, RoleEnd, NameB>),
}
type RecursBtoC = <Choose0fromCtoB as Session>::Dual;

// C
type Choose0fromCtoA = Send<(End, Branching0fromCtoA), End>;
type Choose0fromCtoB = Send<(End, Branching0fromCtoB), End>;
type EndpointDoneC = MeshedChannels<End, End, End, RoleEnd, NameC>;
type EndpointForwardC =
    MeshedChannels<End, Choose0fromCtoA, Recv<(), Choose0fromCtoB>, RoleB<RoleBroadcast>, NameC>;
type EndpointBackwardC =
    MeshedChannels<End, Choose0fromCtoA, Send<(), Choose0fromCtoB>, RoleB<RoleBroadcast>, NameC>;

// Creating the MP sessions
type EndpointCentral = MeshedChannels<End, End, End, RoleEnd, NameCentral>;
type EndpointA = MeshedChannels<End, End, RecursAtoC, RoleC<RoleEnd>, NameA>;
type EndpointB = MeshedChannels<End, End, RecursBtoC, RoleC<RoleEnd>, NameB>;
type EndpointC = MeshedChannels<End, Choose0fromCtoA, Choose0fromCtoB, RoleBroadcast, NameC>;

create_fn_choose_mpst_cancel_multi_to_all_bundle!(
    done_from_c_to_all, forward_from_c_to_all, backward_from_c_to_all, =>
    Done, Forward, Backward, =>
    EndpointDoneC, EndpointForwardC, EndpointBackwardC, =>
    Branching0fromCtoA, Branching0fromCtoB, =>
    NameA, NameB, =>
    NameCentral, NameC, MeshedChannels, 4
);

fn endpoint_central(s: EndpointCentral) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, 4)
}

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_a_from_c, {
        Branching0fromCtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromCtoA::Forward(s) => {
            let s = send_mpst_a_to_b((), s)?;
            endpoint_a(s)
        },
        Branching0fromCtoA::Backward(s) => {
            let (_, s) = recv_mpst_a_from_b(s)?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_b_from_c, {
        Branching0fromCtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromCtoB::Forward(s) => {
            let ((), s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_c((), s)?;
            endpoint_b(s)
        },
        Branching0fromCtoB::Backward(s) => {
            let ((), s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_a((), s)?;
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
            let s = done_from_c_to_all(s)?;

            close_mpst_multi(s)
        }
        i if i % 2 == 0 => {
            let s = forward_from_c_to_all(s)?;

            let (_, s) = recv_mpst_c_from_b(s)?;

            recurs_c(s, i - 1)
        }
        i => {
            let s = backward_from_c_to_all(s)?;

            let s = send_mpst_c_to_b((), s)?;

            recurs_c(s, i - 1)
        }
    }
}

fn aux() {
    let (thread_central, thread_a, thread_b, thread_c) = fork_mpst(
        black_box(endpoint_central),
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
    );

    thread_central.join().unwrap();
    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(
        &format!("ring three cancel broadcast protocol {LOOPS}"),
        |b| b.iter(aux),
    );
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(20000);
    targets = ring_protocol_mpst,
}

criterion_main! {
    bench
}
