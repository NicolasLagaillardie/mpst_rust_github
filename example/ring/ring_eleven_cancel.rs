use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    broadcast_cancel, bundle_struct_fork_close_multi,
    create_fn_choose_mpst_cancel_multi_to_all_bundle, create_multiple_normal_role_short,
    create_recv_mpst_session_bundle, create_send_check_cancel_bundle, offer_cancel_mpst,
};

use std::error::Error;

// Create the new SessionMpst for eleven participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstTwelve, 12);

// Create new roles
// normal
create_multiple_normal_role_short!(Central, A, B, C, D, E, F, G, H, I, J, K);

// Create new send functions
// A
create_send_check_cancel_bundle!(
    send_mpst_a_to_b, RoleB, 2 | =>
    RoleA, SessionMpstTwelve, 12
);
// B
create_send_check_cancel_bundle!(
    send_mpst_b_to_a, RoleA, 2 |
    send_mpst_b_to_c, RoleC, 3 | =>
    RoleB, SessionMpstTwelve, 12
);
// C
create_send_check_cancel_bundle!(
    send_mpst_c_to_b, RoleB, 3 |
    send_mpst_c_to_d, RoleD, 4 | =>
    RoleC, SessionMpstTwelve, 12
);
// D
create_send_check_cancel_bundle!(
    send_mpst_d_to_c, RoleC, 4 |
    send_mpst_d_to_e, RoleE, 5 | =>
    RoleD, SessionMpstTwelve, 12
);
// E
create_send_check_cancel_bundle!(
    send_mpst_e_to_d, RoleD, 5 |
    send_mpst_e_to_f, RoleF, 6 | =>
    RoleE, SessionMpstTwelve, 12
);
// F
create_send_check_cancel_bundle!(
    send_mpst_f_to_e, RoleE, 6 |
    send_mpst_f_to_g, RoleG, 7 | =>
    RoleF, SessionMpstTwelve, 12
);
// G
create_send_check_cancel_bundle!(
    send_mpst_g_to_f, RoleF, 7 |
    send_mpst_g_to_h, RoleH, 8 | =>
    RoleG, SessionMpstTwelve, 12
);
// H
create_send_check_cancel_bundle!(
    send_mpst_h_to_g, RoleG, 8 |
    send_mpst_h_to_i, RoleI, 9 | =>
    RoleH, SessionMpstTwelve, 12
);
// I
create_send_check_cancel_bundle!(
    send_mpst_i_to_h, RoleH, 9 |
    send_mpst_i_to_j, RoleJ, 10 | =>
    RoleI, SessionMpstTwelve, 12
);
// J
create_send_check_cancel_bundle!(
    send_mpst_j_to_i, RoleI, 10 |
    send_mpst_j_to_k, RoleK, 11 | =>
    RoleJ, SessionMpstTwelve, 12
);
// K
create_send_check_cancel_bundle!(
    send_mpst_k_to_j, RoleJ, 11 | =>
    RoleK, SessionMpstTwelve, 12
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 2 |
    recv_mpst_a_from_k, RoleK, 11 | =>
    RoleA, SessionMpstTwelve, 12
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 2 |
    recv_mpst_b_from_c, RoleC, 3 |
    recv_mpst_b_from_k, RoleK, 11 | =>
    RoleB, SessionMpstTwelve, 12
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_b, RoleB, 3 |
    recv_mpst_c_from_d, RoleD, 4 |
    recv_mpst_c_from_k, RoleK, 11 | =>
    RoleC, SessionMpstTwelve, 12
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_c, RoleC, 4 |
    recv_mpst_d_from_e, RoleE, 5 |
    recv_mpst_d_from_k, RoleK, 11 | =>
    RoleD, SessionMpstTwelve, 12
);
// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_d, RoleD, 5 |
    recv_mpst_e_from_f, RoleF, 6 |
    recv_mpst_e_from_k, RoleK, 11 | =>
    RoleE, SessionMpstTwelve, 12
);
// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_from_e, RoleE, 6 |
    recv_mpst_f_from_g, RoleG, 7 |
    recv_mpst_f_from_k, RoleK, 11 | =>
    RoleF, SessionMpstTwelve, 12
);
// G
create_recv_mpst_session_bundle!(
    recv_mpst_g_from_f, RoleF, 7 |
    recv_mpst_g_from_h, RoleH, 8 |
    recv_mpst_g_from_k, RoleK, 11 | =>
    RoleG, SessionMpstTwelve, 12
);
// H
create_recv_mpst_session_bundle!(
    recv_mpst_h_from_g, RoleG, 8 |
    recv_mpst_h_from_i, RoleI, 9 |
    recv_mpst_h_from_k, RoleK, 11 | =>
    RoleH, SessionMpstTwelve, 12
);
// I
create_recv_mpst_session_bundle!(
    recv_mpst_i_from_h, RoleH, 9 |
    recv_mpst_i_from_j, RoleJ, 10 |
    recv_mpst_i_from_k, RoleK, 11 | =>
    RoleI, SessionMpstTwelve, 12
);
// J
create_recv_mpst_session_bundle!(
    recv_mpst_j_from_i, RoleI, 10 |
    recv_mpst_j_from_k, RoleK, 11 | =>
    RoleJ, SessionMpstTwelve, 12
);
// K
create_recv_mpst_session_bundle!(
    recv_mpst_k_from_j, RoleJ, 11 | =>
    RoleK, SessionMpstTwelve, 12
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
        SessionMpstTwelve<
            End,
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
        SessionMpstTwelve<
            End,
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
    Done(SessionMpstTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoK = <Choose0fromKtoA as Session>::Dual;
// B
enum Branching0fromKtoB {
    Forward(
        SessionMpstTwelve<
            End,
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
        SessionMpstTwelve<
            End,
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
    Done(SessionMpstTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoK = <Choose0fromKtoB as Session>::Dual;
// C
enum Branching0fromKtoC {
    Forward(
        SessionMpstTwelve<
            End,
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
        SessionMpstTwelve<
            End,
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
    Done(SessionMpstTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoK = <Choose0fromKtoC as Session>::Dual;
// D
enum Branching0fromKtoD {
    Forward(
        SessionMpstTwelve<
            End,
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
        SessionMpstTwelve<
            End,
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
    Done(SessionMpstTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoK = <Choose0fromKtoD as Session>::Dual;
// E
enum Branching0fromKtoE {
    Forward(
        SessionMpstTwelve<
            End,
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
        SessionMpstTwelve<
            End,
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
    Done(SessionMpstTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoK = <Choose0fromKtoE as Session>::Dual;
// F
enum Branching0fromKtoF {
    Forward(
        SessionMpstTwelve<
            End,
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
        SessionMpstTwelve<
            End,
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
    Done(SessionMpstTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoK = <Choose0fromKtoF as Session>::Dual;
// G
enum Branching0fromKtoG {
    Forward(
        SessionMpstTwelve<
            End,
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
        SessionMpstTwelve<
            End,
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
    Done(SessionMpstTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoK = <Choose0fromKtoG as Session>::Dual;
// H
enum Branching0fromKtoH {
    Forward(
        SessionMpstTwelve<
            End,
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
        SessionMpstTwelve<
            End,
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
    Done(SessionMpstTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameH>),
}
type RecursHtoK = <Choose0fromKtoH as Session>::Dual;
// I
enum Branching0fromKtoI {
    Forward(
        SessionMpstTwelve<
            End,
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
        SessionMpstTwelve<
            End,
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
    Done(SessionMpstTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameI>),
}
type RecursItoK = <Choose0fromKtoI as Session>::Dual;
// J
enum Branching0fromKtoJ {
    Forward(
        SessionMpstTwelve<
            End,
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
        SessionMpstTwelve<
            End,
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
    Done(SessionMpstTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameJ>),
}
type RecursJtoK = <Choose0fromKtoJ as Session>::Dual;
// K
type Choose0fromKtoA = Send<(End, Branching0fromKtoA), End>;
type Choose0fromKtoB = Send<(End, Branching0fromKtoB), End>;
type Choose0fromKtoC = Send<(End, Branching0fromKtoC), End>;
type Choose0fromKtoD = Send<(End, Branching0fromKtoD), End>;
type Choose0fromKtoE = Send<(End, Branching0fromKtoE), End>;
type Choose0fromKtoF = Send<(End, Branching0fromKtoF), End>;
type Choose0fromKtoG = Send<(End, Branching0fromKtoG), End>;
type Choose0fromKtoH = Send<(End, Branching0fromKtoH), End>;
type Choose0fromKtoI = Send<(End, Branching0fromKtoI), End>;
type Choose0fromKtoJ = Send<(End, Branching0fromKtoJ), End>;
type EndpointDoneK =
    SessionMpstTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameK>;
type EndpointForwardK = SessionMpstTwelve<
    End,
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
type EndpointBackwardK = SessionMpstTwelve<
    End,
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
type EndpointCentral = SessionMpstTwelve<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RoleEnd,
    RoleCentral<RoleEnd>,
>;
type EndpointA = SessionMpstTwelve<
    End,
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
type EndpointB = SessionMpstTwelve<
    End,
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
type EndpointC = SessionMpstTwelve<
    End,
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
type EndpointD = SessionMpstTwelve<
    End,
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
type EndpointE = SessionMpstTwelve<
    End,
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
type EndpointF = SessionMpstTwelve<
    End,
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
type EndpointG = SessionMpstTwelve<
    End,
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
type EndpointH = SessionMpstTwelve<
    End,
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
type EndpointI = SessionMpstTwelve<
    End,
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
type EndpointJ = SessionMpstTwelve<
    End,
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
type EndpointK = SessionMpstTwelve<
    End,
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

create_fn_choose_mpst_cancel_multi_to_all_bundle!(
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
    RoleCentral, RoleK, SessionMpstTwelve, 12
);

fn endpoint_central(s: EndpointCentral) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, 12);
    Ok(())
}

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_a_from_k, {
        Branching0fromKtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoA::Forward(s) => {
            let s = send_mpst_a_to_b((), s)?;
            endpoint_a(s)
        },
        Branching0fromKtoA::Backward(s) => {
            let (_, s) = recv_mpst_a_from_b(s)?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_b_from_k, {
        Branching0fromKtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoB::Forward(s) => {
            let ((), s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_c((), s)?;
            endpoint_b(s)
        },
        Branching0fromKtoB::Backward(s) => {
            let ((), s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_a((), s)?;
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_c_from_k, {
        Branching0fromKtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoC::Forward(s) => {
            let ((), s) = recv_mpst_c_from_b(s)?;
            let s = send_mpst_c_to_d((), s)?;
            endpoint_c(s)
        },
        Branching0fromKtoC::Backward(s) => {
            let ((), s) = recv_mpst_c_from_d(s)?;
            let s = send_mpst_c_to_b((), s)?;
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_d_from_k, {
        Branching0fromKtoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoD::Forward(s) => {
            let ((), s) = recv_mpst_d_from_c(s)?;
            let s = send_mpst_d_to_e((), s)?;
            endpoint_d(s)
        },
        Branching0fromKtoD::Backward(s) => {
            let ((), s) = recv_mpst_d_from_e(s)?;
            let s = send_mpst_d_to_c((), s)?;
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_e_from_k, {
        Branching0fromKtoE::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoE::Forward(s) => {
            let ((), s) = recv_mpst_e_from_d(s)?;
            let s = send_mpst_e_to_f((), s)?;
            endpoint_e(s)
        },
        Branching0fromKtoE::Backward(s) => {
            let ((), s) = recv_mpst_e_from_f(s)?;
            let s = send_mpst_e_to_d((), s)?;
            endpoint_e(s)
        },
    })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_f_from_k, {
        Branching0fromKtoF::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoF::Forward(s) => {
            let ((), s) = recv_mpst_f_from_e(s)?;
            let s = send_mpst_f_to_g((), s)?;
            endpoint_f(s)
        },
        Branching0fromKtoF::Backward(s) => {
            let ((), s) = recv_mpst_f_from_g(s)?;
            let s = send_mpst_f_to_e((), s)?;
            endpoint_f(s)
        },
    })
}

fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_g_from_k, {
        Branching0fromKtoG::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoG::Forward(s) => {
            let ((), s) = recv_mpst_g_from_f(s)?;
            let s = send_mpst_g_to_h((), s)?;
            endpoint_g(s)
        },
        Branching0fromKtoG::Backward(s) => {
            let ((), s) = recv_mpst_g_from_h(s)?;
            let s = send_mpst_g_to_f((), s)?;
            endpoint_g(s)
        },
    })
}

fn endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_h_from_k, {
        Branching0fromKtoH::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoH::Forward(s) => {
            let ((), s) = recv_mpst_h_from_g(s)?;
            let s = send_mpst_h_to_i((), s)?;
            endpoint_h(s)
        },
        Branching0fromKtoH::Backward(s) => {
            let ((), s) = recv_mpst_h_from_i(s)?;
            let s = send_mpst_h_to_g((), s)?;
            endpoint_h(s)
        },
    })
}

fn endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_i_from_k, {
        Branching0fromKtoI::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoI::Forward(s) => {
            let ((), s) = recv_mpst_i_from_h(s)?;
            let s = send_mpst_i_to_j((), s)?;
            endpoint_i(s)
        },
        Branching0fromKtoI::Backward(s) => {
            let ((), s) = recv_mpst_i_from_j(s)?;
            let s = send_mpst_i_to_h((), s)?;
            endpoint_i(s)
        },
    })
}

fn endpoint_j(s: EndpointJ) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_j_from_k, {
        Branching0fromKtoJ::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoJ::Forward(s) => {
            let ((), s) = recv_mpst_j_from_i(s)?;
            let s = send_mpst_j_to_k((), s)?;
            endpoint_j(s)
        },
        Branching0fromKtoJ::Backward(s) => {
            let ((), s) = recv_mpst_j_from_k(s)?;
            let s = send_mpst_j_to_i((), s)?;
            endpoint_j(s)
        },
    })
}

fn endpoint_k(s: EndpointK) -> Result<(), Box<dyn Error>> {
    recurs_k(s, 100)
}

fn recurs_k(s: EndpointK, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_k_to_all(s)?;

            close_mpst_multi(s)
        }
        i if i % 2 == 0 => {
            let s = forward_from_k_to_all(s)?;

            let (_, s) = recv_mpst_k_from_j(s)?;

            recurs_k(s, i - 1)
        }
        i => {
            let s = backward_from_k_to_all(s)?;

            let s = send_mpst_k_to_j((), s)?;

            recurs_k(s, i - 1)
        }
    }
}

fn main() {
    let (
        thread_central,
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
        endpoint_central,
        endpoint_a,
        endpoint_b,
        endpoint_c,
        endpoint_d,
        endpoint_e,
        endpoint_f,
        endpoint_g,
        endpoint_h,
        endpoint_i,
        endpoint_j,
        endpoint_k,
    );

    thread_central.join().unwrap();
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
