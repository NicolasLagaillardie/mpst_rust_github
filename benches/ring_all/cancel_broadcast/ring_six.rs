use criterion::{black_box, Criterion};

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

// use std::time::Duration;

// Create the new MeshedChannels for six participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsSeven, 7);

// Create new roles
// normal
create_multiple_normal_role_short!(Central, A, B, C, D, E, F);

// Create new names
create_multiple_normal_name_short!(Central, A, B, C, D, E, F);

// Create new send functions
// A
create_send_check_cancel_bundle!(
    send_mpst_a_to_b, RoleB, 2 | =>
    NameA, MeshedChannelsSeven, 7
);

// B
create_send_check_cancel_bundle!(
    send_mpst_b_to_a, RoleA, 2 |
    send_mpst_b_to_c, RoleC, 3 | =>
    NameB, MeshedChannelsSeven, 7
);

// C
create_send_check_cancel_bundle!(
    send_mpst_c_to_b, RoleB, 3 |
    send_mpst_c_to_d, RoleD, 4 | =>
    NameC, MeshedChannelsSeven, 7
);

// D
create_send_check_cancel_bundle!(
    send_mpst_d_to_c, RoleC, 4 |
    send_mpst_d_to_e, RoleE, 5 | =>
    NameD, MeshedChannelsSeven, 7
);

// E
create_send_check_cancel_bundle!(
    send_mpst_e_to_d, RoleD, 5 |
    send_mpst_e_to_f, RoleF, 6 | =>
    NameE, MeshedChannelsSeven, 7
);

// F
create_send_check_cancel_bundle!(
    send_mpst_f_to_e, RoleE, 6 | =>
    NameF, MeshedChannelsSeven, 7
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 2 |
    recv_mpst_a_from_f, RoleF, 6 | =>
    NameA, MeshedChannelsSeven, 7
);

// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 2 |
    recv_mpst_b_from_c, RoleC, 3 |
    recv_mpst_b_from_f, RoleF, 6 | =>
    NameB, MeshedChannelsSeven, 7
);

// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_b, RoleB, 3 |
    recv_mpst_c_from_d, RoleD, 4 |
    recv_mpst_c_from_f, RoleF, 6 | =>
    NameC, MeshedChannelsSeven, 7
);

// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_c, RoleC, 4 |
    recv_mpst_d_from_e, RoleE, 5 |
    recv_mpst_d_from_f, RoleF, 6 | =>
    NameD, MeshedChannelsSeven, 7
);

// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_d, RoleD, 5 |
    recv_mpst_e_from_f, RoleF, 6 | =>
    NameE, MeshedChannelsSeven, 7
);

// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_from_e, RoleE, 6 | =>
    NameF, MeshedChannelsSeven, 7
);

// Types
// A
enum Branching0fromFtoA {
    Forward(
        MeshedChannelsSeven<
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
        MeshedChannelsSeven<
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
    Done(MeshedChannelsSeven<End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoF = <Choose0fromFtoA as Session>::Dual;

// B
enum Branching0fromFtoB {
    Forward(
        MeshedChannelsSeven<
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
        MeshedChannelsSeven<
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
    Done(MeshedChannelsSeven<End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoF = <Choose0fromFtoB as Session>::Dual;

// C
enum Branching0fromFtoC {
    Forward(
        MeshedChannelsSeven<
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
        MeshedChannelsSeven<
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
    Done(MeshedChannelsSeven<End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoF = <Choose0fromFtoC as Session>::Dual;

// D
enum Branching0fromFtoD {
    Forward(
        MeshedChannelsSeven<
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
        MeshedChannelsSeven<
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
    Done(MeshedChannelsSeven<End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoF = <Choose0fromFtoD as Session>::Dual;

// E
enum Branching0fromFtoE {
    Forward(
        MeshedChannelsSeven<
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
        MeshedChannelsSeven<
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
    Done(MeshedChannelsSeven<End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoF = <Choose0fromFtoE as Session>::Dual;

// F
type Choose0fromFtoA = Send<(End, Branching0fromFtoA), End>;
type Choose0fromFtoB = Send<(End, Branching0fromFtoB), End>;
type Choose0fromFtoC = Send<(End, Branching0fromFtoC), End>;
type Choose0fromFtoD = Send<(End, Branching0fromFtoD), End>;
type Choose0fromFtoE = Send<(End, Branching0fromFtoE), End>;
type EndpointDoneF = MeshedChannelsSeven<End, End, End, End, End, End, RoleEnd, NameF>;
type EndpointForwardF = MeshedChannelsSeven<
    End,
    Choose0fromFtoA,
    Choose0fromFtoB,
    Choose0fromFtoC,
    Choose0fromFtoD,
    Recv<(), Choose0fromFtoE>,
    RoleE<RoleBroadcast>,
    NameF,
>;
type EndpointBackwardF = MeshedChannelsSeven<
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
type EndpointCentral = MeshedChannelsSeven<End, End, End, End, End, End, RoleEnd, NameCentral>;
type EndpointA = MeshedChannelsSeven<End, End, End, End, End, RecursAtoF, RoleF<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsSeven<End, End, End, End, End, RecursBtoF, RoleF<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsSeven<End, End, End, End, End, RecursCtoF, RoleF<RoleEnd>, NameC>;
type EndpointD = MeshedChannelsSeven<End, End, End, End, End, RecursDtoF, RoleF<RoleEnd>, NameD>;
type EndpointE = MeshedChannelsSeven<End, End, End, End, End, RecursEtoF, RoleF<RoleEnd>, NameE>;
type EndpointF = MeshedChannelsSeven<
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
    NameA, NameB, NameC, NameD, NameE, =>
    NameCentral, NameF, MeshedChannelsSeven, 7
);

fn endpoint_central(s: EndpointCentral) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, 7)
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
    recurs_f(s, LOOPS)
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

fn all_mpst() {
    let (thread_central, thread_a, thread_b, thread_c, thread_d, thread_e, thread_f) = fork_mpst(
        black_box(endpoint_central),
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
        black_box(endpoint_d),
        black_box(endpoint_e),
        black_box(endpoint_f),
    );

    thread_central.join().unwrap();
    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
    thread_f.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(
        &format!("ring six cancel broadcast protocol MPST {LOOPS}"),
        |b| b.iter(all_mpst),
    );
}
