#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    broadcast_cancel, bundle_struct_fork_close_multi,
    create_fn_choose_mpst_cancel_multi_to_all_bundle, create_multiple_normal_role_short,
    create_recv_mpst_session_bundle, create_send_check_cancel_bundle, offer_cancel_mpst,
};

use std::error::Error;
use std::time::Duration;

// Create the new SessionMpst for six participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstSeven, 7);

// Create new roles
// normal
create_multiple_normal_role_short!(Central, A, B, C, D, E, F);

// Create new send functions
// A
create_send_check_cancel_bundle!(
    send_mpst_a_to_b, RoleB, 2 | =>
    RoleA, SessionMpstSeven, 7
);
// B
create_send_check_cancel_bundle!(
    send_mpst_b_to_a, RoleA, 2 |
    send_mpst_b_to_c, RoleC, 3 | =>
    RoleB, SessionMpstSeven, 7
);
// C
create_send_check_cancel_bundle!(
    send_mpst_c_to_b, RoleB, 3 |
    send_mpst_c_to_d, RoleD, 4 | =>
    RoleC, SessionMpstSeven, 7
);
// D
create_send_check_cancel_bundle!(
    send_mpst_d_to_c, RoleC, 4 |
    send_mpst_d_to_e, RoleE, 5 | =>
    RoleD, SessionMpstSeven, 7
);
// E
create_send_check_cancel_bundle!(
    send_mpst_e_to_d, RoleD, 5 |
    send_mpst_e_to_f, RoleF, 6 | =>
    RoleE, SessionMpstSeven, 7
);
// F
create_send_check_cancel_bundle!(
    send_mpst_f_to_e, RoleE, 6 | =>
    RoleF, SessionMpstSeven, 7
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 2 |
    recv_mpst_a_from_f, RoleF, 6 | =>
    RoleA, SessionMpstSeven, 7
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 2 |
    recv_mpst_b_from_c, RoleC, 3 |
    recv_mpst_b_from_f, RoleF, 6 | =>
    RoleB, SessionMpstSeven, 7
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_b, RoleB, 3 |
    recv_mpst_c_from_d, RoleD, 4 |
    recv_mpst_c_from_f, RoleF, 6 | =>
    RoleC, SessionMpstSeven, 7
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_c, RoleC, 4 |
    recv_mpst_d_from_e, RoleE, 5 |
    recv_mpst_d_from_f, RoleF, 6 | =>
    RoleD, SessionMpstSeven, 7
);
// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_d, RoleD, 5 |
    recv_mpst_e_from_f, RoleF, 6 | =>
    RoleE, SessionMpstSeven, 7
);
// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_from_e, RoleE, 6 | =>
    RoleF, SessionMpstSeven, 7
);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;
type NameE = RoleE<RoleEnd>;
type NameF = RoleF<RoleEnd>;

// Types
// A
enum Branching0fromFtoA {
    Forward(
        SessionMpstSeven<
            End,
            Send<(), End>,
            End,
            End,
            End,
            RecursAtoF,
            RoleB<RoleF<RoleEnd>>,
            NameA,
        >,
    ),
    Backward(
        SessionMpstSeven<
            End,
            Recv<(), End>,
            End,
            End,
            End,
            RecursAtoF,
            RoleB<RoleF<RoleEnd>>,
            NameA,
        >,
    ),
    Done(SessionMpstSeven<End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoF = <Choose0fromFtoA as Session>::Dual;
// B
enum Branching0fromFtoB {
    Forward(
        SessionMpstSeven<
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            RecursBtoF,
            RoleA<RoleC<RoleF<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        SessionMpstSeven<
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            RecursBtoF,
            RoleC<RoleA<RoleF<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(SessionMpstSeven<End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoF = <Choose0fromFtoB as Session>::Dual;
// C
enum Branching0fromFtoC {
    Forward(
        SessionMpstSeven<
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            RecursCtoF,
            RoleB<RoleD<RoleF<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        SessionMpstSeven<
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            RecursCtoF,
            RoleD<RoleB<RoleF<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(SessionMpstSeven<End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoF = <Choose0fromFtoC as Session>::Dual;
// D
enum Branching0fromFtoD {
    Forward(
        SessionMpstSeven<
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            RecursDtoF,
            RoleC<RoleE<RoleF<RoleEnd>>>,
            NameD,
        >,
    ),
    Backward(
        SessionMpstSeven<
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            RecursDtoF,
            RoleE<RoleC<RoleF<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(SessionMpstSeven<End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoF = <Choose0fromFtoD as Session>::Dual;
// E
enum Branching0fromFtoE {
    Forward(
        SessionMpstSeven<
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), RecursEtoF>,
            RoleD<RoleF<RoleF<RoleEnd>>>,
            NameE,
        >,
    ),
    Backward(
        SessionMpstSeven<
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), RecursEtoF>,
            RoleF<RoleD<RoleF<RoleEnd>>>,
            NameE,
        >,
    ),
    Done(SessionMpstSeven<End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoF = <Choose0fromFtoE as Session>::Dual;
// F
type Choose0fromFtoA = Send<(End, Branching0fromFtoA), End>;
type Choose0fromFtoB = Send<(End, Branching0fromFtoB), End>;
type Choose0fromFtoC = Send<(End, Branching0fromFtoC), End>;
type Choose0fromFtoD = Send<(End, Branching0fromFtoD), End>;
type Choose0fromFtoE = Send<(End, Branching0fromFtoE), End>;
type EndpointDoneF = SessionMpstSeven<End, End, End, End, End, End, RoleEnd, NameF>;
type EndpointForwardF = SessionMpstSeven<
    End,
    Choose0fromFtoA,
    Choose0fromFtoB,
    Choose0fromFtoC,
    Choose0fromFtoD,
    Recv<(), Choose0fromFtoE>,
    RoleE<RoleBroadcast>,
    NameF,
>;
type EndpointBackwardF = SessionMpstSeven<
    End,
    Choose0fromFtoA,
    Choose0fromFtoB,
    Choose0fromFtoC,
    Choose0fromFtoD,
    Send<(), Choose0fromFtoE>,
    RoleE<RoleBroadcast>,
    NameF,
>;

// Creating the MP sessions
type EndpointCentral =
    SessionMpstSeven<End, End, End, End, End, End, RoleEnd, RoleCentral<RoleEnd>>;
type EndpointA = SessionMpstSeven<End, End, End, End, End, RecursAtoF, RoleF<RoleEnd>, NameA>;
type EndpointB = SessionMpstSeven<End, End, End, End, End, RecursBtoF, RoleF<RoleEnd>, NameB>;
type EndpointC = SessionMpstSeven<End, End, End, End, End, RecursCtoF, RoleF<RoleEnd>, NameC>;
type EndpointD = SessionMpstSeven<End, End, End, End, End, RecursDtoF, RoleF<RoleEnd>, NameD>;
type EndpointE = SessionMpstSeven<End, End, End, End, End, RecursEtoF, RoleF<RoleEnd>, NameE>;
type EndpointF = SessionMpstSeven<
    End,
    Choose0fromFtoA,
    Choose0fromFtoB,
    Choose0fromFtoC,
    Choose0fromFtoD,
    Choose0fromFtoE,
    RoleBroadcast,
    NameF,
>;

create_fn_choose_mpst_cancel_multi_to_all_bundle!(
    done_from_f_to_all, forward_from_f_to_all, backward_from_f_to_all, =>
    Done, Forward, Backward, =>
    EndpointDoneF, EndpointForwardF, EndpointBackwardF, =>
    Branching0fromFtoA,
    Branching0fromFtoB,
    Branching0fromFtoC,
    Branching0fromFtoD,
    Branching0fromFtoE, =>
    RoleA, RoleB, RoleC, RoleD, RoleE, =>
    RoleCentral, RoleF, SessionMpstSeven, 7, 7
);

fn endpoint_central(s: EndpointCentral) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, RoleCentral, 7);
    Ok(())
}

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_a_from_f, {
        Branching0fromFtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromFtoA::Forward(s) => {
            let s = send_mpst_a_to_b((), s)?;
            endpoint_a(s)
        },
        Branching0fromFtoA::Backward(s) => {
            let (_, s) = recv_mpst_a_from_b(s)?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_b_from_f, {
        Branching0fromFtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromFtoB::Forward(s) => {
            let ((), s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_c((), s)?;
            endpoint_b(s)
        },
        Branching0fromFtoB::Backward(s) => {
            let ((), s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_a((), s)?;
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_c_from_f, {
        Branching0fromFtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromFtoC::Forward(s) => {
            let ((), s) = recv_mpst_c_from_b(s)?;
            let s = send_mpst_c_to_d((), s)?;
            endpoint_c(s)
        },
        Branching0fromFtoC::Backward(s) => {
            let ((), s) = recv_mpst_c_from_d(s)?;
            let s = send_mpst_c_to_b((), s)?;
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_d_from_f, {
        Branching0fromFtoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromFtoD::Forward(s) => {
            let ((), s) = recv_mpst_d_from_c(s)?;
            let s = send_mpst_d_to_e((), s)?;
            endpoint_d(s)
        },
        Branching0fromFtoD::Backward(s) => {
            let ((), s) = recv_mpst_d_from_e(s)?;
            let s = send_mpst_d_to_c((), s)?;
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_e_from_f, {
        Branching0fromFtoE::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromFtoE::Forward(s) => {
            let ((), s) = recv_mpst_e_from_d(s)?;
            let s = send_mpst_e_to_f((), s)?;
            endpoint_e(s)
        },
        Branching0fromFtoE::Backward(s) => {
            let ((), s) = recv_mpst_e_from_f(s)?;
            let s = send_mpst_e_to_d((), s)?;
            endpoint_e(s)
        },
    })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    recurs_f(s, SIZE)
}

fn recurs_f(s: EndpointF, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_f_to_all(s)?;

            close_mpst_multi(s)
        }
        i if i % 2 == 0 => {
            let s = forward_from_f_to_all(s)?;

            let (_, s) = recv_mpst_f_from_e(s)?;

            recurs_f(s, i - 1)
        }
        i => {
            let s = backward_from_f_to_all(s)?;

            let s = send_mpst_f_to_e((), s)?;

            recurs_f(s, i - 1)
        }
    }
}

fn all_mpst() -> Result<(), Box<dyn std::any::Any + std::marker::Send>> {
    let (thread_central, thread_a, thread_b, thread_c, thread_d, thread_e, thread_f) = fork_mpst(
        black_box(endpoint_central),
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
        black_box(endpoint_d),
        black_box(endpoint_e),
        black_box(endpoint_f),
    );

    thread_central.join()?;
    thread_a.join()?;
    thread_b.join()?;
    thread_c.join()?;
    thread_d.join()?;
    thread_e.join()?;
    thread_f.join()?;

    Ok(())
}

/////////////////////////

static SIZE: i64 = 100;

fn ring_six_mpst(c: &mut Criterion) {
    c.bench_function(&format!("ring six cancel protocol MPST {}", SIZE), |b| {
        b.iter(|| all_mpst())
    });
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(1800, 0))
}

criterion_group! {
    name = ring_six;
    // config = long_warmup();
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = ring_six_mpst
}

criterion_main!(ring_six);
