#![allow(dead_code)]

use crossbeam_channel::bounded;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose, create_fn_choose_mpst_multi_to_all_bundle,
    create_multiple_normal_role_short, create_recv_mpst_session_bundle,
    create_send_mpst_session_bundle, offer, offer_mpst,
};

use std::error::Error;
use std::thread::{spawn, JoinHandle};
// use std::time::Duration;

// Create the new MeshedChannels for eleven participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsEleven, 11);

// Create new roles
// normal
create_multiple_normal_role_short!(A, B, C, D, E, F, G, H, I, J, K);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, 1 | =>
    RoleA, MeshedChannelsEleven, 11
);
// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a, RoleA, 1 |
    send_mpst_b_to_c, RoleC, 2 | =>
    RoleB, MeshedChannelsEleven, 11
);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_b, RoleB, 2 |
    send_mpst_c_to_d, RoleD, 3 | =>
    RoleC, MeshedChannelsEleven, 11
);
// D
create_send_mpst_session_bundle!(
    send_mpst_d_to_c, RoleC, 3 |
    send_mpst_d_to_e, RoleE, 4 | =>
    RoleD, MeshedChannelsEleven, 11
);
// E
create_send_mpst_session_bundle!(
    send_mpst_e_to_d, RoleD, 4 |
    send_mpst_e_to_f, RoleF, 5 | =>
    RoleE, MeshedChannelsEleven, 11
);
// F
create_send_mpst_session_bundle!(
    send_mpst_f_to_e, RoleE, 5 |
    send_mpst_f_to_g, RoleG, 6 | =>
    RoleF, MeshedChannelsEleven, 11
);
// G
create_send_mpst_session_bundle!(
    send_mpst_g_to_f, RoleF, 6 |
    send_mpst_g_to_h, RoleH, 7 | =>
    RoleG, MeshedChannelsEleven, 11
);
// H
create_send_mpst_session_bundle!(
    send_mpst_h_to_g, RoleG, 7 |
    send_mpst_h_to_i, RoleI, 8 | =>
    RoleH, MeshedChannelsEleven, 11
);
// I
create_send_mpst_session_bundle!(
    send_mpst_i_to_h, RoleH, 8 |
    send_mpst_i_to_j, RoleJ, 9 | =>
    RoleI, MeshedChannelsEleven, 11
);
// J
create_send_mpst_session_bundle!(
    send_mpst_j_to_i, RoleI, 9 |
    send_mpst_j_to_k, RoleK, 10 | =>
    RoleJ, MeshedChannelsEleven, 11
);
// K
create_send_mpst_session_bundle!(
    send_mpst_k_to_j, RoleJ, 10 | =>
    RoleK, MeshedChannelsEleven, 11
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 1 |
    recv_mpst_a_from_k, RoleK, 10 | =>
    RoleA, MeshedChannelsEleven, 11
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 1 |
    recv_mpst_b_from_c, RoleC, 2 |
    recv_mpst_b_from_k, RoleK, 10 | =>
    RoleB, MeshedChannelsEleven, 11
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_b, RoleB, 2 |
    recv_mpst_c_from_d, RoleD, 3 |
    recv_mpst_c_from_k, RoleK, 10 | =>
    RoleC, MeshedChannelsEleven, 11
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_c, RoleC, 3 |
    recv_mpst_d_from_e, RoleE, 4 |
    recv_mpst_d_from_k, RoleK, 10 | =>
    RoleD, MeshedChannelsEleven, 11
);
// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_d, RoleD, 4 |
    recv_mpst_e_from_f, RoleF, 5 |
    recv_mpst_e_from_k, RoleK, 10 | =>
    RoleE, MeshedChannelsEleven, 11
);
// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_from_e, RoleE, 5 |
    recv_mpst_f_from_g, RoleG, 6 |
    recv_mpst_f_from_k, RoleK, 10 | =>
    RoleF, MeshedChannelsEleven, 11
);
// G
create_recv_mpst_session_bundle!(
    recv_mpst_g_from_f, RoleF, 6 |
    recv_mpst_g_from_h, RoleH, 7 |
    recv_mpst_g_from_k, RoleK, 10 | =>
    RoleG, MeshedChannelsEleven, 11
);
// H
create_recv_mpst_session_bundle!(
    recv_mpst_h_from_g, RoleG, 7 |
    recv_mpst_h_from_i, RoleI, 8 |
    recv_mpst_h_from_k, RoleK, 10 | =>
    RoleH, MeshedChannelsEleven, 11
);
// I
create_recv_mpst_session_bundle!(
    recv_mpst_i_from_h, RoleH, 8 |
    recv_mpst_i_from_j, RoleJ, 9 |
    recv_mpst_i_from_k, RoleK, 10 | =>
    RoleI, MeshedChannelsEleven, 11
);
// J
create_recv_mpst_session_bundle!(
    recv_mpst_j_from_i, RoleI, 9 |
    recv_mpst_j_from_k, RoleK, 10 | =>
    RoleJ, MeshedChannelsEleven, 11
);
// K
create_recv_mpst_session_bundle!(
    recv_mpst_k_from_j, RoleJ, 10 | =>
    RoleK, MeshedChannelsEleven, 11
);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;
type NameE = RoleE<RoleEnd>;
type NameF = RoleF<RoleEnd>;
type NameG = RoleG<RoleEnd>;
type NameH = RoleH<RoleEnd>;
type NameI = RoleI<RoleEnd>;
type NameJ = RoleJ<RoleEnd>;
type NameK = RoleK<RoleEnd>;

// Types
// A
enum Branching0fromKtoA {
    Forward(
        MeshedChannelsEleven<
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursAtoK,
            RoleB<RoleK<RoleEnd>>,
            NameA,
        >,
    ),
    Backward(
        MeshedChannelsEleven<
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursAtoK,
            RoleB<RoleK<RoleEnd>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoK = <Choose0fromKtoA as Session>::Dual;
// B
enum Branching0fromKtoB {
    Forward(
        MeshedChannelsEleven<
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursBtoK,
            RoleA<RoleC<RoleK<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannelsEleven<
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursBtoK,
            RoleC<RoleA<RoleK<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoK = <Choose0fromKtoB as Session>::Dual;
// C
enum Branching0fromKtoC {
    Forward(
        MeshedChannelsEleven<
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursCtoK,
            RoleB<RoleD<RoleK<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannelsEleven<
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursCtoK,
            RoleD<RoleB<RoleK<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoK = <Choose0fromKtoC as Session>::Dual;
// D
enum Branching0fromKtoD {
    Forward(
        MeshedChannelsEleven<
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            RecursDtoK,
            RoleC<RoleE<RoleK<RoleEnd>>>,
            NameD,
        >,
    ),
    Backward(
        MeshedChannelsEleven<
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            RecursDtoK,
            RoleE<RoleC<RoleK<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoK = <Choose0fromKtoD as Session>::Dual;
// E
enum Branching0fromKtoE {
    Forward(
        MeshedChannelsEleven<
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            RecursEtoK,
            RoleD<RoleF<RoleK<RoleEnd>>>,
            NameE,
        >,
    ),
    Backward(
        MeshedChannelsEleven<
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            RecursEtoK,
            RoleF<RoleD<RoleK<RoleEnd>>>,
            NameE,
        >,
    ),
    Done(MeshedChannelsEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoK = <Choose0fromKtoE as Session>::Dual;
// F
enum Branching0fromKtoF {
    Forward(
        MeshedChannelsEleven<
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            RecursFtoK,
            RoleE<RoleG<RoleK<RoleEnd>>>,
            NameF,
        >,
    ),
    Backward(
        MeshedChannelsEleven<
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            RecursFtoK,
            RoleG<RoleE<RoleK<RoleEnd>>>,
            NameF,
        >,
    ),
    Done(MeshedChannelsEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoK = <Choose0fromKtoF as Session>::Dual;
// G
enum Branching0fromKtoG {
    Forward(
        MeshedChannelsEleven<
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            RecursGtoK,
            RoleF<RoleH<RoleK<RoleEnd>>>,
            NameG,
        >,
    ),
    Backward(
        MeshedChannelsEleven<
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            RecursGtoK,
            RoleH<RoleF<RoleK<RoleEnd>>>,
            NameG,
        >,
    ),
    Done(MeshedChannelsEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoK = <Choose0fromKtoG as Session>::Dual;
// H
enum Branching0fromKtoH {
    Forward(
        MeshedChannelsEleven<
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            RecursHtoK,
            RoleG<RoleI<RoleK<RoleEnd>>>,
            NameH,
        >,
    ),
    Backward(
        MeshedChannelsEleven<
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            RecursHtoK,
            RoleI<RoleG<RoleK<RoleEnd>>>,
            NameH,
        >,
    ),
    Done(MeshedChannelsEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameH>),
}
type RecursHtoK = <Choose0fromKtoH as Session>::Dual;
// I
enum Branching0fromKtoI {
    Forward(
        MeshedChannelsEleven<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            RecursItoK,
            RoleH<RoleJ<RoleK<RoleEnd>>>,
            NameI,
        >,
    ),
    Backward(
        MeshedChannelsEleven<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            RecursItoK,
            RoleJ<RoleH<RoleK<RoleEnd>>>,
            NameI,
        >,
    ),
    Done(MeshedChannelsEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameI>),
}
type RecursItoK = <Choose0fromKtoI as Session>::Dual;
// J
enum Branching0fromKtoJ {
    Forward(
        MeshedChannelsEleven<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), RecursJtoK>,
            RoleI<RoleK<RoleK<RoleEnd>>>,
            NameJ,
        >,
    ),
    Backward(
        MeshedChannelsEleven<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), RecursJtoK>,
            RoleK<RoleI<RoleK<RoleEnd>>>,
            NameJ,
        >,
    ),
    Done(MeshedChannelsEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameJ>),
}
type RecursJtoK = <Choose0fromKtoJ as Session>::Dual;
// K
type Choose0fromKtoA = Send<Branching0fromKtoA, End>;
type Choose0fromKtoB = Send<Branching0fromKtoB, End>;
type Choose0fromKtoC = Send<Branching0fromKtoC, End>;
type Choose0fromKtoD = Send<Branching0fromKtoD, End>;
type Choose0fromKtoE = Send<Branching0fromKtoE, End>;
type Choose0fromKtoF = Send<Branching0fromKtoF, End>;
type Choose0fromKtoG = Send<Branching0fromKtoG, End>;
type Choose0fromKtoH = Send<Branching0fromKtoH, End>;
type Choose0fromKtoI = Send<Branching0fromKtoI, End>;
type Choose0fromKtoJ = Send<Branching0fromKtoJ, End>;
type EndpointDoneK =
    MeshedChannelsEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameK>;
type EndpointForwardK = MeshedChannelsEleven<
    Choose0fromKtoA,
    Choose0fromKtoB,
    Choose0fromKtoC,
    Choose0fromKtoD,
    Choose0fromKtoE,
    Choose0fromKtoF,
    Choose0fromKtoG,
    Choose0fromKtoH,
    Choose0fromKtoI,
    Recv<(), Choose0fromKtoJ>,
    RoleJ<RoleBroadcast>,
    NameK,
>;
type EndpointBackwardK = MeshedChannelsEleven<
    Choose0fromKtoA,
    Choose0fromKtoB,
    Choose0fromKtoC,
    Choose0fromKtoD,
    Choose0fromKtoE,
    Choose0fromKtoF,
    Choose0fromKtoG,
    Choose0fromKtoH,
    Choose0fromKtoI,
    Send<(), Choose0fromKtoJ>,
    RoleJ<RoleBroadcast>,
    NameK,
>;

// Creating the MP sessions
type EndpointA = MeshedChannelsEleven<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursAtoK,
    RoleK<RoleEnd>,
    NameA,
>;
type EndpointB = MeshedChannelsEleven<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursBtoK,
    RoleK<RoleEnd>,
    NameB,
>;
type EndpointC = MeshedChannelsEleven<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursCtoK,
    RoleK<RoleEnd>,
    NameC,
>;
type EndpointD = MeshedChannelsEleven<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursDtoK,
    RoleK<RoleEnd>,
    NameD,
>;
type EndpointE = MeshedChannelsEleven<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursEtoK,
    RoleK<RoleEnd>,
    NameE,
>;
type EndpointF = MeshedChannelsEleven<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursFtoK,
    RoleK<RoleEnd>,
    NameF,
>;
type EndpointG = MeshedChannelsEleven<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursGtoK,
    RoleK<RoleEnd>,
    NameG,
>;
type EndpointH = MeshedChannelsEleven<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursHtoK,
    RoleK<RoleEnd>,
    NameH,
>;
type EndpointI = MeshedChannelsEleven<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursItoK,
    RoleK<RoleEnd>,
    NameI,
>;
type EndpointJ = MeshedChannelsEleven<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursJtoK,
    RoleK<RoleEnd>,
    NameJ,
>;
type EndpointK = MeshedChannelsEleven<
    Choose0fromKtoA,
    Choose0fromKtoB,
    Choose0fromKtoC,
    Choose0fromKtoD,
    Choose0fromKtoE,
    Choose0fromKtoF,
    Choose0fromKtoG,
    Choose0fromKtoH,
    Choose0fromKtoI,
    Choose0fromKtoJ,
    RoleBroadcast,
    NameK,
>;

create_fn_choose_mpst_multi_to_all_bundle!(
    done_from_k_to_all, forward_from_k_to_all, backward_from_k_to_all, =>
    Done, Forward, Backward, =>
    EndpointDoneK, EndpointForwardK, EndpointBackwardK, =>
    Branching0fromKtoA,
    Branching0fromKtoB,
    Branching0fromKtoC,
    Branching0fromKtoD,
    Branching0fromKtoE,
    Branching0fromKtoF,
    Branching0fromKtoG,
    Branching0fromKtoH,
    Branching0fromKtoI,
    Branching0fromKtoJ, =>
    RoleA,
    RoleB,
    RoleC,
    RoleD,
    RoleE,
    RoleF,
    RoleG,
    RoleH,
    RoleI,
    RoleJ, =>
    RoleK, MeshedChannelsEleven, 11
);

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_k, {
        Branching0fromKtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoA::Forward(s) => {
            let s = send_mpst_a_to_b((), s);
            endpoint_a(s)
        },
        Branching0fromKtoA::Backward(s) => {
            let (_, s) = recv_mpst_a_from_b(s)?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_k, {
        Branching0fromKtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoB::Forward(s) => {
            let ((), s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_c((), s);
            endpoint_b(s)
        },
        Branching0fromKtoB::Backward(s) => {
            let ((), s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_a((), s);
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_k, {
        Branching0fromKtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoC::Forward(s) => {
            let ((), s) = recv_mpst_c_from_b(s)?;
            let s = send_mpst_c_to_d((), s);
            endpoint_c(s)
        },
        Branching0fromKtoC::Backward(s) => {
            let ((), s) = recv_mpst_c_from_d(s)?;
            let s = send_mpst_c_to_b((), s);
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_from_k, {
        Branching0fromKtoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoD::Forward(s) => {
            let ((), s) = recv_mpst_d_from_c(s)?;
            let s = send_mpst_d_to_e((), s);
            endpoint_d(s)
        },
        Branching0fromKtoD::Backward(s) => {
            let ((), s) = recv_mpst_d_from_e(s)?;
            let s = send_mpst_d_to_c((), s);
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_e_from_k, {
        Branching0fromKtoE::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoE::Forward(s) => {
            let ((), s) = recv_mpst_e_from_d(s)?;
            let s = send_mpst_e_to_f((), s);
            endpoint_e(s)
        },
        Branching0fromKtoE::Backward(s) => {
            let ((), s) = recv_mpst_e_from_f(s)?;
            let s = send_mpst_e_to_d((), s);
            endpoint_e(s)
        },
    })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_f_from_k, {
        Branching0fromKtoF::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoF::Forward(s) => {
            let ((), s) = recv_mpst_f_from_e(s)?;
            let s = send_mpst_f_to_g((), s);
            endpoint_f(s)
        },
        Branching0fromKtoF::Backward(s) => {
            let ((), s) = recv_mpst_f_from_g(s)?;
            let s = send_mpst_f_to_e((), s);
            endpoint_f(s)
        },
    })
}

fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_g_from_k, {
        Branching0fromKtoG::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoG::Forward(s) => {
            let ((), s) = recv_mpst_g_from_f(s)?;
            let s = send_mpst_g_to_h((), s);
            endpoint_g(s)
        },
        Branching0fromKtoG::Backward(s) => {
            let ((), s) = recv_mpst_g_from_h(s)?;
            let s = send_mpst_g_to_f((), s);
            endpoint_g(s)
        },
    })
}

fn endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_h_from_k, {
        Branching0fromKtoH::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoH::Forward(s) => {
            let ((), s) = recv_mpst_h_from_g(s)?;
            let s = send_mpst_h_to_i((), s);
            endpoint_h(s)
        },
        Branching0fromKtoH::Backward(s) => {
            let ((), s) = recv_mpst_h_from_i(s)?;
            let s = send_mpst_h_to_g((), s);
            endpoint_h(s)
        },
    })
}

fn endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_i_from_k, {
        Branching0fromKtoI::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoI::Forward(s) => {
            let ((), s) = recv_mpst_i_from_h(s)?;
            let s = send_mpst_i_to_j((), s);
            endpoint_i(s)
        },
        Branching0fromKtoI::Backward(s) => {
            let ((), s) = recv_mpst_i_from_j(s)?;
            let s = send_mpst_i_to_h((), s);
            endpoint_i(s)
        },
    })
}

fn endpoint_j(s: EndpointJ) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_j_from_k, {
        Branching0fromKtoJ::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoJ::Forward(s) => {
            let ((), s) = recv_mpst_j_from_i(s)?;
            let s = send_mpst_j_to_k((), s);
            endpoint_j(s)
        },
        Branching0fromKtoJ::Backward(s) => {
            let ((), s) = recv_mpst_j_from_k(s)?;
            let s = send_mpst_j_to_i((), s);
            endpoint_j(s)
        },
    })
}

fn endpoint_k(s: EndpointK) -> Result<(), Box<dyn Error>> {
    recurs_k(s, LOOPS)
}

fn recurs_k(s: EndpointK, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_k_to_all(s);

            close_mpst_multi(s)
        }
        i if i % 2 == 0 => {
            let s = forward_from_k_to_all(s);

            let (_, s) = recv_mpst_k_from_j(s)?;

            recurs_k(s, i - 1)
        }
        i => {
            let s = backward_from_k_to_all(s);

            let s = send_mpst_k_to_j((), s);

            recurs_k(s, i - 1)
        }
    }
}

fn all_mpst() {
    let (
        thread_a,
        thread_b,
        thread_c,
        thread_d,
        thread_e,
        thread_f,
        thread_g,
        thread_h,
        thread_i,
        thread_j,
        thread_k,
    ) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
        black_box(endpoint_d),
        black_box(endpoint_e),
        black_box(endpoint_f),
        black_box(endpoint_g),
        black_box(endpoint_h),
        black_box(endpoint_i),
        black_box(endpoint_j),
        black_box(endpoint_k),
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
    thread_j.join().unwrap();
    thread_k.join().unwrap();
}

/////////////////////////
// A
enum BinaryA {
    Forward(Recv<(), Send<(), RecursA>>),
    Done(End),
}
type RecursA = Recv<BinaryA, End>;
fn binary_a_to_b(s: RecursA) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        BinaryA::Done(s) => {
            close(s)
        },
        BinaryA::Forward(s) => {
            let (_, s) = recv(s)?;
            let s = send((), s);
            binary_a_to_b(s)
        },
    })
}

// B
type RecursB = <RecursA as Session>::Dual;
fn binary_b_to_a(s: Send<(), Recv<(), RecursB>>) -> Result<RecursB, Box<dyn Error>> {
    let s = send((), s);
    let (_, s) = recv(s)?;
    Ok(s)
}

fn all_binaries() {
    let mut threads = Vec::new();
    let mut sessions = Vec::new();

    for _ in 0..11 {
        let (thread, s): (JoinHandle<()>, RecursB) = fork_with_thread_id(black_box(binary_a_to_b));

        threads.push(thread);
        sessions.push(s);
    }

    let main = spawn(move || {
        for _ in 0..LOOPS {
            sessions = sessions
                .into_iter()
                .map(|s| binary_b_to_a(choose!(BinaryA::Forward, s)).unwrap())
                .collect::<Vec<_>>();
        }

        sessions
            .into_iter()
            .for_each(|s| close(choose!(BinaryA::Done, s)).unwrap());

        threads.into_iter().for_each(|elt| elt.join().unwrap());
    });

    main.join().unwrap();
}

/////////////////////////

type ReceivingSendingReceiving = crossbeam_channel::Receiver<SendingReceiving>;
type SendingReceivingSending = crossbeam_channel::Sender<ReceivingSending>;

type SendingReceiving = crossbeam_channel::Sender<Receiving>;
type ReceivingSending = crossbeam_channel::Receiver<Sending>;

type Receiving = crossbeam_channel::Receiver<()>;
type Sending = crossbeam_channel::Sender<()>;

fn all_crossbeam() {
    let mut threads = Vec::new();

    for _ in 0..11 {
        let main = spawn(move || {
            for _ in 0..LOOPS {
                let (sender_0, receiver_0) = bounded::<ReceivingSendingReceiving>(1);
                let (sender_4, receiver_4) = bounded::<SendingReceivingSending>(1);

                let (sender_1, receiver_1) = bounded::<SendingReceiving>(1);
                let (sender_5, receiver_5) = bounded::<ReceivingSending>(1);

                let (sender_2, receiver_2) = bounded::<Receiving>(1);
                let (sender_6, receiver_6) = bounded::<Sending>(1);

                let (sender_3, receiver_3) = bounded::<()>(1);
                let (sender_7, receiver_7) = bounded::<()>(1);

                sender_0.send(receiver_1).unwrap();
                sender_4.send(sender_5).unwrap();

                let receiver_1_bis = receiver_0.recv().unwrap();
                let sender_5_bis = receiver_4.recv().unwrap();

                sender_1.send(sender_2).unwrap();
                sender_5_bis.send(receiver_6).unwrap();

                let sender_2_bis = receiver_1_bis.recv().unwrap();
                let receiver_6_bis = receiver_5.recv().unwrap();

                sender_2_bis.send(receiver_3).unwrap();
                sender_6.send(sender_7).unwrap();

                let receiver_2_bis = receiver_2.recv().unwrap();
                let sender_7_bis = receiver_6_bis.recv().unwrap();

                sender_3.send(()).unwrap();
                sender_7_bis.send(()).unwrap();

                receiver_2_bis.recv().unwrap();
                receiver_7.recv().unwrap();
            }

            // "Close" connection
            let (sender_close_1, receiver_close_1) = bounded::<()>(1);
            let (sender_close_2, receiver_close_2) = bounded::<()>(1);

            sender_close_1.send(()).unwrap_or(());
            sender_close_2.send(()).unwrap_or(());

            receiver_close_1.recv().unwrap_or(());
            receiver_close_2.recv().unwrap_or(());
        });

        threads.push(main);
    }

    threads.into_iter().for_each(|elt| elt.join().unwrap());
}

/////////////////////////

static LOOPS: i64 = 100;

fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("ring eleven protocol MPST {}", LOOPS), |b| {
        b.iter(all_mpst)
    });
}

fn ring_protocol_binary(c: &mut Criterion) {
    c.bench_function(&format!("ring eleven protocol binary {}", LOOPS), |b| {
        b.iter(all_binaries)
    });
}

fn ring_protocol_crossbeam(c: &mut Criterion) {
    c.bench_function(&format!("ring eleven protocol crossbeam {}", LOOPS), |b| {
        b.iter(all_crossbeam)
    });
}

// fn long_warmup() -> Criterion {
//     Criterion::default().measurement_time(Duration::new(1800, 0))
// }

criterion_group! {
    name = ring_eleven;
    // config = long_warmup();
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = ring_protocol_mpst, ring_protocol_binary, ring_protocol_crossbeam
}

criterion_main!(ring_eleven);
