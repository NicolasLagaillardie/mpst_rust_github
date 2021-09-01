use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, create_fn_choose_mpst_multi_to_all_bundle,
    create_multiple_normal_role_short, create_recv_mpst_session_bundle,
    create_send_mpst_session_bundle, offer_mpst,
};

use std::error::Error;

// Create the new MeshedChannels for six participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsSix, 6);

// Create new roles
// normal
create_multiple_normal_role_short!(A, B, C, D, E, F);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, 1 | =>
    RoleA, MeshedChannelsSix, 6
);
// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a, RoleA, 1 |
    send_mpst_b_to_c, RoleC, 2 | =>
    RoleB, MeshedChannelsSix, 6
);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_b, RoleB, 2 |
    send_mpst_c_to_d, RoleD, 3 | =>
    RoleC, MeshedChannelsSix, 6
);
// D
create_send_mpst_session_bundle!(
    send_mpst_d_to_c, RoleC, 3 |
    send_mpst_d_to_e, RoleE, 4 | =>
    RoleD, MeshedChannelsSix, 6
);
// E
create_send_mpst_session_bundle!(
    send_mpst_e_to_d, RoleD, 4 |
    send_mpst_e_to_f, RoleF, 5 | =>
    RoleE, MeshedChannelsSix, 6
);
// F
create_send_mpst_session_bundle!(
    send_mpst_f_to_e, RoleE, 5 | =>
    RoleF, MeshedChannelsSix, 6
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 1 |
    recv_mpst_a_from_f, RoleF, 5 | =>
    RoleA, MeshedChannelsSix, 6
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 1 |
    recv_mpst_b_from_c, RoleC, 2 |
    recv_mpst_b_from_f, RoleF, 5 | =>
    RoleB, MeshedChannelsSix, 6
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_b, RoleB, 2 |
    recv_mpst_c_from_d, RoleD, 3 |
    recv_mpst_c_from_f, RoleF, 5 | =>
    RoleC, MeshedChannelsSix, 6
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_c, RoleC, 3 |
    recv_mpst_d_from_e, RoleE, 4 |
    recv_mpst_d_from_f, RoleF, 5 | =>
    RoleD, MeshedChannelsSix, 6
);
// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_d, RoleD, 4 |
    recv_mpst_e_from_f, RoleF, 5 | =>
    RoleE, MeshedChannelsSix, 6
);
// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_from_e, RoleE, 5 | =>
    RoleF, MeshedChannelsSix, 6
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
    Forward(MeshedChannelsSix<Send<(), End>, End, End, End, RecursAtoF, RoleB<RoleF<RoleEnd>>, NameA>),
    Backward(
        MeshedChannelsSix<Recv<(), End>, End, End, End, RecursAtoF, RoleB<RoleF<RoleEnd>>, NameA>,
    ),
    Done(MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoF = <Choose0fromFtoA as Session>::Dual;
// B
enum Branching0fromFtoB {
    Forward(
        MeshedChannelsSix<
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
        MeshedChannelsSix<
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            RecursBtoF,
            RoleC<RoleA<RoleF<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoF = <Choose0fromFtoB as Session>::Dual;
// C
enum Branching0fromFtoC {
    Forward(
        MeshedChannelsSix<
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
        MeshedChannelsSix<
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            RecursCtoF,
            RoleD<RoleB<RoleF<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoF = <Choose0fromFtoC as Session>::Dual;
// D
enum Branching0fromFtoD {
    Forward(
        MeshedChannelsSix<
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
        MeshedChannelsSix<
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            RecursDtoF,
            RoleE<RoleC<RoleF<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoF = <Choose0fromFtoD as Session>::Dual;
// E
enum Branching0fromFtoE {
    Forward(
        MeshedChannelsSix<
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
        MeshedChannelsSix<
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), RecursEtoF>,
            RoleF<RoleD<RoleF<RoleEnd>>>,
            NameE,
        >,
    ),
    Done(MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoF = <Choose0fromFtoE as Session>::Dual;
// F
type Choose0fromFtoA = Send<Branching0fromFtoA, End>;
type Choose0fromFtoB = Send<Branching0fromFtoB, End>;
type Choose0fromFtoC = Send<Branching0fromFtoC, End>;
type Choose0fromFtoD = Send<Branching0fromFtoD, End>;
type Choose0fromFtoE = Send<Branching0fromFtoE, End>;
type EndpointDoneF = MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameF>;
type EndpointForwardF = MeshedChannelsSix<
    Choose0fromFtoA,
    Choose0fromFtoB,
    Choose0fromFtoC,
    Choose0fromFtoD,
    Recv<(), Choose0fromFtoE>,
    RoleE<RoleBroadcast>,
    NameF,
>;
type EndpointBackwardF = MeshedChannelsSix<
    Choose0fromFtoA,
    Choose0fromFtoB,
    Choose0fromFtoC,
    Choose0fromFtoD,
    Send<(), Choose0fromFtoE>,
    RoleE<RoleBroadcast>,
    NameF,
>;

// Creating the MP sessions
type EndpointA = MeshedChannelsSix<End, End, End, End, RecursAtoF, RoleF<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsSix<End, End, End, End, RecursBtoF, RoleF<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsSix<End, End, End, End, RecursCtoF, RoleF<RoleEnd>, NameC>;
type EndpointD = MeshedChannelsSix<End, End, End, End, RecursDtoF, RoleF<RoleEnd>, NameD>;
type EndpointE = MeshedChannelsSix<End, End, End, End, RecursEtoF, RoleF<RoleEnd>, NameE>;
type EndpointF = MeshedChannelsSix<
    Choose0fromFtoA,
    Choose0fromFtoB,
    Choose0fromFtoC,
    Choose0fromFtoD,
    Choose0fromFtoE,
    RoleBroadcast,
    NameF,
>;

create_fn_choose_mpst_multi_to_all_bundle!(
    done_from_f_to_all, forward_from_f_to_all, backward_from_f_to_all, =>
    Done, Forward, Backward, =>
    EndpointDoneF, EndpointForwardF, EndpointBackwardF, =>
    Branching0fromFtoA,
    Branching0fromFtoB,
    Branching0fromFtoC,
    Branching0fromFtoD,
    Branching0fromFtoE, =>
    RoleA, RoleB, RoleC, RoleD, RoleE, =>
    RoleF, MeshedChannelsSix, 6
);

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_f, {
        Branching0fromFtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromFtoA::Forward(s) => {
            let s = send_mpst_a_to_b((), s);
            endpoint_a(s)
        },
        Branching0fromFtoA::Backward(s) => {
            let (_, s) = recv_mpst_a_from_b(s)?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_f, {
        Branching0fromFtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromFtoB::Forward(s) => {
            let ((), s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_c((), s);
            endpoint_b(s)
        },
        Branching0fromFtoB::Backward(s) => {
            let ((), s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_a((), s);
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_f, {
        Branching0fromFtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromFtoC::Forward(s) => {
            let ((), s) = recv_mpst_c_from_b(s)?;
            let s = send_mpst_c_to_d((), s);
            endpoint_c(s)
        },
        Branching0fromFtoC::Backward(s) => {
            let ((), s) = recv_mpst_c_from_d(s)?;
            let s = send_mpst_c_to_b((), s);
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_from_f, {
        Branching0fromFtoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromFtoD::Forward(s) => {
            let ((), s) = recv_mpst_d_from_c(s)?;
            let s = send_mpst_d_to_e((), s);
            endpoint_d(s)
        },
        Branching0fromFtoD::Backward(s) => {
            let ((), s) = recv_mpst_d_from_e(s)?;
            let s = send_mpst_d_to_c((), s);
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_e_from_f, {
        Branching0fromFtoE::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromFtoE::Forward(s) => {
            let ((), s) = recv_mpst_e_from_d(s)?;
            let s = send_mpst_e_to_f((), s);
            endpoint_e(s)
        },
        Branching0fromFtoE::Backward(s) => {
            let ((), s) = recv_mpst_e_from_f(s)?;
            let s = send_mpst_e_to_d((), s);
            endpoint_e(s)
        },
    })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    recurs_f(s, 100)
}

fn recurs_f(s: EndpointF, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_f_to_all(s);

            close_mpst_multi(s)
        }
        i if i % 2 == 0 => {
            let s = forward_from_f_to_all(s);

            let (_, s) = recv_mpst_f_from_e(s)?;

            recurs_f(s, i - 1)
        }
        i => {
            let s = backward_from_f_to_all(s);

            let s = send_mpst_f_to_e((), s);

            recurs_f(s, i - 1)
        }
    }
}

fn main() {
    let (thread_a, thread_b, thread_c, thread_d, thread_e, thread_f) = fork_mpst(
        endpoint_a, endpoint_b, endpoint_c, endpoint_d, endpoint_e, endpoint_f,
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
    thread_f.join().unwrap();
}
