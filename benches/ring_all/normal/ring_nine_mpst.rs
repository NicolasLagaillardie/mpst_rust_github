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

// Create the new MeshedChannels for nine participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannels, 9);

// Create new roles
// normal
create_multiple_normal_role_short!(A, B, C, D, E, F, G, H, I);

// Create new names
create_multiple_normal_name_short!(A, B, C, D, E, F, G, H, I);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, 1 | =>
    NameA, MeshedChannels, 9
);

// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a, RoleA, 1 |
    send_mpst_b_to_c, RoleC, 2 | =>
    NameB, MeshedChannels, 9
);

// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_b, RoleB, 2 |
    send_mpst_c_to_d, RoleD, 3 | =>
    NameC, MeshedChannels, 9
);

// D
create_send_mpst_session_bundle!(
    send_mpst_d_to_c, RoleC, 3 |
    send_mpst_d_to_e, RoleE, 4 | =>
    NameD, MeshedChannels, 9
);

// E
create_send_mpst_session_bundle!(
    send_mpst_e_to_d, RoleD, 4 |
    send_mpst_e_to_f, RoleF, 5 | =>
    NameE, MeshedChannels, 9
);

// F
create_send_mpst_session_bundle!(
    send_mpst_f_to_e, RoleE, 5 |
    send_mpst_f_to_g, RoleG, 6 | =>
    NameF, MeshedChannels, 9
);

// G
create_send_mpst_session_bundle!(
    send_mpst_g_to_f, RoleF, 6 |
    send_mpst_g_to_h, RoleH, 7 | =>
    NameG, MeshedChannels, 9
);

// H
create_send_mpst_session_bundle!(
    send_mpst_h_to_g, RoleG, 7 |
    send_mpst_h_to_i, RoleI, 8 | =>
    NameH, MeshedChannels, 9
);

// I
create_send_mpst_session_bundle!(
    send_mpst_i_to_h, RoleH, 8 | =>
    NameI, MeshedChannels, 9
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 1 |
    recv_mpst_a_from_i, RoleI, 8 | =>
    NameA, MeshedChannels, 9
);

// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 1 |
    recv_mpst_b_from_c, RoleC, 2 |
    recv_mpst_b_from_i, RoleI, 8 | =>
    NameB, MeshedChannels, 9
);

// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_b, RoleB, 2 |
    recv_mpst_c_from_d, RoleD, 3 |
    recv_mpst_c_from_i, RoleI, 8 | =>
    NameC, MeshedChannels, 9
);

// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_c, RoleC, 3 |
    recv_mpst_d_from_e, RoleE, 4 |
    recv_mpst_d_from_i, RoleI, 8 | =>
    NameD, MeshedChannels, 9
);

// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_d, RoleD, 4 |
    recv_mpst_e_from_f, RoleF, 5 |
    recv_mpst_e_from_i, RoleI, 8 | =>
    NameE, MeshedChannels, 9
);

// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_from_e, RoleE, 5 |
    recv_mpst_f_from_g, RoleG, 6 |
    recv_mpst_f_from_i, RoleI, 8 | =>
    NameF, MeshedChannels, 9
);

// G
create_recv_mpst_session_bundle!(
    recv_mpst_g_from_f, RoleF, 6 |
    recv_mpst_g_from_h, RoleH, 7 |
    recv_mpst_g_from_i, RoleI, 8 | =>
    NameG, MeshedChannels, 9
);

// H
create_recv_mpst_session_bundle!(
    recv_mpst_h_from_g, RoleG, 7 |
    recv_mpst_h_from_i, RoleI, 8 | =>
    NameH, MeshedChannels, 9
);

// I
create_recv_mpst_session_bundle!(
    recv_mpst_i_from_h, RoleH, 8 | =>
    NameI, MeshedChannels, 9
);

// Types
// A
enum Branching0fromItoA {
    Forward(
        MeshedChannels<
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursAtoI,
            RoleB<RoleI<RoleEnd>>,
            NameA,
        >,
    ),
    Backward(
        MeshedChannels<
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursAtoI,
            RoleB<RoleI<RoleEnd>>,
            NameA,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoI = <Choose0fromItoA as Session>::Dual;

// B
enum Branching0fromItoB {
    Forward(
        MeshedChannels<
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            RecursBtoI,
            RoleA<RoleC<RoleI<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannels<
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            RecursBtoI,
            RoleC<RoleA<RoleI<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoI = <Choose0fromItoB as Session>::Dual;

// C
enum Branching0fromItoC {
    Forward(
        MeshedChannels<
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            RecursCtoI,
            RoleB<RoleD<RoleI<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            RecursCtoI,
            RoleD<RoleB<RoleI<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoI = <Choose0fromItoC as Session>::Dual;

// D
enum Branching0fromItoD {
    Forward(
        MeshedChannels<
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            RecursDtoI,
            RoleC<RoleE<RoleI<RoleEnd>>>,
            NameD,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            RecursDtoI,
            RoleE<RoleC<RoleI<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoI = <Choose0fromItoD as Session>::Dual;

// E
enum Branching0fromItoE {
    Forward(
        MeshedChannels<
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            RecursEtoI,
            RoleD<RoleF<RoleI<RoleEnd>>>,
            NameE,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            RecursEtoI,
            RoleF<RoleD<RoleI<RoleEnd>>>,
            NameE,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoI = <Choose0fromItoE as Session>::Dual;

// F
enum Branching0fromItoF {
    Forward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            RecursFtoI,
            RoleE<RoleG<RoleI<RoleEnd>>>,
            NameF,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            RecursFtoI,
            RoleG<RoleE<RoleI<RoleEnd>>>,
            NameF,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoI = <Choose0fromItoF as Session>::Dual;

// G
enum Branching0fromItoG {
    Forward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            RecursGtoI,
            RoleF<RoleH<RoleI<RoleEnd>>>,
            NameG,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            RecursGtoI,
            RoleH<RoleF<RoleI<RoleEnd>>>,
            NameG,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoI = <Choose0fromItoG as Session>::Dual;

// H
enum Branching0fromItoH {
    Forward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), RecursHtoI>,
            RoleG<RoleI<RoleI<RoleEnd>>>,
            NameH,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), RecursHtoI>,
            RoleI<RoleG<RoleI<RoleEnd>>>,
            NameH,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameH>),
}
type RecursHtoI = <Choose0fromItoH as Session>::Dual;

// I
type Choose0fromItoA = Send<Branching0fromItoA, End>;
type Choose0fromItoB = Send<Branching0fromItoB, End>;
type Choose0fromItoC = Send<Branching0fromItoC, End>;
type Choose0fromItoD = Send<Branching0fromItoD, End>;
type Choose0fromItoE = Send<Branching0fromItoE, End>;
type Choose0fromItoF = Send<Branching0fromItoF, End>;
type Choose0fromItoG = Send<Branching0fromItoG, End>;
type Choose0fromItoH = Send<Branching0fromItoH, End>;
type EndpointDoneI = MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameI>;
type EndpointForwardI = MeshedChannels<
    Choose0fromItoA,
    Choose0fromItoB,
    Choose0fromItoC,
    Choose0fromItoD,
    Choose0fromItoE,
    Choose0fromItoF,
    Choose0fromItoG,
    Recv<(), Choose0fromItoH>,
    RoleH<RoleBroadcast>,
    NameI,
>;
type EndpointBackwardI = MeshedChannels<
    Choose0fromItoA,
    Choose0fromItoB,
    Choose0fromItoC,
    Choose0fromItoD,
    Choose0fromItoE,
    Choose0fromItoF,
    Choose0fromItoG,
    Send<(), Choose0fromItoH>,
    RoleH<RoleBroadcast>,
    NameI,
>;

// Creating the MP sessions
type EndpointA =
    MeshedChannels<End, End, End, End, End, End, End, RecursAtoI, RoleI<RoleEnd>, NameA>;
type EndpointB =
    MeshedChannels<End, End, End, End, End, End, End, RecursBtoI, RoleI<RoleEnd>, NameB>;
type EndpointC =
    MeshedChannels<End, End, End, End, End, End, End, RecursCtoI, RoleI<RoleEnd>, NameC>;
type EndpointD =
    MeshedChannels<End, End, End, End, End, End, End, RecursDtoI, RoleI<RoleEnd>, NameD>;
type EndpointE =
    MeshedChannels<End, End, End, End, End, End, End, RecursEtoI, RoleI<RoleEnd>, NameE>;
type EndpointF =
    MeshedChannels<End, End, End, End, End, End, End, RecursFtoI, RoleI<RoleEnd>, NameF>;
type EndpointG =
    MeshedChannels<End, End, End, End, End, End, End, RecursGtoI, RoleI<RoleEnd>, NameG>;
type EndpointH =
    MeshedChannels<End, End, End, End, End, End, End, RecursHtoI, RoleI<RoleEnd>, NameH>;
type EndpointI = MeshedChannels<
    Choose0fromItoA,
    Choose0fromItoB,
    Choose0fromItoC,
    Choose0fromItoD,
    Choose0fromItoE,
    Choose0fromItoF,
    Choose0fromItoG,
    Choose0fromItoH,
    RoleBroadcast,
    NameI,
>;

create_fn_choose_mpst_multi_to_all_bundle!(
    done_from_i_to_all, forward_from_i_to_all, backward_from_i_to_all, =>
    Done, Forward, Backward, =>
    EndpointDoneI, EndpointForwardI, EndpointBackwardI, =>
    Branching0fromItoA,
    Branching0fromItoB,
    Branching0fromItoC,
    Branching0fromItoD,
    Branching0fromItoE,
    Branching0fromItoF,
    Branching0fromItoG,
    Branching0fromItoH, =>
    NameA, NameB, NameC, NameD, NameE, NameF, NameG, NameH, =>
    NameI, MeshedChannels, 9
);

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_i, {
        Branching0fromItoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoA::Forward(s) => {
            let s = send_mpst_a_to_b((), s);
            endpoint_a(s)
        },
        Branching0fromItoA::Backward(s) => {
            let (_, s) = recv_mpst_a_from_b(s)?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_i, {
        Branching0fromItoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoB::Forward(s) => {
            let ((), s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_c((), s);
            endpoint_b(s)
        },
        Branching0fromItoB::Backward(s) => {
            let ((), s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_a((), s);
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_i, {
        Branching0fromItoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoC::Forward(s) => {
            let ((), s) = recv_mpst_c_from_b(s)?;
            let s = send_mpst_c_to_d((), s);
            endpoint_c(s)
        },
        Branching0fromItoC::Backward(s) => {
            let ((), s) = recv_mpst_c_from_d(s)?;
            let s = send_mpst_c_to_b((), s);
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_from_i, {
        Branching0fromItoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoD::Forward(s) => {
            let ((), s) = recv_mpst_d_from_c(s)?;
            let s = send_mpst_d_to_e((), s);
            endpoint_d(s)
        },
        Branching0fromItoD::Backward(s) => {
            let ((), s) = recv_mpst_d_from_e(s)?;
            let s = send_mpst_d_to_c((), s);
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_e_from_i, {
        Branching0fromItoE::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoE::Forward(s) => {
            let ((), s) = recv_mpst_e_from_d(s)?;
            let s = send_mpst_e_to_f((), s);
            endpoint_e(s)
        },
        Branching0fromItoE::Backward(s) => {
            let ((), s) = recv_mpst_e_from_f(s)?;
            let s = send_mpst_e_to_d((), s);
            endpoint_e(s)
        },
    })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_f_from_i, {
        Branching0fromItoF::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoF::Forward(s) => {
            let ((), s) = recv_mpst_f_from_e(s)?;
            let s = send_mpst_f_to_g((), s);
            endpoint_f(s)
        },
        Branching0fromItoF::Backward(s) => {
            let ((), s) = recv_mpst_f_from_g(s)?;
            let s = send_mpst_f_to_e((), s);
            endpoint_f(s)
        },
    })
}

fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_g_from_i, {
        Branching0fromItoG::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoG::Forward(s) => {
            let ((), s) = recv_mpst_g_from_f(s)?;
            let s = send_mpst_g_to_h((), s);
            endpoint_g(s)
        },
        Branching0fromItoG::Backward(s) => {
            let ((), s) = recv_mpst_g_from_h(s)?;
            let s = send_mpst_g_to_f((), s);
            endpoint_g(s)
        },
    })
}

fn endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_h_from_i, {
        Branching0fromItoH::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoH::Forward(s) => {
            let ((), s) = recv_mpst_h_from_g(s)?;
            let s = send_mpst_h_to_i((), s);
            endpoint_h(s)
        },
        Branching0fromItoH::Backward(s) => {
            let ((), s) = recv_mpst_h_from_i(s)?;
            let s = send_mpst_h_to_g((), s);
            endpoint_h(s)
        },
    })
}

fn endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>> {
    recurs_i(s, LOOPS)
}

fn recurs_i(s: EndpointI, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_i_to_all(s);

            close_mpst_multi(s)
        }
        i if i % 2 == 0 => {
            let s = forward_from_i_to_all(s);

            let (_, s) = recv_mpst_i_from_h(s)?;

            recurs_i(s, i - 1)
        }
        i => {
            let s = backward_from_i_to_all(s);

            let s = send_mpst_i_to_h((), s);

            recurs_i(s, i - 1)
        }
    }
}

fn aux() {
    let (thread_a, thread_b, thread_c, thread_d, thread_e, thread_f, thread_g, thread_h, thread_i) =
        fork_mpst(
            black_box(endpoint_a),
            black_box(endpoint_b),
            black_box(endpoint_c),
            black_box(endpoint_d),
            black_box(endpoint_e),
            black_box(endpoint_f),
            black_box(endpoint_g),
            black_box(endpoint_h),
            black_box(endpoint_i),
        );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
    thread_f.join().unwrap();
    thread_g.join().unwrap();
    thread_h.join().unwrap();
    thread_i.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("ring nine {LOOPS}"), |b| b.iter(aux));
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
