use mpstthree::binary::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::{
    bundle_fork_multi, choose_mpst_multi_to_all, close_mpst, create_normal_role,
    create_recv_mpst_session, create_recv_mpst_session_bundle, create_send_mpst_session,
    create_send_mpst_session_bundle, create_sessionmpst, offer_mpst,
};

use std::error::Error;

// Create new SessionMpst for four participants
create_sessionmpst!(SessionMpstTwenty, 20);

// Create new roles
// normal
create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
create_normal_role!(RoleD, next_d, RoleDDual, next_d_dual);
create_normal_role!(RoleE, next_e, RoleEDual, next_e_dual);
create_normal_role!(RoleF, next_f, RoleFDual, next_f_dual);
create_normal_role!(RoleG, next_g, RoleGDual, next_g_dual);
create_normal_role!(RoleH, next_h, RoleHDual, next_h_dual);
create_normal_role!(RoleI, next_i, RoleIDual, next_i_dual);
create_normal_role!(RoleJ, next_j, RoleJDual, next_j_dual);
create_normal_role!(RoleK, next_k, RoleKDual, next_k_dual);
create_normal_role!(RoleL, next_l, RoleLDual, next_l_dual);
create_normal_role!(RoleM, next_m, RoleMDual, next_m_dual);
create_normal_role!(RoleN, next_n, RoleNDual, next_n_dual);
create_normal_role!(RoleO, next_o, RoleODual, next_o_dual);
create_normal_role!(RoleP, next_p, RolePDual, next_p_dual);
create_normal_role!(RoleQ, next_q, RoleQDual, next_q_dual);
create_normal_role!(RoleR, next_r, RoleRDual, next_r_dual);
create_normal_role!(RoleS, next_s, RoleSDual, next_s_dual);
create_normal_role!(RoleT, next_t, RoleTDual, next_t_dual);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b,
    RoleB,
    next_b,
    1, |
    send_mpst_a_to_c,
    RoleC,
    next_c,
    2, |
    send_mpst_a_to_d,
    RoleD,
    next_d,
    3, |
    send_mpst_a_to_e,
    RoleE,
    next_e,
    4, |
    send_mpst_a_to_f,
    RoleF,
    next_f,
    5, |
    send_mpst_a_to_g,
    RoleG,
    next_g,
    6, |
    send_mpst_a_to_h,
    RoleH,
    next_h,
    7, |
    send_mpst_a_to_i,
    RoleI,
    next_i,
    8, |
    send_mpst_a_to_j,
    RoleJ,
    next_j,
    9, |
    send_mpst_a_to_k,
    RoleK,
    next_k,
    10, |
    send_mpst_a_to_l,
    RoleL,
    next_l,
    11, |
    send_mpst_a_to_m,
    RoleM,
    next_m,
    12, |
    send_mpst_a_to_n,
    RoleN,
    next_n,
    13, |
    send_mpst_a_to_o,
    RoleO,
    next_o,
    14, |
    send_mpst_a_to_p,
    RoleP,
    next_p,
    15, |
    send_mpst_a_to_q,
    RoleQ,
    next_q,
    16, |
    send_mpst_a_to_r,
    RoleR,
    next_r,
    17, |
    send_mpst_a_to_s,
    RoleS,
    next_s,
    18, |
    send_mpst_a_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleA,
    SessionMpstTwenty,
    20
);
// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_b_to_c,
    RoleC,
    next_c,
    2, |
    send_mpst_b_to_d,
    RoleD,
    next_d,
    3, |
    send_mpst_b_to_e,
    RoleE,
    next_e,
    4, |
    send_mpst_b_to_f,
    RoleF,
    next_f,
    5, |
    send_mpst_b_to_g,
    RoleG,
    next_g,
    6, |
    send_mpst_b_to_h,
    RoleH,
    next_h,
    7, |
    send_mpst_b_to_i,
    RoleI,
    next_i,
    8, |
    send_mpst_b_to_j,
    RoleJ,
    next_j,
    9, |
    send_mpst_b_to_k,
    RoleK,
    next_k,
    10, |
    send_mpst_b_to_l,
    RoleL,
    next_l,
    11, |
    send_mpst_b_to_m,
    RoleM,
    next_m,
    12, |
    send_mpst_b_to_n,
    RoleN,
    next_n,
    13, |
    send_mpst_b_to_o,
    RoleO,
    next_o,
    14, |
    send_mpst_b_to_p,
    RoleP,
    next_p,
    15, |
    send_mpst_b_to_q,
    RoleQ,
    next_q,
    16, |
    send_mpst_b_to_r,
    RoleR,
    next_r,
    17, |
    send_mpst_b_to_s,
    RoleS,
    next_s,
    18, |
    send_mpst_b_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleB,
    SessionMpstTwenty,
    20
);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_c_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_c_to_d,
    RoleD,
    next_d,
    3, |
    send_mpst_c_to_e,
    RoleE,
    next_e,
    4, |
    send_mpst_c_to_f,
    RoleF,
    next_f,
    5, |
    send_mpst_c_to_g,
    RoleG,
    next_g,
    6, |
    send_mpst_c_to_h,
    RoleH,
    next_h,
    7, |
    send_mpst_c_to_i,
    RoleI,
    next_i,
    8, |
    send_mpst_c_to_j,
    RoleJ,
    next_j,
    9, |
    send_mpst_c_to_k,
    RoleK,
    next_k,
    10, |
    send_mpst_c_to_l,
    RoleL,
    next_l,
    11, |
    send_mpst_c_to_m,
    RoleM,
    next_m,
    12, |
    send_mpst_c_to_n,
    RoleN,
    next_n,
    13, |
    send_mpst_c_to_o,
    RoleO,
    next_o,
    14, |
    send_mpst_c_to_p,
    RoleP,
    next_p,
    15, |
    send_mpst_c_to_q,
    RoleQ,
    next_q,
    16, |
    send_mpst_c_to_r,
    RoleR,
    next_r,
    17, |
    send_mpst_c_to_s,
    RoleS,
    next_s,
    18, |
    send_mpst_c_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleC,
    SessionMpstTwenty,
    20
);
// D
create_send_mpst_session_bundle!(
    send_mpst_d_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_d_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_d_to_c,
    RoleC,
    next_c,
    3, |
    send_mpst_d_to_e,
    RoleE,
    next_e,
    4, |
    send_mpst_d_to_f,
    RoleF,
    next_f,
    5, |
    send_mpst_d_to_g,
    RoleG,
    next_g,
    6, |
    send_mpst_d_to_h,
    RoleH,
    next_h,
    7, |
    send_mpst_d_to_i,
    RoleI,
    next_i,
    8, |
    send_mpst_d_to_j,
    RoleJ,
    next_j,
    9, |
    send_mpst_d_to_k,
    RoleK,
    next_k,
    10, |
    send_mpst_d_to_l,
    RoleL,
    next_l,
    11, |
    send_mpst_d_to_m,
    RoleM,
    next_m,
    12, |
    send_mpst_d_to_n,
    RoleN,
    next_n,
    13, |
    send_mpst_d_to_o,
    RoleO,
    next_o,
    14, |
    send_mpst_d_to_p,
    RoleP,
    next_p,
    15, |
    send_mpst_d_to_q,
    RoleQ,
    next_q,
    16, |
    send_mpst_d_to_r,
    RoleR,
    next_r,
    17, |
    send_mpst_d_to_s,
    RoleS,
    next_s,
    18, |
    send_mpst_d_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleD,
    SessionMpstTwenty,
    20
);
// E
create_send_mpst_session_bundle!(
    send_mpst_e_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_e_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_e_to_c,
    RoleC,
    next_c,
    3, |
    send_mpst_e_to_d,
    RoleD,
    next_d,
    4, |
    send_mpst_e_to_f,
    RoleF,
    next_f,
    5, |
    send_mpst_e_to_g,
    RoleG,
    next_g,
    6, |
    send_mpst_e_to_h,
    RoleH,
    next_h,
    7, |
    send_mpst_e_to_i,
    RoleI,
    next_i,
    8, |
    send_mpst_e_to_j,
    RoleJ,
    next_j,
    9, |
    send_mpst_e_to_k,
    RoleK,
    next_k,
    10, |
    send_mpst_e_to_l,
    RoleL,
    next_l,
    11, |
    send_mpst_e_to_m,
    RoleM,
    next_m,
    12, |
    send_mpst_e_to_n,
    RoleN,
    next_n,
    13, |
    send_mpst_e_to_o,
    RoleO,
    next_o,
    14, |
    send_mpst_e_to_p,
    RoleP,
    next_p,
    15, |
    send_mpst_e_to_q,
    RoleQ,
    next_q,
    16, |
    send_mpst_e_to_r,
    RoleR,
    next_r,
    17, |
    send_mpst_e_to_s,
    RoleS,
    next_s,
    18, |
    send_mpst_e_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleE,
    SessionMpstTwenty,
    20
);
// F
create_send_mpst_session_bundle!(
    send_mpst_f_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_f_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_f_to_c,
    RoleC,
    next_c,
    3, |
    send_mpst_f_to_d,
    RoleD,
    next_d,
    4, |
    send_mpst_f_to_e,
    RoleE,
    next_e,
    5, |
    send_mpst_f_to_g,
    RoleG,
    next_g,
    6, |
    send_mpst_f_to_h,
    RoleH,
    next_h,
    7, |
    send_mpst_f_to_i,
    RoleI,
    next_i,
    8, |
    send_mpst_f_to_j,
    RoleJ,
    next_j,
    9, |
    send_mpst_f_to_k,
    RoleK,
    next_k,
    10, |
    send_mpst_f_to_l,
    RoleL,
    next_l,
    11, |
    send_mpst_f_to_m,
    RoleM,
    next_m,
    12, |
    send_mpst_f_to_n,
    RoleN,
    next_n,
    13, |
    send_mpst_f_to_o,
    RoleO,
    next_o,
    14, |
    send_mpst_f_to_p,
    RoleP,
    next_p,
    15, |
    send_mpst_f_to_q,
    RoleQ,
    next_q,
    16, |
    send_mpst_f_to_r,
    RoleR,
    next_r,
    17, |
    send_mpst_f_to_s,
    RoleS,
    next_s,
    18, |
    send_mpst_f_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleF,
    SessionMpstTwenty,
    20
);
// G
create_send_mpst_session_bundle!(
    send_mpst_g_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_g_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_g_to_c,
    RoleC,
    next_c,
    3, |
    send_mpst_g_to_d,
    RoleD,
    next_d,
    4, |
    send_mpst_g_to_e,
    RoleE,
    next_e,
    5, |
    send_mpst_g_to_f,
    RoleF,
    next_f,
    6, |
    send_mpst_g_to_h,
    RoleH,
    next_h,
    7, |
    send_mpst_g_to_i,
    RoleI,
    next_i,
    8, |
    send_mpst_g_to_j,
    RoleJ,
    next_j,
    9, |
    send_mpst_g_to_k,
    RoleK,
    next_k,
    10, |
    send_mpst_g_to_l,
    RoleL,
    next_l,
    11, |
    send_mpst_g_to_m,
    RoleM,
    next_m,
    12, |
    send_mpst_g_to_n,
    RoleN,
    next_n,
    13, |
    send_mpst_g_to_o,
    RoleO,
    next_o,
    14, |
    send_mpst_g_to_p,
    RoleP,
    next_p,
    15, |
    send_mpst_g_to_q,
    RoleQ,
    next_q,
    16, |
    send_mpst_g_to_r,
    RoleR,
    next_r,
    17, |
    send_mpst_g_to_s,
    RoleS,
    next_s,
    18, |
    send_mpst_g_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleG,
    SessionMpstTwenty,
    20
);
// H
create_send_mpst_session_bundle!(
    send_mpst_h_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_h_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_h_to_c,
    RoleC,
    next_c,
    3, |
    send_mpst_h_to_d,
    RoleD,
    next_d,
    4, |
    send_mpst_h_to_e,
    RoleE,
    next_e,
    5, |
    send_mpst_h_to_f,
    RoleF,
    next_f,
    6, |
    send_mpst_h_to_g,
    RoleG,
    next_g,
    7, |
    send_mpst_h_to_i,
    RoleI,
    next_i,
    8, |
    send_mpst_h_to_j,
    RoleJ,
    next_j,
    9, |
    send_mpst_h_to_k,
    RoleK,
    next_k,
    10, |
    send_mpst_h_to_l,
    RoleL,
    next_l,
    11, |
    send_mpst_h_to_m,
    RoleM,
    next_m,
    12, |
    send_mpst_h_to_n,
    RoleN,
    next_n,
    13, |
    send_mpst_h_to_o,
    RoleO,
    next_o,
    14, |
    send_mpst_h_to_p,
    RoleP,
    next_p,
    15, |
    send_mpst_h_to_q,
    RoleQ,
    next_q,
    16, |
    send_mpst_h_to_r,
    RoleR,
    next_r,
    17, |
    send_mpst_h_to_s,
    RoleS,
    next_s,
    18, |
    send_mpst_h_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleH,
    SessionMpstTwenty,
    20
);
// I
create_send_mpst_session_bundle!(
    send_mpst_i_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_i_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_i_to_c,
    RoleC,
    next_c,
    3, |
    send_mpst_i_to_d,
    RoleD,
    next_d,
    4, |
    send_mpst_i_to_e,
    RoleE,
    next_e,
    5, |
    send_mpst_i_to_f,
    RoleF,
    next_f,
    6, |
    send_mpst_i_to_g,
    RoleG,
    next_g,
    7, |
    send_mpst_i_to_h,
    RoleH,
    next_h,
    8, |
    send_mpst_i_to_j,
    RoleJ,
    next_j,
    9, |
    send_mpst_i_to_k,
    RoleK,
    next_k,
    10, |
    send_mpst_i_to_l,
    RoleL,
    next_l,
    11, |
    send_mpst_i_to_m,
    RoleM,
    next_m,
    12, |
    send_mpst_i_to_n,
    RoleN,
    next_n,
    13, |
    send_mpst_i_to_o,
    RoleO,
    next_o,
    14, |
    send_mpst_i_to_p,
    RoleP,
    next_p,
    15, |
    send_mpst_i_to_q,
    RoleQ,
    next_q,
    16, |
    send_mpst_i_to_r,
    RoleR,
    next_r,
    17, |
    send_mpst_i_to_s,
    RoleS,
    next_s,
    18, |
    send_mpst_i_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleI,
    SessionMpstTwenty,
    20
);
// J
create_send_mpst_session_bundle!(
    send_mpst_j_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_j_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_j_to_c,
    RoleC,
    next_c,
    3, |
    send_mpst_j_to_d,
    RoleD,
    next_d,
    4, |
    send_mpst_j_to_e,
    RoleE,
    next_e,
    5, |
    send_mpst_j_to_f,
    RoleF,
    next_f,
    6, |
    send_mpst_j_to_g,
    RoleG,
    next_g,
    7, |
    send_mpst_j_to_h,
    RoleH,
    next_h,
    8, |
    send_mpst_j_to_i,
    RoleI,
    next_i,
    9, |
    send_mpst_j_to_k,
    RoleK,
    next_k,
    10, |
    send_mpst_j_to_l,
    RoleL,
    next_l,
    11, |
    send_mpst_j_to_m,
    RoleM,
    next_m,
    12, |
    send_mpst_j_to_n,
    RoleN,
    next_n,
    13, |
    send_mpst_j_to_o,
    RoleO,
    next_o,
    14, |
    send_mpst_j_to_p,
    RoleP,
    next_p,
    15, |
    send_mpst_j_to_q,
    RoleQ,
    next_q,
    16, |
    send_mpst_j_to_r,
    RoleR,
    next_r,
    17, |
    send_mpst_j_to_s,
    RoleS,
    next_s,
    18, |
    send_mpst_j_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleJ,
    SessionMpstTwenty,
    20
);
// K
create_send_mpst_session_bundle!(
    send_mpst_k_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_k_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_k_to_c,
    RoleC,
    next_c,
    3, |
    send_mpst_k_to_d,
    RoleD,
    next_d,
    4, |
    send_mpst_k_to_e,
    RoleE,
    next_e,
    5, |
    send_mpst_k_to_f,
    RoleF,
    next_f,
    6, |
    send_mpst_k_to_g,
    RoleG,
    next_g,
    7, |
    send_mpst_k_to_h,
    RoleH,
    next_h,
    8, |
    send_mpst_k_to_i,
    RoleI,
    next_i,
    9, |
    send_mpst_k_to_j,
    RoleJ,
    next_j,
    10, |
    send_mpst_k_to_l,
    RoleL,
    next_l,
    11, |
    send_mpst_k_to_m,
    RoleM,
    next_m,
    12, |
    send_mpst_k_to_n,
    RoleN,
    next_n,
    13, |
    send_mpst_k_to_o,
    RoleO,
    next_o,
    14, |
    send_mpst_k_to_p,
    RoleP,
    next_p,
    15, |
    send_mpst_k_to_q,
    RoleQ,
    next_q,
    16, |
    send_mpst_k_to_r,
    RoleR,
    next_r,
    17, |
    send_mpst_k_to_s,
    RoleS,
    next_s,
    18, |
    send_mpst_k_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleK,
    SessionMpstTwenty,
    20
);
// L
create_send_mpst_session_bundle!(
    send_mpst_l_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_l_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_l_to_c,
    RoleC,
    next_c,
    3, |
    send_mpst_l_to_d,
    RoleD,
    next_d,
    4, |
    send_mpst_l_to_e,
    RoleE,
    next_e,
    5, |
    send_mpst_l_to_f,
    RoleF,
    next_f,
    6, |
    send_mpst_l_to_g,
    RoleG,
    next_g,
    7, |
    send_mpst_l_to_h,
    RoleH,
    next_h,
    8, |
    send_mpst_l_to_i,
    RoleI,
    next_i,
    9, |
    send_mpst_l_to_j,
    RoleJ,
    next_j,
    10, |
    send_mpst_l_to_k,
    RoleK,
    next_k,
    11, |
    send_mpst_l_to_m,
    RoleM,
    next_m,
    12, |
    send_mpst_l_to_n,
    RoleN,
    next_n,
    13, |
    send_mpst_l_to_o,
    RoleO,
    next_o,
    14, |
    send_mpst_l_to_p,
    RoleP,
    next_p,
    15, |
    send_mpst_l_to_q,
    RoleQ,
    next_q,
    16, |
    send_mpst_l_to_r,
    RoleR,
    next_r,
    17, |
    send_mpst_l_to_s,
    RoleS,
    next_s,
    18, |
    send_mpst_l_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleL,
    SessionMpstTwenty,
    20
);
// M
create_send_mpst_session_bundle!(
    send_mpst_m_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_m_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_m_to_c,
    RoleC,
    next_c,
    3, |
    send_mpst_m_to_d,
    RoleD,
    next_d,
    4, |
    send_mpst_m_to_e,
    RoleE,
    next_e,
    5, |
    send_mpst_m_to_f,
    RoleF,
    next_f,
    6, |
    send_mpst_m_to_g,
    RoleG,
    next_g,
    7, |
    send_mpst_m_to_h,
    RoleH,
    next_h,
    8, |
    send_mpst_m_to_i,
    RoleI,
    next_i,
    9, |
    send_mpst_m_to_j,
    RoleJ,
    next_j,
    10, |
    send_mpst_m_to_k,
    RoleK,
    next_k,
    11, |
    send_mpst_m_to_l,
    RoleL,
    next_l,
    12, |
    send_mpst_m_to_n,
    RoleN,
    next_n,
    13, |
    send_mpst_m_to_o,
    RoleO,
    next_o,
    14, |
    send_mpst_m_to_p,
    RoleP,
    next_p,
    15, |
    send_mpst_m_to_q,
    RoleQ,
    next_q,
    16, |
    send_mpst_m_to_r,
    RoleR,
    next_r,
    17, |
    send_mpst_m_to_s,
    RoleS,
    next_s,
    18, |
    send_mpst_m_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleM,
    SessionMpstTwenty,
    20
);
// N
create_send_mpst_session_bundle!(
    send_mpst_n_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_n_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_n_to_c,
    RoleC,
    next_c,
    3, |
    send_mpst_n_to_d,
    RoleD,
    next_d,
    4, |
    send_mpst_n_to_e,
    RoleE,
    next_e,
    5, |
    send_mpst_n_to_f,
    RoleF,
    next_f,
    6, |
    send_mpst_n_to_g,
    RoleG,
    next_g,
    7, |
    send_mpst_n_to_h,
    RoleH,
    next_h,
    8, |
    send_mpst_n_to_i,
    RoleI,
    next_i,
    9, |
    send_mpst_n_to_j,
    RoleJ,
    next_j,
    10, |
    send_mpst_n_to_k,
    RoleK,
    next_k,
    11, |
    send_mpst_n_to_l,
    RoleL,
    next_l,
    12, |
    send_mpst_n_to_m,
    RoleM,
    next_m,
    13, |
    send_mpst_n_to_o,
    RoleO,
    next_o,
    14, |
    send_mpst_n_to_p,
    RoleP,
    next_p,
    15, |
    send_mpst_n_to_q,
    RoleQ,
    next_q,
    16, |
    send_mpst_n_to_r,
    RoleR,
    next_r,
    17, |
    send_mpst_n_to_s,
    RoleS,
    next_s,
    18, |
    send_mpst_n_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleN,
    SessionMpstTwenty,
    20
);
// O
create_send_mpst_session_bundle!(
    send_mpst_o_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_o_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_o_to_c,
    RoleC,
    next_c,
    3, |
    send_mpst_o_to_d,
    RoleD,
    next_d,
    4, |
    send_mpst_o_to_e,
    RoleE,
    next_e,
    5, |
    send_mpst_o_to_f,
    RoleF,
    next_f,
    6, |
    send_mpst_o_to_g,
    RoleG,
    next_g,
    7, |
    send_mpst_o_to_h,
    RoleH,
    next_h,
    8, |
    send_mpst_o_to_i,
    RoleI,
    next_i,
    9, |
    send_mpst_o_to_j,
    RoleJ,
    next_j,
    10, |
    send_mpst_o_to_k,
    RoleK,
    next_k,
    11, |
    send_mpst_o_to_l,
    RoleL,
    next_l,
    12, |
    send_mpst_o_to_m,
    RoleM,
    next_m,
    13, |
    send_mpst_o_to_n,
    RoleN,
    next_n,
    14, |
    send_mpst_o_to_p,
    RoleP,
    next_p,
    15, |
    send_mpst_o_to_q,
    RoleQ,
    next_q,
    16, |
    send_mpst_o_to_r,
    RoleR,
    next_r,
    17, |
    send_mpst_o_to_s,
    RoleS,
    next_s,
    18, |
    send_mpst_o_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleO,
    SessionMpstTwenty,
    20
);
// P
create_send_mpst_session_bundle!(
    send_mpst_p_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_p_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_p_to_c,
    RoleC,
    next_c,
    3, |
    send_mpst_p_to_d,
    RoleD,
    next_d,
    4, |
    send_mpst_p_to_e,
    RoleE,
    next_e,
    5, |
    send_mpst_p_to_f,
    RoleF,
    next_f,
    6, |
    send_mpst_p_to_g,
    RoleG,
    next_g,
    7, |
    send_mpst_p_to_h,
    RoleH,
    next_h,
    8, |
    send_mpst_p_to_i,
    RoleI,
    next_i,
    9, |
    send_mpst_p_to_j,
    RoleJ,
    next_j,
    10, |
    send_mpst_p_to_k,
    RoleK,
    next_k,
    11, |
    send_mpst_p_to_l,
    RoleL,
    next_l,
    12, |
    send_mpst_p_to_m,
    RoleM,
    next_m,
    13, |
    send_mpst_p_to_n,
    RoleN,
    next_n,
    14, |
    send_mpst_p_to_o,
    RoleO,
    next_o,
    15, |
    send_mpst_p_to_q,
    RoleQ,
    next_q,
    16, |
    send_mpst_p_to_r,
    RoleR,
    next_r,
    17, |
    send_mpst_p_to_s,
    RoleS,
    next_s,
    18, |
    send_mpst_p_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleP,
    SessionMpstTwenty,
    20
);
// Q
create_send_mpst_session_bundle!(
    send_mpst_q_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_q_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_q_to_c,
    RoleC,
    next_c,
    3, |
    send_mpst_q_to_d,
    RoleD,
    next_d,
    4, |
    send_mpst_q_to_e,
    RoleE,
    next_e,
    5, |
    send_mpst_q_to_f,
    RoleF,
    next_f,
    6, |
    send_mpst_q_to_g,
    RoleG,
    next_g,
    7, |
    send_mpst_q_to_h,
    RoleH,
    next_h,
    8, |
    send_mpst_q_to_i,
    RoleI,
    next_i,
    9, |
    send_mpst_q_to_j,
    RoleJ,
    next_j,
    10, |
    send_mpst_q_to_k,
    RoleK,
    next_k,
    11, |
    send_mpst_q_to_l,
    RoleL,
    next_l,
    12, |
    send_mpst_q_to_m,
    RoleM,
    next_m,
    13, |
    send_mpst_q_to_n,
    RoleN,
    next_n,
    14, |
    send_mpst_q_to_o,
    RoleO,
    next_o,
    15, |
    send_mpst_q_to_p,
    RoleP,
    next_p,
    16, |
    send_mpst_q_to_r,
    RoleR,
    next_r,
    17, |
    send_mpst_q_to_s,
    RoleS,
    next_s,
    18, |
    send_mpst_q_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleQ,
    SessionMpstTwenty,
    20
);
// R
create_send_mpst_session_bundle!(
    send_mpst_r_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_r_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_r_to_c,
    RoleC,
    next_c,
    3, |
    send_mpst_r_to_d,
    RoleD,
    next_d,
    4, |
    send_mpst_r_to_e,
    RoleE,
    next_e,
    5, |
    send_mpst_r_to_f,
    RoleF,
    next_f,
    6, |
    send_mpst_r_to_g,
    RoleG,
    next_g,
    7, |
    send_mpst_r_to_h,
    RoleH,
    next_h,
    8, |
    send_mpst_r_to_i,
    RoleI,
    next_i,
    9, |
    send_mpst_r_to_j,
    RoleJ,
    next_j,
    10, |
    send_mpst_r_to_k,
    RoleK,
    next_k,
    11, |
    send_mpst_r_to_l,
    RoleL,
    next_l,
    12, |
    send_mpst_r_to_m,
    RoleM,
    next_m,
    13, |
    send_mpst_r_to_n,
    RoleN,
    next_n,
    14, |
    send_mpst_r_to_o,
    RoleO,
    next_o,
    15, |
    send_mpst_r_to_p,
    RoleP,
    next_p,
    16, |
    send_mpst_r_to_q,
    RoleQ,
    next_q,
    17, |
    send_mpst_r_to_s,
    RoleS,
    next_s,
    18, |
    send_mpst_r_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleR,
    SessionMpstTwenty,
    20
);
// S
create_send_mpst_session_bundle!(
    send_mpst_s_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_s_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_s_to_c,
    RoleC,
    next_c,
    3, |
    send_mpst_s_to_d,
    RoleD,
    next_d,
    4, |
    send_mpst_s_to_e,
    RoleE,
    next_e,
    5, |
    send_mpst_s_to_f,
    RoleF,
    next_f,
    6, |
    send_mpst_s_to_g,
    RoleG,
    next_g,
    7, |
    send_mpst_s_to_h,
    RoleH,
    next_h,
    8, |
    send_mpst_s_to_i,
    RoleI,
    next_i,
    9, |
    send_mpst_s_to_j,
    RoleJ,
    next_j,
    10, |
    send_mpst_s_to_k,
    RoleK,
    next_k,
    11, |
    send_mpst_s_to_l,
    RoleL,
    next_l,
    12, |
    send_mpst_s_to_m,
    RoleM,
    next_m,
    13, |
    send_mpst_s_to_n,
    RoleN,
    next_n,
    14, |
    send_mpst_s_to_o,
    RoleO,
    next_o,
    15, |
    send_mpst_s_to_p,
    RoleP,
    next_p,
    16, |
    send_mpst_s_to_q,
    RoleQ,
    next_q,
    17, |
    send_mpst_s_to_r,
    RoleR,
    next_r,
    18, |
    send_mpst_s_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleS,
    SessionMpstTwenty,
    20
);
// T
create_send_mpst_session_bundle!(
    send_mpst_t_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_t_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_t_to_c,
    RoleC,
    next_c,
    3, |
    send_mpst_t_to_d,
    RoleD,
    next_d,
    4, |
    send_mpst_t_to_e,
    RoleE,
    next_e,
    5, |
    send_mpst_t_to_f,
    RoleF,
    next_f,
    6, |
    send_mpst_t_to_g,
    RoleG,
    next_g,
    7, |
    send_mpst_t_to_h,
    RoleH,
    next_h,
    8, |
    send_mpst_t_to_i,
    RoleI,
    next_i,
    9, |
    send_mpst_t_to_j,
    RoleJ,
    next_j,
    10, |
    send_mpst_t_to_k,
    RoleK,
    next_k,
    11, |
    send_mpst_t_to_l,
    RoleL,
    next_l,
    12, |
    send_mpst_t_to_m,
    RoleM,
    next_m,
    13, |
    send_mpst_t_to_n,
    RoleN,
    next_n,
    14, |
    send_mpst_t_to_o,
    RoleO,
    next_o,
    15, |
    send_mpst_t_to_p,
    RoleP,
    next_p,
    16, |
    send_mpst_t_to_q,
    RoleQ,
    next_q,
    17, |
    send_mpst_t_to_r,
    RoleR,
    next_r,
    18, |
    send_mpst_t_to_s,
    RoleS,
    next_s,
    19, | =>
    RoleT,
    SessionMpstTwenty,
    20
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_to_b,
    RoleB,
    next_b,
    1, |
    recv_mpst_a_to_c,
    RoleC,
    next_c,
    2, |
    recv_mpst_a_to_d,
    RoleD,
    next_d,
    3, |
    recv_mpst_a_to_e,
    RoleE,
    next_e,
    4, |
    recv_mpst_a_to_f,
    RoleF,
    next_f,
    5, |
    recv_mpst_a_to_g,
    RoleG,
    next_g,
    6, |
    recv_mpst_a_to_h,
    RoleH,
    next_h,
    7, |
    recv_mpst_a_to_i,
    RoleI,
    next_i,
    8, |
    recv_mpst_a_to_j,
    RoleJ,
    next_j,
    9, |
    recv_mpst_a_to_k,
    RoleK,
    next_k,
    10, |
    recv_mpst_a_to_l,
    RoleL,
    next_l,
    11, |
    recv_mpst_a_to_m,
    RoleM,
    next_m,
    12, |
    recv_mpst_a_to_n,
    RoleN,
    next_n,
    13, |
    recv_mpst_a_to_o,
    RoleO,
    next_o,
    14, |
    recv_mpst_a_to_p,
    RoleP,
    next_p,
    15, |
    recv_mpst_a_to_q,
    RoleQ,
    next_q,
    16, |
    recv_mpst_a_to_r,
    RoleR,
    next_r,
    17, |
    recv_mpst_a_to_s,
    RoleS,
    next_s,
    18, |
    recv_mpst_a_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleA,
    SessionMpstTwenty,
    20
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_b_to_c,
    RoleC,
    next_c,
    2, |
    recv_mpst_b_to_d,
    RoleD,
    next_d,
    3, |
    recv_mpst_b_to_e,
    RoleE,
    next_e,
    4, |
    recv_mpst_b_to_f,
    RoleF,
    next_f,
    5, |
    recv_mpst_b_to_g,
    RoleG,
    next_g,
    6, |
    recv_mpst_b_to_h,
    RoleH,
    next_h,
    7, |
    recv_mpst_b_to_i,
    RoleI,
    next_i,
    8, |
    recv_mpst_b_to_j,
    RoleJ,
    next_j,
    9, |
    recv_mpst_b_to_k,
    RoleK,
    next_k,
    10, |
    recv_mpst_b_to_l,
    RoleL,
    next_l,
    11, |
    recv_mpst_b_to_m,
    RoleM,
    next_m,
    12, |
    recv_mpst_b_to_n,
    RoleN,
    next_n,
    13, |
    recv_mpst_b_to_o,
    RoleO,
    next_o,
    14, |
    recv_mpst_b_to_p,
    RoleP,
    next_p,
    15, |
    recv_mpst_b_to_q,
    RoleQ,
    next_q,
    16, |
    recv_mpst_b_to_r,
    RoleR,
    next_r,
    17, |
    recv_mpst_b_to_s,
    RoleS,
    next_s,
    18, |
    recv_mpst_b_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleB,
    SessionMpstTwenty,
    20
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_c_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_c_to_d,
    RoleD,
    next_d,
    3, |
    recv_mpst_c_to_e,
    RoleE,
    next_e,
    4, |
    recv_mpst_c_to_f,
    RoleF,
    next_f,
    5, |
    recv_mpst_c_to_g,
    RoleG,
    next_g,
    6, |
    recv_mpst_c_to_h,
    RoleH,
    next_h,
    7, |
    recv_mpst_c_to_i,
    RoleI,
    next_i,
    8, |
    recv_mpst_c_to_j,
    RoleJ,
    next_j,
    9, |
    recv_mpst_c_to_k,
    RoleK,
    next_k,
    10, |
    recv_mpst_c_to_l,
    RoleL,
    next_l,
    11, |
    recv_mpst_c_to_m,
    RoleM,
    next_m,
    12, |
    recv_mpst_c_to_n,
    RoleN,
    next_n,
    13, |
    recv_mpst_c_to_o,
    RoleO,
    next_o,
    14, |
    recv_mpst_c_to_p,
    RoleP,
    next_p,
    15, |
    recv_mpst_c_to_q,
    RoleQ,
    next_q,
    16, |
    recv_mpst_c_to_r,
    RoleR,
    next_r,
    17, |
    recv_mpst_c_to_s,
    RoleS,
    next_s,
    18, |
    recv_mpst_c_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleC,
    SessionMpstTwenty,
    20
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_d_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_d_to_c,
    RoleC,
    next_c,
    3, |
    recv_mpst_d_to_e,
    RoleE,
    next_e,
    4, |
    recv_mpst_d_to_f,
    RoleF,
    next_f,
    5, |
    recv_mpst_d_to_g,
    RoleG,
    next_g,
    6, |
    recv_mpst_d_to_h,
    RoleH,
    next_h,
    7, |
    recv_mpst_d_to_i,
    RoleI,
    next_i,
    8, |
    recv_mpst_d_to_j,
    RoleJ,
    next_j,
    9, |
    recv_mpst_d_to_k,
    RoleK,
    next_k,
    10, |
    recv_mpst_d_to_l,
    RoleL,
    next_l,
    11, |
    recv_mpst_d_to_m,
    RoleM,
    next_m,
    12, |
    recv_mpst_d_to_n,
    RoleN,
    next_n,
    13, |
    recv_mpst_d_to_o,
    RoleO,
    next_o,
    14, |
    recv_mpst_d_to_p,
    RoleP,
    next_p,
    15, |
    recv_mpst_d_to_q,
    RoleQ,
    next_q,
    16, |
    recv_mpst_d_to_r,
    RoleR,
    next_r,
    17, |
    recv_mpst_d_to_s,
    RoleS,
    next_s,
    18, |
    recv_mpst_d_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleD,
    SessionMpstTwenty,
    20
);
// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_e_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_e_to_c,
    RoleC,
    next_c,
    3, |
    recv_mpst_e_to_d,
    RoleD,
    next_d,
    4, |
    recv_mpst_e_to_f,
    RoleF,
    next_f,
    5, |
    recv_mpst_e_to_g,
    RoleG,
    next_g,
    6, |
    recv_mpst_e_to_h,
    RoleH,
    next_h,
    7, |
    recv_mpst_e_to_i,
    RoleI,
    next_i,
    8, |
    recv_mpst_e_to_j,
    RoleJ,
    next_j,
    9, |
    recv_mpst_e_to_k,
    RoleK,
    next_k,
    10, |
    recv_mpst_e_to_l,
    RoleL,
    next_l,
    11, |
    recv_mpst_e_to_m,
    RoleM,
    next_m,
    12, |
    recv_mpst_e_to_n,
    RoleN,
    next_n,
    13, |
    recv_mpst_e_to_o,
    RoleO,
    next_o,
    14, |
    recv_mpst_e_to_p,
    RoleP,
    next_p,
    15, |
    recv_mpst_e_to_q,
    RoleQ,
    next_q,
    16, |
    recv_mpst_e_to_r,
    RoleR,
    next_r,
    17, |
    recv_mpst_e_to_s,
    RoleS,
    next_s,
    18, |
    recv_mpst_e_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleE,
    SessionMpstTwenty,
    20
);
// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_f_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_f_to_c,
    RoleC,
    next_c,
    3, |
    recv_mpst_f_to_d,
    RoleD,
    next_d,
    4, |
    recv_mpst_f_to_e,
    RoleE,
    next_e,
    5, |
    recv_mpst_f_to_g,
    RoleG,
    next_g,
    6, |
    recv_mpst_f_to_h,
    RoleH,
    next_h,
    7, |
    recv_mpst_f_to_i,
    RoleI,
    next_i,
    8, |
    recv_mpst_f_to_j,
    RoleJ,
    next_j,
    9, |
    recv_mpst_f_to_k,
    RoleK,
    next_k,
    10, |
    recv_mpst_f_to_l,
    RoleL,
    next_l,
    11, |
    recv_mpst_f_to_m,
    RoleM,
    next_m,
    12, |
    recv_mpst_f_to_n,
    RoleN,
    next_n,
    13, |
    recv_mpst_f_to_o,
    RoleO,
    next_o,
    14, |
    recv_mpst_f_to_p,
    RoleP,
    next_p,
    15, |
    recv_mpst_f_to_q,
    RoleQ,
    next_q,
    16, |
    recv_mpst_f_to_r,
    RoleR,
    next_r,
    17, |
    recv_mpst_f_to_s,
    RoleS,
    next_s,
    18, |
    recv_mpst_f_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleF,
    SessionMpstTwenty,
    20
);
// G
create_recv_mpst_session_bundle!(
    recv_mpst_g_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_g_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_g_to_c,
    RoleC,
    next_c,
    3, |
    recv_mpst_g_to_d,
    RoleD,
    next_d,
    4, |
    recv_mpst_g_to_e,
    RoleE,
    next_e,
    5, |
    recv_mpst_g_to_f,
    RoleF,
    next_f,
    6, |
    recv_mpst_g_to_h,
    RoleH,
    next_h,
    7, |
    recv_mpst_g_to_i,
    RoleI,
    next_i,
    8, |
    recv_mpst_g_to_j,
    RoleJ,
    next_j,
    9, |
    recv_mpst_g_to_k,
    RoleK,
    next_k,
    10, |
    recv_mpst_g_to_l,
    RoleL,
    next_l,
    11, |
    recv_mpst_g_to_m,
    RoleM,
    next_m,
    12, |
    recv_mpst_g_to_n,
    RoleN,
    next_n,
    13, |
    recv_mpst_g_to_o,
    RoleO,
    next_o,
    14, |
    recv_mpst_g_to_p,
    RoleP,
    next_p,
    15, |
    recv_mpst_g_to_q,
    RoleQ,
    next_q,
    16, |
    recv_mpst_g_to_r,
    RoleR,
    next_r,
    17, |
    recv_mpst_g_to_s,
    RoleS,
    next_s,
    18, |
    recv_mpst_g_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleG,
    SessionMpstTwenty,
    20
);
// H
create_recv_mpst_session_bundle!(
    recv_mpst_h_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_h_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_h_to_c,
    RoleC,
    next_c,
    3, |
    recv_mpst_h_to_d,
    RoleD,
    next_d,
    4, |
    recv_mpst_h_to_e,
    RoleE,
    next_e,
    5, |
    recv_mpst_h_to_f,
    RoleF,
    next_f,
    6, |
    recv_mpst_h_to_g,
    RoleG,
    next_g,
    7, |
    recv_mpst_h_to_i,
    RoleI,
    next_i,
    8, |
    recv_mpst_h_to_j,
    RoleJ,
    next_j,
    9, |
    recv_mpst_h_to_k,
    RoleK,
    next_k,
    10, |
    recv_mpst_h_to_l,
    RoleL,
    next_l,
    11, |
    recv_mpst_h_to_m,
    RoleM,
    next_m,
    12, |
    recv_mpst_h_to_n,
    RoleN,
    next_n,
    13, |
    recv_mpst_h_to_o,
    RoleO,
    next_o,
    14, |
    recv_mpst_h_to_p,
    RoleP,
    next_p,
    15, |
    recv_mpst_h_to_q,
    RoleQ,
    next_q,
    16, |
    recv_mpst_h_to_r,
    RoleR,
    next_r,
    17, |
    recv_mpst_h_to_s,
    RoleS,
    next_s,
    18, |
    recv_mpst_h_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleH,
    SessionMpstTwenty,
    20
);
// I
create_recv_mpst_session_bundle!(
    recv_mpst_i_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_i_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_i_to_c,
    RoleC,
    next_c,
    3, |
    recv_mpst_i_to_d,
    RoleD,
    next_d,
    4, |
    recv_mpst_i_to_e,
    RoleE,
    next_e,
    5, |
    recv_mpst_i_to_f,
    RoleF,
    next_f,
    6, |
    recv_mpst_i_to_g,
    RoleG,
    next_g,
    7, |
    recv_mpst_i_to_h,
    RoleH,
    next_h,
    8, |
    recv_mpst_i_to_j,
    RoleJ,
    next_j,
    9, |
    recv_mpst_i_to_k,
    RoleK,
    next_k,
    10, |
    recv_mpst_i_to_l,
    RoleL,
    next_l,
    11, |
    recv_mpst_i_to_m,
    RoleM,
    next_m,
    12, |
    recv_mpst_i_to_n,
    RoleN,
    next_n,
    13, |
    recv_mpst_i_to_o,
    RoleO,
    next_o,
    14, |
    recv_mpst_i_to_p,
    RoleP,
    next_p,
    15, |
    recv_mpst_i_to_q,
    RoleQ,
    next_q,
    16, |
    recv_mpst_i_to_r,
    RoleR,
    next_r,
    17, |
    recv_mpst_i_to_s,
    RoleS,
    next_s,
    18, |
    recv_mpst_i_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleI,
    SessionMpstTwenty,
    20
);
// J
create_recv_mpst_session_bundle!(
    recv_mpst_j_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_j_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_j_to_c,
    RoleC,
    next_c,
    3, |
    recv_mpst_j_to_d,
    RoleD,
    next_d,
    4, |
    recv_mpst_j_to_e,
    RoleE,
    next_e,
    5, |
    recv_mpst_j_to_f,
    RoleF,
    next_f,
    6, |
    recv_mpst_j_to_g,
    RoleG,
    next_g,
    7, |
    recv_mpst_j_to_h,
    RoleH,
    next_h,
    8, |
    recv_mpst_j_to_i,
    RoleI,
    next_i,
    9, |
    recv_mpst_j_to_k,
    RoleK,
    next_k,
    10, |
    recv_mpst_j_to_l,
    RoleL,
    next_l,
    11, |
    recv_mpst_j_to_m,
    RoleM,
    next_m,
    12, |
    recv_mpst_j_to_n,
    RoleN,
    next_n,
    13, |
    recv_mpst_j_to_o,
    RoleO,
    next_o,
    14, |
    recv_mpst_j_to_p,
    RoleP,
    next_p,
    15, |
    recv_mpst_j_to_q,
    RoleQ,
    next_q,
    16, |
    recv_mpst_j_to_r,
    RoleR,
    next_r,
    17, |
    recv_mpst_j_to_s,
    RoleS,
    next_s,
    18, |
    recv_mpst_j_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleJ,
    SessionMpstTwenty,
    20
);
// K
create_recv_mpst_session_bundle!(
    recv_mpst_k_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_k_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_k_to_c,
    RoleC,
    next_c,
    3, |
    recv_mpst_k_to_d,
    RoleD,
    next_d,
    4, |
    recv_mpst_k_to_e,
    RoleE,
    next_e,
    5, |
    recv_mpst_k_to_f,
    RoleF,
    next_f,
    6, |
    recv_mpst_k_to_g,
    RoleG,
    next_g,
    7, |
    recv_mpst_k_to_h,
    RoleH,
    next_h,
    8, |
    recv_mpst_k_to_i,
    RoleI,
    next_i,
    9, |
    recv_mpst_k_to_j,
    RoleJ,
    next_j,
    10, |
    recv_mpst_k_to_l,
    RoleL,
    next_l,
    11, |
    recv_mpst_k_to_m,
    RoleM,
    next_m,
    12, |
    recv_mpst_k_to_n,
    RoleN,
    next_n,
    13, |
    recv_mpst_k_to_o,
    RoleO,
    next_o,
    14, |
    recv_mpst_k_to_p,
    RoleP,
    next_p,
    15, |
    recv_mpst_k_to_q,
    RoleQ,
    next_q,
    16, |
    recv_mpst_k_to_r,
    RoleR,
    next_r,
    17, |
    recv_mpst_k_to_s,
    RoleS,
    next_s,
    18, |
    recv_mpst_k_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleK,
    SessionMpstTwenty,
    20
);
// L
create_recv_mpst_session_bundle!(
    recv_mpst_l_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_l_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_l_to_c,
    RoleC,
    next_c,
    3, |
    recv_mpst_l_to_d,
    RoleD,
    next_d,
    4, |
    recv_mpst_l_to_e,
    RoleE,
    next_e,
    5, |
    recv_mpst_l_to_f,
    RoleF,
    next_f,
    6, |
    recv_mpst_l_to_g,
    RoleG,
    next_g,
    7, |
    recv_mpst_l_to_h,
    RoleH,
    next_h,
    8, |
    recv_mpst_l_to_i,
    RoleI,
    next_i,
    9, |
    recv_mpst_l_to_j,
    RoleJ,
    next_j,
    10, |
    recv_mpst_l_to_k,
    RoleK,
    next_k,
    11, |
    recv_mpst_l_to_m,
    RoleM,
    next_m,
    12, |
    recv_mpst_l_to_n,
    RoleN,
    next_n,
    13, |
    recv_mpst_l_to_o,
    RoleO,
    next_o,
    14, |
    recv_mpst_l_to_p,
    RoleP,
    next_p,
    15, |
    recv_mpst_l_to_q,
    RoleQ,
    next_q,
    16, |
    recv_mpst_l_to_r,
    RoleR,
    next_r,
    17, |
    recv_mpst_l_to_s,
    RoleS,
    next_s,
    18, |
    recv_mpst_l_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleL,
    SessionMpstTwenty,
    20
);
// M
create_recv_mpst_session_bundle!(
    recv_mpst_m_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_m_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_m_to_c,
    RoleC,
    next_c,
    3, |
    recv_mpst_m_to_d,
    RoleD,
    next_d,
    4, |
    recv_mpst_m_to_e,
    RoleE,
    next_e,
    5, |
    recv_mpst_m_to_f,
    RoleF,
    next_f,
    6, |
    recv_mpst_m_to_g,
    RoleG,
    next_g,
    7, |
    recv_mpst_m_to_h,
    RoleH,
    next_h,
    8, |
    recv_mpst_m_to_i,
    RoleI,
    next_i,
    9, |
    recv_mpst_m_to_j,
    RoleJ,
    next_j,
    10, |
    recv_mpst_m_to_k,
    RoleK,
    next_k,
    11, |
    recv_mpst_m_to_l,
    RoleL,
    next_l,
    12, |
    recv_mpst_m_to_n,
    RoleN,
    next_n,
    13, |
    recv_mpst_m_to_o,
    RoleO,
    next_o,
    14, |
    recv_mpst_m_to_p,
    RoleP,
    next_p,
    15, |
    recv_mpst_m_to_q,
    RoleQ,
    next_q,
    16, |
    recv_mpst_m_to_r,
    RoleR,
    next_r,
    17, |
    recv_mpst_m_to_s,
    RoleS,
    next_s,
    18, |
    recv_mpst_m_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleM,
    SessionMpstTwenty,
    20
);
// N
create_recv_mpst_session_bundle!(
    recv_mpst_n_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_n_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_n_to_c,
    RoleC,
    next_c,
    3, |
    recv_mpst_n_to_d,
    RoleD,
    next_d,
    4, |
    recv_mpst_n_to_e,
    RoleE,
    next_e,
    5, |
    recv_mpst_n_to_f,
    RoleF,
    next_f,
    6, |
    recv_mpst_n_to_g,
    RoleG,
    next_g,
    7, |
    recv_mpst_n_to_h,
    RoleH,
    next_h,
    8, |
    recv_mpst_n_to_i,
    RoleI,
    next_i,
    9, |
    recv_mpst_n_to_j,
    RoleJ,
    next_j,
    10, |
    recv_mpst_n_to_k,
    RoleK,
    next_k,
    11, |
    recv_mpst_n_to_l,
    RoleL,
    next_l,
    12, |
    recv_mpst_n_to_m,
    RoleM,
    next_m,
    13, |
    recv_mpst_n_to_o,
    RoleO,
    next_o,
    14, |
    recv_mpst_n_to_p,
    RoleP,
    next_p,
    15, |
    recv_mpst_n_to_q,
    RoleQ,
    next_q,
    16, |
    recv_mpst_n_to_r,
    RoleR,
    next_r,
    17, |
    recv_mpst_n_to_s,
    RoleS,
    next_s,
    18, |
    recv_mpst_n_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleN,
    SessionMpstTwenty,
    20
);
// O
create_recv_mpst_session_bundle!(
    recv_mpst_o_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_o_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_o_to_c,
    RoleC,
    next_c,
    3, |
    recv_mpst_o_to_d,
    RoleD,
    next_d,
    4, |
    recv_mpst_o_to_e,
    RoleE,
    next_e,
    5, |
    recv_mpst_o_to_f,
    RoleF,
    next_f,
    6, |
    recv_mpst_o_to_g,
    RoleG,
    next_g,
    7, |
    recv_mpst_o_to_h,
    RoleH,
    next_h,
    8, |
    recv_mpst_o_to_i,
    RoleI,
    next_i,
    9, |
    recv_mpst_o_to_j,
    RoleJ,
    next_j,
    10, |
    recv_mpst_o_to_k,
    RoleK,
    next_k,
    11, |
    recv_mpst_o_to_l,
    RoleL,
    next_l,
    12, |
    recv_mpst_o_to_m,
    RoleM,
    next_m,
    13, |
    recv_mpst_o_to_n,
    RoleN,
    next_n,
    14, |
    recv_mpst_o_to_p,
    RoleP,
    next_p,
    15, |
    recv_mpst_o_to_q,
    RoleQ,
    next_q,
    16, |
    recv_mpst_o_to_r,
    RoleR,
    next_r,
    17, |
    recv_mpst_o_to_s,
    RoleS,
    next_s,
    18, |
    recv_mpst_o_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleO,
    SessionMpstTwenty,
    20
);
// P
create_recv_mpst_session_bundle!(
    recv_mpst_p_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_p_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_p_to_c,
    RoleC,
    next_c,
    3, |
    recv_mpst_p_to_d,
    RoleD,
    next_d,
    4, |
    recv_mpst_p_to_e,
    RoleE,
    next_e,
    5, |
    recv_mpst_p_to_f,
    RoleF,
    next_f,
    6, |
    recv_mpst_p_to_g,
    RoleG,
    next_g,
    7, |
    recv_mpst_p_to_h,
    RoleH,
    next_h,
    8, |
    recv_mpst_p_to_i,
    RoleI,
    next_i,
    9, |
    recv_mpst_p_to_j,
    RoleJ,
    next_j,
    10, |
    recv_mpst_p_to_k,
    RoleK,
    next_k,
    11, |
    recv_mpst_p_to_l,
    RoleL,
    next_l,
    12, |
    recv_mpst_p_to_m,
    RoleM,
    next_m,
    13, |
    recv_mpst_p_to_n,
    RoleN,
    next_n,
    14, |
    recv_mpst_p_to_o,
    RoleO,
    next_o,
    15, |
    recv_mpst_p_to_q,
    RoleQ,
    next_q,
    16, |
    recv_mpst_p_to_r,
    RoleR,
    next_r,
    17, |
    recv_mpst_p_to_s,
    RoleS,
    next_s,
    18, |
    recv_mpst_p_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleP,
    SessionMpstTwenty,
    20
);
// Q
create_recv_mpst_session_bundle!(
    recv_mpst_q_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_q_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_q_to_c,
    RoleC,
    next_c,
    3, |
    recv_mpst_q_to_d,
    RoleD,
    next_d,
    4, |
    recv_mpst_q_to_e,
    RoleE,
    next_e,
    5, |
    recv_mpst_q_to_f,
    RoleF,
    next_f,
    6, |
    recv_mpst_q_to_g,
    RoleG,
    next_g,
    7, |
    recv_mpst_q_to_h,
    RoleH,
    next_h,
    8, |
    recv_mpst_q_to_i,
    RoleI,
    next_i,
    9, |
    recv_mpst_q_to_j,
    RoleJ,
    next_j,
    10, |
    recv_mpst_q_to_k,
    RoleK,
    next_k,
    11, |
    recv_mpst_q_to_l,
    RoleL,
    next_l,
    12, |
    recv_mpst_q_to_m,
    RoleM,
    next_m,
    13, |
    recv_mpst_q_to_n,
    RoleN,
    next_n,
    14, |
    recv_mpst_q_to_o,
    RoleO,
    next_o,
    15, |
    recv_mpst_q_to_p,
    RoleP,
    next_p,
    16, |
    recv_mpst_q_to_r,
    RoleR,
    next_r,
    17, |
    recv_mpst_q_to_s,
    RoleS,
    next_s,
    18, |
    recv_mpst_q_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleQ,
    SessionMpstTwenty,
    20
);
// R
create_recv_mpst_session_bundle!(
    recv_mpst_r_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_r_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_r_to_c,
    RoleC,
    next_c,
    3, |
    recv_mpst_r_to_d,
    RoleD,
    next_d,
    4, |
    recv_mpst_r_to_e,
    RoleE,
    next_e,
    5, |
    recv_mpst_r_to_f,
    RoleF,
    next_f,
    6, |
    recv_mpst_r_to_g,
    RoleG,
    next_g,
    7, |
    recv_mpst_r_to_h,
    RoleH,
    next_h,
    8, |
    recv_mpst_r_to_i,
    RoleI,
    next_i,
    9, |
    recv_mpst_r_to_j,
    RoleJ,
    next_j,
    10, |
    recv_mpst_r_to_k,
    RoleK,
    next_k,
    11, |
    recv_mpst_r_to_l,
    RoleL,
    next_l,
    12, |
    recv_mpst_r_to_m,
    RoleM,
    next_m,
    13, |
    recv_mpst_r_to_n,
    RoleN,
    next_n,
    14, |
    recv_mpst_r_to_o,
    RoleO,
    next_o,
    15, |
    recv_mpst_r_to_p,
    RoleP,
    next_p,
    16, |
    recv_mpst_r_to_q,
    RoleQ,
    next_q,
    17, |
    recv_mpst_r_to_s,
    RoleS,
    next_s,
    18, |
    recv_mpst_r_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleR,
    SessionMpstTwenty,
    20
);
// S
create_recv_mpst_session_bundle!(
    recv_mpst_s_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_s_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_s_to_c,
    RoleC,
    next_c,
    3, |
    recv_mpst_s_to_d,
    RoleD,
    next_d,
    4, |
    recv_mpst_s_to_e,
    RoleE,
    next_e,
    5, |
    recv_mpst_s_to_f,
    RoleF,
    next_f,
    6, |
    recv_mpst_s_to_g,
    RoleG,
    next_g,
    7, |
    recv_mpst_s_to_h,
    RoleH,
    next_h,
    8, |
    recv_mpst_s_to_i,
    RoleI,
    next_i,
    9, |
    recv_mpst_s_to_j,
    RoleJ,
    next_j,
    10, |
    recv_mpst_s_to_k,
    RoleK,
    next_k,
    11, |
    recv_mpst_s_to_l,
    RoleL,
    next_l,
    12, |
    recv_mpst_s_to_m,
    RoleM,
    next_m,
    13, |
    recv_mpst_s_to_n,
    RoleN,
    next_n,
    14, |
    recv_mpst_s_to_o,
    RoleO,
    next_o,
    15, |
    recv_mpst_s_to_p,
    RoleP,
    next_p,
    16, |
    recv_mpst_s_to_q,
    RoleQ,
    next_q,
    17, |
    recv_mpst_s_to_r,
    RoleR,
    next_r,
    18, |
    recv_mpst_s_to_t,
    RoleT,
    next_t,
    19, | =>
    RoleS,
    SessionMpstTwenty,
    20
);
// T
create_recv_mpst_session_bundle!(
    recv_mpst_t_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_t_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_t_to_c,
    RoleC,
    next_c,
    3, |
    recv_mpst_t_to_d,
    RoleD,
    next_d,
    4, |
    recv_mpst_t_to_e,
    RoleE,
    next_e,
    5, |
    recv_mpst_t_to_f,
    RoleF,
    next_f,
    6, |
    recv_mpst_t_to_g,
    RoleG,
    next_g,
    7, |
    recv_mpst_t_to_h,
    RoleH,
    next_h,
    8, |
    recv_mpst_t_to_i,
    RoleI,
    next_i,
    9, |
    recv_mpst_t_to_j,
    RoleJ,
    next_j,
    10, |
    recv_mpst_t_to_k,
    RoleK,
    next_k,
    11, |
    recv_mpst_t_to_l,
    RoleL,
    next_l,
    12, |
    recv_mpst_t_to_m,
    RoleM,
    next_m,
    13, |
    recv_mpst_t_to_n,
    RoleN,
    next_n,
    14, |
    recv_mpst_t_to_o,
    RoleO,
    next_o,
    15, |
    recv_mpst_t_to_p,
    RoleP,
    next_p,
    16, |
    recv_mpst_t_to_q,
    RoleQ,
    next_q,
    17, |
    recv_mpst_t_to_r,
    RoleR,
    next_r,
    18, |
    recv_mpst_t_to_s,
    RoleS,
    next_s,
    19, | =>
    RoleT,
    SessionMpstTwenty,
    20
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstTwenty, 20);

// Create fork function
bundle_fork_multi!(fork_mpst, fork_simple, SessionMpstTwenty, 20);

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
type NameL = RoleL<RoleEnd>;
type NameM = RoleM<RoleEnd>;
type NameN = RoleN<RoleEnd>;
type NameO = RoleO<RoleEnd>;
type NameP = RoleP<RoleEnd>;
type NameQ = RoleQ<RoleEnd>;
type NameR = RoleR<RoleEnd>;
type NameS = RoleS<RoleEnd>;
type NameT = RoleT<RoleEnd>;

// Types
// Send/Recv
type RS = Recv<(), Send<(), End>>;
type SR = Send<(), Recv<(), End>>;
// Roles
type R2A<R> = RoleA<RoleA<R>>;
type R2B<R> = RoleB<RoleB<R>>;
type R2C<R> = RoleC<RoleC<R>>;
type R2D<R> = RoleD<RoleD<R>>;
type R2E<R> = RoleE<RoleE<R>>;
type R2F<R> = RoleF<RoleF<R>>;
type R2G<R> = RoleG<RoleG<R>>;
type R2H<R> = RoleH<RoleH<R>>;
type R2I<R> = RoleI<RoleI<R>>;
type R2J<R> = RoleJ<RoleJ<R>>;
type R2K<R> = RoleK<RoleK<R>>;
type R2L<R> = RoleL<RoleL<R>>;
type R2M<R> = RoleM<RoleM<R>>;
type R2N<R> = RoleN<RoleN<R>>;
type R2O<R> = RoleO<RoleO<R>>;
type R2P<R> = RoleP<RoleP<R>>;
type R2Q<R> = RoleQ<RoleQ<R>>;
type R2R<R> = RoleR<RoleR<R>>;
type R2S<R> = RoleS<RoleS<R>>;
type R2T<R> = RoleT<RoleT<R>>;
// Binary
// A
enum BranchingTforA {
    More(
        SessionMpstTwenty<
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursAtoT>>,
            R2T<
                R2B<
                    R2C<
                        R2D<
                            R2E<
                                R2F<
                                    R2G<
                                        R2H<
                                            R2I<
                                                R2J<
                                                    R2K<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameA,
        >,
    ),
    Done(
        SessionMpstTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursAtoT = Recv<BranchingTforA, End>;
// B
enum BranchingTforB {
    More(
        SessionMpstTwenty<
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursBtoT>>,
            R2T<
                R2A<
                    R2C<
                        R2D<
                            R2E<
                                R2F<
                                    R2G<
                                        R2H<
                                            R2I<
                                                R2J<
                                                    R2K<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameB,
        >,
    ),
    Done(
        SessionMpstTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursBtoT = Recv<BranchingTforB, End>;
// C
enum BranchingTforC {
    More(
        SessionMpstTwenty<
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursCtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2D<
                            R2E<
                                R2F<
                                    R2G<
                                        R2H<
                                            R2I<
                                                R2J<
                                                    R2K<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameC,
        >,
    ),
    Done(
        SessionMpstTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursCtoT = Recv<BranchingTforC, End>;
// D
enum BranchingTforD {
    More(
        SessionMpstTwenty<
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursDtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2E<
                                R2F<
                                    R2G<
                                        R2H<
                                            R2I<
                                                R2J<
                                                    R2K<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameD,
        >,
    ),
    Done(
        SessionMpstTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursDtoT = Recv<BranchingTforD, End>;
// E
enum BranchingTforE {
    More(
        SessionMpstTwenty<
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursEtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2F<
                                    R2G<
                                        R2H<
                                            R2I<
                                                R2J<
                                                    R2K<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameE,
        >,
    ),
    Done(
        SessionMpstTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursEtoT = Recv<BranchingTforE, End>;
// F
enum BranchingTforF {
    More(
        SessionMpstTwenty<
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursFtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2G<
                                        R2H<
                                            R2I<
                                                R2J<
                                                    R2K<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameF,
        >,
    ),
    Done(
        SessionMpstTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursFtoT = Recv<BranchingTforF, End>;
// G
enum BranchingTforG {
    More(
        SessionMpstTwenty<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursGtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2H<
                                            R2I<
                                                R2J<
                                                    R2K<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameG,
        >,
    ),
    Done(
        SessionMpstTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursGtoT = Recv<BranchingTforG, End>;
// H
enum BranchingTforH {
    More(
        SessionMpstTwenty<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursHtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2I<
                                                R2J<
                                                    R2K<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameH,
        >,
    ),
    Done(
        SessionMpstTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursHtoT = Recv<BranchingTforH, End>;
// I
enum BranchingTforI {
    More(
        SessionMpstTwenty<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursItoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2J<
                                                    R2K<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameI,
        >,
    ),
    Done(
        SessionMpstTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursItoT = Recv<BranchingTforI, End>;
// J
enum BranchingTforJ {
    More(
        SessionMpstTwenty<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursJtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2I<
                                                    R2K<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameJ,
        >,
    ),
    Done(
        SessionMpstTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursJtoT = Recv<BranchingTforJ, End>;
// K
enum BranchingTforK {
    More(
        SessionMpstTwenty<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursKtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2I<
                                                    R2J<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameK,
        >,
    ),
    Done(
        SessionMpstTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursKtoT = Recv<BranchingTforK, End>;
// L
enum BranchingTforL {
    More(
        SessionMpstTwenty<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursLtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2I<
                                                    R2J<
                                                        R2K<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameL,
        >,
    ),
    Done(
        SessionMpstTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursLtoT = Recv<BranchingTforL, End>;
// M
enum BranchingTforM {
    More(
        SessionMpstTwenty<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursMtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2I<
                                                    R2J<
                                                        R2K<
                                                            R2L<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameM,
        >,
    ),
    Done(
        SessionMpstTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursMtoT = Recv<BranchingTforM, End>;
// N
enum BranchingTforN {
    More(
        SessionMpstTwenty<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursNtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2I<
                                                    R2J<
                                                        R2K<
                                                            R2L<
                                                                R2M<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameN,
        >,
    ),
    Done(
        SessionMpstTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursNtoT = Recv<BranchingTforN, End>;
// O
enum BranchingTforO {
    More(
        SessionMpstTwenty<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursOtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2I<
                                                    R2J<
                                                        R2K<
                                                            R2L<
                                                                R2M<
                                                                    R2N<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameO,
        >,
    ),
    Done(
        SessionMpstTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursOtoT = Recv<BranchingTforO, End>;
// P
enum BranchingTforP {
    More(
        SessionMpstTwenty<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursPtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2I<
                                                    R2J<
                                                        R2K<
                                                            R2L<
                                                                R2M<
                                                                    R2N<
                                                                        R2O<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameP,
        >,
    ),
    Done(
        SessionMpstTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursPtoT = Recv<BranchingTforP, End>;
// Q
enum BranchingTforQ {
    More(
        SessionMpstTwenty<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            Recv<(), Send<(), RecursQtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2I<
                                                    R2J<
                                                        R2K<
                                                            R2L<
                                                                R2M<
                                                                    R2N<
                                                                        R2O<
                                                                            R2P<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameQ,
        >,
    ),
    Done(
        SessionMpstTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursQtoT = Recv<BranchingTforQ, End>;
// R
enum BranchingTforR {
    More(
        SessionMpstTwenty<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            Recv<(), Send<(), RecursRtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2I<
                                                    R2J<
                                                        R2K<
                                                            R2L<
                                                                R2M<
                                                                    R2N<
                                                                        R2O<
                                                                            R2P<
                                                                                R2Q<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameR,
        >,
    ),
    Done(
        SessionMpstTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursRtoT = Recv<BranchingTforR, End>;
// S
enum BranchingTforS {
    More(
        SessionMpstTwenty<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            Recv<(), Send<(), RecursStoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2I<
                                                    R2J<
                                                        R2K<
                                                            R2L<
                                                                R2M<
                                                                    R2N<
                                                                        R2O<
                                                                            R2P<
                                                                                R2Q<
                                                                                    R2R<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameS,
        >,
    ),
    Done(
        SessionMpstTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursStoT = Recv<BranchingTforS, End>;
// T
type ChooseTforAtoT = Send<BranchingTforA, End>;
type ChooseTforBtoT = Send<BranchingTforB, End>;
type ChooseTforCtoT = Send<BranchingTforC, End>;
type ChooseTforDtoT = Send<BranchingTforD, End>;
type ChooseTforEtoT = Send<BranchingTforE, End>;
type ChooseTforFtoT = Send<BranchingTforF, End>;
type ChooseTforGtoT = Send<BranchingTforG, End>;
type ChooseTforHtoT = Send<BranchingTforH, End>;
type ChooseTforItoT = Send<BranchingTforI, End>;
type ChooseTforJtoT = Send<BranchingTforJ, End>;
type ChooseTforKtoT = Send<BranchingTforK, End>;
type ChooseTforLtoT = Send<BranchingTforL, End>;
type ChooseTforMtoT = Send<BranchingTforM, End>;
type ChooseTforNtoT = Send<BranchingTforN, End>;
type ChooseTforOtoT = Send<BranchingTforO, End>;
type ChooseTforPtoT = Send<BranchingTforP, End>;
type ChooseTforQtoT = Send<BranchingTforQ, End>;
type ChooseTforRtoT = Send<BranchingTforR, End>;
type ChooseTforStoT = Send<BranchingTforS, End>;

// Creating the MP sessions
type EndpointA = SessionMpstTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointB = SessionMpstTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointC = SessionMpstTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointD = SessionMpstTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointE = SessionMpstTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointF = SessionMpstTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointG = SessionMpstTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointH = SessionMpstTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointI = SessionMpstTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointJ = SessionMpstTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointK = SessionMpstTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointL = SessionMpstTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointM = SessionMpstTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointN = SessionMpstTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointO = SessionMpstTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointP = SessionMpstTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointQ = SessionMpstTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointR = SessionMpstTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointS = SessionMpstTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointT = SessionMpstTwenty<
    ChooseTforAtoT,
    ChooseTforBtoT,
    ChooseTforCtoT,
    ChooseTforDtoT,
    ChooseTforEtoT,
    ChooseTforFtoT,
    ChooseTforGtoT,
    ChooseTforHtoT,
    ChooseTforItoT,
    ChooseTforJtoT,
    ChooseTforKtoT,
    ChooseTforLtoT,
    ChooseTforMtoT,
    ChooseTforNtoT,
    ChooseTforOtoT,
    ChooseTforPtoT,
    ChooseTforQtoT,
    ChooseTforRtoT,
    ChooseTforStoT,
    RoleA<
        RoleB<
            RoleC<
                RoleD<
                    RoleE<
                        RoleF<
                            RoleG<
                                RoleH<
                                    RoleI<
                                        RoleJ<
                                            RoleK<
                                                RoleL<
                                                    RoleM<
                                                        RoleN<
                                                            RoleO<
                                                                RoleP<RoleQ<RoleR<RoleS<RoleEnd>>>>,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >,
    NameT,
>;

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_to_t, {
          BranchingTforA::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingTforA::More(s) => {
            let (_, s) = recv_mpst_a_to_t(s)?;
            let s = send_mpst_a_to_t((), s);
            let (_, s) = recv_mpst_a_to_b(s)?;
            let s = send_mpst_a_to_b((), s);
            let (_, s) = recv_mpst_a_to_c(s)?;
            let s = send_mpst_a_to_c((), s);
            let (_, s) = recv_mpst_a_to_d(s)?;
            let s = send_mpst_a_to_d((), s);
            let (_, s) = recv_mpst_a_to_e(s)?;
            let s = send_mpst_a_to_e((), s);
            let (_, s) = recv_mpst_a_to_f(s)?;
            let s = send_mpst_a_to_f((), s);
            let (_, s) = recv_mpst_a_to_g(s)?;
            let s = send_mpst_a_to_g((), s);
            let (_, s) = recv_mpst_a_to_h(s)?;
            let s = send_mpst_a_to_h((), s);
            let (_, s) = recv_mpst_a_to_i(s)?;
            let s = send_mpst_a_to_i((), s);
            let (_, s) = recv_mpst_a_to_j(s)?;
            let s = send_mpst_a_to_j((), s);
            let (_, s) = recv_mpst_a_to_k(s)?;
            let s = send_mpst_a_to_k((), s);
            let (_, s) = recv_mpst_a_to_l(s)?;
            let s = send_mpst_a_to_l((), s);
            let (_, s) = recv_mpst_a_to_m(s)?;
            let s = send_mpst_a_to_m((), s);
            let (_, s) = recv_mpst_a_to_n(s)?;
            let s = send_mpst_a_to_n((), s);
            let (_, s) = recv_mpst_a_to_o(s)?;
            let s = send_mpst_a_to_o((), s);
            let (_, s) = recv_mpst_a_to_p(s)?;
            let s = send_mpst_a_to_p((), s);
            let (_, s) = recv_mpst_a_to_q(s)?;
            let s = send_mpst_a_to_q((), s);
            let (_, s) = recv_mpst_a_to_r(s)?;
            let s = send_mpst_a_to_r((), s);
            let (_, s) = recv_mpst_a_to_s(s)?;
            let s = send_mpst_a_to_s((), s);
            simple_five_endpoint_a(s)
        },
    })
}

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_to_t, {
          BranchingTforB::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingTforB::More(s) => {
            let (_, s) = recv_mpst_b_to_t(s)?;
            let s = send_mpst_b_to_t((), s);
            let s = send_mpst_b_to_a((), s);
            let (_, s) = recv_mpst_b_to_a(s)?;
            let (_, s) = recv_mpst_b_to_c(s)?;
            let s = send_mpst_b_to_c((), s);
            let (_, s) = recv_mpst_b_to_d(s)?;
            let s = send_mpst_b_to_d((), s);
            let (_, s) = recv_mpst_b_to_e(s)?;
            let s = send_mpst_b_to_e((), s);
            let (_, s) = recv_mpst_b_to_f(s)?;
            let s = send_mpst_b_to_f((), s);
            let (_, s) = recv_mpst_b_to_g(s)?;
            let s = send_mpst_b_to_g((), s);
            let (_, s) = recv_mpst_b_to_h(s)?;
            let s = send_mpst_b_to_h((), s);
            let (_, s) = recv_mpst_b_to_i(s)?;
            let s = send_mpst_b_to_i((), s);
            let (_, s) = recv_mpst_b_to_j(s)?;
            let s = send_mpst_b_to_j((), s);
            let (_, s) = recv_mpst_b_to_k(s)?;
            let s = send_mpst_b_to_k((), s);
            let (_, s) = recv_mpst_b_to_l(s)?;
            let s = send_mpst_b_to_l((), s);
            let (_, s) = recv_mpst_b_to_m(s)?;
            let s = send_mpst_b_to_m((), s);
            let (_, s) = recv_mpst_b_to_n(s)?;
            let s = send_mpst_b_to_n((), s);
            let (_, s) = recv_mpst_b_to_o(s)?;
            let s = send_mpst_b_to_o((), s);
            let (_, s) = recv_mpst_b_to_p(s)?;
            let s = send_mpst_b_to_p((), s);
            let (_, s) = recv_mpst_b_to_q(s)?;
            let s = send_mpst_b_to_q((), s);
            let (_, s) = recv_mpst_b_to_r(s)?;
            let s = send_mpst_b_to_r((), s);
            let (_, s) = recv_mpst_b_to_s(s)?;
            let s = send_mpst_b_to_s((), s);
            simple_five_endpoint_b(s)
        },
    })
}

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_to_t, {
          BranchingTforC::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingTforC::More(s) => {
            let (_, s) = recv_mpst_c_to_t(s)?;
            let s = send_mpst_c_to_t((), s);
            let s = send_mpst_c_to_a((), s);
            let (_, s) = recv_mpst_c_to_a(s)?;
            let s = send_mpst_c_to_b((), s);
            let (_, s) = recv_mpst_c_to_b(s)?;
            let (_, s) = recv_mpst_c_to_d(s)?;
            let s = send_mpst_c_to_d((), s);
            let (_, s) = recv_mpst_c_to_e(s)?;
            let s = send_mpst_c_to_e((), s);
            let (_, s) = recv_mpst_c_to_f(s)?;
            let s = send_mpst_c_to_f((), s);
            let (_, s) = recv_mpst_c_to_g(s)?;
            let s = send_mpst_c_to_g((), s);
            let (_, s) = recv_mpst_c_to_h(s)?;
            let s = send_mpst_c_to_h((), s);
            let (_, s) = recv_mpst_c_to_i(s)?;
            let s = send_mpst_c_to_i((), s);
            let (_, s) = recv_mpst_c_to_j(s)?;
            let s = send_mpst_c_to_j((), s);
            let (_, s) = recv_mpst_c_to_k(s)?;
            let s = send_mpst_c_to_k((), s);
            let (_, s) = recv_mpst_c_to_l(s)?;
            let s = send_mpst_c_to_l((), s);
            let (_, s) = recv_mpst_c_to_m(s)?;
            let s = send_mpst_c_to_m((), s);
            let (_, s) = recv_mpst_c_to_n(s)?;
            let s = send_mpst_c_to_n((), s);
            let (_, s) = recv_mpst_c_to_o(s)?;
            let s = send_mpst_c_to_o((), s);
            let (_, s) = recv_mpst_c_to_p(s)?;
            let s = send_mpst_c_to_p((), s);
            let (_, s) = recv_mpst_c_to_q(s)?;
            let s = send_mpst_c_to_q((), s);
            let (_, s) = recv_mpst_c_to_r(s)?;
            let s = send_mpst_c_to_r((), s);
            let (_, s) = recv_mpst_c_to_s(s)?;
            let s = send_mpst_c_to_s((), s);
            simple_five_endpoint_c(s)
        },
    })
}

fn simple_five_endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_to_t, {
          BranchingTforD::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingTforD::More(s) => {
            let (_, s) = recv_mpst_d_to_t(s)?;
            let s = send_mpst_d_to_t((), s);
            let s = send_mpst_d_to_a((), s);
            let (_, s) = recv_mpst_d_to_a(s)?;
            let s = send_mpst_d_to_b((), s);
            let (_, s) = recv_mpst_d_to_b(s)?;
            let s = send_mpst_d_to_c((), s);
            let (_, s) = recv_mpst_d_to_c(s)?;
            let (_, s) = recv_mpst_d_to_e(s)?;
            let s = send_mpst_d_to_e((), s);
            let (_, s) = recv_mpst_d_to_f(s)?;
            let s = send_mpst_d_to_f((), s);
            let (_, s) = recv_mpst_d_to_g(s)?;
            let s = send_mpst_d_to_g((), s);
            let (_, s) = recv_mpst_d_to_h(s)?;
            let s = send_mpst_d_to_h((), s);
            let (_, s) = recv_mpst_d_to_i(s)?;
            let s = send_mpst_d_to_i((), s);
            let (_, s) = recv_mpst_d_to_j(s)?;
            let s = send_mpst_d_to_j((), s);
            let (_, s) = recv_mpst_d_to_k(s)?;
            let s = send_mpst_d_to_k((), s);
            let (_, s) = recv_mpst_d_to_l(s)?;
            let s = send_mpst_d_to_l((), s);
            let (_, s) = recv_mpst_d_to_m(s)?;
            let s = send_mpst_d_to_m((), s);
            let (_, s) = recv_mpst_d_to_n(s)?;
            let s = send_mpst_d_to_n((), s);
            let (_, s) = recv_mpst_d_to_o(s)?;
            let s = send_mpst_d_to_o((), s);
            let (_, s) = recv_mpst_d_to_p(s)?;
            let s = send_mpst_d_to_p((), s);
            let (_, s) = recv_mpst_d_to_q(s)?;
            let s = send_mpst_d_to_q((), s);
            let (_, s) = recv_mpst_d_to_r(s)?;
            let s = send_mpst_d_to_r((), s);
            let (_, s) = recv_mpst_d_to_s(s)?;
            let s = send_mpst_d_to_s((), s);
            simple_five_endpoint_d(s)
        },
    })
}

fn simple_five_endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_e_to_t, {
          BranchingTforE::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingTforE::More(s) => {
            let (_, s) = recv_mpst_e_to_t(s)?;
            let s = send_mpst_e_to_t((), s);
            let s = send_mpst_e_to_a((), s);
            let (_, s) = recv_mpst_e_to_a(s)?;
            let s = send_mpst_e_to_b((), s);
            let (_, s) = recv_mpst_e_to_b(s)?;
            let s = send_mpst_e_to_c((), s);
            let (_, s) = recv_mpst_e_to_c(s)?;
            let s = send_mpst_e_to_d((), s);
            let (_, s) = recv_mpst_e_to_d(s)?;
            let (_, s) = recv_mpst_e_to_f(s)?;
            let s = send_mpst_e_to_f((), s);
            let (_, s) = recv_mpst_e_to_g(s)?;
            let s = send_mpst_e_to_g((), s);
            let (_, s) = recv_mpst_e_to_h(s)?;
            let s = send_mpst_e_to_h((), s);
            let (_, s) = recv_mpst_e_to_i(s)?;
            let s = send_mpst_e_to_i((), s);
            let (_, s) = recv_mpst_e_to_j(s)?;
            let s = send_mpst_e_to_j((), s);
            let (_, s) = recv_mpst_e_to_k(s)?;
            let s = send_mpst_e_to_k((), s);
            let (_, s) = recv_mpst_e_to_l(s)?;
            let s = send_mpst_e_to_l((), s);
            let (_, s) = recv_mpst_e_to_m(s)?;
            let s = send_mpst_e_to_m((), s);
            let (_, s) = recv_mpst_e_to_n(s)?;
            let s = send_mpst_e_to_n((), s);
            let (_, s) = recv_mpst_e_to_o(s)?;
            let s = send_mpst_e_to_o((), s);
            let (_, s) = recv_mpst_e_to_p(s)?;
            let s = send_mpst_e_to_p((), s);
            let (_, s) = recv_mpst_e_to_q(s)?;
            let s = send_mpst_e_to_q((), s);
            let (_, s) = recv_mpst_e_to_r(s)?;
            let s = send_mpst_e_to_r((), s);
            let (_, s) = recv_mpst_e_to_s(s)?;
            let s = send_mpst_e_to_s((), s);
            simple_five_endpoint_e(s)
        },
    })
}

fn simple_five_endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_f_to_t, {
          BranchingTforF::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingTforF::More(s) => {
            let (_, s) = recv_mpst_f_to_t(s)?;
            let s = send_mpst_f_to_t((), s);
            let s = send_mpst_f_to_a((), s);
            let (_, s) = recv_mpst_f_to_a(s)?;
            let s = send_mpst_f_to_b((), s);
            let (_, s) = recv_mpst_f_to_b(s)?;
            let s = send_mpst_f_to_c((), s);
            let (_, s) = recv_mpst_f_to_c(s)?;
            let s = send_mpst_f_to_d((), s);
            let (_, s) = recv_mpst_f_to_d(s)?;
            let s = send_mpst_f_to_e((), s);
            let (_, s) = recv_mpst_f_to_e(s)?;
            let (_, s) = recv_mpst_f_to_g(s)?;
            let s = send_mpst_f_to_g((), s);
            let (_, s) = recv_mpst_f_to_h(s)?;
            let s = send_mpst_f_to_h((), s);
            let (_, s) = recv_mpst_f_to_i(s)?;
            let s = send_mpst_f_to_i((), s);
            let (_, s) = recv_mpst_f_to_j(s)?;
            let s = send_mpst_f_to_j((), s);
            let (_, s) = recv_mpst_f_to_k(s)?;
            let s = send_mpst_f_to_k((), s);
            let (_, s) = recv_mpst_f_to_l(s)?;
            let s = send_mpst_f_to_l((), s);
            let (_, s) = recv_mpst_f_to_m(s)?;
            let s = send_mpst_f_to_m((), s);
            let (_, s) = recv_mpst_f_to_n(s)?;
            let s = send_mpst_f_to_n((), s);
            let (_, s) = recv_mpst_f_to_o(s)?;
            let s = send_mpst_f_to_o((), s);
            let (_, s) = recv_mpst_f_to_p(s)?;
            let s = send_mpst_f_to_p((), s);
            let (_, s) = recv_mpst_f_to_q(s)?;
            let s = send_mpst_f_to_q((), s);
            let (_, s) = recv_mpst_f_to_r(s)?;
            let s = send_mpst_f_to_r((), s);
            let (_, s) = recv_mpst_f_to_s(s)?;
            let s = send_mpst_f_to_s((), s);
            simple_five_endpoint_f(s)
        },
    })
}

fn simple_five_endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_g_to_t, {
          BranchingTforG::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingTforG::More(s) => {
            let (_, s) = recv_mpst_g_to_t(s)?;
            let s = send_mpst_g_to_t((), s);
            let s = send_mpst_g_to_a((), s);
            let (_, s) = recv_mpst_g_to_a(s)?;
            let s = send_mpst_g_to_b((), s);
            let (_, s) = recv_mpst_g_to_b(s)?;
            let s = send_mpst_g_to_c((), s);
            let (_, s) = recv_mpst_g_to_c(s)?;
            let s = send_mpst_g_to_d((), s);
            let (_, s) = recv_mpst_g_to_d(s)?;
            let s = send_mpst_g_to_e((), s);
            let (_, s) = recv_mpst_g_to_e(s)?;
            let s = send_mpst_g_to_f((), s);
            let (_, s) = recv_mpst_g_to_f(s)?;
            let (_, s) = recv_mpst_g_to_h(s)?;
            let s = send_mpst_g_to_h((), s);
            let (_, s) = recv_mpst_g_to_i(s)?;
            let s = send_mpst_g_to_i((), s);
            let (_, s) = recv_mpst_g_to_j(s)?;
            let s = send_mpst_g_to_j((), s);
            let (_, s) = recv_mpst_g_to_k(s)?;
            let s = send_mpst_g_to_k((), s);
            let (_, s) = recv_mpst_g_to_l(s)?;
            let s = send_mpst_g_to_l((), s);
            let (_, s) = recv_mpst_g_to_m(s)?;
            let s = send_mpst_g_to_m((), s);
            let (_, s) = recv_mpst_g_to_n(s)?;
            let s = send_mpst_g_to_n((), s);
            let (_, s) = recv_mpst_g_to_o(s)?;
            let s = send_mpst_g_to_o((), s);
            let (_, s) = recv_mpst_g_to_p(s)?;
            let s = send_mpst_g_to_p((), s);
            let (_, s) = recv_mpst_g_to_q(s)?;
            let s = send_mpst_g_to_q((), s);
            let (_, s) = recv_mpst_g_to_r(s)?;
            let s = send_mpst_g_to_r((), s);
            let (_, s) = recv_mpst_g_to_s(s)?;
            let s = send_mpst_g_to_s((), s);
            simple_five_endpoint_g(s)
        },
    })
}

fn simple_five_endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_h_to_t, {
          BranchingTforH::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingTforH::More(s) => {
            let (_, s) = recv_mpst_h_to_t(s)?;
            let s = send_mpst_h_to_t((), s);
            let s = send_mpst_h_to_a((), s);
            let (_, s) = recv_mpst_h_to_a(s)?;
            let s = send_mpst_h_to_b((), s);
            let (_, s) = recv_mpst_h_to_b(s)?;
            let s = send_mpst_h_to_c((), s);
            let (_, s) = recv_mpst_h_to_c(s)?;
            let s = send_mpst_h_to_d((), s);
            let (_, s) = recv_mpst_h_to_d(s)?;
            let s = send_mpst_h_to_e((), s);
            let (_, s) = recv_mpst_h_to_e(s)?;
            let s = send_mpst_h_to_f((), s);
            let (_, s) = recv_mpst_h_to_f(s)?;
            let s = send_mpst_h_to_g((), s);
            let (_, s) = recv_mpst_h_to_g(s)?;
            let (_, s) = recv_mpst_h_to_i(s)?;
            let s = send_mpst_h_to_i((), s);
            let (_, s) = recv_mpst_h_to_j(s)?;
            let s = send_mpst_h_to_j((), s);
            let (_, s) = recv_mpst_h_to_k(s)?;
            let s = send_mpst_h_to_k((), s);
            let (_, s) = recv_mpst_h_to_l(s)?;
            let s = send_mpst_h_to_l((), s);
            let (_, s) = recv_mpst_h_to_m(s)?;
            let s = send_mpst_h_to_m((), s);
            let (_, s) = recv_mpst_h_to_n(s)?;
            let s = send_mpst_h_to_n((), s);
            let (_, s) = recv_mpst_h_to_o(s)?;
            let s = send_mpst_h_to_o((), s);
            let (_, s) = recv_mpst_h_to_p(s)?;
            let s = send_mpst_h_to_p((), s);
            let (_, s) = recv_mpst_h_to_q(s)?;
            let s = send_mpst_h_to_q((), s);
            let (_, s) = recv_mpst_h_to_r(s)?;
            let s = send_mpst_h_to_r((), s);
            let (_, s) = recv_mpst_h_to_s(s)?;
            let s = send_mpst_h_to_s((), s);
            simple_five_endpoint_h(s)
        },
    })
}

fn simple_five_endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_i_to_t, {
          BranchingTforI::Done(s) => {
            close_mpst_multi(s)
        },
        BranchingTforI::More(s) => {
            let (_, s) = recv_mpst_i_to_t(s)?;
            let s = send_mpst_i_to_t((), s);
            let s = send_mpst_i_to_a((), s);
            let (_, s) = recv_mpst_i_to_a(s)?;
            let s = send_mpst_i_to_b((), s);
            let (_, s) = recv_mpst_i_to_b(s)?;
            let s = send_mpst_i_to_c((), s);
            let (_, s) = recv_mpst_i_to_c(s)?;
            let s = send_mpst_i_to_d((), s);
            let (_, s) = recv_mpst_i_to_d(s)?;
            let s = send_mpst_i_to_e((), s);
            let (_, s) = recv_mpst_i_to_e(s)?;
            let s = send_mpst_i_to_f((), s);
            let (_, s) = recv_mpst_i_to_f(s)?;
            let s = send_mpst_i_to_g((), s);
            let (_, s) = recv_mpst_i_to_g(s)?;
            let s = send_mpst_i_to_h((), s);
            let (_, s) = recv_mpst_i_to_h(s)?;
            let (_, s) = recv_mpst_i_to_j(s)?;
            let s = send_mpst_i_to_j((), s);
            let (_, s) = recv_mpst_i_to_k(s)?;
            let s = send_mpst_i_to_k((), s);
            let (_, s) = recv_mpst_i_to_l(s)?;
            let s = send_mpst_i_to_l((), s);
            let (_, s) = recv_mpst_i_to_m(s)?;
            let s = send_mpst_i_to_m((), s);
            let (_, s) = recv_mpst_i_to_n(s)?;
            let s = send_mpst_i_to_n((), s);
            let (_, s) = recv_mpst_i_to_o(s)?;
            let s = send_mpst_i_to_o((), s);
            let (_, s) = recv_mpst_i_to_p(s)?;
            let s = send_mpst_i_to_p((), s);
            let (_, s) = recv_mpst_i_to_q(s)?;
            let s = send_mpst_i_to_q((), s);
            let (_, s) = recv_mpst_i_to_r(s)?;
            let s = send_mpst_i_to_r((), s);
            let (_, s) = recv_mpst_i_to_s(s)?;
            let s = send_mpst_i_to_s((), s);
            simple_five_endpoint_i(s)
        },
    })
}

fn simple_five_endpoint_j(s: EndpointJ) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_j_to_t, {
          BranchingTforJ::Done(s) => {
            close_mpst_multi(s)
        },
        BranchingTforJ::More(s) => {
            let (_, s) = recv_mpst_j_to_t(s)?;
            let s = send_mpst_j_to_t((), s);
            let s = send_mpst_j_to_a((), s);
            let (_, s) = recv_mpst_j_to_a(s)?;
            let s = send_mpst_j_to_b((), s);
            let (_, s) = recv_mpst_j_to_b(s)?;
            let s = send_mpst_j_to_c((), s);
            let (_, s) = recv_mpst_j_to_c(s)?;
            let s = send_mpst_j_to_d((), s);
            let (_, s) = recv_mpst_j_to_d(s)?;
            let s = send_mpst_j_to_e((), s);
            let (_, s) = recv_mpst_j_to_e(s)?;
            let s = send_mpst_j_to_f((), s);
            let (_, s) = recv_mpst_j_to_f(s)?;
            let s = send_mpst_j_to_g((), s);
            let (_, s) = recv_mpst_j_to_g(s)?;
            let s = send_mpst_j_to_h((), s);
            let (_, s) = recv_mpst_j_to_h(s)?;
            let s = send_mpst_j_to_i((), s);
            let (_, s) = recv_mpst_j_to_i(s)?;
            let (_, s) = recv_mpst_j_to_k(s)?;
            let s = send_mpst_j_to_k((), s);
            let (_, s) = recv_mpst_j_to_l(s)?;
            let s = send_mpst_j_to_l((), s);
            let (_, s) = recv_mpst_j_to_m(s)?;
            let s = send_mpst_j_to_m((), s);
            let (_, s) = recv_mpst_j_to_n(s)?;
            let s = send_mpst_j_to_n((), s);
            let (_, s) = recv_mpst_j_to_o(s)?;
            let s = send_mpst_j_to_o((), s);
            let (_, s) = recv_mpst_j_to_p(s)?;
            let s = send_mpst_j_to_p((), s);
            let (_, s) = recv_mpst_j_to_q(s)?;
            let s = send_mpst_j_to_q((), s);
            let (_, s) = recv_mpst_j_to_r(s)?;
            let s = send_mpst_j_to_r((), s);
            let (_, s) = recv_mpst_j_to_s(s)?;
            let s = send_mpst_j_to_s((), s);
            simple_five_endpoint_j(s)
        },
    })
}

fn simple_five_endpoint_k(s: EndpointK) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_k_to_t, {
          BranchingTforK::Done(s) => {
            close_mpst_multi(s)
        },
        BranchingTforK::More(s) => {
            let (_, s) = recv_mpst_k_to_t(s)?;
            let s = send_mpst_k_to_t((), s);
            let s = send_mpst_k_to_a((), s);
            let (_, s) = recv_mpst_k_to_a(s)?;
            let s = send_mpst_k_to_b((), s);
            let (_, s) = recv_mpst_k_to_b(s)?;
            let s = send_mpst_k_to_c((), s);
            let (_, s) = recv_mpst_k_to_c(s)?;
            let s = send_mpst_k_to_d((), s);
            let (_, s) = recv_mpst_k_to_d(s)?;
            let s = send_mpst_k_to_e((), s);
            let (_, s) = recv_mpst_k_to_e(s)?;
            let s = send_mpst_k_to_f((), s);
            let (_, s) = recv_mpst_k_to_f(s)?;
            let s = send_mpst_k_to_g((), s);
            let (_, s) = recv_mpst_k_to_g(s)?;
            let s = send_mpst_k_to_h((), s);
            let (_, s) = recv_mpst_k_to_h(s)?;
            let s = send_mpst_k_to_i((), s);
            let (_, s) = recv_mpst_k_to_i(s)?;
            let s = send_mpst_k_to_j((), s);
            let (_, s) = recv_mpst_k_to_j(s)?;
            let (_, s) = recv_mpst_k_to_l(s)?;
            let s = send_mpst_k_to_l((), s);
            let (_, s) = recv_mpst_k_to_m(s)?;
            let s = send_mpst_k_to_m((), s);
            let (_, s) = recv_mpst_k_to_n(s)?;
            let s = send_mpst_k_to_n((), s);
            let (_, s) = recv_mpst_k_to_o(s)?;
            let s = send_mpst_k_to_o((), s);
            let (_, s) = recv_mpst_k_to_p(s)?;
            let s = send_mpst_k_to_p((), s);
            let (_, s) = recv_mpst_k_to_q(s)?;
            let s = send_mpst_k_to_q((), s);
            let (_, s) = recv_mpst_k_to_r(s)?;
            let s = send_mpst_k_to_r((), s);
            let (_, s) = recv_mpst_k_to_s(s)?;
            let s = send_mpst_k_to_s((), s);
            simple_five_endpoint_k(s)
        },
    })
}

fn simple_five_endpoint_l(s: EndpointL) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_l_to_t, {
          BranchingTforL::Done(s) => {
            close_mpst_multi(s)
        },
        BranchingTforL::More(s) => {
            let (_, s) = recv_mpst_l_to_t(s)?;
            let s = send_mpst_l_to_t((), s);
            let s = send_mpst_l_to_a((), s);
            let (_, s) = recv_mpst_l_to_a(s)?;
            let s = send_mpst_l_to_b((), s);
            let (_, s) = recv_mpst_l_to_b(s)?;
            let s = send_mpst_l_to_c((), s);
            let (_, s) = recv_mpst_l_to_c(s)?;
            let s = send_mpst_l_to_d((), s);
            let (_, s) = recv_mpst_l_to_d(s)?;
            let s = send_mpst_l_to_e((), s);
            let (_, s) = recv_mpst_l_to_e(s)?;
            let s = send_mpst_l_to_f((), s);
            let (_, s) = recv_mpst_l_to_f(s)?;
            let s = send_mpst_l_to_g((), s);
            let (_, s) = recv_mpst_l_to_g(s)?;
            let s = send_mpst_l_to_h((), s);
            let (_, s) = recv_mpst_l_to_h(s)?;
            let s = send_mpst_l_to_i((), s);
            let (_, s) = recv_mpst_l_to_i(s)?;
            let s = send_mpst_l_to_j((), s);
            let (_, s) = recv_mpst_l_to_j(s)?;
            let s = send_mpst_l_to_k((), s);
            let (_, s) = recv_mpst_l_to_k(s)?;
            let (_, s) = recv_mpst_l_to_m(s)?;
            let s = send_mpst_l_to_m((), s);
            let (_, s) = recv_mpst_l_to_n(s)?;
            let s = send_mpst_l_to_n((), s);
            let (_, s) = recv_mpst_l_to_o(s)?;
            let s = send_mpst_l_to_o((), s);
            let (_, s) = recv_mpst_l_to_p(s)?;
            let s = send_mpst_l_to_p((), s);
            let (_, s) = recv_mpst_l_to_q(s)?;
            let s = send_mpst_l_to_q((), s);
            let (_, s) = recv_mpst_l_to_r(s)?;
            let s = send_mpst_l_to_r((), s);
            let (_, s) = recv_mpst_l_to_s(s)?;
            let s = send_mpst_l_to_s((), s);
            simple_five_endpoint_l(s)
        },
    })
}

fn simple_five_endpoint_m(s: EndpointM) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_m_to_t, {
          BranchingTforM::Done(s) => {
            close_mpst_multi(s)
        },
        BranchingTforM::More(s) => {
            let (_, s) = recv_mpst_m_to_t(s)?;
            let s = send_mpst_m_to_t((), s);
            let s = send_mpst_m_to_a((), s);
            let (_, s) = recv_mpst_m_to_a(s)?;
            let s = send_mpst_m_to_b((), s);
            let (_, s) = recv_mpst_m_to_b(s)?;
            let s = send_mpst_m_to_c((), s);
            let (_, s) = recv_mpst_m_to_c(s)?;
            let s = send_mpst_m_to_d((), s);
            let (_, s) = recv_mpst_m_to_d(s)?;
            let s = send_mpst_m_to_e((), s);
            let (_, s) = recv_mpst_m_to_e(s)?;
            let s = send_mpst_m_to_f((), s);
            let (_, s) = recv_mpst_m_to_f(s)?;
            let s = send_mpst_m_to_g((), s);
            let (_, s) = recv_mpst_m_to_g(s)?;
            let s = send_mpst_m_to_h((), s);
            let (_, s) = recv_mpst_m_to_h(s)?;
            let s = send_mpst_m_to_i((), s);
            let (_, s) = recv_mpst_m_to_i(s)?;
            let s = send_mpst_m_to_j((), s);
            let (_, s) = recv_mpst_m_to_j(s)?;
            let s = send_mpst_m_to_k((), s);
            let (_, s) = recv_mpst_m_to_k(s)?;
            let s = send_mpst_m_to_l((), s);
            let (_, s) = recv_mpst_m_to_l(s)?;
            let (_, s) = recv_mpst_m_to_n(s)?;
            let s = send_mpst_m_to_n((), s);
            let (_, s) = recv_mpst_m_to_o(s)?;
            let s = send_mpst_m_to_o((), s);
            let (_, s) = recv_mpst_m_to_p(s)?;
            let s = send_mpst_m_to_p((), s);
            let (_, s) = recv_mpst_m_to_q(s)?;
            let s = send_mpst_m_to_q((), s);
            let (_, s) = recv_mpst_m_to_r(s)?;
            let s = send_mpst_m_to_r((), s);
            let (_, s) = recv_mpst_m_to_s(s)?;
            let s = send_mpst_m_to_s((), s);
            simple_five_endpoint_m(s)
        },
    })
}

fn simple_five_endpoint_n(s: EndpointN) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_n_to_t, {
          BranchingTforN::Done(s) => {
            close_mpst_multi(s)
        },
        BranchingTforN::More(s) => {
            let (_, s) = recv_mpst_n_to_t(s)?;
            let s = send_mpst_n_to_t((), s);
            let s = send_mpst_n_to_a((), s);
            let (_, s) = recv_mpst_n_to_a(s)?;
            let s = send_mpst_n_to_b((), s);
            let (_, s) = recv_mpst_n_to_b(s)?;
            let s = send_mpst_n_to_c((), s);
            let (_, s) = recv_mpst_n_to_c(s)?;
            let s = send_mpst_n_to_d((), s);
            let (_, s) = recv_mpst_n_to_d(s)?;
            let s = send_mpst_n_to_e((), s);
            let (_, s) = recv_mpst_n_to_e(s)?;
            let s = send_mpst_n_to_f((), s);
            let (_, s) = recv_mpst_n_to_f(s)?;
            let s = send_mpst_n_to_g((), s);
            let (_, s) = recv_mpst_n_to_g(s)?;
            let s = send_mpst_n_to_h((), s);
            let (_, s) = recv_mpst_n_to_h(s)?;
            let s = send_mpst_n_to_i((), s);
            let (_, s) = recv_mpst_n_to_i(s)?;
            let s = send_mpst_n_to_j((), s);
            let (_, s) = recv_mpst_n_to_j(s)?;
            let s = send_mpst_n_to_k((), s);
            let (_, s) = recv_mpst_n_to_k(s)?;
            let s = send_mpst_n_to_l((), s);
            let (_, s) = recv_mpst_n_to_l(s)?;
            let s = send_mpst_n_to_m((), s);
            let (_, s) = recv_mpst_n_to_m(s)?;
            let (_, s) = recv_mpst_n_to_o(s)?;
            let s = send_mpst_n_to_o((), s);
            let (_, s) = recv_mpst_n_to_p(s)?;
            let s = send_mpst_n_to_p((), s);
            let (_, s) = recv_mpst_n_to_q(s)?;
            let s = send_mpst_n_to_q((), s);
            let (_, s) = recv_mpst_n_to_r(s)?;
            let s = send_mpst_n_to_r((), s);
            let (_, s) = recv_mpst_n_to_s(s)?;
            let s = send_mpst_n_to_s((), s);
            simple_five_endpoint_n(s)
        },
    })
}

fn simple_five_endpoint_o(s: EndpointO) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_o_to_t, {
          BranchingTforO::Done(s) => {
            close_mpst_multi(s)
        },
        BranchingTforO::More(s) => {
            let (_, s) = recv_mpst_o_to_t(s)?;
            let s = send_mpst_o_to_t((), s);
            let s = send_mpst_o_to_a((), s);
            let (_, s) = recv_mpst_o_to_a(s)?;
            let s = send_mpst_o_to_b((), s);
            let (_, s) = recv_mpst_o_to_b(s)?;
            let s = send_mpst_o_to_c((), s);
            let (_, s) = recv_mpst_o_to_c(s)?;
            let s = send_mpst_o_to_d((), s);
            let (_, s) = recv_mpst_o_to_d(s)?;
            let s = send_mpst_o_to_e((), s);
            let (_, s) = recv_mpst_o_to_e(s)?;
            let s = send_mpst_o_to_f((), s);
            let (_, s) = recv_mpst_o_to_f(s)?;
            let s = send_mpst_o_to_g((), s);
            let (_, s) = recv_mpst_o_to_g(s)?;
            let s = send_mpst_o_to_h((), s);
            let (_, s) = recv_mpst_o_to_h(s)?;
            let s = send_mpst_o_to_i((), s);
            let (_, s) = recv_mpst_o_to_i(s)?;
            let s = send_mpst_o_to_j((), s);
            let (_, s) = recv_mpst_o_to_j(s)?;
            let s = send_mpst_o_to_k((), s);
            let (_, s) = recv_mpst_o_to_k(s)?;
            let s = send_mpst_o_to_l((), s);
            let (_, s) = recv_mpst_o_to_l(s)?;
            let s = send_mpst_o_to_m((), s);
            let (_, s) = recv_mpst_o_to_m(s)?;
            let s = send_mpst_o_to_n((), s);
            let (_, s) = recv_mpst_o_to_n(s)?;
            let (_, s) = recv_mpst_o_to_p(s)?;
            let s = send_mpst_o_to_p((), s);
            let (_, s) = recv_mpst_o_to_q(s)?;
            let s = send_mpst_o_to_q((), s);
            let (_, s) = recv_mpst_o_to_r(s)?;
            let s = send_mpst_o_to_r((), s);
            let (_, s) = recv_mpst_o_to_s(s)?;
            let s = send_mpst_o_to_s((), s);
            simple_five_endpoint_o(s)
        },
    })
}

fn simple_five_endpoint_p(s: EndpointP) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_p_to_t, {
          BranchingTforP::Done(s) => {
            close_mpst_multi(s)
        },
        BranchingTforP::More(s) => {
            let (_, s) = recv_mpst_p_to_t(s)?;
            let s = send_mpst_p_to_t((), s);
            let s = send_mpst_p_to_a((), s);
            let (_, s) = recv_mpst_p_to_a(s)?;
            let s = send_mpst_p_to_b((), s);
            let (_, s) = recv_mpst_p_to_b(s)?;
            let s = send_mpst_p_to_c((), s);
            let (_, s) = recv_mpst_p_to_c(s)?;
            let s = send_mpst_p_to_d((), s);
            let (_, s) = recv_mpst_p_to_d(s)?;
            let s = send_mpst_p_to_e((), s);
            let (_, s) = recv_mpst_p_to_e(s)?;
            let s = send_mpst_p_to_f((), s);
            let (_, s) = recv_mpst_p_to_f(s)?;
            let s = send_mpst_p_to_g((), s);
            let (_, s) = recv_mpst_p_to_g(s)?;
            let s = send_mpst_p_to_h((), s);
            let (_, s) = recv_mpst_p_to_h(s)?;
            let s = send_mpst_p_to_i((), s);
            let (_, s) = recv_mpst_p_to_i(s)?;
            let s = send_mpst_p_to_j((), s);
            let (_, s) = recv_mpst_p_to_j(s)?;
            let s = send_mpst_p_to_k((), s);
            let (_, s) = recv_mpst_p_to_k(s)?;
            let s = send_mpst_p_to_l((), s);
            let (_, s) = recv_mpst_p_to_l(s)?;
            let s = send_mpst_p_to_m((), s);
            let (_, s) = recv_mpst_p_to_m(s)?;
            let s = send_mpst_p_to_n((), s);
            let (_, s) = recv_mpst_p_to_n(s)?;
            let s = send_mpst_p_to_o((), s);
            let (_, s) = recv_mpst_p_to_o(s)?;
            let (_, s) = recv_mpst_p_to_q(s)?;
            let s = send_mpst_p_to_q((), s);
            let (_, s) = recv_mpst_p_to_r(s)?;
            let s = send_mpst_p_to_r((), s);
            let (_, s) = recv_mpst_p_to_s(s)?;
            let s = send_mpst_p_to_s((), s);
            simple_five_endpoint_p(s)
        },
    })
}

fn simple_five_endpoint_q(s: EndpointQ) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_q_to_t, {
          BranchingTforQ::Done(s) => {
            close_mpst_multi(s)
        },
        BranchingTforQ::More(s) => {
            let (_, s) = recv_mpst_q_to_t(s)?;
            let s = send_mpst_q_to_t((), s);
            let s = send_mpst_q_to_a((), s);
            let (_, s) = recv_mpst_q_to_a(s)?;
            let s = send_mpst_q_to_b((), s);
            let (_, s) = recv_mpst_q_to_b(s)?;
            let s = send_mpst_q_to_c((), s);
            let (_, s) = recv_mpst_q_to_c(s)?;
            let s = send_mpst_q_to_d((), s);
            let (_, s) = recv_mpst_q_to_d(s)?;
            let s = send_mpst_q_to_e((), s);
            let (_, s) = recv_mpst_q_to_e(s)?;
            let s = send_mpst_q_to_f((), s);
            let (_, s) = recv_mpst_q_to_f(s)?;
            let s = send_mpst_q_to_g((), s);
            let (_, s) = recv_mpst_q_to_g(s)?;
            let s = send_mpst_q_to_h((), s);
            let (_, s) = recv_mpst_q_to_h(s)?;
            let s = send_mpst_q_to_i((), s);
            let (_, s) = recv_mpst_q_to_i(s)?;
            let s = send_mpst_q_to_j((), s);
            let (_, s) = recv_mpst_q_to_j(s)?;
            let s = send_mpst_q_to_k((), s);
            let (_, s) = recv_mpst_q_to_k(s)?;
            let s = send_mpst_q_to_l((), s);
            let (_, s) = recv_mpst_q_to_l(s)?;
            let s = send_mpst_q_to_m((), s);
            let (_, s) = recv_mpst_q_to_m(s)?;
            let s = send_mpst_q_to_n((), s);
            let (_, s) = recv_mpst_q_to_n(s)?;
            let s = send_mpst_q_to_o((), s);
            let (_, s) = recv_mpst_q_to_o(s)?;
            let s = send_mpst_q_to_p((), s);
            let (_, s) = recv_mpst_q_to_p(s)?;
            let (_, s) = recv_mpst_q_to_r(s)?;
            let s = send_mpst_q_to_r((), s);
            let (_, s) = recv_mpst_q_to_s(s)?;
            let s = send_mpst_q_to_s((), s);
            simple_five_endpoint_q(s)
        },
    })
}

fn simple_five_endpoint_r(s: EndpointR) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_r_to_t, {
          BranchingTforR::Done(s) => {
            close_mpst_multi(s)
        },
        BranchingTforR::More(s) => {
            let (_, s) = recv_mpst_r_to_t(s)?;
            let s = send_mpst_r_to_t((), s);
            let s = send_mpst_r_to_a((), s);
            let (_, s) = recv_mpst_r_to_a(s)?;
            let s = send_mpst_r_to_b((), s);
            let (_, s) = recv_mpst_r_to_b(s)?;
            let s = send_mpst_r_to_c((), s);
            let (_, s) = recv_mpst_r_to_c(s)?;
            let s = send_mpst_r_to_d((), s);
            let (_, s) = recv_mpst_r_to_d(s)?;
            let s = send_mpst_r_to_e((), s);
            let (_, s) = recv_mpst_r_to_e(s)?;
            let s = send_mpst_r_to_f((), s);
            let (_, s) = recv_mpst_r_to_f(s)?;
            let s = send_mpst_r_to_g((), s);
            let (_, s) = recv_mpst_r_to_g(s)?;
            let s = send_mpst_r_to_h((), s);
            let (_, s) = recv_mpst_r_to_h(s)?;
            let s = send_mpst_r_to_i((), s);
            let (_, s) = recv_mpst_r_to_i(s)?;
            let s = send_mpst_r_to_j((), s);
            let (_, s) = recv_mpst_r_to_j(s)?;
            let s = send_mpst_r_to_k((), s);
            let (_, s) = recv_mpst_r_to_k(s)?;
            let s = send_mpst_r_to_l((), s);
            let (_, s) = recv_mpst_r_to_l(s)?;
            let s = send_mpst_r_to_m((), s);
            let (_, s) = recv_mpst_r_to_m(s)?;
            let s = send_mpst_r_to_n((), s);
            let (_, s) = recv_mpst_r_to_n(s)?;
            let s = send_mpst_r_to_o((), s);
            let (_, s) = recv_mpst_r_to_o(s)?;
            let s = send_mpst_r_to_p((), s);
            let (_, s) = recv_mpst_r_to_p(s)?;
            let s = send_mpst_r_to_q((), s);
            let (_, s) = recv_mpst_r_to_q(s)?;
            let (_, s) = recv_mpst_r_to_s(s)?;
            let s = send_mpst_r_to_s((), s);
            simple_five_endpoint_r(s)
        },
    })
}

fn simple_five_endpoint_s(s: EndpointS) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_s_to_t, {
          BranchingTforS::Done(s) => {
            close_mpst_multi(s)
        },
        BranchingTforS::More(s) => {
            let (_, s) = recv_mpst_s_to_t(s)?;
            let s = send_mpst_s_to_t((), s);
            let s = send_mpst_s_to_a((), s);
            let (_, s) = recv_mpst_s_to_a(s)?;
            let s = send_mpst_s_to_b((), s);
            let (_, s) = recv_mpst_s_to_b(s)?;
            let s = send_mpst_s_to_c((), s);
            let (_, s) = recv_mpst_s_to_c(s)?;
            let s = send_mpst_s_to_d((), s);
            let (_, s) = recv_mpst_s_to_d(s)?;
            let s = send_mpst_s_to_e((), s);
            let (_, s) = recv_mpst_s_to_e(s)?;
            let s = send_mpst_s_to_f((), s);
            let (_, s) = recv_mpst_s_to_f(s)?;
            let s = send_mpst_s_to_g((), s);
            let (_, s) = recv_mpst_s_to_g(s)?;
            let s = send_mpst_s_to_h((), s);
            let (_, s) = recv_mpst_s_to_h(s)?;
            let s = send_mpst_s_to_i((), s);
            let (_, s) = recv_mpst_s_to_i(s)?;
            let s = send_mpst_s_to_j((), s);
            let (_, s) = recv_mpst_s_to_j(s)?;
            let s = send_mpst_s_to_k((), s);
            let (_, s) = recv_mpst_s_to_k(s)?;
            let s = send_mpst_s_to_l((), s);
            let (_, s) = recv_mpst_s_to_l(s)?;
            let s = send_mpst_s_to_m((), s);
            let (_, s) = recv_mpst_s_to_m(s)?;
            let s = send_mpst_s_to_n((), s);
            let (_, s) = recv_mpst_s_to_n(s)?;
            let s = send_mpst_s_to_o((), s);
            let (_, s) = recv_mpst_s_to_o(s)?;
            let s = send_mpst_s_to_p((), s);
            let (_, s) = recv_mpst_s_to_p(s)?;
            let s = send_mpst_s_to_q((), s);
            let (_, s) = recv_mpst_s_to_q(s)?;
            let s = send_mpst_s_to_r((), s);
            let (_, s) = recv_mpst_s_to_r(s)?;
            simple_five_endpoint_s(s)
        },
    })
}

fn simple_five_endpoint_t(s: EndpointT) -> Result<(), Box<dyn Error>> {
    recurs_t(s, SIZE)
}

fn recurs_t(s: EndpointT, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_t_to_a,
                send_mpst_t_to_b,
                send_mpst_t_to_c,
                send_mpst_t_to_d,
                send_mpst_t_to_e,
                send_mpst_t_to_f,
                send_mpst_t_to_g,
                send_mpst_t_to_h,
                send_mpst_t_to_i,
                send_mpst_t_to_j,
                send_mpst_t_to_k,
                send_mpst_t_to_l,
                send_mpst_t_to_m,
                send_mpst_t_to_n,
                send_mpst_t_to_o,
                send_mpst_t_to_p,
                send_mpst_t_to_q,
                send_mpst_t_to_r,
                send_mpst_t_to_s, =>
                BranchingTforA::Done,
                BranchingTforB::Done,
                BranchingTforC::Done,
                BranchingTforD::Done,
                BranchingTforE::Done,
                BranchingTforF::Done,
                BranchingTforG::Done,
                BranchingTforH::Done,
                BranchingTforI::Done,
                BranchingTforJ::Done,
                BranchingTforK::Done,
                BranchingTforL::Done,
                BranchingTforM::Done,
                BranchingTforN::Done,
                BranchingTforO::Done,
                BranchingTforP::Done,
                BranchingTforQ::Done,
                BranchingTforR::Done,
                BranchingTforS::Done, =>
                RoleA,
                RoleB,
                RoleC,
                RoleD,
                RoleE,
                RoleF,
                RoleG,
                RoleH,
                RoleI,
                RoleJ,
                RoleK,
                RoleL,
                RoleM,
                RoleN,
                RoleO,
                RoleP,
                RoleQ,
                RoleR,
                RoleS, =>
                RoleT,
                SessionMpstTwenty,
                20,
                20
            );

            close_mpst_multi(s)
        }
        i => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_t_to_a,
                send_mpst_t_to_b,
                send_mpst_t_to_c,
                send_mpst_t_to_d,
                send_mpst_t_to_e,
                send_mpst_t_to_f,
                send_mpst_t_to_g,
                send_mpst_t_to_h,
                send_mpst_t_to_i,
                send_mpst_t_to_j,
                send_mpst_t_to_k,
                send_mpst_t_to_l,
                send_mpst_t_to_m,
                send_mpst_t_to_n,
                send_mpst_t_to_o,
                send_mpst_t_to_p,
                send_mpst_t_to_q,
                send_mpst_t_to_r,
                send_mpst_t_to_s, =>
                BranchingTforA::More,
                BranchingTforB::More,
                BranchingTforC::More,
                BranchingTforD::More,
                BranchingTforE::More,
                BranchingTforF::More,
                BranchingTforG::More,
                BranchingTforH::More,
                BranchingTforI::More,
                BranchingTforJ::More,
                BranchingTforK::More,
                BranchingTforL::More,
                BranchingTforM::More,
                BranchingTforN::More,
                BranchingTforO::More,
                BranchingTforP::More,
                BranchingTforQ::More,
                BranchingTforR::More,
                BranchingTforS::More, =>
                RoleA,
                RoleB,
                RoleC,
                RoleD,
                RoleE,
                RoleF,
                RoleG,
                RoleH,
                RoleI,
                RoleJ,
                RoleK,
                RoleL,
                RoleM,
                RoleN,
                RoleO,
                RoleP,
                RoleQ,
                RoleR,
                RoleS, =>
                RoleT,
                SessionMpstTwenty,
                20,
                20
            );

            let s = send_mpst_t_to_a((), s);
            let (_, s) = recv_mpst_t_to_a(s)?;
            let s = send_mpst_t_to_b((), s);
            let (_, s) = recv_mpst_t_to_b(s)?;
            let s = send_mpst_t_to_c((), s);
            let (_, s) = recv_mpst_t_to_c(s)?;
            let s = send_mpst_t_to_d((), s);
            let (_, s) = recv_mpst_t_to_d(s)?;
            let s = send_mpst_t_to_e((), s);
            let (_, s) = recv_mpst_t_to_e(s)?;
            let s = send_mpst_t_to_f((), s);
            let (_, s) = recv_mpst_t_to_f(s)?;
            let s = send_mpst_t_to_g((), s);
            let (_, s) = recv_mpst_t_to_g(s)?;
            let s = send_mpst_t_to_h((), s);
            let (_, s) = recv_mpst_t_to_h(s)?;
            let s = send_mpst_t_to_i((), s);
            let (_, s) = recv_mpst_t_to_i(s)?;
            let s = send_mpst_t_to_j((), s);
            let (_, s) = recv_mpst_t_to_j(s)?;
            let s = send_mpst_t_to_k((), s);
            let (_, s) = recv_mpst_t_to_k(s)?;
            let s = send_mpst_t_to_l((), s);
            let (_, s) = recv_mpst_t_to_l(s)?;
            let s = send_mpst_t_to_m((), s);
            let (_, s) = recv_mpst_t_to_m(s)?;
            let s = send_mpst_t_to_n((), s);
            let (_, s) = recv_mpst_t_to_n(s)?;
            let s = send_mpst_t_to_o((), s);
            let (_, s) = recv_mpst_t_to_o(s)?;
            let s = send_mpst_t_to_p((), s);
            let (_, s) = recv_mpst_t_to_p(s)?;
            let s = send_mpst_t_to_q((), s);
            let (_, s) = recv_mpst_t_to_q(s)?;
            let s = send_mpst_t_to_r((), s);
            let (_, s) = recv_mpst_t_to_r(s)?;
            let s = send_mpst_t_to_s((), s);
            let (_, s) = recv_mpst_t_to_s(s)?;

            recurs_t(s, i - 1)
        }
    }
}

fn all_mpst() -> Result<(), Box<dyn Error>> {
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
        simple_five_endpoint_a,
        simple_five_endpoint_b,
        simple_five_endpoint_c,
        simple_five_endpoint_d,
        simple_five_endpoint_e,
        simple_five_endpoint_f,
        simple_five_endpoint_g,
        simple_five_endpoint_h,
        simple_five_endpoint_i,
        simple_five_endpoint_j,
        simple_five_endpoint_k,
        simple_five_endpoint_l,
        simple_five_endpoint_m,
        simple_five_endpoint_n,
        simple_five_endpoint_o,
        simple_five_endpoint_p,
        simple_five_endpoint_q,
        simple_five_endpoint_r,
        simple_five_endpoint_s,
        simple_five_endpoint_t,
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
    thread_l.join().unwrap();
    thread_m.join().unwrap();
    thread_n.join().unwrap();
    thread_o.join().unwrap();
    thread_p.join().unwrap();
    thread_q.join().unwrap();
    thread_r.join().unwrap();
    thread_s.join().unwrap();
    thread_t.join().unwrap();

    Ok(())
}

/////////////////////////

static SIZE: i64 = 15;

fn main() {
    all_mpst().unwrap();
}
