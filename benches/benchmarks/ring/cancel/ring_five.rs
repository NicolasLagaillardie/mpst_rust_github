#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi_cancel, create_fn_choose_mpst_multi_to_all_bundle,
    create_multiple_normal_role_short, create_recv_mpst_session_bundle,
    create_send_mpst_cancel_bundle, offer_mpst,
};

use std::error::Error;
use std::time::Duration;

// Create the new MeshedChannels for five participants and the close and fork functions
bundle_struct_fork_close_multi_cancel!(close_mpst_multi, fork_mpst, MeshedChannelsFive, 5);

// Create new roles
// normal
create_multiple_normal_role_short!(A, B, C, D, E);

// Create new send functions
// A
create_send_mpst_cancel_bundle!(
    send_mpst_a_to_b, RoleB, 1 | =>
    RoleA, MeshedChannelsFive, 5
);
// B
create_send_mpst_cancel_bundle!(
    send_mpst_b_to_a, RoleA, 1 |
    send_mpst_b_to_c, RoleC, 2 | =>
    RoleB, MeshedChannelsFive, 5
);
// C
create_send_mpst_cancel_bundle!(
    send_mpst_c_to_b, RoleB, 2 |
    send_mpst_c_to_d, RoleD, 3 | =>
    RoleC, MeshedChannelsFive, 5
);
// D
create_send_mpst_cancel_bundle!(
    send_mpst_d_to_c, RoleC, 3 |
    send_mpst_d_to_e, RoleE, 4 | =>
    RoleD, MeshedChannelsFive, 5
);
// E
create_send_mpst_cancel_bundle!(
    send_mpst_e_to_d, RoleD, 4 | =>
    RoleE, MeshedChannelsFive, 5
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 1 |
    recv_mpst_a_from_e, RoleE, 4 | =>
    RoleA, MeshedChannelsFive, 5
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 1 |
    recv_mpst_b_from_c, RoleC, 2 |
    recv_mpst_b_from_e, RoleE, 4 | =>
    RoleB, MeshedChannelsFive, 5
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_b, RoleB, 2 |
    recv_mpst_c_from_d, RoleD, 3 |
    recv_mpst_c_from_e, RoleE, 4 | =>
    RoleC, MeshedChannelsFive, 5
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_c, RoleC, 3 |
    recv_mpst_d_from_e, RoleE, 4 | =>
    RoleD, MeshedChannelsFive, 5
);
// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_d, RoleD, 4 | =>
    RoleE, MeshedChannelsFive, 5
);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;
type NameE = RoleE<RoleEnd>;

// Types
// A
enum Branching0fromEtoA {
    Forward(MeshedChannelsFive<Send<(), End>, End, End, RecursAtoE, RoleB<RoleE<RoleEnd>>, NameA>),
    Backward(MeshedChannelsFive<Recv<(), End>, End, End, RecursAtoE, RoleB<RoleE<RoleEnd>>, NameA>),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoE = <Choose0fromEtoA as Session>::Dual;
// B
enum Branching0fromEtoB {
    Forward(
        MeshedChannelsFive<
            Recv<(), End>,
            Send<(), End>,
            End,
            RecursBtoE,
            RoleA<RoleC<RoleE<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannelsFive<
            Send<(), End>,
            Recv<(), End>,
            End,
            RecursBtoE,
            RoleC<RoleA<RoleE<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoE = <Choose0fromEtoB as Session>::Dual;
// C
enum Branching0fromEtoC {
    Forward(
        MeshedChannelsFive<
            End,
            Recv<(), End>,
            Send<(), End>,
            RecursCtoE,
            RoleB<RoleD<RoleE<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannelsFive<
            End,
            Send<(), End>,
            Recv<(), End>,
            RecursCtoE,
            RoleD<RoleB<RoleE<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoE = <Choose0fromEtoC as Session>::Dual;
// D
enum Branching0fromEtoD {
    Forward(
        MeshedChannelsFive<
            End,
            End,
            Recv<(), End>,
            Send<(), RecursDtoE>,
            RoleC<RoleE<RoleE<RoleEnd>>>,
            NameD,
        >,
    ),
    Backward(
        MeshedChannelsFive<
            End,
            End,
            Send<(), End>,
            Recv<(), RecursDtoE>,
            RoleE<RoleC<RoleE<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoE = <Choose0fromEtoD as Session>::Dual;
// E
type Choose0fromEtoA = Send<Branching0fromEtoA, End>;
type Choose0fromEtoB = Send<Branching0fromEtoB, End>;
type Choose0fromEtoC = Send<Branching0fromEtoC, End>;
type Choose0fromEtoD = Send<Branching0fromEtoD, End>;
type EndpointDoneE = MeshedChannelsFive<End, End, End, End, RoleEnd, NameE>;
type EndpointForwardE = MeshedChannelsFive<
    Choose0fromEtoA,
    Choose0fromEtoB,
    Choose0fromEtoC,
    Recv<(), Choose0fromEtoD>,
    RoleD<RoleBroadcast>,
    NameE,
>;
type EndpointBackwardE = MeshedChannelsFive<
    Choose0fromEtoA,
    Choose0fromEtoB,
    Choose0fromEtoC,
    Send<(), Choose0fromEtoD>,
    RoleD<RoleBroadcast>,
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

create_fn_choose_mpst_multi_to_all_bundle!(
    done_from_e_to_all, forward_from_e_to_all, backward_from_e_to_all, =>
    Done, Forward, Backward, =>
    EndpointDoneE, EndpointForwardE, EndpointBackwardE, =>
    Branching0fromEtoA,
    Branching0fromEtoB,
    Branching0fromEtoC,
    Branching0fromEtoD, =>
    RoleA, RoleB, RoleC, RoleD, =>
    RoleE, MeshedChannelsFive, 5
);

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_e, {
        Branching0fromEtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromEtoA::Forward(s) => {
            let s = send_mpst_a_to_b((), s)?;            endpoint_a(s)
        },
        Branching0fromEtoA::Backward(s) => {
            let (_, s) = recv_mpst_a_from_b(s)?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_e, {
        Branching0fromEtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromEtoB::Forward(s) => {
            let ((), s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_c((), s)?;            endpoint_b(s)
        },
        Branching0fromEtoB::Backward(s) => {
            let ((), s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_a((), s)?;            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_e, {
        Branching0fromEtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromEtoC::Forward(s) => {
            let ((), s) = recv_mpst_c_from_b(s)?;
            let s = send_mpst_c_to_d((), s)?;            endpoint_c(s)
        },
        Branching0fromEtoC::Backward(s) => {
            let ((), s) = recv_mpst_c_from_d(s)?;
            let s = send_mpst_c_to_b((), s)?;            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_from_e, {
        Branching0fromEtoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromEtoD::Forward(s) => {
            let ((), s) = recv_mpst_d_from_c(s)?;
            let s = send_mpst_d_to_e((), s)?;            endpoint_d(s)
        },
        Branching0fromEtoD::Backward(s) => {
            let ((), s) = recv_mpst_d_from_e(s)?;
            let s = send_mpst_d_to_c((), s)?;            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    recurs_e(s, SIZE)
}

fn recurs_e(s: EndpointE, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_e_to_all(s);

            close_mpst_multi(s)
        }
        i if i % 2 == 0 => {
            let s = forward_from_e_to_all(s);

            let (_, s) = recv_mpst_e_from_d(s)?;

            recurs_e(s, i - 1)
        }
        i => {
            let s = backward_from_e_to_all(s);

            let s = send_mpst_e_to_d((), s)?;
            recurs_e(s, i - 1)
        }
    }
}

fn all_mpst() -> Result<(), Box<dyn std::any::Any + std::marker::Send>> {
    let (thread_a, thread_b, thread_c, thread_d, thread_e) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
        black_box(endpoint_d),
        black_box(endpoint_e),
    );

    thread_a.join()?;
    thread_b.join()?;
    thread_c.join()?;
    thread_d.join()?;
    thread_e.join()?;

    Ok(())
}

/////////////////////////

static SIZE: i64 = 100;

fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("ring five cancel protocol MPST {}", SIZE), |b| {
        b.iter(|| all_mpst())
    });
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(1800, 0))
}

criterion_group! {
    name = ring_five;
    // config = long_warmup();
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = ring_protocol_mpst
}

criterion_main!(ring_five);
