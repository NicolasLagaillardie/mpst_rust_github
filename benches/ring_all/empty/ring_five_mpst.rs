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
    bundle_struct_fork_close_multi, create_fn_choose_mpst_multi_to_all_bundle,
    create_multiple_normal_name_short, create_multiple_normal_role_short,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, offer_mpst,
};

use std::error::Error;

// use std::time::Duration;

// Create the new MeshedChannels for five participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannels, 5);

// Create new roles
// normal
create_multiple_normal_role_short!(A, B, C, D, E);

// Create new names
create_multiple_normal_name_short!(A, B, C, D, E);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, 1 | =>
    NameA, MeshedChannels, 5
);

// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a, RoleA, 1 |
    send_mpst_b_to_c, RoleC, 2 | =>
    NameB, MeshedChannels, 5
);

// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_b, RoleB, 2 |
    send_mpst_c_to_d, RoleD, 3 | =>
    NameC, MeshedChannels, 5
);

// D
create_send_mpst_session_bundle!(
    send_mpst_d_to_c, RoleC, 3 |
    send_mpst_d_to_e, RoleE, 4 | =>
    NameD, MeshedChannels, 5
);

// E
create_send_mpst_session_bundle!(
    send_mpst_e_to_d, RoleD, 4 | =>
    NameE, MeshedChannels, 5
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 1 |
    recv_mpst_a_from_e, RoleE, 4 | =>
    NameA, MeshedChannels, 5
);

// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 1 |
    recv_mpst_b_from_c, RoleC, 2 |
    recv_mpst_b_from_e, RoleE, 4 | =>
    NameB, MeshedChannels, 5
);

// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_b, RoleB, 2 |
    recv_mpst_c_from_d, RoleD, 3 |
    recv_mpst_c_from_e, RoleE, 4 | =>
    NameC, MeshedChannels, 5
);

// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_c, RoleC, 3 |
    recv_mpst_d_from_e, RoleE, 4 | =>
    NameD, MeshedChannels, 5
);

// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_d, RoleD, 4 | =>
    NameE, MeshedChannels, 5
);

// Types
// A
enum Branching0fromEtoA {
    Forward(MeshedChannels<Send<(), End>, End, End, RecursAtoE, RoleB<RoleE<RoleEnd>>, NameA>),
    Backward(MeshedChannels<Recv<(), End>, End, End, RecursAtoE, RoleB<RoleE<RoleEnd>>, NameA>),
    Done(MeshedChannels<End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoE = <Choose0fromEtoA as Session>::Dual;

// B
enum Branching0fromEtoB {
    Forward(
        MeshedChannels<
            Recv<(), End>,
            Send<(), End>,
            End,
            RecursBtoE,
            RoleA<RoleC<RoleE<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannels<
            Send<(), End>,
            Recv<(), End>,
            End,
            RecursBtoE,
            RoleC<RoleA<RoleE<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoE = <Choose0fromEtoB as Session>::Dual;

// C
enum Branching0fromEtoC {
    Forward(
        MeshedChannels<
            End,
            Recv<(), End>,
            Send<(), End>,
            RecursCtoE,
            RoleB<RoleD<RoleE<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            Send<(), End>,
            Recv<(), End>,
            RecursCtoE,
            RoleD<RoleB<RoleE<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoE = <Choose0fromEtoC as Session>::Dual;

// D
enum Branching0fromEtoD {
    Forward(
        MeshedChannels<
            End,
            End,
            Recv<(), End>,
            Send<(), RecursDtoE>,
            RoleC<RoleE<RoleE<RoleEnd>>>,
            NameD,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            End,
            Send<(), End>,
            Recv<(), RecursDtoE>,
            RoleE<RoleC<RoleE<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoE = <Choose0fromEtoD as Session>::Dual;

// E
type Choose0fromEtoA = Send<Branching0fromEtoA, End>;
type Choose0fromEtoB = Send<Branching0fromEtoB, End>;
type Choose0fromEtoC = Send<Branching0fromEtoC, End>;
type Choose0fromEtoD = Send<Branching0fromEtoD, End>;
type EndpointDoneE = MeshedChannels<End, End, End, End, RoleEnd, NameE>;
type EndpointForwardE = MeshedChannels<
    Choose0fromEtoA,
    Choose0fromEtoB,
    Choose0fromEtoC,
    Recv<(), Choose0fromEtoD>,
    RoleD<RoleBroadcast>,
    NameE,
>;
type EndpointBackwardE = MeshedChannels<
    Choose0fromEtoA,
    Choose0fromEtoB,
    Choose0fromEtoC,
    Send<(), Choose0fromEtoD>,
    RoleD<RoleBroadcast>,
    NameE,
>;

// Creating the MP sessions
type EndpointA = MeshedChannels<End, End, End, RecursAtoE, RoleE<RoleEnd>, NameA>;
type EndpointB = MeshedChannels<End, End, End, RecursBtoE, RoleE<RoleEnd>, NameB>;
type EndpointC = MeshedChannels<End, End, End, RecursCtoE, RoleE<RoleEnd>, NameC>;
type EndpointD = MeshedChannels<End, End, End, RecursDtoE, RoleE<RoleEnd>, NameD>;
type EndpointE = MeshedChannels<
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
    NameA, NameB, NameC, NameD, =>
    NameE, MeshedChannels, 5
);

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_e, {
        Branching0fromEtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromEtoA::Forward(s) => {
            let s = send_mpst_a_to_b((), s);
            endpoint_a(s)
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
            let s = send_mpst_b_to_c((), s);
            endpoint_b(s)
        },
        Branching0fromEtoB::Backward(s) => {
            let ((), s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_a((), s);
            endpoint_b(s)
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
            let s = send_mpst_c_to_d((), s);
            endpoint_c(s)
        },
        Branching0fromEtoC::Backward(s) => {
            let ((), s) = recv_mpst_c_from_d(s)?;
            let s = send_mpst_c_to_b((), s);
            endpoint_c(s)
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
            let s = send_mpst_d_to_e((), s);
            endpoint_d(s)
        },
        Branching0fromEtoD::Backward(s) => {
            let ((), s) = recv_mpst_d_from_e(s)?;
            let s = send_mpst_d_to_c((), s);
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

            let s = send_mpst_e_to_d((), s);

            recurs_e(s, i - 1)
        }
    }
}

fn aux() {
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

static LOOPS: i64 = 0;

pub fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("ring five empty protocol {LOOPS}"), |b| {
        b.iter(aux)
    });
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
