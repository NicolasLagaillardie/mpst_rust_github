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

// use std::time::Duration;

// Create the new MeshedChannels for twenty participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsTwentyOne, 21);

// Create new roles
// normal
create_multiple_normal_role_short!(
    Central, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T
);

// Create new names
create_multiple_normal_name_short!(
    Central, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T
);

// Create new send functions
// A
create_send_check_cancel_bundle!(
    send_mpst_a_to_b, RoleB, 2 | =>
    NameA, MeshedChannelsTwentyOne, 21
);

// B
create_send_check_cancel_bundle!(
    send_mpst_b_to_a, RoleA, 2 |
    send_mpst_b_to_c, RoleC, 3 | =>
    NameB, MeshedChannelsTwentyOne, 21
);

// C
create_send_check_cancel_bundle!(
    send_mpst_c_to_b, RoleB, 3 |
    send_mpst_c_to_d, RoleD, 4 | =>
    NameC, MeshedChannelsTwentyOne, 21
);

// D
create_send_check_cancel_bundle!(
    send_mpst_d_to_c, RoleC, 4 |
    send_mpst_d_to_e, RoleE, 5 | =>
    NameD, MeshedChannelsTwentyOne, 21
);

// E
create_send_check_cancel_bundle!(
    send_mpst_e_to_d, RoleD, 5 |
    send_mpst_e_to_f, RoleF, 6 | =>
    NameE, MeshedChannelsTwentyOne, 21
);

// F
create_send_check_cancel_bundle!(
    send_mpst_f_to_e, RoleE, 6 |
    send_mpst_f_to_g, RoleG, 7 | =>
    NameF, MeshedChannelsTwentyOne, 21
);

// G
create_send_check_cancel_bundle!(
    send_mpst_g_to_f, RoleF, 7 |
    send_mpst_g_to_h, RoleH, 8 | =>
    NameG, MeshedChannelsTwentyOne, 21
);

// H
create_send_check_cancel_bundle!(
    send_mpst_h_to_g, RoleG, 8 |
    send_mpst_h_to_i, RoleI, 9 | =>
    NameH, MeshedChannelsTwentyOne, 21
);

// I
create_send_check_cancel_bundle!(
    send_mpst_i_to_h, RoleH, 9 |
    send_mpst_i_to_j, RoleJ, 10 | =>
    NameI, MeshedChannelsTwentyOne, 21
);

// J
create_send_check_cancel_bundle!(
    send_mpst_j_to_i, RoleI, 10 |
    send_mpst_j_to_k, RoleK, 11 | =>
    NameJ, MeshedChannelsTwentyOne, 21
);

// K
create_send_check_cancel_bundle!(
    send_mpst_k_to_j, RoleJ, 11 |
    send_mpst_k_to_l, RoleL, 12 | =>
    NameK, MeshedChannelsTwentyOne, 21
);

// L
create_send_check_cancel_bundle!(
    send_mpst_l_to_k, RoleK, 12 |
    send_mpst_l_to_m, RoleM, 13 | =>
    NameL, MeshedChannelsTwentyOne, 21
);

// M
create_send_check_cancel_bundle!(
    send_mpst_m_to_l, RoleL, 13 |
    send_mpst_m_to_n, RoleN, 14 | =>
    NameM, MeshedChannelsTwentyOne, 21
);

// N
create_send_check_cancel_bundle!(
    send_mpst_n_to_m, RoleM, 14 |
    send_mpst_n_to_o, RoleO, 15 | =>
    NameN, MeshedChannelsTwentyOne, 21
);

// O
create_send_check_cancel_bundle!(
    send_mpst_o_to_n, RoleN, 15 |
    send_mpst_o_to_p, RoleP, 16 | =>
    NameO, MeshedChannelsTwentyOne, 21
);

// P
create_send_check_cancel_bundle!(
    send_mpst_p_to_o, RoleO, 16 |
    send_mpst_p_to_q, RoleQ, 17 | =>
    NameP, MeshedChannelsTwentyOne, 21
);

// Q
create_send_check_cancel_bundle!(
    send_mpst_q_to_p, RoleP, 17 |
    send_mpst_q_to_r, RoleR, 18 | =>
    NameQ, MeshedChannelsTwentyOne, 21
);

// R
create_send_check_cancel_bundle!(
    send_mpst_r_to_q, RoleQ, 18 |
    send_mpst_r_to_s, RoleS, 19 | =>
    NameR, MeshedChannelsTwentyOne, 21
);

// S
create_send_check_cancel_bundle!(
    send_mpst_s_to_r, RoleR, 19 |
    send_mpst_s_to_t, RoleT, 20 | =>
    NameS, MeshedChannelsTwentyOne, 21
);

// T
create_send_check_cancel_bundle!(
    send_mpst_t_to_s, RoleS, 20 | =>
    NameT, MeshedChannelsTwentyOne, 21
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 2 |
    recv_mpst_a_from_t, RoleT, 20 | =>
    NameA, MeshedChannelsTwentyOne, 21
);

// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 2 |
    recv_mpst_b_from_c, RoleC, 3 |
    recv_mpst_b_from_t, RoleT, 20 | =>
    NameB, MeshedChannelsTwentyOne, 21
);

// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_b, RoleB, 3 |
    recv_mpst_c_from_d, RoleD, 4 |
    recv_mpst_c_from_t, RoleT, 20 | =>
    NameC, MeshedChannelsTwentyOne, 21
);

// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_c, RoleC, 4 |
    recv_mpst_d_from_e, RoleE, 5 |
    recv_mpst_d_from_t, RoleT, 20 | =>
    NameD, MeshedChannelsTwentyOne, 21
);

// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_d, RoleD, 5 |
    recv_mpst_e_from_f, RoleF, 6 |
    recv_mpst_e_from_t, RoleT, 20 | =>
    NameE, MeshedChannelsTwentyOne, 21
);

// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_from_e, RoleE, 6 |
    recv_mpst_f_from_g, RoleG, 7 |
    recv_mpst_f_from_t, RoleT, 20 | =>
    NameF, MeshedChannelsTwentyOne, 21
);

// G
create_recv_mpst_session_bundle!(
    recv_mpst_g_from_f, RoleF, 7 |
    recv_mpst_g_from_h, RoleH, 8 |
    recv_mpst_g_from_t, RoleT, 20 | =>
    NameG, MeshedChannelsTwentyOne, 21
);

// H
create_recv_mpst_session_bundle!(
    recv_mpst_h_from_g, RoleG, 8 |
    recv_mpst_h_from_i, RoleI, 9 |
    recv_mpst_h_from_t, RoleT, 20 | =>
    NameH, MeshedChannelsTwentyOne, 21
);

// I
create_recv_mpst_session_bundle!(
    recv_mpst_i_from_h, RoleH, 9 |
    recv_mpst_i_from_j, RoleJ, 10 |
    recv_mpst_i_from_t, RoleT, 20 | =>
    NameI, MeshedChannelsTwentyOne, 21
);

// J
create_recv_mpst_session_bundle!(
    recv_mpst_j_from_i, RoleI, 10 |
    recv_mpst_j_from_k, RoleK, 11 |
    recv_mpst_j_from_t, RoleT, 20 | =>
    NameJ, MeshedChannelsTwentyOne, 21
);

// K
create_recv_mpst_session_bundle!(
    recv_mpst_k_from_j, RoleJ, 11 |
    recv_mpst_k_from_l, RoleL, 12 |
    recv_mpst_k_from_t, RoleT, 20 | =>
    NameK, MeshedChannelsTwentyOne, 21
);

// L
create_recv_mpst_session_bundle!(
    recv_mpst_l_from_k, RoleK, 12 |
    recv_mpst_l_from_m, RoleM, 13 |
    recv_mpst_l_from_t, RoleT, 20 | =>
    NameL, MeshedChannelsTwentyOne, 21
);

// M
create_recv_mpst_session_bundle!(
    recv_mpst_m_from_l, RoleL, 13 |
    recv_mpst_m_from_n, RoleN, 14 |
    recv_mpst_m_from_t, RoleT, 20 | =>
    NameM, MeshedChannelsTwentyOne, 21
);

// N
create_recv_mpst_session_bundle!(
    recv_mpst_n_from_m, RoleM, 14 |
    recv_mpst_n_from_o, RoleO, 15 |
    recv_mpst_n_from_t, RoleT, 20 | =>
    NameN, MeshedChannelsTwentyOne, 21
);

// O
create_recv_mpst_session_bundle!(
    recv_mpst_o_from_n, RoleN, 15 |
    recv_mpst_o_from_p, RoleP, 16 |
    recv_mpst_o_from_t, RoleT, 20 | =>
    NameO, MeshedChannelsTwentyOne, 21
);

// P
create_recv_mpst_session_bundle!(
    recv_mpst_p_from_o, RoleO, 16 |
    recv_mpst_p_from_q, RoleQ, 17 |
    recv_mpst_p_from_t, RoleT, 20 | =>
    NameP, MeshedChannelsTwentyOne, 21
);

// Q
create_recv_mpst_session_bundle!(
    recv_mpst_q_from_p, RoleP, 17 |
    recv_mpst_q_from_r, RoleR, 18 |
    recv_mpst_q_from_t, RoleT, 20 | =>
    NameQ, MeshedChannelsTwentyOne, 21
);

// R
create_recv_mpst_session_bundle!(
    recv_mpst_r_from_q, RoleQ, 18 |
    recv_mpst_r_from_s, RoleS, 19 |
    recv_mpst_r_from_t, RoleT, 20 | =>
    NameR, MeshedChannelsTwentyOne, 21
);

// S
create_recv_mpst_session_bundle!(
    recv_mpst_s_from_r, RoleR, 19 |
    recv_mpst_s_from_t, RoleT, 20 | =>
    NameS, MeshedChannelsTwentyOne, 21
);

// T
create_recv_mpst_session_bundle!(
    recv_mpst_t_from_s, RoleS, 20 | =>
    NameT, MeshedChannelsTwentyOne, 21
);

// Types
// A
enum Branching0fromTtoA {
    Forward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursAtoT,
            RoleB<RoleT<RoleEnd>>,
            NameA,
        >,
    ),
    Backward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursAtoT,
            RoleB<RoleT<RoleEnd>>,
            NameA,
        >,
    ),
    Done(
        MeshedChannelsTwentyOne<
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
            NameA,
        >,
    ),
}
type RecursAtoT = <Choose0fromTtoA as Session>::Dual;

// B
enum Branching0fromTtoB {
    Forward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursBtoT,
            RoleA<RoleC<RoleT<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursBtoT,
            RoleC<RoleA<RoleT<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(
        MeshedChannelsTwentyOne<
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
            NameB,
        >,
    ),
}
type RecursBtoT = <Choose0fromTtoB as Session>::Dual;

// C
enum Branching0fromTtoC {
    Forward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursCtoT,
            RoleB<RoleD<RoleT<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursCtoT,
            RoleD<RoleB<RoleT<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(
        MeshedChannelsTwentyOne<
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
            NameC,
        >,
    ),
}
type RecursCtoT = <Choose0fromTtoC as Session>::Dual;

// D
enum Branching0fromTtoD {
    Forward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursDtoT,
            RoleC<RoleE<RoleT<RoleEnd>>>,
            NameD,
        >,
    ),
    Backward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursDtoT,
            RoleE<RoleC<RoleT<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(
        MeshedChannelsTwentyOne<
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
            NameD,
        >,
    ),
}
type RecursDtoT = <Choose0fromTtoD as Session>::Dual;

// E
enum Branching0fromTtoE {
    Forward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursEtoT,
            RoleD<RoleF<RoleT<RoleEnd>>>,
            NameE,
        >,
    ),
    Backward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursEtoT,
            RoleF<RoleD<RoleT<RoleEnd>>>,
            NameE,
        >,
    ),
    Done(
        MeshedChannelsTwentyOne<
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
            NameE,
        >,
    ),
}
type RecursEtoT = <Choose0fromTtoE as Session>::Dual;

// F
enum Branching0fromTtoF {
    Forward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursFtoT,
            RoleE<RoleG<RoleT<RoleEnd>>>,
            NameF,
        >,
    ),
    Backward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursFtoT,
            RoleG<RoleE<RoleT<RoleEnd>>>,
            NameF,
        >,
    ),
    Done(
        MeshedChannelsTwentyOne<
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
            NameF,
        >,
    ),
}
type RecursFtoT = <Choose0fromTtoF as Session>::Dual;

// G
enum Branching0fromTtoG {
    Forward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursGtoT,
            RoleF<RoleH<RoleT<RoleEnd>>>,
            NameG,
        >,
    ),
    Backward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursGtoT,
            RoleH<RoleF<RoleT<RoleEnd>>>,
            NameG,
        >,
    ),
    Done(
        MeshedChannelsTwentyOne<
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
            NameG,
        >,
    ),
}
type RecursGtoT = <Choose0fromTtoG as Session>::Dual;

// H
enum Branching0fromTtoH {
    Forward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursHtoT,
            RoleG<RoleI<RoleT<RoleEnd>>>,
            NameH,
        >,
    ),
    Backward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursHtoT,
            RoleI<RoleG<RoleT<RoleEnd>>>,
            NameH,
        >,
    ),
    Done(
        MeshedChannelsTwentyOne<
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
            NameH,
        >,
    ),
}
type RecursHtoT = <Choose0fromTtoH as Session>::Dual;

// I
enum Branching0fromTtoI {
    Forward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursItoT,
            RoleH<RoleJ<RoleT<RoleEnd>>>,
            NameI,
        >,
    ),
    Backward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursItoT,
            RoleJ<RoleH<RoleT<RoleEnd>>>,
            NameI,
        >,
    ),
    Done(
        MeshedChannelsTwentyOne<
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
            NameI,
        >,
    ),
}
type RecursItoT = <Choose0fromTtoI as Session>::Dual;

// J
enum Branching0fromTtoJ {
    Forward(
        MeshedChannelsTwentyOne<
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
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursJtoT,
            RoleI<RoleK<RoleT<RoleEnd>>>,
            NameJ,
        >,
    ),
    Backward(
        MeshedChannelsTwentyOne<
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
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursJtoT,
            RoleK<RoleI<RoleT<RoleEnd>>>,
            NameJ,
        >,
    ),
    Done(
        MeshedChannelsTwentyOne<
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
            NameJ,
        >,
    ),
}
type RecursJtoT = <Choose0fromTtoJ as Session>::Dual;

// K
enum Branching0fromTtoK {
    Forward(
        MeshedChannelsTwentyOne<
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
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursKtoT,
            RoleJ<RoleL<RoleT<RoleEnd>>>,
            NameK,
        >,
    ),
    Backward(
        MeshedChannelsTwentyOne<
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
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursKtoT,
            RoleL<RoleJ<RoleT<RoleEnd>>>,
            NameK,
        >,
    ),
    Done(
        MeshedChannelsTwentyOne<
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
            NameK,
        >,
    ),
}
type RecursKtoT = <Choose0fromTtoK as Session>::Dual;

// L
enum Branching0fromTtoL {
    Forward(
        MeshedChannelsTwentyOne<
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
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursLtoT,
            RoleK<RoleM<RoleT<RoleEnd>>>,
            NameL,
        >,
    ),
    Backward(
        MeshedChannelsTwentyOne<
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
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursLtoT,
            RoleM<RoleK<RoleT<RoleEnd>>>,
            NameL,
        >,
    ),
    Done(
        MeshedChannelsTwentyOne<
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
            NameL,
        >,
    ),
}
type RecursLtoT = <Choose0fromTtoL as Session>::Dual;

// M
enum Branching0fromTtoM {
    Forward(
        MeshedChannelsTwentyOne<
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
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            RecursMtoT,
            RoleL<RoleN<RoleT<RoleEnd>>>,
            NameM,
        >,
    ),
    Backward(
        MeshedChannelsTwentyOne<
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
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            RecursMtoT,
            RoleN<RoleL<RoleT<RoleEnd>>>,
            NameM,
        >,
    ),
    Done(
        MeshedChannelsTwentyOne<
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
            NameM,
        >,
    ),
}
type RecursMtoT = <Choose0fromTtoM as Session>::Dual;

// N
enum Branching0fromTtoN {
    Forward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            RecursNtoT,
            RoleM<RoleO<RoleT<RoleEnd>>>,
            NameN,
        >,
    ),
    Backward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            RecursNtoT,
            RoleO<RoleM<RoleT<RoleEnd>>>,
            NameN,
        >,
    ),
    Done(
        MeshedChannelsTwentyOne<
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
            NameN,
        >,
    ),
}
type RecursNtoT = <Choose0fromTtoN as Session>::Dual;

// O
enum Branching0fromTtoO {
    Forward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            RecursOtoT,
            RoleN<RoleP<RoleT<RoleEnd>>>,
            NameO,
        >,
    ),
    Backward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            RecursOtoT,
            RoleP<RoleN<RoleT<RoleEnd>>>,
            NameO,
        >,
    ),
    Done(
        MeshedChannelsTwentyOne<
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
            NameO,
        >,
    ),
}
type RecursOtoT = <Choose0fromTtoO as Session>::Dual;

// P
enum Branching0fromTtoP {
    Forward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            RecursPtoT,
            RoleO<RoleQ<RoleT<RoleEnd>>>,
            NameP,
        >,
    ),
    Backward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            RecursPtoT,
            RoleQ<RoleO<RoleT<RoleEnd>>>,
            NameP,
        >,
    ),
    Done(
        MeshedChannelsTwentyOne<
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
            NameP,
        >,
    ),
}
type RecursPtoT = <Choose0fromTtoP as Session>::Dual;

// Q
enum Branching0fromTtoQ {
    Forward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            RecursQtoT,
            RoleP<RoleR<RoleT<RoleEnd>>>,
            NameQ,
        >,
    ),
    Backward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            RecursQtoT,
            RoleR<RoleP<RoleT<RoleEnd>>>,
            NameQ,
        >,
    ),
    Done(
        MeshedChannelsTwentyOne<
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
            NameQ,
        >,
    ),
}
type RecursQtoT = <Choose0fromTtoQ as Session>::Dual;

// R
enum Branching0fromTtoR {
    Forward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            RecursRtoT,
            RoleQ<RoleS<RoleT<RoleEnd>>>,
            NameR,
        >,
    ),
    Backward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            RecursRtoT,
            RoleS<RoleQ<RoleT<RoleEnd>>>,
            NameR,
        >,
    ),
    Done(
        MeshedChannelsTwentyOne<
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
            NameR,
        >,
    ),
}
type RecursRtoT = <Choose0fromTtoR as Session>::Dual;

// S
enum Branching0fromTtoS {
    Forward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), RecursStoT>,
            RoleR<RoleT<RoleT<RoleEnd>>>,
            NameS,
        >,
    ),
    Backward(
        MeshedChannelsTwentyOne<
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
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), RecursStoT>,
            RoleT<RoleR<RoleT<RoleEnd>>>,
            NameS,
        >,
    ),
    Done(
        MeshedChannelsTwentyOne<
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
            NameS,
        >,
    ),
}
type RecursStoT = <Choose0fromTtoS as Session>::Dual;

// T
type Choose0fromTtoA = Send<(End, Branching0fromTtoA), End>;
type Choose0fromTtoB = Send<(End, Branching0fromTtoB), End>;
type Choose0fromTtoC = Send<(End, Branching0fromTtoC), End>;
type Choose0fromTtoD = Send<(End, Branching0fromTtoD), End>;
type Choose0fromTtoE = Send<(End, Branching0fromTtoE), End>;
type Choose0fromTtoF = Send<(End, Branching0fromTtoF), End>;
type Choose0fromTtoG = Send<(End, Branching0fromTtoG), End>;
type Choose0fromTtoH = Send<(End, Branching0fromTtoH), End>;
type Choose0fromTtoI = Send<(End, Branching0fromTtoI), End>;
type Choose0fromTtoJ = Send<(End, Branching0fromTtoJ), End>;
type Choose0fromTtoK = Send<(End, Branching0fromTtoK), End>;
type Choose0fromTtoL = Send<(End, Branching0fromTtoL), End>;
type Choose0fromTtoM = Send<(End, Branching0fromTtoM), End>;
type Choose0fromTtoN = Send<(End, Branching0fromTtoN), End>;
type Choose0fromTtoO = Send<(End, Branching0fromTtoO), End>;
type Choose0fromTtoP = Send<(End, Branching0fromTtoP), End>;
type Choose0fromTtoQ = Send<(End, Branching0fromTtoQ), End>;
type Choose0fromTtoR = Send<(End, Branching0fromTtoR), End>;
type Choose0fromTtoS = Send<(End, Branching0fromTtoS), End>;
type EndpointDoneT = MeshedChannelsTwentyOne<
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
    NameT,
>;
type EndpointForwardT = MeshedChannelsTwentyOne<
    End,
    Choose0fromTtoA,
    Choose0fromTtoB,
    Choose0fromTtoC,
    Choose0fromTtoD,
    Choose0fromTtoE,
    Choose0fromTtoF,
    Choose0fromTtoG,
    Choose0fromTtoH,
    Choose0fromTtoI,
    Choose0fromTtoJ,
    Choose0fromTtoK,
    Choose0fromTtoL,
    Choose0fromTtoM,
    Choose0fromTtoN,
    Choose0fromTtoO,
    Choose0fromTtoP,
    Choose0fromTtoQ,
    Choose0fromTtoR,
    Recv<(), Choose0fromTtoS>,
    RoleS<RoleBroadcast>,
    NameT,
>;
type EndpointBackwardT = MeshedChannelsTwentyOne<
    End,
    Choose0fromTtoA,
    Choose0fromTtoB,
    Choose0fromTtoC,
    Choose0fromTtoD,
    Choose0fromTtoE,
    Choose0fromTtoF,
    Choose0fromTtoG,
    Choose0fromTtoH,
    Choose0fromTtoI,
    Choose0fromTtoJ,
    Choose0fromTtoK,
    Choose0fromTtoL,
    Choose0fromTtoM,
    Choose0fromTtoN,
    Choose0fromTtoO,
    Choose0fromTtoP,
    Choose0fromTtoQ,
    Choose0fromTtoR,
    Send<(), Choose0fromTtoS>,
    RoleS<RoleBroadcast>,
    NameT,
>;

// Creating the MP sessions
type EndpointCentral = MeshedChannelsTwentyOne<
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
    NameCentral,
>;
type EndpointA = MeshedChannelsTwentyOne<
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursAtoT,
    RoleT<RoleEnd>,
    NameA,
>;
type EndpointB = MeshedChannelsTwentyOne<
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursBtoT,
    RoleT<RoleEnd>,
    NameB,
>;
type EndpointC = MeshedChannelsTwentyOne<
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursCtoT,
    RoleT<RoleEnd>,
    NameC,
>;
type EndpointD = MeshedChannelsTwentyOne<
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursDtoT,
    RoleT<RoleEnd>,
    NameD,
>;
type EndpointE = MeshedChannelsTwentyOne<
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursEtoT,
    RoleT<RoleEnd>,
    NameE,
>;
type EndpointF = MeshedChannelsTwentyOne<
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursFtoT,
    RoleT<RoleEnd>,
    NameF,
>;
type EndpointG = MeshedChannelsTwentyOne<
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursGtoT,
    RoleT<RoleEnd>,
    NameG,
>;
type EndpointH = MeshedChannelsTwentyOne<
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursHtoT,
    RoleT<RoleEnd>,
    NameH,
>;
type EndpointI = MeshedChannelsTwentyOne<
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursItoT,
    RoleT<RoleEnd>,
    NameI,
>;
type EndpointJ = MeshedChannelsTwentyOne<
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursJtoT,
    RoleT<RoleEnd>,
    NameJ,
>;
type EndpointK = MeshedChannelsTwentyOne<
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursKtoT,
    RoleT<RoleEnd>,
    NameK,
>;
type EndpointL = MeshedChannelsTwentyOne<
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursLtoT,
    RoleT<RoleEnd>,
    NameL,
>;
type EndpointM = MeshedChannelsTwentyOne<
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursMtoT,
    RoleT<RoleEnd>,
    NameM,
>;
type EndpointN = MeshedChannelsTwentyOne<
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursNtoT,
    RoleT<RoleEnd>,
    NameN,
>;
type EndpointO = MeshedChannelsTwentyOne<
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursOtoT,
    RoleT<RoleEnd>,
    NameO,
>;
type EndpointP = MeshedChannelsTwentyOne<
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursPtoT,
    RoleT<RoleEnd>,
    NameP,
>;
type EndpointQ = MeshedChannelsTwentyOne<
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursQtoT,
    RoleT<RoleEnd>,
    NameQ,
>;
type EndpointR = MeshedChannelsTwentyOne<
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursRtoT,
    RoleT<RoleEnd>,
    NameR,
>;
type EndpointS = MeshedChannelsTwentyOne<
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursStoT,
    RoleT<RoleEnd>,
    NameS,
>;
type EndpointT = MeshedChannelsTwentyOne<
    End,
    Choose0fromTtoA,
    Choose0fromTtoB,
    Choose0fromTtoC,
    Choose0fromTtoD,
    Choose0fromTtoE,
    Choose0fromTtoF,
    Choose0fromTtoG,
    Choose0fromTtoH,
    Choose0fromTtoI,
    Choose0fromTtoJ,
    Choose0fromTtoK,
    Choose0fromTtoL,
    Choose0fromTtoM,
    Choose0fromTtoN,
    Choose0fromTtoO,
    Choose0fromTtoP,
    Choose0fromTtoQ,
    Choose0fromTtoR,
    Choose0fromTtoS,
    RoleBroadcast,
    NameT,
>;

create_fn_choose_mpst_cancel_multi_to_all_bundle!(
    done_from_t_to_all, forward_from_t_to_all, backward_from_t_to_all, =>
    Done, Forward, Backward, =>
    EndpointDoneT, EndpointForwardT, EndpointBackwardT, =>
    Branching0fromTtoA,
    Branching0fromTtoB,
    Branching0fromTtoC,
    Branching0fromTtoD,
    Branching0fromTtoE,
    Branching0fromTtoF,
    Branching0fromTtoG,
    Branching0fromTtoH,
    Branching0fromTtoI,
    Branching0fromTtoJ,
    Branching0fromTtoK,
    Branching0fromTtoL,
    Branching0fromTtoM,
    Branching0fromTtoN,
    Branching0fromTtoO,
    Branching0fromTtoP,
    Branching0fromTtoQ,
    Branching0fromTtoR,
    Branching0fromTtoS, =>
    NameA,
    NameB,
    NameC,
    NameD,
    NameE,
    NameF,
    NameG,
    NameH,
    NameI,
    NameJ,
    NameK,
    NameL,
    NameM,
    NameN,
    NameO,
    NameP,
    NameQ,
    NameR,
    NameS, =>
    NameCentral, NameT, MeshedChannelsTwentyOne, 21
);

fn endpoint_central(s: EndpointCentral) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, 21)
}

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_a_from_t, {
        Branching0fromTtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoA::Forward(s) => {
            let s = send_mpst_a_to_b((), s)?;
            endpoint_a(s)
        },
        Branching0fromTtoA::Backward(s) => {
            let (_, s) = recv_mpst_a_from_b(s)?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_b_from_t, {
        Branching0fromTtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoB::Forward(s) => {
            let ((), s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_c((), s)?;
            endpoint_b(s)
        },
        Branching0fromTtoB::Backward(s) => {
            let ((), s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_a((), s)?;
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_c_from_t, {
        Branching0fromTtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoC::Forward(s) => {
            let ((), s) = recv_mpst_c_from_b(s)?;
            let s = send_mpst_c_to_d((), s)?;
            endpoint_c(s)
        },
        Branching0fromTtoC::Backward(s) => {
            let ((), s) = recv_mpst_c_from_d(s)?;
            let s = send_mpst_c_to_b((), s)?;
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_d_from_t, {
        Branching0fromTtoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoD::Forward(s) => {
            let ((), s) = recv_mpst_d_from_c(s)?;
            let s = send_mpst_d_to_e((), s)?;
            endpoint_d(s)
        },
        Branching0fromTtoD::Backward(s) => {
            let ((), s) = recv_mpst_d_from_e(s)?;
            let s = send_mpst_d_to_c((), s)?;
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_e_from_t, {
        Branching0fromTtoE::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoE::Forward(s) => {
            let ((), s) = recv_mpst_e_from_d(s)?;
            let s = send_mpst_e_to_f((), s)?;
            endpoint_e(s)
        },
        Branching0fromTtoE::Backward(s) => {
            let ((), s) = recv_mpst_e_from_f(s)?;
            let s = send_mpst_e_to_d((), s)?;
            endpoint_e(s)
        },
    })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_f_from_t, {
        Branching0fromTtoF::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoF::Forward(s) => {
            let ((), s) = recv_mpst_f_from_e(s)?;
            let s = send_mpst_f_to_g((), s)?;
            endpoint_f(s)
        },
        Branching0fromTtoF::Backward(s) => {
            let ((), s) = recv_mpst_f_from_g(s)?;
            let s = send_mpst_f_to_e((), s)?;
            endpoint_f(s)
        },
    })
}

fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_g_from_t, {
        Branching0fromTtoG::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoG::Forward(s) => {
            let ((), s) = recv_mpst_g_from_f(s)?;
            let s = send_mpst_g_to_h((), s)?;
            endpoint_g(s)
        },
        Branching0fromTtoG::Backward(s) => {
            let ((), s) = recv_mpst_g_from_h(s)?;
            let s = send_mpst_g_to_f((), s)?;
            endpoint_g(s)
        },
    })
}

fn endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_h_from_t, {
        Branching0fromTtoH::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoH::Forward(s) => {
            let ((), s) = recv_mpst_h_from_g(s)?;
            let s = send_mpst_h_to_i((), s)?;
            endpoint_h(s)
        },
        Branching0fromTtoH::Backward(s) => {
            let ((), s) = recv_mpst_h_from_i(s)?;
            let s = send_mpst_h_to_g((), s)?;
            endpoint_h(s)
        },
    })
}

fn endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_i_from_t, {
        Branching0fromTtoI::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoI::Forward(s) => {
            let ((), s) = recv_mpst_i_from_h(s)?;
            let s = send_mpst_i_to_j((), s)?;
            endpoint_i(s)
        },
        Branching0fromTtoI::Backward(s) => {
            let ((), s) = recv_mpst_i_from_j(s)?;
            let s = send_mpst_i_to_h((), s)?;
            endpoint_i(s)
        },
    })
}

fn endpoint_j(s: EndpointJ) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_j_from_t, {
        Branching0fromTtoJ::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoJ::Forward(s) => {
            let ((), s) = recv_mpst_j_from_i(s)?;
            let s = send_mpst_j_to_k((), s)?;
            endpoint_j(s)
        },
        Branching0fromTtoJ::Backward(s) => {
            let ((), s) = recv_mpst_j_from_k(s)?;
            let s = send_mpst_j_to_i((), s)?;
            endpoint_j(s)
        },
    })
}

fn endpoint_k(s: EndpointK) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_k_from_t, {
        Branching0fromTtoK::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoK::Forward(s) => {
            let ((), s) = recv_mpst_k_from_j(s)?;
            let s = send_mpst_k_to_l((), s)?;
            endpoint_k(s)
        },
        Branching0fromTtoK::Backward(s) => {
            let ((), s) = recv_mpst_k_from_l(s)?;
            let s = send_mpst_k_to_j((), s)?;
            endpoint_k(s)
        },
    })
}

fn endpoint_l(s: EndpointL) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_l_from_t, {
        Branching0fromTtoL::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoL::Forward(s) => {
            let ((), s) = recv_mpst_l_from_k(s)?;
            let s = send_mpst_l_to_m((), s)?;
            endpoint_l(s)
        },
        Branching0fromTtoL::Backward(s) => {
            let ((), s) = recv_mpst_l_from_m(s)?;
            let s = send_mpst_l_to_k((), s)?;
            endpoint_l(s)
        },
    })
}

fn endpoint_m(s: EndpointM) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_m_from_t, {
        Branching0fromTtoM::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoM::Forward(s) => {
            let ((), s) = recv_mpst_m_from_l(s)?;
            let s = send_mpst_m_to_n((), s)?;
            endpoint_m(s)
        },
        Branching0fromTtoM::Backward(s) => {
            let ((), s) = recv_mpst_m_from_n(s)?;
            let s = send_mpst_m_to_l((), s)?;
            endpoint_m(s)
        },
    })
}

fn endpoint_n(s: EndpointN) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_n_from_t, {
        Branching0fromTtoN::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoN::Forward(s) => {
            let ((), s) = recv_mpst_n_from_m(s)?;
            let s = send_mpst_n_to_o((), s)?;
            endpoint_n(s)
        },
        Branching0fromTtoN::Backward(s) => {
            let ((), s) = recv_mpst_n_from_o(s)?;
            let s = send_mpst_n_to_m((), s)?;
            endpoint_n(s)
        },
    })
}

fn endpoint_o(s: EndpointO) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_o_from_t, {
        Branching0fromTtoO::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoO::Forward(s) => {
            let ((), s) = recv_mpst_o_from_n(s)?;
            let s = send_mpst_o_to_p((), s)?;
            endpoint_o(s)
        },
        Branching0fromTtoO::Backward(s) => {
            let ((), s) = recv_mpst_o_from_p(s)?;
            let s = send_mpst_o_to_n((), s)?;
            endpoint_o(s)
        },
    })
}

fn endpoint_p(s: EndpointP) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_p_from_t, {
        Branching0fromTtoP::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoP::Forward(s) => {
            let ((), s) = recv_mpst_p_from_o(s)?;
            let s = send_mpst_p_to_q((), s)?;
            endpoint_p(s)
        },
        Branching0fromTtoP::Backward(s) => {
            let ((), s) = recv_mpst_p_from_q(s)?;
            let s = send_mpst_p_to_o((), s)?;
            endpoint_p(s)
        },
    })
}

fn endpoint_q(s: EndpointQ) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_q_from_t, {
        Branching0fromTtoQ::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoQ::Forward(s) => {
            let ((), s) = recv_mpst_q_from_p(s)?;
            let s = send_mpst_q_to_r((), s)?;
            endpoint_q(s)
        },
        Branching0fromTtoQ::Backward(s) => {
            let ((), s) = recv_mpst_q_from_r(s)?;
            let s = send_mpst_q_to_p((), s)?;
            endpoint_q(s)
        },
    })
}

fn endpoint_r(s: EndpointR) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_r_from_t, {
        Branching0fromTtoR::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoR::Forward(s) => {
            let ((), s) = recv_mpst_r_from_q(s)?;
            let s = send_mpst_r_to_s((), s)?;
            endpoint_r(s)
        },
        Branching0fromTtoR::Backward(s) => {
            let ((), s) = recv_mpst_r_from_s(s)?;
            let s = send_mpst_r_to_q((), s)?;
            endpoint_r(s)
        },
    })
}

fn endpoint_s(s: EndpointS) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_s_from_t, {
        Branching0fromTtoS::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoS::Forward(s) => {
            let ((), s) = recv_mpst_s_from_r(s)?;
            let s = send_mpst_s_to_t((), s)?;
            endpoint_s(s)
        },
        Branching0fromTtoS::Backward(s) => {
            let ((), s) = recv_mpst_s_from_t(s)?;
            let s = send_mpst_s_to_r((), s)?;
            endpoint_s(s)
        },
    })
}

fn endpoint_t(s: EndpointT) -> Result<(), Box<dyn Error>> {
    recurs_t(s, LOOPS)
}

fn recurs_t(s: EndpointT, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_t_to_all(s)?;

            close_mpst_multi(s)
        }
        i if i % 2 == 0 => {
            let s = forward_from_t_to_all(s)?;

            let (_, s) = recv_mpst_t_from_s(s)?;

            recurs_t(s, i - 1)
        }
        i => {
            let s = backward_from_t_to_all(s)?;

            let s = send_mpst_t_to_s((), s)?;

            recurs_t(s, i - 1)
        }
    }
}

fn aux() {
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
        thread_l,
        thread_m,
        thread_n,
        thread_o,
        thread_p,
        thread_q,
        thread_r,
        thread_s,
        thread_t,
    ) = fork_mpst(
        black_box(endpoint_central),
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
        black_box(endpoint_l),
        black_box(endpoint_m),
        black_box(endpoint_n),
        black_box(endpoint_o),
        black_box(endpoint_p),
        black_box(endpoint_q),
        black_box(endpoint_r),
        black_box(endpoint_s),
        black_box(endpoint_t),
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
    thread_l.join().unwrap();
    thread_m.join().unwrap();
    thread_n.join().unwrap();
    thread_o.join().unwrap();
    thread_p.join().unwrap();
    thread_q.join().unwrap();
    thread_r.join().unwrap();
    thread_s.join().unwrap();
    thread_t.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(
        &format!("ring twenty cancel broadcast protocol MPST {LOOPS}"),
        |b| b.iter(aux),
    );
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.1).sample_size(10000);
    targets = ring_protocol_mpst,
}

criterion_main! {
    bench
}
