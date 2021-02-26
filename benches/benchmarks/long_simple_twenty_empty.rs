#![allow(dead_code)]

use crossbeam_channel::bounded;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::{close, fork_with_thread_id, recv, send, End, Recv, Send, Session};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    fork_mpst_multi, choose, choose_mpst_multi_to_all, close_mpst, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, create_sessionmpst, offer,
    offer_mpst,
};

use std::error::Error;
use std::thread::{spawn, JoinHandle};
use std::time::Duration;

// Create new SessionMpst for four participants
create_sessionmpst!(SessionMpstTwenty, 20);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, next_a, RoleADual, next_a_dual |
    RoleB, next_b, RoleBDual, next_b_dual |
    RoleC, next_c, RoleCDual, next_c_dual |
    RoleD, next_d, RoleDDual, next_d_dual |
    RoleE, next_e, RoleEDual, next_e_dual |
    RoleF, next_f, RoleFDual, next_f_dual |
    RoleG, next_g, RoleGDual, next_g_dual |
    RoleH, next_h, RoleHDual, next_h_dual |
    RoleI, next_i, RoleIDual, next_i_dual |
    RoleJ, next_j, RoleJDual, next_j_dual |
    RoleK, next_k, RoleKDual, next_k_dual |
    RoleL, next_l, RoleLDual, next_l_dual |
    RoleM, next_m, RoleMDual, next_m_dual |
    RoleN, next_n, RoleNDual, next_n_dual |
    RoleO, next_o, RoleODual, next_o_dual |
    RoleP, next_p, RolePDual, next_p_dual |
    RoleQ, next_q, RoleQDual, next_q_dual |
    RoleR, next_r, RoleRDual, next_r_dual |
    RoleS, next_s, RoleSDual, next_s_dual |
    RoleT, next_t, RoleTDual, next_t_dual |
);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, next_b, 1 |
    send_mpst_a_to_c, RoleC, next_c, 2 |
    send_mpst_a_to_d, RoleD, next_d, 3 |
    send_mpst_a_to_e, RoleE, next_e, 4 |
    send_mpst_a_to_f, RoleF, next_f, 5 |
    send_mpst_a_to_g, RoleG, next_g, 6 |
    send_mpst_a_to_h, RoleH, next_h, 7 |
    send_mpst_a_to_i, RoleI, next_i, 8 |
    send_mpst_a_to_j, RoleJ, next_j, 9 |
    send_mpst_a_to_k, RoleK, next_k, 10 |
    send_mpst_a_to_l, RoleL, next_l, 11 |
    send_mpst_a_to_m, RoleM, next_m, 12 |
    send_mpst_a_to_n, RoleN, next_n, 13 |
    send_mpst_a_to_o, RoleO, next_o, 14 |
    send_mpst_a_to_p, RoleP, next_p, 15 |
    send_mpst_a_to_q, RoleQ, next_q, 16 |
    send_mpst_a_to_r, RoleR, next_r, 17 |
    send_mpst_a_to_s, RoleS, next_s, 18 |
    send_mpst_a_to_t, RoleT, next_t, 19 | =>
    RoleA, SessionMpstTwenty, 20
);
// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a, RoleA, next_a, 1 |
    send_mpst_b_to_c, RoleC, next_c, 2 |
    send_mpst_b_to_d, RoleD, next_d, 3 |
    send_mpst_b_to_e, RoleE, next_e, 4 |
    send_mpst_b_to_f, RoleF, next_f, 5 |
    send_mpst_b_to_g, RoleG, next_g, 6 |
    send_mpst_b_to_h, RoleH, next_h, 7 |
    send_mpst_b_to_i, RoleI, next_i, 8 |
    send_mpst_b_to_j, RoleJ, next_j, 9 |
    send_mpst_b_to_k, RoleK, next_k, 10 |
    send_mpst_b_to_l, RoleL, next_l, 11 |
    send_mpst_b_to_m, RoleM, next_m, 12 |
    send_mpst_b_to_n, RoleN, next_n, 13 |
    send_mpst_b_to_o, RoleO, next_o, 14 |
    send_mpst_b_to_p, RoleP, next_p, 15 |
    send_mpst_b_to_q, RoleQ, next_q, 16 |
    send_mpst_b_to_r, RoleR, next_r, 17 |
    send_mpst_b_to_s, RoleS, next_s, 18 |
    send_mpst_b_to_t, RoleT, next_t, 19 | =>
    RoleB, SessionMpstTwenty, 20
);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a, RoleA, next_a, 1 |
    send_mpst_c_to_b, RoleB, next_b, 2 |
    send_mpst_c_to_d, RoleD, next_d, 3 |
    send_mpst_c_to_e, RoleE, next_e, 4 |
    send_mpst_c_to_f, RoleF, next_f, 5 |
    send_mpst_c_to_g, RoleG, next_g, 6 |
    send_mpst_c_to_h, RoleH, next_h, 7 |
    send_mpst_c_to_i, RoleI, next_i, 8 |
    send_mpst_c_to_j, RoleJ, next_j, 9 |
    send_mpst_c_to_k, RoleK, next_k, 10 |
    send_mpst_c_to_l, RoleL, next_l, 11 |
    send_mpst_c_to_m, RoleM, next_m, 12 |
    send_mpst_c_to_n, RoleN, next_n, 13 |
    send_mpst_c_to_o, RoleO, next_o, 14 |
    send_mpst_c_to_p, RoleP, next_p, 15 |
    send_mpst_c_to_q, RoleQ, next_q, 16 |
    send_mpst_c_to_r, RoleR, next_r, 17 |
    send_mpst_c_to_s, RoleS, next_s, 18 |
    send_mpst_c_to_t, RoleT, next_t, 19 | =>
    RoleC, SessionMpstTwenty, 20
);
// D
create_send_mpst_session_bundle!(
    send_mpst_d_to_a, RoleA, next_a, 1 |
    send_mpst_d_to_b, RoleB, next_b, 2 |
    send_mpst_d_to_c, RoleC, next_c, 3 |
    send_mpst_d_to_e, RoleE, next_e, 4 |
    send_mpst_d_to_f, RoleF, next_f, 5 |
    send_mpst_d_to_g, RoleG, next_g, 6 |
    send_mpst_d_to_h, RoleH, next_h, 7 |
    send_mpst_d_to_i, RoleI, next_i, 8 |
    send_mpst_d_to_j, RoleJ, next_j, 9 |
    send_mpst_d_to_k, RoleK, next_k, 10 |
    send_mpst_d_to_l, RoleL, next_l, 11 |
    send_mpst_d_to_m, RoleM, next_m, 12 |
    send_mpst_d_to_n, RoleN, next_n, 13 |
    send_mpst_d_to_o, RoleO, next_o, 14 |
    send_mpst_d_to_p, RoleP, next_p, 15 |
    send_mpst_d_to_q, RoleQ, next_q, 16 |
    send_mpst_d_to_r, RoleR, next_r, 17 |
    send_mpst_d_to_s, RoleS, next_s, 18 |
    send_mpst_d_to_t, RoleT, next_t, 19 | =>
    RoleD, SessionMpstTwenty, 20
);
// E
create_send_mpst_session_bundle!(
    send_mpst_e_to_a, RoleA, next_a, 1 |
    send_mpst_e_to_b, RoleB, next_b, 2 |
    send_mpst_e_to_c, RoleC, next_c, 3 |
    send_mpst_e_to_d, RoleD, next_d, 4 |
    send_mpst_e_to_f, RoleF, next_f, 5 |
    send_mpst_e_to_g, RoleG, next_g, 6 |
    send_mpst_e_to_h, RoleH, next_h, 7 |
    send_mpst_e_to_i, RoleI, next_i, 8 |
    send_mpst_e_to_j, RoleJ, next_j, 9 |
    send_mpst_e_to_k, RoleK, next_k, 10 |
    send_mpst_e_to_l, RoleL, next_l, 11 |
    send_mpst_e_to_m, RoleM, next_m, 12 |
    send_mpst_e_to_n, RoleN, next_n, 13 |
    send_mpst_e_to_o, RoleO, next_o, 14 |
    send_mpst_e_to_p, RoleP, next_p, 15 |
    send_mpst_e_to_q, RoleQ, next_q, 16 |
    send_mpst_e_to_r, RoleR, next_r, 17 |
    send_mpst_e_to_s, RoleS, next_s, 18 |
    send_mpst_e_to_t, RoleT, next_t, 19 | =>
    RoleE, SessionMpstTwenty, 20
);
// F
create_send_mpst_session_bundle!(
    send_mpst_f_to_a, RoleA, next_a, 1 |
    send_mpst_f_to_b, RoleB, next_b, 2 |
    send_mpst_f_to_c, RoleC, next_c, 3 |
    send_mpst_f_to_d, RoleD, next_d, 4 |
    send_mpst_f_to_e, RoleE, next_e, 5 |
    send_mpst_f_to_g, RoleG, next_g, 6 |
    send_mpst_f_to_h, RoleH, next_h, 7 |
    send_mpst_f_to_i, RoleI, next_i, 8 |
    send_mpst_f_to_j, RoleJ, next_j, 9 |
    send_mpst_f_to_k, RoleK, next_k, 10 |
    send_mpst_f_to_l, RoleL, next_l, 11 |
    send_mpst_f_to_m, RoleM, next_m, 12 |
    send_mpst_f_to_n, RoleN, next_n, 13 |
    send_mpst_f_to_o, RoleO, next_o, 14 |
    send_mpst_f_to_p, RoleP, next_p, 15 |
    send_mpst_f_to_q, RoleQ, next_q, 16 |
    send_mpst_f_to_r, RoleR, next_r, 17 |
    send_mpst_f_to_s, RoleS, next_s, 18 |
    send_mpst_f_to_t, RoleT, next_t, 19 | =>
    RoleF, SessionMpstTwenty, 20
);
// G
create_send_mpst_session_bundle!(
    send_mpst_g_to_a, RoleA, next_a, 1 |
    send_mpst_g_to_b, RoleB, next_b, 2 |
    send_mpst_g_to_c, RoleC, next_c, 3 |
    send_mpst_g_to_d, RoleD, next_d, 4 |
    send_mpst_g_to_e, RoleE, next_e, 5 |
    send_mpst_g_to_f, RoleF, next_f, 6 |
    send_mpst_g_to_h, RoleH, next_h, 7 |
    send_mpst_g_to_i, RoleI, next_i, 8 |
    send_mpst_g_to_j, RoleJ, next_j, 9 |
    send_mpst_g_to_k, RoleK, next_k, 10 |
    send_mpst_g_to_l, RoleL, next_l, 11 |
    send_mpst_g_to_m, RoleM, next_m, 12 |
    send_mpst_g_to_n, RoleN, next_n, 13 |
    send_mpst_g_to_o, RoleO, next_o, 14 |
    send_mpst_g_to_p, RoleP, next_p, 15 |
    send_mpst_g_to_q, RoleQ, next_q, 16 |
    send_mpst_g_to_r, RoleR, next_r, 17 |
    send_mpst_g_to_s, RoleS, next_s, 18 |
    send_mpst_g_to_t, RoleT, next_t, 19 | =>
    RoleG, SessionMpstTwenty, 20
);
// H
create_send_mpst_session_bundle!(
    send_mpst_h_to_a, RoleA, next_a, 1 |
    send_mpst_h_to_b, RoleB, next_b, 2 |
    send_mpst_h_to_c, RoleC, next_c, 3 |
    send_mpst_h_to_d, RoleD, next_d, 4 |
    send_mpst_h_to_e, RoleE, next_e, 5 |
    send_mpst_h_to_f, RoleF, next_f, 6 |
    send_mpst_h_to_g, RoleG, next_g, 7 |
    send_mpst_h_to_i, RoleI, next_i, 8 |
    send_mpst_h_to_j, RoleJ, next_j, 9 |
    send_mpst_h_to_k, RoleK, next_k, 10 |
    send_mpst_h_to_l, RoleL, next_l, 11 |
    send_mpst_h_to_m, RoleM, next_m, 12 |
    send_mpst_h_to_n, RoleN, next_n, 13 |
    send_mpst_h_to_o, RoleO, next_o, 14 |
    send_mpst_h_to_p, RoleP, next_p, 15 |
    send_mpst_h_to_q, RoleQ, next_q, 16 |
    send_mpst_h_to_r, RoleR, next_r, 17 |
    send_mpst_h_to_s, RoleS, next_s, 18 |
    send_mpst_h_to_t, RoleT, next_t, 19 | =>
    RoleH, SessionMpstTwenty, 20
);
// I
create_send_mpst_session_bundle!(
    send_mpst_i_to_a, RoleA, next_a, 1 |
    send_mpst_i_to_b, RoleB, next_b, 2 |
    send_mpst_i_to_c, RoleC, next_c, 3 |
    send_mpst_i_to_d, RoleD, next_d, 4 |
    send_mpst_i_to_e, RoleE, next_e, 5 |
    send_mpst_i_to_f, RoleF, next_f, 6 |
    send_mpst_i_to_g, RoleG, next_g, 7 |
    send_mpst_i_to_h, RoleH, next_h, 8 |
    send_mpst_i_to_j, RoleJ, next_j, 9 |
    send_mpst_i_to_k, RoleK, next_k, 10 |
    send_mpst_i_to_l, RoleL, next_l, 11 |
    send_mpst_i_to_m, RoleM, next_m, 12 |
    send_mpst_i_to_n, RoleN, next_n, 13 |
    send_mpst_i_to_o, RoleO, next_o, 14 |
    send_mpst_i_to_p, RoleP, next_p, 15 |
    send_mpst_i_to_q, RoleQ, next_q, 16 |
    send_mpst_i_to_r, RoleR, next_r, 17 |
    send_mpst_i_to_s, RoleS, next_s, 18 |
    send_mpst_i_to_t, RoleT, next_t, 19 | =>
    RoleI, SessionMpstTwenty, 20
);
// J
create_send_mpst_session_bundle!(
    send_mpst_j_to_a, RoleA, next_a, 1 |
    send_mpst_j_to_b, RoleB, next_b, 2 |
    send_mpst_j_to_c, RoleC, next_c, 3 |
    send_mpst_j_to_d, RoleD, next_d, 4 |
    send_mpst_j_to_e, RoleE, next_e, 5 |
    send_mpst_j_to_f, RoleF, next_f, 6 |
    send_mpst_j_to_g, RoleG, next_g, 7 |
    send_mpst_j_to_h, RoleH, next_h, 8 |
    send_mpst_j_to_i, RoleI, next_i, 9 |
    send_mpst_j_to_k, RoleK, next_k, 10 |
    send_mpst_j_to_l, RoleL, next_l, 11 |
    send_mpst_j_to_m, RoleM, next_m, 12 |
    send_mpst_j_to_n, RoleN, next_n, 13 |
    send_mpst_j_to_o, RoleO, next_o, 14 |
    send_mpst_j_to_p, RoleP, next_p, 15 |
    send_mpst_j_to_q, RoleQ, next_q, 16 |
    send_mpst_j_to_r, RoleR, next_r, 17 |
    send_mpst_j_to_s, RoleS, next_s, 18 |
    send_mpst_j_to_t, RoleT, next_t, 19 | =>
    RoleJ, SessionMpstTwenty, 20
);
// K
create_send_mpst_session_bundle!(
    send_mpst_k_to_a, RoleA, next_a, 1 |
    send_mpst_k_to_b, RoleB, next_b, 2 |
    send_mpst_k_to_c, RoleC, next_c, 3 |
    send_mpst_k_to_d, RoleD, next_d, 4 |
    send_mpst_k_to_e, RoleE, next_e, 5 |
    send_mpst_k_to_f, RoleF, next_f, 6 |
    send_mpst_k_to_g, RoleG, next_g, 7 |
    send_mpst_k_to_h, RoleH, next_h, 8 |
    send_mpst_k_to_i, RoleI, next_i, 9 |
    send_mpst_k_to_j, RoleJ, next_j, 10 |
    send_mpst_k_to_l, RoleL, next_l, 11 |
    send_mpst_k_to_m, RoleM, next_m, 12 |
    send_mpst_k_to_n, RoleN, next_n, 13 |
    send_mpst_k_to_o, RoleO, next_o, 14 |
    send_mpst_k_to_p, RoleP, next_p, 15 |
    send_mpst_k_to_q, RoleQ, next_q, 16 |
    send_mpst_k_to_r, RoleR, next_r, 17 |
    send_mpst_k_to_s, RoleS, next_s, 18 |
    send_mpst_k_to_t, RoleT, next_t, 19 | =>
    RoleK, SessionMpstTwenty, 20
);
// L
create_send_mpst_session_bundle!(
    send_mpst_l_to_a, RoleA, next_a, 1 |
    send_mpst_l_to_b, RoleB, next_b, 2 |
    send_mpst_l_to_c, RoleC, next_c, 3 |
    send_mpst_l_to_d, RoleD, next_d, 4 |
    send_mpst_l_to_e, RoleE, next_e, 5 |
    send_mpst_l_to_f, RoleF, next_f, 6 |
    send_mpst_l_to_g, RoleG, next_g, 7 |
    send_mpst_l_to_h, RoleH, next_h, 8 |
    send_mpst_l_to_i, RoleI, next_i, 9 |
    send_mpst_l_to_j, RoleJ, next_j, 10 |
    send_mpst_l_to_k, RoleK, next_k, 11 |
    send_mpst_l_to_m, RoleM, next_m, 12 |
    send_mpst_l_to_n, RoleN, next_n, 13 |
    send_mpst_l_to_o, RoleO, next_o, 14 |
    send_mpst_l_to_p, RoleP, next_p, 15 |
    send_mpst_l_to_q, RoleQ, next_q, 16 |
    send_mpst_l_to_r, RoleR, next_r, 17 |
    send_mpst_l_to_s, RoleS, next_s, 18 |
    send_mpst_l_to_t, RoleT, next_t, 19 | =>
    RoleL, SessionMpstTwenty, 20
);
// M
create_send_mpst_session_bundle!(
    send_mpst_m_to_a, RoleA, next_a, 1 |
    send_mpst_m_to_b, RoleB, next_b, 2 |
    send_mpst_m_to_c, RoleC, next_c, 3 |
    send_mpst_m_to_d, RoleD, next_d, 4 |
    send_mpst_m_to_e, RoleE, next_e, 5 |
    send_mpst_m_to_f, RoleF, next_f, 6 |
    send_mpst_m_to_g, RoleG, next_g, 7 |
    send_mpst_m_to_h, RoleH, next_h, 8 |
    send_mpst_m_to_i, RoleI, next_i, 9 |
    send_mpst_m_to_j, RoleJ, next_j, 10 |
    send_mpst_m_to_k, RoleK, next_k, 11 |
    send_mpst_m_to_l, RoleL, next_l, 12 |
    send_mpst_m_to_n, RoleN, next_n, 13 |
    send_mpst_m_to_o, RoleO, next_o, 14 |
    send_mpst_m_to_p, RoleP, next_p, 15 |
    send_mpst_m_to_q, RoleQ, next_q, 16 |
    send_mpst_m_to_r, RoleR, next_r, 17 |
    send_mpst_m_to_s, RoleS, next_s, 18 |
    send_mpst_m_to_t, RoleT, next_t, 19 | =>
    RoleM, SessionMpstTwenty, 20
);
// N
create_send_mpst_session_bundle!(
    send_mpst_n_to_a, RoleA, next_a, 1 |
    send_mpst_n_to_b, RoleB, next_b, 2 |
    send_mpst_n_to_c, RoleC, next_c, 3 |
    send_mpst_n_to_d, RoleD, next_d, 4 |
    send_mpst_n_to_e, RoleE, next_e, 5 |
    send_mpst_n_to_f, RoleF, next_f, 6 |
    send_mpst_n_to_g, RoleG, next_g, 7 |
    send_mpst_n_to_h, RoleH, next_h, 8 |
    send_mpst_n_to_i, RoleI, next_i, 9 |
    send_mpst_n_to_j, RoleJ, next_j, 10 |
    send_mpst_n_to_k, RoleK, next_k, 11 |
    send_mpst_n_to_l, RoleL, next_l, 12 |
    send_mpst_n_to_m, RoleM, next_m, 13 |
    send_mpst_n_to_o, RoleO, next_o, 14 |
    send_mpst_n_to_p, RoleP, next_p, 15 |
    send_mpst_n_to_q, RoleQ, next_q, 16 |
    send_mpst_n_to_r, RoleR, next_r, 17 |
    send_mpst_n_to_s, RoleS, next_s, 18 |
    send_mpst_n_to_t, RoleT, next_t, 19 | =>
    RoleN, SessionMpstTwenty, 20
);
// O
create_send_mpst_session_bundle!(
    send_mpst_o_to_a, RoleA, next_a, 1 |
    send_mpst_o_to_b, RoleB, next_b, 2 |
    send_mpst_o_to_c, RoleC, next_c, 3 |
    send_mpst_o_to_d, RoleD, next_d, 4 |
    send_mpst_o_to_e, RoleE, next_e, 5 |
    send_mpst_o_to_f, RoleF, next_f, 6 |
    send_mpst_o_to_g, RoleG, next_g, 7 |
    send_mpst_o_to_h, RoleH, next_h, 8 |
    send_mpst_o_to_i, RoleI, next_i, 9 |
    send_mpst_o_to_j, RoleJ, next_j, 10 |
    send_mpst_o_to_k, RoleK, next_k, 11 |
    send_mpst_o_to_l, RoleL, next_l, 12 |
    send_mpst_o_to_m, RoleM, next_m, 13 |
    send_mpst_o_to_n, RoleN, next_n, 14 |
    send_mpst_o_to_p, RoleP, next_p, 15 |
    send_mpst_o_to_q, RoleQ, next_q, 16 |
    send_mpst_o_to_r, RoleR, next_r, 17 |
    send_mpst_o_to_s, RoleS, next_s, 18 |
    send_mpst_o_to_t, RoleT, next_t, 19 | =>
    RoleO, SessionMpstTwenty, 20
);
// P
create_send_mpst_session_bundle!(
    send_mpst_p_to_a, RoleA, next_a, 1 |
    send_mpst_p_to_b, RoleB, next_b, 2 |
    send_mpst_p_to_c, RoleC, next_c, 3 |
    send_mpst_p_to_d, RoleD, next_d, 4 |
    send_mpst_p_to_e, RoleE, next_e, 5 |
    send_mpst_p_to_f, RoleF, next_f, 6 |
    send_mpst_p_to_g, RoleG, next_g, 7 |
    send_mpst_p_to_h, RoleH, next_h, 8 |
    send_mpst_p_to_i, RoleI, next_i, 9 |
    send_mpst_p_to_j, RoleJ, next_j, 10 |
    send_mpst_p_to_k, RoleK, next_k, 11 |
    send_mpst_p_to_l, RoleL, next_l, 12 |
    send_mpst_p_to_m, RoleM, next_m, 13 |
    send_mpst_p_to_n, RoleN, next_n, 14 |
    send_mpst_p_to_o, RoleO, next_o, 15 |
    send_mpst_p_to_q, RoleQ, next_q, 16 |
    send_mpst_p_to_r, RoleR, next_r, 17 |
    send_mpst_p_to_s, RoleS, next_s, 18 |
    send_mpst_p_to_t, RoleT, next_t, 19 | =>
    RoleP, SessionMpstTwenty, 20
);
// Q
create_send_mpst_session_bundle!(
    send_mpst_q_to_a, RoleA, next_a, 1 |
    send_mpst_q_to_b, RoleB, next_b, 2 |
    send_mpst_q_to_c, RoleC, next_c, 3 |
    send_mpst_q_to_d, RoleD, next_d, 4 |
    send_mpst_q_to_e, RoleE, next_e, 5 |
    send_mpst_q_to_f, RoleF, next_f, 6 |
    send_mpst_q_to_g, RoleG, next_g, 7 |
    send_mpst_q_to_h, RoleH, next_h, 8 |
    send_mpst_q_to_i, RoleI, next_i, 9 |
    send_mpst_q_to_j, RoleJ, next_j, 10 |
    send_mpst_q_to_k, RoleK, next_k, 11 |
    send_mpst_q_to_l, RoleL, next_l, 12 |
    send_mpst_q_to_m, RoleM, next_m, 13 |
    send_mpst_q_to_n, RoleN, next_n, 14 |
    send_mpst_q_to_o, RoleO, next_o, 15 |
    send_mpst_q_to_p, RoleP, next_p, 16 |
    send_mpst_q_to_r, RoleR, next_r, 17 |
    send_mpst_q_to_s, RoleS, next_s, 18 |
    send_mpst_q_to_t, RoleT, next_t, 19 | =>
    RoleQ, SessionMpstTwenty, 20
);
// R
create_send_mpst_session_bundle!(
    send_mpst_r_to_a, RoleA, next_a, 1 |
    send_mpst_r_to_b, RoleB, next_b, 2 |
    send_mpst_r_to_c, RoleC, next_c, 3 |
    send_mpst_r_to_d, RoleD, next_d, 4 |
    send_mpst_r_to_e, RoleE, next_e, 5 |
    send_mpst_r_to_f, RoleF, next_f, 6 |
    send_mpst_r_to_g, RoleG, next_g, 7 |
    send_mpst_r_to_h, RoleH, next_h, 8 |
    send_mpst_r_to_i, RoleI, next_i, 9 |
    send_mpst_r_to_j, RoleJ, next_j, 10 |
    send_mpst_r_to_k, RoleK, next_k, 11 |
    send_mpst_r_to_l, RoleL, next_l, 12 |
    send_mpst_r_to_m, RoleM, next_m, 13 |
    send_mpst_r_to_n, RoleN, next_n, 14 |
    send_mpst_r_to_o, RoleO, next_o, 15 |
    send_mpst_r_to_p, RoleP, next_p, 16 |
    send_mpst_r_to_q, RoleQ, next_q, 17 |
    send_mpst_r_to_s, RoleS, next_s, 18 |
    send_mpst_r_to_t, RoleT, next_t, 19 | =>
    RoleR, SessionMpstTwenty, 20
);
// S
create_send_mpst_session_bundle!(
    send_mpst_s_to_a, RoleA, next_a, 1 |
    send_mpst_s_to_b, RoleB, next_b, 2 |
    send_mpst_s_to_c, RoleC, next_c, 3 |
    send_mpst_s_to_d, RoleD, next_d, 4 |
    send_mpst_s_to_e, RoleE, next_e, 5 |
    send_mpst_s_to_f, RoleF, next_f, 6 |
    send_mpst_s_to_g, RoleG, next_g, 7 |
    send_mpst_s_to_h, RoleH, next_h, 8 |
    send_mpst_s_to_i, RoleI, next_i, 9 |
    send_mpst_s_to_j, RoleJ, next_j, 10 |
    send_mpst_s_to_k, RoleK, next_k, 11 |
    send_mpst_s_to_l, RoleL, next_l, 12 |
    send_mpst_s_to_m, RoleM, next_m, 13 |
    send_mpst_s_to_n, RoleN, next_n, 14 |
    send_mpst_s_to_o, RoleO, next_o, 15 |
    send_mpst_s_to_p, RoleP, next_p, 16 |
    send_mpst_s_to_q, RoleQ, next_q, 17 |
    send_mpst_s_to_r, RoleR, next_r, 18 |
    send_mpst_s_to_t, RoleT, next_t, 19 | =>
    RoleS, SessionMpstTwenty, 20
);
// T
create_send_mpst_session_bundle!(
    send_mpst_t_to_a, RoleA, next_a, 1 |
    send_mpst_t_to_b, RoleB, next_b, 2 |
    send_mpst_t_to_c, RoleC, next_c, 3 |
    send_mpst_t_to_d, RoleD, next_d, 4 |
    send_mpst_t_to_e, RoleE, next_e, 5 |
    send_mpst_t_to_f, RoleF, next_f, 6 |
    send_mpst_t_to_g, RoleG, next_g, 7 |
    send_mpst_t_to_h, RoleH, next_h, 8 |
    send_mpst_t_to_i, RoleI, next_i, 9 |
    send_mpst_t_to_j, RoleJ, next_j, 10 |
    send_mpst_t_to_k, RoleK, next_k, 11 |
    send_mpst_t_to_l, RoleL, next_l, 12 |
    send_mpst_t_to_m, RoleM, next_m, 13 |
    send_mpst_t_to_n, RoleN, next_n, 14 |
    send_mpst_t_to_o, RoleO, next_o, 15 |
    send_mpst_t_to_p, RoleP, next_p, 16 |
    send_mpst_t_to_q, RoleQ, next_q, 17 |
    send_mpst_t_to_r, RoleR, next_r, 18 |
    send_mpst_t_to_s, RoleS, next_s, 19 | =>
    RoleT, SessionMpstTwenty, 20
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_to_b, RoleB, next_b, 1 |
    recv_mpst_a_to_c, RoleC, next_c, 2 |
    recv_mpst_a_to_d, RoleD, next_d, 3 |
    recv_mpst_a_to_e, RoleE, next_e, 4 |
    recv_mpst_a_to_f, RoleF, next_f, 5 |
    recv_mpst_a_to_g, RoleG, next_g, 6 |
    recv_mpst_a_to_h, RoleH, next_h, 7 |
    recv_mpst_a_to_i, RoleI, next_i, 8 |
    recv_mpst_a_to_j, RoleJ, next_j, 9 |
    recv_mpst_a_to_k, RoleK, next_k, 10 |
    recv_mpst_a_to_l, RoleL, next_l, 11 |
    recv_mpst_a_to_m, RoleM, next_m, 12 |
    recv_mpst_a_to_n, RoleN, next_n, 13 |
    recv_mpst_a_to_o, RoleO, next_o, 14 |
    recv_mpst_a_to_p, RoleP, next_p, 15 |
    recv_mpst_a_to_q, RoleQ, next_q, 16 |
    recv_mpst_a_to_r, RoleR, next_r, 17 |
    recv_mpst_a_to_s, RoleS, next_s, 18 |
    recv_mpst_a_to_t, RoleT, next_t, 19 | =>
    RoleA, SessionMpstTwenty, 20
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_to_a, RoleA, next_a, 1 |
    recv_mpst_b_to_c, RoleC, next_c, 2 |
    recv_mpst_b_to_d, RoleD, next_d, 3 |
    recv_mpst_b_to_e, RoleE, next_e, 4 |
    recv_mpst_b_to_f, RoleF, next_f, 5 |
    recv_mpst_b_to_g, RoleG, next_g, 6 |
    recv_mpst_b_to_h, RoleH, next_h, 7 |
    recv_mpst_b_to_i, RoleI, next_i, 8 |
    recv_mpst_b_to_j, RoleJ, next_j, 9 |
    recv_mpst_b_to_k, RoleK, next_k, 10 |
    recv_mpst_b_to_l, RoleL, next_l, 11 |
    recv_mpst_b_to_m, RoleM, next_m, 12 |
    recv_mpst_b_to_n, RoleN, next_n, 13 |
    recv_mpst_b_to_o, RoleO, next_o, 14 |
    recv_mpst_b_to_p, RoleP, next_p, 15 |
    recv_mpst_b_to_q, RoleQ, next_q, 16 |
    recv_mpst_b_to_r, RoleR, next_r, 17 |
    recv_mpst_b_to_s, RoleS, next_s, 18 |
    recv_mpst_b_to_t, RoleT, next_t, 19 | =>
    RoleB, SessionMpstTwenty, 20
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_to_a, RoleA, next_a, 1 |
    recv_mpst_c_to_b, RoleB, next_b, 2 |
    recv_mpst_c_to_d, RoleD, next_d, 3 |
    recv_mpst_c_to_e, RoleE, next_e, 4 |
    recv_mpst_c_to_f, RoleF, next_f, 5 |
    recv_mpst_c_to_g, RoleG, next_g, 6 |
    recv_mpst_c_to_h, RoleH, next_h, 7 |
    recv_mpst_c_to_i, RoleI, next_i, 8 |
    recv_mpst_c_to_j, RoleJ, next_j, 9 |
    recv_mpst_c_to_k, RoleK, next_k, 10 |
    recv_mpst_c_to_l, RoleL, next_l, 11 |
    recv_mpst_c_to_m, RoleM, next_m, 12 |
    recv_mpst_c_to_n, RoleN, next_n, 13 |
    recv_mpst_c_to_o, RoleO, next_o, 14 |
    recv_mpst_c_to_p, RoleP, next_p, 15 |
    recv_mpst_c_to_q, RoleQ, next_q, 16 |
    recv_mpst_c_to_r, RoleR, next_r, 17 |
    recv_mpst_c_to_s, RoleS, next_s, 18 |
    recv_mpst_c_to_t, RoleT, next_t, 19 | =>
    RoleC, SessionMpstTwenty, 20
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_to_a, RoleA, next_a, 1 |
    recv_mpst_d_to_b, RoleB, next_b, 2 |
    recv_mpst_d_to_c, RoleC, next_c, 3 |
    recv_mpst_d_to_e, RoleE, next_e, 4 |
    recv_mpst_d_to_f, RoleF, next_f, 5 |
    recv_mpst_d_to_g, RoleG, next_g, 6 |
    recv_mpst_d_to_h, RoleH, next_h, 7 |
    recv_mpst_d_to_i, RoleI, next_i, 8 |
    recv_mpst_d_to_j, RoleJ, next_j, 9 |
    recv_mpst_d_to_k, RoleK, next_k, 10 |
    recv_mpst_d_to_l, RoleL, next_l, 11 |
    recv_mpst_d_to_m, RoleM, next_m, 12 |
    recv_mpst_d_to_n, RoleN, next_n, 13 |
    recv_mpst_d_to_o, RoleO, next_o, 14 |
    recv_mpst_d_to_p, RoleP, next_p, 15 |
    recv_mpst_d_to_q, RoleQ, next_q, 16 |
    recv_mpst_d_to_r, RoleR, next_r, 17 |
    recv_mpst_d_to_s, RoleS, next_s, 18 |
    recv_mpst_d_to_t, RoleT, next_t, 19 | =>
    RoleD, SessionMpstTwenty, 20
);
// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_to_a, RoleA, next_a, 1 |
    recv_mpst_e_to_b, RoleB, next_b, 2 |
    recv_mpst_e_to_c, RoleC, next_c, 3 |
    recv_mpst_e_to_d, RoleD, next_d, 4 |
    recv_mpst_e_to_f, RoleF, next_f, 5 |
    recv_mpst_e_to_g, RoleG, next_g, 6 |
    recv_mpst_e_to_h, RoleH, next_h, 7 |
    recv_mpst_e_to_i, RoleI, next_i, 8 |
    recv_mpst_e_to_j, RoleJ, next_j, 9 |
    recv_mpst_e_to_k, RoleK, next_k, 10 |
    recv_mpst_e_to_l, RoleL, next_l, 11 |
    recv_mpst_e_to_m, RoleM, next_m, 12 |
    recv_mpst_e_to_n, RoleN, next_n, 13 |
    recv_mpst_e_to_o, RoleO, next_o, 14 |
    recv_mpst_e_to_p, RoleP, next_p, 15 |
    recv_mpst_e_to_q, RoleQ, next_q, 16 |
    recv_mpst_e_to_r, RoleR, next_r, 17 |
    recv_mpst_e_to_s, RoleS, next_s, 18 |
    recv_mpst_e_to_t, RoleT, next_t, 19 | =>
    RoleE, SessionMpstTwenty, 20
);
// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_to_a, RoleA, next_a, 1 |
    recv_mpst_f_to_b, RoleB, next_b, 2 |
    recv_mpst_f_to_c, RoleC, next_c, 3 |
    recv_mpst_f_to_d, RoleD, next_d, 4 |
    recv_mpst_f_to_e, RoleE, next_e, 5 |
    recv_mpst_f_to_g, RoleG, next_g, 6 |
    recv_mpst_f_to_h, RoleH, next_h, 7 |
    recv_mpst_f_to_i, RoleI, next_i, 8 |
    recv_mpst_f_to_j, RoleJ, next_j, 9 |
    recv_mpst_f_to_k, RoleK, next_k, 10 |
    recv_mpst_f_to_l, RoleL, next_l, 11 |
    recv_mpst_f_to_m, RoleM, next_m, 12 |
    recv_mpst_f_to_n, RoleN, next_n, 13 |
    recv_mpst_f_to_o, RoleO, next_o, 14 |
    recv_mpst_f_to_p, RoleP, next_p, 15 |
    recv_mpst_f_to_q, RoleQ, next_q, 16 |
    recv_mpst_f_to_r, RoleR, next_r, 17 |
    recv_mpst_f_to_s, RoleS, next_s, 18 |
    recv_mpst_f_to_t, RoleT, next_t, 19 | =>
    RoleF, SessionMpstTwenty, 20
);
// G
create_recv_mpst_session_bundle!(
    recv_mpst_g_to_a, RoleA, next_a, 1 |
    recv_mpst_g_to_b, RoleB, next_b, 2 |
    recv_mpst_g_to_c, RoleC, next_c, 3 |
    recv_mpst_g_to_d, RoleD, next_d, 4 |
    recv_mpst_g_to_e, RoleE, next_e, 5 |
    recv_mpst_g_to_f, RoleF, next_f, 6 |
    recv_mpst_g_to_h, RoleH, next_h, 7 |
    recv_mpst_g_to_i, RoleI, next_i, 8 |
    recv_mpst_g_to_j, RoleJ, next_j, 9 |
    recv_mpst_g_to_k, RoleK, next_k, 10 |
    recv_mpst_g_to_l, RoleL, next_l, 11 |
    recv_mpst_g_to_m, RoleM, next_m, 12 |
    recv_mpst_g_to_n, RoleN, next_n, 13 |
    recv_mpst_g_to_o, RoleO, next_o, 14 |
    recv_mpst_g_to_p, RoleP, next_p, 15 |
    recv_mpst_g_to_q, RoleQ, next_q, 16 |
    recv_mpst_g_to_r, RoleR, next_r, 17 |
    recv_mpst_g_to_s, RoleS, next_s, 18 |
    recv_mpst_g_to_t, RoleT, next_t, 19 | =>
    RoleG, SessionMpstTwenty, 20
);
// H
create_recv_mpst_session_bundle!(
    recv_mpst_h_to_a, RoleA, next_a, 1 |
    recv_mpst_h_to_b, RoleB, next_b, 2 |
    recv_mpst_h_to_c, RoleC, next_c, 3 |
    recv_mpst_h_to_d, RoleD, next_d, 4 |
    recv_mpst_h_to_e, RoleE, next_e, 5 |
    recv_mpst_h_to_f, RoleF, next_f, 6 |
    recv_mpst_h_to_g, RoleG, next_g, 7 |
    recv_mpst_h_to_i, RoleI, next_i, 8 |
    recv_mpst_h_to_j, RoleJ, next_j, 9 |
    recv_mpst_h_to_k, RoleK, next_k, 10 |
    recv_mpst_h_to_l, RoleL, next_l, 11 |
    recv_mpst_h_to_m, RoleM, next_m, 12 |
    recv_mpst_h_to_n, RoleN, next_n, 13 |
    recv_mpst_h_to_o, RoleO, next_o, 14 |
    recv_mpst_h_to_p, RoleP, next_p, 15 |
    recv_mpst_h_to_q, RoleQ, next_q, 16 |
    recv_mpst_h_to_r, RoleR, next_r, 17 |
    recv_mpst_h_to_s, RoleS, next_s, 18 |
    recv_mpst_h_to_t, RoleT, next_t, 19 | =>
    RoleH, SessionMpstTwenty, 20
);
// I
create_recv_mpst_session_bundle!(
    recv_mpst_i_to_a, RoleA, next_a, 1 |
    recv_mpst_i_to_b, RoleB, next_b, 2 |
    recv_mpst_i_to_c, RoleC, next_c, 3 |
    recv_mpst_i_to_d, RoleD, next_d, 4 |
    recv_mpst_i_to_e, RoleE, next_e, 5 |
    recv_mpst_i_to_f, RoleF, next_f, 6 |
    recv_mpst_i_to_g, RoleG, next_g, 7 |
    recv_mpst_i_to_h, RoleH, next_h, 8 |
    recv_mpst_i_to_j, RoleJ, next_j, 9 |
    recv_mpst_i_to_k, RoleK, next_k, 10 |
    recv_mpst_i_to_l, RoleL, next_l, 11 |
    recv_mpst_i_to_m, RoleM, next_m, 12 |
    recv_mpst_i_to_n, RoleN, next_n, 13 |
    recv_mpst_i_to_o, RoleO, next_o, 14 |
    recv_mpst_i_to_p, RoleP, next_p, 15 |
    recv_mpst_i_to_q, RoleQ, next_q, 16 |
    recv_mpst_i_to_r, RoleR, next_r, 17 |
    recv_mpst_i_to_s, RoleS, next_s, 18 |
    recv_mpst_i_to_t, RoleT, next_t, 19 | =>
    RoleI, SessionMpstTwenty, 20
);
// J
create_recv_mpst_session_bundle!(
    recv_mpst_j_to_a, RoleA, next_a, 1 |
    recv_mpst_j_to_b, RoleB, next_b, 2 |
    recv_mpst_j_to_c, RoleC, next_c, 3 |
    recv_mpst_j_to_d, RoleD, next_d, 4 |
    recv_mpst_j_to_e, RoleE, next_e, 5 |
    recv_mpst_j_to_f, RoleF, next_f, 6 |
    recv_mpst_j_to_g, RoleG, next_g, 7 |
    recv_mpst_j_to_h, RoleH, next_h, 8 |
    recv_mpst_j_to_i, RoleI, next_i, 9 |
    recv_mpst_j_to_k, RoleK, next_k, 10 |
    recv_mpst_j_to_l, RoleL, next_l, 11 |
    recv_mpst_j_to_m, RoleM, next_m, 12 |
    recv_mpst_j_to_n, RoleN, next_n, 13 |
    recv_mpst_j_to_o, RoleO, next_o, 14 |
    recv_mpst_j_to_p, RoleP, next_p, 15 |
    recv_mpst_j_to_q, RoleQ, next_q, 16 |
    recv_mpst_j_to_r, RoleR, next_r, 17 |
    recv_mpst_j_to_s, RoleS, next_s, 18 |
    recv_mpst_j_to_t, RoleT, next_t, 19 | =>
    RoleJ, SessionMpstTwenty, 20
);
// K
create_recv_mpst_session_bundle!(
    recv_mpst_k_to_a, RoleA, next_a, 1 |
    recv_mpst_k_to_b, RoleB, next_b, 2 |
    recv_mpst_k_to_c, RoleC, next_c, 3 |
    recv_mpst_k_to_d, RoleD, next_d, 4 |
    recv_mpst_k_to_e, RoleE, next_e, 5 |
    recv_mpst_k_to_f, RoleF, next_f, 6 |
    recv_mpst_k_to_g, RoleG, next_g, 7 |
    recv_mpst_k_to_h, RoleH, next_h, 8 |
    recv_mpst_k_to_i, RoleI, next_i, 9 |
    recv_mpst_k_to_j, RoleJ, next_j, 10 |
    recv_mpst_k_to_l, RoleL, next_l, 11 |
    recv_mpst_k_to_m, RoleM, next_m, 12 |
    recv_mpst_k_to_n, RoleN, next_n, 13 |
    recv_mpst_k_to_o, RoleO, next_o, 14 |
    recv_mpst_k_to_p, RoleP, next_p, 15 |
    recv_mpst_k_to_q, RoleQ, next_q, 16 |
    recv_mpst_k_to_r, RoleR, next_r, 17 |
    recv_mpst_k_to_s, RoleS, next_s, 18 |
    recv_mpst_k_to_t, RoleT, next_t, 19 | =>
    RoleK, SessionMpstTwenty, 20
);
// L
create_recv_mpst_session_bundle!(
    recv_mpst_l_to_a, RoleA, next_a, 1 |
    recv_mpst_l_to_b, RoleB, next_b, 2 |
    recv_mpst_l_to_c, RoleC, next_c, 3 |
    recv_mpst_l_to_d, RoleD, next_d, 4 |
    recv_mpst_l_to_e, RoleE, next_e, 5 |
    recv_mpst_l_to_f, RoleF, next_f, 6 |
    recv_mpst_l_to_g, RoleG, next_g, 7 |
    recv_mpst_l_to_h, RoleH, next_h, 8 |
    recv_mpst_l_to_i, RoleI, next_i, 9 |
    recv_mpst_l_to_j, RoleJ, next_j, 10 |
    recv_mpst_l_to_k, RoleK, next_k, 11 |
    recv_mpst_l_to_m, RoleM, next_m, 12 |
    recv_mpst_l_to_n, RoleN, next_n, 13 |
    recv_mpst_l_to_o, RoleO, next_o, 14 |
    recv_mpst_l_to_p, RoleP, next_p, 15 |
    recv_mpst_l_to_q, RoleQ, next_q, 16 |
    recv_mpst_l_to_r, RoleR, next_r, 17 |
    recv_mpst_l_to_s, RoleS, next_s, 18 |
    recv_mpst_l_to_t, RoleT, next_t, 19 | =>
    RoleL, SessionMpstTwenty, 20
);
// M
create_recv_mpst_session_bundle!(
    recv_mpst_m_to_a, RoleA, next_a, 1 |
    recv_mpst_m_to_b, RoleB, next_b, 2 |
    recv_mpst_m_to_c, RoleC, next_c, 3 |
    recv_mpst_m_to_d, RoleD, next_d, 4 |
    recv_mpst_m_to_e, RoleE, next_e, 5 |
    recv_mpst_m_to_f, RoleF, next_f, 6 |
    recv_mpst_m_to_g, RoleG, next_g, 7 |
    recv_mpst_m_to_h, RoleH, next_h, 8 |
    recv_mpst_m_to_i, RoleI, next_i, 9 |
    recv_mpst_m_to_j, RoleJ, next_j, 10 |
    recv_mpst_m_to_k, RoleK, next_k, 11 |
    recv_mpst_m_to_l, RoleL, next_l, 12 |
    recv_mpst_m_to_n, RoleN, next_n, 13 |
    recv_mpst_m_to_o, RoleO, next_o, 14 |
    recv_mpst_m_to_p, RoleP, next_p, 15 |
    recv_mpst_m_to_q, RoleQ, next_q, 16 |
    recv_mpst_m_to_r, RoleR, next_r, 17 |
    recv_mpst_m_to_s, RoleS, next_s, 18 |
    recv_mpst_m_to_t, RoleT, next_t, 19 | =>
    RoleM, SessionMpstTwenty, 20
);
// N
create_recv_mpst_session_bundle!(
    recv_mpst_n_to_a, RoleA, next_a, 1 |
    recv_mpst_n_to_b, RoleB, next_b, 2 |
    recv_mpst_n_to_c, RoleC, next_c, 3 |
    recv_mpst_n_to_d, RoleD, next_d, 4 |
    recv_mpst_n_to_e, RoleE, next_e, 5 |
    recv_mpst_n_to_f, RoleF, next_f, 6 |
    recv_mpst_n_to_g, RoleG, next_g, 7 |
    recv_mpst_n_to_h, RoleH, next_h, 8 |
    recv_mpst_n_to_i, RoleI, next_i, 9 |
    recv_mpst_n_to_j, RoleJ, next_j, 10 |
    recv_mpst_n_to_k, RoleK, next_k, 11 |
    recv_mpst_n_to_l, RoleL, next_l, 12 |
    recv_mpst_n_to_m, RoleM, next_m, 13 |
    recv_mpst_n_to_o, RoleO, next_o, 14 |
    recv_mpst_n_to_p, RoleP, next_p, 15 |
    recv_mpst_n_to_q, RoleQ, next_q, 16 |
    recv_mpst_n_to_r, RoleR, next_r, 17 |
    recv_mpst_n_to_s, RoleS, next_s, 18 |
    recv_mpst_n_to_t, RoleT, next_t, 19 | =>
    RoleN, SessionMpstTwenty, 20
);
// O
create_recv_mpst_session_bundle!(
    recv_mpst_o_to_a, RoleA, next_a, 1 |
    recv_mpst_o_to_b, RoleB, next_b, 2 |
    recv_mpst_o_to_c, RoleC, next_c, 3 |
    recv_mpst_o_to_d, RoleD, next_d, 4 |
    recv_mpst_o_to_e, RoleE, next_e, 5 |
    recv_mpst_o_to_f, RoleF, next_f, 6 |
    recv_mpst_o_to_g, RoleG, next_g, 7 |
    recv_mpst_o_to_h, RoleH, next_h, 8 |
    recv_mpst_o_to_i, RoleI, next_i, 9 |
    recv_mpst_o_to_j, RoleJ, next_j, 10 |
    recv_mpst_o_to_k, RoleK, next_k, 11 |
    recv_mpst_o_to_l, RoleL, next_l, 12 |
    recv_mpst_o_to_m, RoleM, next_m, 13 |
    recv_mpst_o_to_n, RoleN, next_n, 14 |
    recv_mpst_o_to_p, RoleP, next_p, 15 |
    recv_mpst_o_to_q, RoleQ, next_q, 16 |
    recv_mpst_o_to_r, RoleR, next_r, 17 |
    recv_mpst_o_to_s, RoleS, next_s, 18 |
    recv_mpst_o_to_t, RoleT, next_t, 19 | =>
    RoleO, SessionMpstTwenty, 20
);
// P
create_recv_mpst_session_bundle!(
    recv_mpst_p_to_a, RoleA, next_a, 1 |
    recv_mpst_p_to_b, RoleB, next_b, 2 |
    recv_mpst_p_to_c, RoleC, next_c, 3 |
    recv_mpst_p_to_d, RoleD, next_d, 4 |
    recv_mpst_p_to_e, RoleE, next_e, 5 |
    recv_mpst_p_to_f, RoleF, next_f, 6 |
    recv_mpst_p_to_g, RoleG, next_g, 7 |
    recv_mpst_p_to_h, RoleH, next_h, 8 |
    recv_mpst_p_to_i, RoleI, next_i, 9 |
    recv_mpst_p_to_j, RoleJ, next_j, 10 |
    recv_mpst_p_to_k, RoleK, next_k, 11 |
    recv_mpst_p_to_l, RoleL, next_l, 12 |
    recv_mpst_p_to_m, RoleM, next_m, 13 |
    recv_mpst_p_to_n, RoleN, next_n, 14 |
    recv_mpst_p_to_o, RoleO, next_o, 15 |
    recv_mpst_p_to_q, RoleQ, next_q, 16 |
    recv_mpst_p_to_r, RoleR, next_r, 17 |
    recv_mpst_p_to_s, RoleS, next_s, 18 |
    recv_mpst_p_to_t, RoleT, next_t, 19 | =>
    RoleP, SessionMpstTwenty, 20
);
// Q
create_recv_mpst_session_bundle!(
    recv_mpst_q_to_a, RoleA, next_a, 1 |
    recv_mpst_q_to_b, RoleB, next_b, 2 |
    recv_mpst_q_to_c, RoleC, next_c, 3 |
    recv_mpst_q_to_d, RoleD, next_d, 4 |
    recv_mpst_q_to_e, RoleE, next_e, 5 |
    recv_mpst_q_to_f, RoleF, next_f, 6 |
    recv_mpst_q_to_g, RoleG, next_g, 7 |
    recv_mpst_q_to_h, RoleH, next_h, 8 |
    recv_mpst_q_to_i, RoleI, next_i, 9 |
    recv_mpst_q_to_j, RoleJ, next_j, 10 |
    recv_mpst_q_to_k, RoleK, next_k, 11 |
    recv_mpst_q_to_l, RoleL, next_l, 12 |
    recv_mpst_q_to_m, RoleM, next_m, 13 |
    recv_mpst_q_to_n, RoleN, next_n, 14 |
    recv_mpst_q_to_o, RoleO, next_o, 15 |
    recv_mpst_q_to_p, RoleP, next_p, 16 |
    recv_mpst_q_to_r, RoleR, next_r, 17 |
    recv_mpst_q_to_s, RoleS, next_s, 18 |
    recv_mpst_q_to_t, RoleT, next_t, 19 | =>
    RoleQ, SessionMpstTwenty, 20
);
// R
create_recv_mpst_session_bundle!(
    recv_mpst_r_to_a, RoleA, next_a, 1 |
    recv_mpst_r_to_b, RoleB, next_b, 2 |
    recv_mpst_r_to_c, RoleC, next_c, 3 |
    recv_mpst_r_to_d, RoleD, next_d, 4 |
    recv_mpst_r_to_e, RoleE, next_e, 5 |
    recv_mpst_r_to_f, RoleF, next_f, 6 |
    recv_mpst_r_to_g, RoleG, next_g, 7 |
    recv_mpst_r_to_h, RoleH, next_h, 8 |
    recv_mpst_r_to_i, RoleI, next_i, 9 |
    recv_mpst_r_to_j, RoleJ, next_j, 10 |
    recv_mpst_r_to_k, RoleK, next_k, 11 |
    recv_mpst_r_to_l, RoleL, next_l, 12 |
    recv_mpst_r_to_m, RoleM, next_m, 13 |
    recv_mpst_r_to_n, RoleN, next_n, 14 |
    recv_mpst_r_to_o, RoleO, next_o, 15 |
    recv_mpst_r_to_p, RoleP, next_p, 16 |
    recv_mpst_r_to_q, RoleQ, next_q, 17 |
    recv_mpst_r_to_s, RoleS, next_s, 18 |
    recv_mpst_r_to_t, RoleT, next_t, 19 | =>
    RoleR, SessionMpstTwenty, 20
);
// S
create_recv_mpst_session_bundle!(
    recv_mpst_s_to_a, RoleA, next_a, 1 |
    recv_mpst_s_to_b, RoleB, next_b, 2 |
    recv_mpst_s_to_c, RoleC, next_c, 3 |
    recv_mpst_s_to_d, RoleD, next_d, 4 |
    recv_mpst_s_to_e, RoleE, next_e, 5 |
    recv_mpst_s_to_f, RoleF, next_f, 6 |
    recv_mpst_s_to_g, RoleG, next_g, 7 |
    recv_mpst_s_to_h, RoleH, next_h, 8 |
    recv_mpst_s_to_i, RoleI, next_i, 9 |
    recv_mpst_s_to_j, RoleJ, next_j, 10 |
    recv_mpst_s_to_k, RoleK, next_k, 11 |
    recv_mpst_s_to_l, RoleL, next_l, 12 |
    recv_mpst_s_to_m, RoleM, next_m, 13 |
    recv_mpst_s_to_n, RoleN, next_n, 14 |
    recv_mpst_s_to_o, RoleO, next_o, 15 |
    recv_mpst_s_to_p, RoleP, next_p, 16 |
    recv_mpst_s_to_q, RoleQ, next_q, 17 |
    recv_mpst_s_to_r, RoleR, next_r, 18 |
    recv_mpst_s_to_t, RoleT, next_t, 19 | =>
    RoleS, SessionMpstTwenty, 20
);
// T
create_recv_mpst_session_bundle!(
    recv_mpst_t_to_a, RoleA, next_a, 1 |
    recv_mpst_t_to_b, RoleB, next_b, 2 |
    recv_mpst_t_to_c, RoleC, next_c, 3 |
    recv_mpst_t_to_d, RoleD, next_d, 4 |
    recv_mpst_t_to_e, RoleE, next_e, 5 |
    recv_mpst_t_to_f, RoleF, next_f, 6 |
    recv_mpst_t_to_g, RoleG, next_g, 7 |
    recv_mpst_t_to_h, RoleH, next_h, 8 |
    recv_mpst_t_to_i, RoleI, next_i, 9 |
    recv_mpst_t_to_j, RoleJ, next_j, 10 |
    recv_mpst_t_to_k, RoleK, next_k, 11 |
    recv_mpst_t_to_l, RoleL, next_l, 12 |
    recv_mpst_t_to_m, RoleM, next_m, 13 |
    recv_mpst_t_to_n, RoleN, next_n, 14 |
    recv_mpst_t_to_o, RoleO, next_o, 15 |
    recv_mpst_t_to_p, RoleP, next_p, 16 |
    recv_mpst_t_to_q, RoleQ, next_q, 17 |
    recv_mpst_t_to_r, RoleR, next_r, 18 |
    recv_mpst_t_to_s, RoleS, next_s, 19 | =>
    RoleT, SessionMpstTwenty, 20
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstTwenty, 20);

// Create fork function
fork_mpst_multi!(fork_mpst,  SessionMpstTwenty, 20);

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
// A
enum Branching0fromTtoA
{
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
type RecursAtoT = Recv<Branching0fromTtoA, End>;
// B
enum Branching0fromTtoB
{
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
type RecursBtoT = Recv<Branching0fromTtoB, End>;
// C
enum Branching0fromTtoC
{
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
type RecursCtoT = Recv<Branching0fromTtoC, End>;
// D
enum Branching0fromTtoD
{
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
type RecursDtoT = Recv<Branching0fromTtoD, End>;
// E
enum Branching0fromTtoE
{
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
type RecursEtoT = Recv<Branching0fromTtoE, End>;
// F
enum Branching0fromTtoF
{
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
type RecursFtoT = Recv<Branching0fromTtoF, End>;
// G
enum Branching0fromTtoG
{
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
type RecursGtoT = Recv<Branching0fromTtoG, End>;
// H
enum Branching0fromTtoH
{
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
type RecursHtoT = Recv<Branching0fromTtoH, End>;
// I
enum Branching0fromTtoI
{
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
type RecursItoT = Recv<Branching0fromTtoI, End>;
// J
enum Branching0fromTtoJ
{
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
type RecursJtoT = Recv<Branching0fromTtoJ, End>;
// K
enum Branching0fromTtoK
{
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
type RecursKtoT = Recv<Branching0fromTtoK, End>;
// L
enum Branching0fromTtoL
{
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
type RecursLtoT = Recv<Branching0fromTtoL, End>;
// M
enum Branching0fromTtoM
{
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
type RecursMtoT = Recv<Branching0fromTtoM, End>;
// N
enum Branching0fromTtoN
{
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
type RecursNtoT = Recv<Branching0fromTtoN, End>;
// O
enum Branching0fromTtoO
{
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
type RecursOtoT = Recv<Branching0fromTtoO, End>;
// P
enum Branching0fromTtoP
{
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
type RecursPtoT = Recv<Branching0fromTtoP, End>;
// Q
enum Branching0fromTtoQ
{
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
type RecursQtoT = Recv<Branching0fromTtoQ, End>;
// R
enum Branching0fromTtoR
{
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
type RecursRtoT = Recv<Branching0fromTtoR, End>;
// S
enum Branching0fromTtoS
{
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
type RecursStoT = Recv<Branching0fromTtoS, End>;
// T
type Choose0fromTtoA = Send<Branching0fromTtoA, End>;
type Choose0fromTtoB = Send<Branching0fromTtoB, End>;
type Choose0fromTtoC = Send<Branching0fromTtoC, End>;
type Choose0fromTtoD = Send<Branching0fromTtoD, End>;
type Choose0fromTtoE = Send<Branching0fromTtoE, End>;
type Choose0fromTtoF = Send<Branching0fromTtoF, End>;
type Choose0fromTtoG = Send<Branching0fromTtoG, End>;
type Choose0fromTtoH = Send<Branching0fromTtoH, End>;
type Choose0fromTtoI = Send<Branching0fromTtoI, End>;
type Choose0fromTtoJ = Send<Branching0fromTtoJ, End>;
type Choose0fromTtoK = Send<Branching0fromTtoK, End>;
type Choose0fromTtoL = Send<Branching0fromTtoL, End>;
type Choose0fromTtoM = Send<Branching0fromTtoM, End>;
type Choose0fromTtoN = Send<Branching0fromTtoN, End>;
type Choose0fromTtoO = Send<Branching0fromTtoO, End>;
type Choose0fromTtoP = Send<Branching0fromTtoP, End>;
type Choose0fromTtoQ = Send<Branching0fromTtoQ, End>;
type Choose0fromTtoR = Send<Branching0fromTtoR, End>;
type Choose0fromTtoS = Send<Branching0fromTtoS, End>;

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

// Functions
fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_a_to_t, {
        Branching0fromTtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoA::More(s) => {
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

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_b_to_t, {
        Branching0fromTtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoB::More(s) => {
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

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_c_to_t, {
        Branching0fromTtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoC::More(s) => {
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

fn simple_five_endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_d_to_t, {
        Branching0fromTtoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoD::More(s) => {
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

fn simple_five_endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_e_to_t, {
        Branching0fromTtoE::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoE::More(s) => {
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

fn simple_five_endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_f_to_t, {
        Branching0fromTtoF::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoF::More(s) => {
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

fn simple_five_endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_g_to_t, {
        Branching0fromTtoG::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoG::More(s) => {
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

fn simple_five_endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_h_to_t, {
        Branching0fromTtoH::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoH::More(s) => {
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

fn simple_five_endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_i_to_t, {
        Branching0fromTtoI::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoI::More(s) => {
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

fn simple_five_endpoint_j(s: EndpointJ) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_j_to_t, {
        Branching0fromTtoJ::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoJ::More(s) => {
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

fn simple_five_endpoint_k(s: EndpointK) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_k_to_t, {
        Branching0fromTtoK::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoK::More(s) => {
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

fn simple_five_endpoint_l(s: EndpointL) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_l_to_t, {
        Branching0fromTtoL::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoL::More(s) => {
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

fn simple_five_endpoint_m(s: EndpointM) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_m_to_t, {
        Branching0fromTtoM::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoM::More(s) => {
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

fn simple_five_endpoint_n(s: EndpointN) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_n_to_t, {
        Branching0fromTtoN::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoN::More(s) => {
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

fn simple_five_endpoint_o(s: EndpointO) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_o_to_t, {
        Branching0fromTtoO::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoO::More(s) => {
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

fn simple_five_endpoint_p(s: EndpointP) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_p_to_t, {
        Branching0fromTtoP::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoP::More(s) => {
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

fn simple_five_endpoint_q(s: EndpointQ) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_q_to_t, {
        Branching0fromTtoQ::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoQ::More(s) => {
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

fn simple_five_endpoint_r(s: EndpointR) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_r_to_t, {
        Branching0fromTtoR::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoR::More(s) => {
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

fn simple_five_endpoint_s(s: EndpointS) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_s_to_t, {
        Branching0fromTtoS::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoS::More(s) => {
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

fn simple_five_endpoint_t(s: EndpointT) -> Result<(), Box<dyn Error>>
{
    recurs_t(s, SIZE)
}

fn recurs_t(s: EndpointT, index: i64) -> Result<(), Box<dyn Error>>
{
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
                Branching0fromTtoA::Done,
                Branching0fromTtoB::Done,
                Branching0fromTtoC::Done,
                Branching0fromTtoD::Done,
                Branching0fromTtoE::Done,
                Branching0fromTtoF::Done,
                Branching0fromTtoG::Done,
                Branching0fromTtoH::Done,
                Branching0fromTtoI::Done,
                Branching0fromTtoJ::Done,
                Branching0fromTtoK::Done,
                Branching0fromTtoL::Done,
                Branching0fromTtoM::Done,
                Branching0fromTtoN::Done,
                Branching0fromTtoO::Done,
                Branching0fromTtoP::Done,
                Branching0fromTtoQ::Done,
                Branching0fromTtoR::Done,
                Branching0fromTtoS::Done, =>
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
                Branching0fromTtoA::More,
                Branching0fromTtoB::More,
                Branching0fromTtoC::More,
                Branching0fromTtoD::More,
                Branching0fromTtoE::More,
                Branching0fromTtoF::More,
                Branching0fromTtoG::More,
                Branching0fromTtoH::More,
                Branching0fromTtoI::More,
                Branching0fromTtoJ::More,
                Branching0fromTtoK::More,
                Branching0fromTtoL::More,
                Branching0fromTtoM::More,
                Branching0fromTtoN::More,
                Branching0fromTtoO::More,
                Branching0fromTtoP::More,
                Branching0fromTtoQ::More,
                Branching0fromTtoR::More,
                Branching0fromTtoS::More, =>
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

fn all_mpst() -> Result<(), Box<dyn Error>>
{
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
// A
enum BinaryA
{
    More(Recv<(), Send<(), RecursA>>),
    Done(End),
}
type RecursA = Recv<BinaryA, End>;
fn binary_a_to_b(s: RecursA) -> Result<(), Box<dyn Error>>
{
    offer!(s, {
        BinaryA::Done(s) => {
            close(s)
        },
        BinaryA::More(s) => {
            let (_, s) = recv(s)?;
            let s = send((), s);
            binary_a_to_b(s)
        },
    })
}

// B
type RecursB = <RecursA as Session>::Dual;
fn binary_b_to_a(s: Send<(), Recv<(), RecursB>>) -> Result<RecursB, Box<dyn Error>>
{
    let s = send((), s);
    let (_, s) = recv(s)?;
    Ok(s)
}

fn all_binaries() -> Result<(), Box<dyn Error>>
{
    let mut threads = Vec::new();
    let mut sessions = Vec::new();

    for _ in 0..190 {
        let (thread, s): (JoinHandle<()>, RecursB) = fork_with_thread_id(black_box(binary_a_to_b));

        threads.push(thread);
        sessions.push(s);
    }

    let main = spawn(move || {
        for _ in 0..SIZE {
            sessions = sessions
                .into_iter()
                .map(|s| binary_b_to_a(choose!(BinaryA::More, s)).unwrap())
                .collect::<Vec<_>>();
        }

        sessions
            .into_iter()
            .for_each(|s| close(choose!(BinaryA::Done, s)).unwrap());

        threads.into_iter().for_each(|elt| elt.join().unwrap());
    });

    main.join().unwrap();

    Ok(())
}

/////////////////////////

type ReceivingSendingReceiving = crossbeam_channel::Receiver<SendingReceiving>;
type SendingReceivingSending = crossbeam_channel::Sender<ReceivingSending>;

type SendingReceiving = crossbeam_channel::Sender<Receiving>;
type ReceivingSending = crossbeam_channel::Receiver<Sending>;

type Receiving = crossbeam_channel::Receiver<()>;
type Sending = crossbeam_channel::Sender<()>;

fn all_crossbeam() -> Result<(), Box<dyn Error>>
{
    let mut threads = Vec::new();

    for _ in 0..190 {
        let main = spawn(move || {
            for _ in 0..SIZE {
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

    Ok(())
}

/////////////////////////

static SIZE: i64 = 0;

fn long_simple_protocol_mpst(c: &mut Criterion)
{
    c.bench_function(
        &format!("long twenty empty simple protocol MPST {}", SIZE),
        |b| b.iter(|| all_mpst()),
    );
}

fn long_simple_protocol_binary(c: &mut Criterion)
{
    c.bench_function(
        &format!("long twenty empty simple protocol binary {}", SIZE),
        |b| b.iter(|| all_binaries()),
    );
}

fn long_simple_protocol_crossbeam(c: &mut Criterion)
{
    c.bench_function(
        &format!("long twenty empty simple protocol crossbeam {}", SIZE),
        |b| b.iter(|| all_crossbeam()),
    );
}

fn long_warmup() -> Criterion
{
    Criterion::default().measurement_time(Duration::new(20, 0))
}

criterion_group! {
    name = long_twenty_empty_simple_protocols;
    config = long_warmup();
    targets = long_simple_protocol_mpst, long_simple_protocol_binary, long_simple_protocol_crossbeam
}
criterion_main!(long_twenty_empty_simple_protocols);
