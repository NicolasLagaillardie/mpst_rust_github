use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    broadcast_cancel, bundle_struct_fork_close_multi,
    create_fn_choose_mpst_cancel_multi_to_all_bundle, create_multiple_normal_role_short,
    create_recv_mpst_session_bundle, create_send_check_cancel_bundle, offer_cancel_mpst,
};

use std::error::Error;

// Create the new SessionMpst for twenty participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstTwentyOne, 21);

// Create new roles
// normal
create_multiple_normal_role_short!(
    Central, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T
);

// Create new send functions
// A
create_send_check_cancel_bundle!(
    send_mpst_a_to_b, RoleB, 2 |
    send_mpst_a_to_c, RoleC, 3 |
    send_mpst_a_to_d, RoleD, 4 |
    send_mpst_a_to_e, RoleE, 5 |
    send_mpst_a_to_f, RoleF, 6 |
    send_mpst_a_to_g, RoleG, 7 |
    send_mpst_a_to_h, RoleH, 8 |
    send_mpst_a_to_i, RoleI, 9 |
    send_mpst_a_to_j, RoleJ, 10 |
    send_mpst_a_to_k, RoleK, 11 |
    send_mpst_a_to_l, RoleL, 12 |
    send_mpst_a_to_m, RoleM, 13 |
    send_mpst_a_to_n, RoleN, 14 |
    send_mpst_a_to_o, RoleO, 15 |
    send_mpst_a_to_p, RoleP, 16 |
    send_mpst_a_to_q, RoleQ, 17 |
    send_mpst_a_to_r, RoleR, 18 |
    send_mpst_a_to_s, RoleS, 19 |
    send_mpst_a_to_t, RoleT, 20 | =>
    RoleA, SessionMpstTwentyOne, 21
);
// B
create_send_check_cancel_bundle!(
    send_mpst_b_to_a, RoleA, 2 |
    send_mpst_b_to_c, RoleC, 3 |
    send_mpst_b_to_d, RoleD, 4 |
    send_mpst_b_to_e, RoleE, 5 |
    send_mpst_b_to_f, RoleF, 6 |
    send_mpst_b_to_g, RoleG, 7 |
    send_mpst_b_to_h, RoleH, 8 |
    send_mpst_b_to_i, RoleI, 9 |
    send_mpst_b_to_j, RoleJ, 10 |
    send_mpst_b_to_k, RoleK, 11 |
    send_mpst_b_to_l, RoleL, 12 |
    send_mpst_b_to_m, RoleM, 13 |
    send_mpst_b_to_n, RoleN, 14 |
    send_mpst_b_to_o, RoleO, 15 |
    send_mpst_b_to_p, RoleP, 16 |
    send_mpst_b_to_q, RoleQ, 17 |
    send_mpst_b_to_r, RoleR, 18 |
    send_mpst_b_to_s, RoleS, 19 |
    send_mpst_b_to_t, RoleT, 20 | =>
    RoleB, SessionMpstTwentyOne, 21
);
// C
create_send_check_cancel_bundle!(
    send_mpst_c_to_a, RoleA, 2 |
    send_mpst_c_to_b, RoleB, 3 |
    send_mpst_c_to_d, RoleD, 4 |
    send_mpst_c_to_e, RoleE, 5 |
    send_mpst_c_to_f, RoleF, 6 |
    send_mpst_c_to_g, RoleG, 7 |
    send_mpst_c_to_h, RoleH, 8 |
    send_mpst_c_to_i, RoleI, 9 |
    send_mpst_c_to_j, RoleJ, 10 |
    send_mpst_c_to_k, RoleK, 11 |
    send_mpst_c_to_l, RoleL, 12 |
    send_mpst_c_to_m, RoleM, 13 |
    send_mpst_c_to_n, RoleN, 14 |
    send_mpst_c_to_o, RoleO, 15 |
    send_mpst_c_to_p, RoleP, 16 |
    send_mpst_c_to_q, RoleQ, 17 |
    send_mpst_c_to_r, RoleR, 18 |
    send_mpst_c_to_s, RoleS, 19 |
    send_mpst_c_to_t, RoleT, 20 | =>
    RoleC, SessionMpstTwentyOne, 21
);
// D
create_send_check_cancel_bundle!(
    send_mpst_d_to_a, RoleA, 2 |
    send_mpst_d_to_b, RoleB, 3 |
    send_mpst_d_to_c, RoleC, 4 |
    send_mpst_d_to_e, RoleE, 5 |
    send_mpst_d_to_f, RoleF, 6 |
    send_mpst_d_to_g, RoleG, 7 |
    send_mpst_d_to_h, RoleH, 8 |
    send_mpst_d_to_i, RoleI, 9 |
    send_mpst_d_to_j, RoleJ, 10 |
    send_mpst_d_to_k, RoleK, 11 |
    send_mpst_d_to_l, RoleL, 12 |
    send_mpst_d_to_m, RoleM, 13 |
    send_mpst_d_to_n, RoleN, 14 |
    send_mpst_d_to_o, RoleO, 15 |
    send_mpst_d_to_p, RoleP, 16 |
    send_mpst_d_to_q, RoleQ, 17 |
    send_mpst_d_to_r, RoleR, 18 |
    send_mpst_d_to_s, RoleS, 19 |
    send_mpst_d_to_t, RoleT, 20 | =>
    RoleD, SessionMpstTwentyOne, 21
);
// E
create_send_check_cancel_bundle!(
    send_mpst_e_to_a, RoleA, 2 |
    send_mpst_e_to_b, RoleB, 3 |
    send_mpst_e_to_c, RoleC, 4 |
    send_mpst_e_to_d, RoleD, 5 |
    send_mpst_e_to_f, RoleF, 6 |
    send_mpst_e_to_g, RoleG, 7 |
    send_mpst_e_to_h, RoleH, 8 |
    send_mpst_e_to_i, RoleI, 9 |
    send_mpst_e_to_j, RoleJ, 10 |
    send_mpst_e_to_k, RoleK, 11 |
    send_mpst_e_to_l, RoleL, 12 |
    send_mpst_e_to_m, RoleM, 13 |
    send_mpst_e_to_n, RoleN, 14 |
    send_mpst_e_to_o, RoleO, 15 |
    send_mpst_e_to_p, RoleP, 16 |
    send_mpst_e_to_q, RoleQ, 17 |
    send_mpst_e_to_r, RoleR, 18 |
    send_mpst_e_to_s, RoleS, 19 |
    send_mpst_e_to_t, RoleT, 20 | =>
    RoleE, SessionMpstTwentyOne, 21
);
// F
create_send_check_cancel_bundle!(
    send_mpst_f_to_a, RoleA, 2 |
    send_mpst_f_to_b, RoleB, 3 |
    send_mpst_f_to_c, RoleC, 4 |
    send_mpst_f_to_d, RoleD, 5 |
    send_mpst_f_to_e, RoleE, 6 |
    send_mpst_f_to_g, RoleG, 7 |
    send_mpst_f_to_h, RoleH, 8 |
    send_mpst_f_to_i, RoleI, 9 |
    send_mpst_f_to_j, RoleJ, 10 |
    send_mpst_f_to_k, RoleK, 11 |
    send_mpst_f_to_l, RoleL, 12 |
    send_mpst_f_to_m, RoleM, 13 |
    send_mpst_f_to_n, RoleN, 14 |
    send_mpst_f_to_o, RoleO, 15 |
    send_mpst_f_to_p, RoleP, 16 |
    send_mpst_f_to_q, RoleQ, 17 |
    send_mpst_f_to_r, RoleR, 18 |
    send_mpst_f_to_s, RoleS, 19 |
    send_mpst_f_to_t, RoleT, 20 | =>
    RoleF, SessionMpstTwentyOne, 21
);
// G
create_send_check_cancel_bundle!(
    send_mpst_g_to_a, RoleA, 2 |
    send_mpst_g_to_b, RoleB, 3 |
    send_mpst_g_to_c, RoleC, 4 |
    send_mpst_g_to_d, RoleD, 5 |
    send_mpst_g_to_e, RoleE, 6 |
    send_mpst_g_to_f, RoleF, 7 |
    send_mpst_g_to_h, RoleH, 8 |
    send_mpst_g_to_i, RoleI, 9 |
    send_mpst_g_to_j, RoleJ, 10 |
    send_mpst_g_to_k, RoleK, 11 |
    send_mpst_g_to_l, RoleL, 12 |
    send_mpst_g_to_m, RoleM, 13 |
    send_mpst_g_to_n, RoleN, 14 |
    send_mpst_g_to_o, RoleO, 15 |
    send_mpst_g_to_p, RoleP, 16 |
    send_mpst_g_to_q, RoleQ, 17 |
    send_mpst_g_to_r, RoleR, 18 |
    send_mpst_g_to_s, RoleS, 19 |
    send_mpst_g_to_t, RoleT, 20 | =>
    RoleG, SessionMpstTwentyOne, 21
);
// H
create_send_check_cancel_bundle!(
    send_mpst_h_to_a, RoleA, 2 |
    send_mpst_h_to_b, RoleB, 3 |
    send_mpst_h_to_c, RoleC, 4 |
    send_mpst_h_to_d, RoleD, 5 |
    send_mpst_h_to_e, RoleE, 6 |
    send_mpst_h_to_f, RoleF, 7 |
    send_mpst_h_to_g, RoleG, 8 |
    send_mpst_h_to_i, RoleI, 9 |
    send_mpst_h_to_j, RoleJ, 10 |
    send_mpst_h_to_k, RoleK, 11 |
    send_mpst_h_to_l, RoleL, 12 |
    send_mpst_h_to_m, RoleM, 13 |
    send_mpst_h_to_n, RoleN, 14 |
    send_mpst_h_to_o, RoleO, 15 |
    send_mpst_h_to_p, RoleP, 16 |
    send_mpst_h_to_q, RoleQ, 17 |
    send_mpst_h_to_r, RoleR, 18 |
    send_mpst_h_to_s, RoleS, 19 |
    send_mpst_h_to_t, RoleT, 20 | =>
    RoleH, SessionMpstTwentyOne, 21
);
// I
create_send_check_cancel_bundle!(
    send_mpst_i_to_a, RoleA, 2 |
    send_mpst_i_to_b, RoleB, 3 |
    send_mpst_i_to_c, RoleC, 4 |
    send_mpst_i_to_d, RoleD, 5 |
    send_mpst_i_to_e, RoleE, 6 |
    send_mpst_i_to_f, RoleF, 7 |
    send_mpst_i_to_g, RoleG, 8 |
    send_mpst_i_to_h, RoleH, 9 |
    send_mpst_i_to_j, RoleJ, 10 |
    send_mpst_i_to_k, RoleK, 11 |
    send_mpst_i_to_l, RoleL, 12 |
    send_mpst_i_to_m, RoleM, 13 |
    send_mpst_i_to_n, RoleN, 14 |
    send_mpst_i_to_o, RoleO, 15 |
    send_mpst_i_to_p, RoleP, 16 |
    send_mpst_i_to_q, RoleQ, 17 |
    send_mpst_i_to_r, RoleR, 18 |
    send_mpst_i_to_s, RoleS, 19 |
    send_mpst_i_to_t, RoleT, 20 | =>
    RoleI, SessionMpstTwentyOne, 21
);
// J
create_send_check_cancel_bundle!(
    send_mpst_j_to_a, RoleA, 2 |
    send_mpst_j_to_b, RoleB, 3 |
    send_mpst_j_to_c, RoleC, 4 |
    send_mpst_j_to_d, RoleD, 5 |
    send_mpst_j_to_e, RoleE, 6 |
    send_mpst_j_to_f, RoleF, 7 |
    send_mpst_j_to_g, RoleG, 8 |
    send_mpst_j_to_h, RoleH, 9 |
    send_mpst_j_to_i, RoleI, 10 |
    send_mpst_j_to_k, RoleK, 11 |
    send_mpst_j_to_l, RoleL, 12 |
    send_mpst_j_to_m, RoleM, 13 |
    send_mpst_j_to_n, RoleN, 14 |
    send_mpst_j_to_o, RoleO, 15 |
    send_mpst_j_to_p, RoleP, 16 |
    send_mpst_j_to_q, RoleQ, 17 |
    send_mpst_j_to_r, RoleR, 18 |
    send_mpst_j_to_s, RoleS, 19 |
    send_mpst_j_to_t, RoleT, 20 | =>
    RoleJ, SessionMpstTwentyOne, 21
);
// K
create_send_check_cancel_bundle!(
    send_mpst_k_to_a, RoleA, 2 |
    send_mpst_k_to_b, RoleB, 3 |
    send_mpst_k_to_c, RoleC, 4 |
    send_mpst_k_to_d, RoleD, 5 |
    send_mpst_k_to_e, RoleE, 6 |
    send_mpst_k_to_f, RoleF, 7 |
    send_mpst_k_to_g, RoleG, 8 |
    send_mpst_k_to_h, RoleH, 9 |
    send_mpst_k_to_i, RoleI, 10 |
    send_mpst_k_to_j, RoleJ, 11 |
    send_mpst_k_to_l, RoleL, 12 |
    send_mpst_k_to_m, RoleM, 13 |
    send_mpst_k_to_n, RoleN, 14 |
    send_mpst_k_to_o, RoleO, 15 |
    send_mpst_k_to_p, RoleP, 16 |
    send_mpst_k_to_q, RoleQ, 17 |
    send_mpst_k_to_r, RoleR, 18 |
    send_mpst_k_to_s, RoleS, 19 |
    send_mpst_k_to_t, RoleT, 20 | =>
    RoleK, SessionMpstTwentyOne, 21
);
// L
create_send_check_cancel_bundle!(
    send_mpst_l_to_a, RoleA, 2 |
    send_mpst_l_to_b, RoleB, 3 |
    send_mpst_l_to_c, RoleC, 4 |
    send_mpst_l_to_d, RoleD, 5 |
    send_mpst_l_to_e, RoleE, 6 |
    send_mpst_l_to_f, RoleF, 7 |
    send_mpst_l_to_g, RoleG, 8 |
    send_mpst_l_to_h, RoleH, 9 |
    send_mpst_l_to_i, RoleI, 10 |
    send_mpst_l_to_j, RoleJ, 11 |
    send_mpst_l_to_k, RoleK, 12 |
    send_mpst_l_to_m, RoleM, 13 |
    send_mpst_l_to_n, RoleN, 14 |
    send_mpst_l_to_o, RoleO, 15 |
    send_mpst_l_to_p, RoleP, 16 |
    send_mpst_l_to_q, RoleQ, 17 |
    send_mpst_l_to_r, RoleR, 18 |
    send_mpst_l_to_s, RoleS, 19 |
    send_mpst_l_to_t, RoleT, 20 | =>
    RoleL, SessionMpstTwentyOne, 21
);
// M
create_send_check_cancel_bundle!(
    send_mpst_m_to_a, RoleA, 2 |
    send_mpst_m_to_b, RoleB, 3 |
    send_mpst_m_to_c, RoleC, 4 |
    send_mpst_m_to_d, RoleD, 5 |
    send_mpst_m_to_e, RoleE, 6 |
    send_mpst_m_to_f, RoleF, 7 |
    send_mpst_m_to_g, RoleG, 8 |
    send_mpst_m_to_h, RoleH, 9 |
    send_mpst_m_to_i, RoleI, 10 |
    send_mpst_m_to_j, RoleJ, 11 |
    send_mpst_m_to_k, RoleK, 12 |
    send_mpst_m_to_l, RoleL, 13 |
    send_mpst_m_to_n, RoleN, 14 |
    send_mpst_m_to_o, RoleO, 15 |
    send_mpst_m_to_p, RoleP, 16 |
    send_mpst_m_to_q, RoleQ, 17 |
    send_mpst_m_to_r, RoleR, 18 |
    send_mpst_m_to_s, RoleS, 19 |
    send_mpst_m_to_t, RoleT, 20 | =>
    RoleM, SessionMpstTwentyOne, 21
);
// N
create_send_check_cancel_bundle!(
    send_mpst_n_to_a, RoleA, 2 |
    send_mpst_n_to_b, RoleB, 3 |
    send_mpst_n_to_c, RoleC, 4 |
    send_mpst_n_to_d, RoleD, 5 |
    send_mpst_n_to_e, RoleE, 6 |
    send_mpst_n_to_f, RoleF, 7 |
    send_mpst_n_to_g, RoleG, 8 |
    send_mpst_n_to_h, RoleH, 9 |
    send_mpst_n_to_i, RoleI, 10 |
    send_mpst_n_to_j, RoleJ, 11 |
    send_mpst_n_to_k, RoleK, 12 |
    send_mpst_n_to_l, RoleL, 13 |
    send_mpst_n_to_m, RoleM, 14 |
    send_mpst_n_to_o, RoleO, 15 |
    send_mpst_n_to_p, RoleP, 16 |
    send_mpst_n_to_q, RoleQ, 17 |
    send_mpst_n_to_r, RoleR, 18 |
    send_mpst_n_to_s, RoleS, 19 |
    send_mpst_n_to_t, RoleT, 20 | =>
    RoleN, SessionMpstTwentyOne, 21
);
// O
create_send_check_cancel_bundle!(
    send_mpst_o_to_a, RoleA, 2 |
    send_mpst_o_to_b, RoleB, 3 |
    send_mpst_o_to_c, RoleC, 4 |
    send_mpst_o_to_d, RoleD, 5 |
    send_mpst_o_to_e, RoleE, 6 |
    send_mpst_o_to_f, RoleF, 7 |
    send_mpst_o_to_g, RoleG, 8 |
    send_mpst_o_to_h, RoleH, 9 |
    send_mpst_o_to_i, RoleI, 10 |
    send_mpst_o_to_j, RoleJ, 11 |
    send_mpst_o_to_k, RoleK, 12 |
    send_mpst_o_to_l, RoleL, 13 |
    send_mpst_o_to_m, RoleM, 14 |
    send_mpst_o_to_n, RoleN, 15 |
    send_mpst_o_to_p, RoleP, 16 |
    send_mpst_o_to_q, RoleQ, 17 |
    send_mpst_o_to_r, RoleR, 18 |
    send_mpst_o_to_s, RoleS, 19 |
    send_mpst_o_to_t, RoleT, 20 | =>
    RoleO, SessionMpstTwentyOne, 21
);
// P
create_send_check_cancel_bundle!(
    send_mpst_p_to_a, RoleA, 2 |
    send_mpst_p_to_b, RoleB, 3 |
    send_mpst_p_to_c, RoleC, 4 |
    send_mpst_p_to_d, RoleD, 5 |
    send_mpst_p_to_e, RoleE, 6 |
    send_mpst_p_to_f, RoleF, 7 |
    send_mpst_p_to_g, RoleG, 8 |
    send_mpst_p_to_h, RoleH, 9 |
    send_mpst_p_to_i, RoleI, 10 |
    send_mpst_p_to_j, RoleJ, 11 |
    send_mpst_p_to_k, RoleK, 12 |
    send_mpst_p_to_l, RoleL, 13 |
    send_mpst_p_to_m, RoleM, 14 |
    send_mpst_p_to_n, RoleN, 15 |
    send_mpst_p_to_o, RoleO, 16 |
    send_mpst_p_to_q, RoleQ, 17 |
    send_mpst_p_to_r, RoleR, 18 |
    send_mpst_p_to_s, RoleS, 19 |
    send_mpst_p_to_t, RoleT, 20 | =>
    RoleP, SessionMpstTwentyOne, 21
);
// Q
create_send_check_cancel_bundle!(
    send_mpst_q_to_a, RoleA, 2 |
    send_mpst_q_to_b, RoleB, 3 |
    send_mpst_q_to_c, RoleC, 4 |
    send_mpst_q_to_d, RoleD, 5 |
    send_mpst_q_to_e, RoleE, 6 |
    send_mpst_q_to_f, RoleF, 7 |
    send_mpst_q_to_g, RoleG, 8 |
    send_mpst_q_to_h, RoleH, 9 |
    send_mpst_q_to_i, RoleI, 10 |
    send_mpst_q_to_j, RoleJ, 11 |
    send_mpst_q_to_k, RoleK, 12 |
    send_mpst_q_to_l, RoleL, 13 |
    send_mpst_q_to_m, RoleM, 14 |
    send_mpst_q_to_n, RoleN, 15 |
    send_mpst_q_to_o, RoleO, 16 |
    send_mpst_q_to_p, RoleP, 17 |
    send_mpst_q_to_r, RoleR, 18 |
    send_mpst_q_to_s, RoleS, 19 |
    send_mpst_q_to_t, RoleT, 20 | =>
    RoleQ, SessionMpstTwentyOne, 21
);
// R
create_send_check_cancel_bundle!(
    send_mpst_r_to_a, RoleA, 2 |
    send_mpst_r_to_b, RoleB, 3 |
    send_mpst_r_to_c, RoleC, 4 |
    send_mpst_r_to_d, RoleD, 5 |
    send_mpst_r_to_e, RoleE, 6 |
    send_mpst_r_to_f, RoleF, 7 |
    send_mpst_r_to_g, RoleG, 8 |
    send_mpst_r_to_h, RoleH, 9 |
    send_mpst_r_to_i, RoleI, 10 |
    send_mpst_r_to_j, RoleJ, 11 |
    send_mpst_r_to_k, RoleK, 12 |
    send_mpst_r_to_l, RoleL, 13 |
    send_mpst_r_to_m, RoleM, 14 |
    send_mpst_r_to_n, RoleN, 15 |
    send_mpst_r_to_o, RoleO, 16 |
    send_mpst_r_to_p, RoleP, 17 |
    send_mpst_r_to_q, RoleQ, 18 |
    send_mpst_r_to_s, RoleS, 19 |
    send_mpst_r_to_t, RoleT, 20 | =>
    RoleR, SessionMpstTwentyOne, 21
);
// S
create_send_check_cancel_bundle!(
    send_mpst_s_to_a, RoleA, 2 |
    send_mpst_s_to_b, RoleB, 3 |
    send_mpst_s_to_c, RoleC, 4 |
    send_mpst_s_to_d, RoleD, 5 |
    send_mpst_s_to_e, RoleE, 6 |
    send_mpst_s_to_f, RoleF, 7 |
    send_mpst_s_to_g, RoleG, 8 |
    send_mpst_s_to_h, RoleH, 9 |
    send_mpst_s_to_i, RoleI, 10 |
    send_mpst_s_to_j, RoleJ, 11 |
    send_mpst_s_to_k, RoleK, 12 |
    send_mpst_s_to_l, RoleL, 13 |
    send_mpst_s_to_m, RoleM, 14 |
    send_mpst_s_to_n, RoleN, 15 |
    send_mpst_s_to_o, RoleO, 16 |
    send_mpst_s_to_p, RoleP, 17 |
    send_mpst_s_to_q, RoleQ, 18 |
    send_mpst_s_to_r, RoleR, 19 |
    send_mpst_s_to_t, RoleT, 20 | =>
    RoleS, SessionMpstTwentyOne, 21
);
// T
create_send_check_cancel_bundle!(
    send_mpst_t_to_a, RoleA, 2 |
    send_mpst_t_to_b, RoleB, 3 |
    send_mpst_t_to_c, RoleC, 4 |
    send_mpst_t_to_d, RoleD, 5 |
    send_mpst_t_to_e, RoleE, 6 |
    send_mpst_t_to_f, RoleF, 7 |
    send_mpst_t_to_g, RoleG, 8 |
    send_mpst_t_to_h, RoleH, 9 |
    send_mpst_t_to_i, RoleI, 10 |
    send_mpst_t_to_j, RoleJ, 11 |
    send_mpst_t_to_k, RoleK, 12 |
    send_mpst_t_to_l, RoleL, 13 |
    send_mpst_t_to_m, RoleM, 14 |
    send_mpst_t_to_n, RoleN, 15 |
    send_mpst_t_to_o, RoleO, 16 |
    send_mpst_t_to_p, RoleP, 17 |
    send_mpst_t_to_q, RoleQ, 18 |
    send_mpst_t_to_r, RoleR, 19 |
    send_mpst_t_to_s, RoleS, 20 | =>
    RoleT, SessionMpstTwentyOne, 21
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 2 |
    recv_mpst_a_from_c, RoleC, 3 |
    recv_mpst_a_from_d, RoleD, 4 |
    recv_mpst_a_from_e, RoleE, 5 |
    recv_mpst_a_from_f, RoleF, 6 |
    recv_mpst_a_from_g, RoleG, 7 |
    recv_mpst_a_from_h, RoleH, 8 |
    recv_mpst_a_from_i, RoleI, 9 |
    recv_mpst_a_from_j, RoleJ, 10 |
    recv_mpst_a_from_k, RoleK, 11 |
    recv_mpst_a_from_l, RoleL, 12 |
    recv_mpst_a_from_m, RoleM, 13 |
    recv_mpst_a_from_n, RoleN, 14 |
    recv_mpst_a_from_o, RoleO, 15 |
    recv_mpst_a_from_p, RoleP, 16 |
    recv_mpst_a_from_q, RoleQ, 17 |
    recv_mpst_a_from_r, RoleR, 18 |
    recv_mpst_a_from_s, RoleS, 19 |
    recv_mpst_a_from_t, RoleT, 20 | =>
    RoleA, SessionMpstTwentyOne, 21
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 2 |
    recv_mpst_b_from_c, RoleC, 3 |
    recv_mpst_b_from_d, RoleD, 4 |
    recv_mpst_b_from_e, RoleE, 5 |
    recv_mpst_b_from_f, RoleF, 6 |
    recv_mpst_b_from_g, RoleG, 7 |
    recv_mpst_b_from_h, RoleH, 8 |
    recv_mpst_b_from_i, RoleI, 9 |
    recv_mpst_b_from_j, RoleJ, 10 |
    recv_mpst_b_from_k, RoleK, 11 |
    recv_mpst_b_from_l, RoleL, 12 |
    recv_mpst_b_from_m, RoleM, 13 |
    recv_mpst_b_from_n, RoleN, 14 |
    recv_mpst_b_from_o, RoleO, 15 |
    recv_mpst_b_from_p, RoleP, 16 |
    recv_mpst_b_from_q, RoleQ, 17 |
    recv_mpst_b_from_r, RoleR, 18 |
    recv_mpst_b_from_s, RoleS, 19 |
    recv_mpst_b_from_t, RoleT, 20 | =>
    RoleB, SessionMpstTwentyOne, 21
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a, RoleA, 2 |
    recv_mpst_c_from_b, RoleB, 3 |
    recv_mpst_c_from_d, RoleD, 4 |
    recv_mpst_c_from_e, RoleE, 5 |
    recv_mpst_c_from_f, RoleF, 6 |
    recv_mpst_c_from_g, RoleG, 7 |
    recv_mpst_c_from_h, RoleH, 8 |
    recv_mpst_c_from_i, RoleI, 9 |
    recv_mpst_c_from_j, RoleJ, 10 |
    recv_mpst_c_from_k, RoleK, 11 |
    recv_mpst_c_from_l, RoleL, 12 |
    recv_mpst_c_from_m, RoleM, 13 |
    recv_mpst_c_from_n, RoleN, 14 |
    recv_mpst_c_from_o, RoleO, 15 |
    recv_mpst_c_from_p, RoleP, 16 |
    recv_mpst_c_from_q, RoleQ, 17 |
    recv_mpst_c_from_r, RoleR, 18 |
    recv_mpst_c_from_s, RoleS, 19 |
    recv_mpst_c_from_t, RoleT, 20 | =>
    RoleC, SessionMpstTwentyOne, 21
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_a, RoleA, 2 |
    recv_mpst_d_from_b, RoleB, 3 |
    recv_mpst_d_from_c, RoleC, 4 |
    recv_mpst_d_from_e, RoleE, 5 |
    recv_mpst_d_from_f, RoleF, 6 |
    recv_mpst_d_from_g, RoleG, 7 |
    recv_mpst_d_from_h, RoleH, 8 |
    recv_mpst_d_from_i, RoleI, 9 |
    recv_mpst_d_from_j, RoleJ, 10 |
    recv_mpst_d_from_k, RoleK, 11 |
    recv_mpst_d_from_l, RoleL, 12 |
    recv_mpst_d_from_m, RoleM, 13 |
    recv_mpst_d_from_n, RoleN, 14 |
    recv_mpst_d_from_o, RoleO, 15 |
    recv_mpst_d_from_p, RoleP, 16 |
    recv_mpst_d_from_q, RoleQ, 17 |
    recv_mpst_d_from_r, RoleR, 18 |
    recv_mpst_d_from_s, RoleS, 19 |
    recv_mpst_d_from_t, RoleT, 20 | =>
    RoleD, SessionMpstTwentyOne, 21
);
// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_a, RoleA, 2 |
    recv_mpst_e_from_b, RoleB, 3 |
    recv_mpst_e_from_c, RoleC, 4 |
    recv_mpst_e_from_d, RoleD, 5 |
    recv_mpst_e_from_f, RoleF, 6 |
    recv_mpst_e_from_g, RoleG, 7 |
    recv_mpst_e_from_h, RoleH, 8 |
    recv_mpst_e_from_i, RoleI, 9 |
    recv_mpst_e_from_j, RoleJ, 10 |
    recv_mpst_e_from_k, RoleK, 11 |
    recv_mpst_e_from_l, RoleL, 12 |
    recv_mpst_e_from_m, RoleM, 13 |
    recv_mpst_e_from_n, RoleN, 14 |
    recv_mpst_e_from_o, RoleO, 15 |
    recv_mpst_e_from_p, RoleP, 16 |
    recv_mpst_e_from_q, RoleQ, 17 |
    recv_mpst_e_from_r, RoleR, 18 |
    recv_mpst_e_from_s, RoleS, 19 |
    recv_mpst_e_from_t, RoleT, 20 | =>
    RoleE, SessionMpstTwentyOne, 21
);
// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_from_a, RoleA, 2 |
    recv_mpst_f_from_b, RoleB, 3 |
    recv_mpst_f_from_c, RoleC, 4 |
    recv_mpst_f_from_d, RoleD, 5 |
    recv_mpst_f_from_e, RoleE, 6 |
    recv_mpst_f_from_g, RoleG, 7 |
    recv_mpst_f_from_h, RoleH, 8 |
    recv_mpst_f_from_i, RoleI, 9 |
    recv_mpst_f_from_j, RoleJ, 10 |
    recv_mpst_f_from_k, RoleK, 11 |
    recv_mpst_f_from_l, RoleL, 12 |
    recv_mpst_f_from_m, RoleM, 13 |
    recv_mpst_f_from_n, RoleN, 14 |
    recv_mpst_f_from_o, RoleO, 15 |
    recv_mpst_f_from_p, RoleP, 16 |
    recv_mpst_f_from_q, RoleQ, 17 |
    recv_mpst_f_from_r, RoleR, 18 |
    recv_mpst_f_from_s, RoleS, 19 |
    recv_mpst_f_from_t, RoleT, 20 | =>
    RoleF, SessionMpstTwentyOne, 21
);
// G
create_recv_mpst_session_bundle!(
    recv_mpst_g_from_a, RoleA, 2 |
    recv_mpst_g_from_b, RoleB, 3 |
    recv_mpst_g_from_c, RoleC, 4 |
    recv_mpst_g_from_d, RoleD, 5 |
    recv_mpst_g_from_e, RoleE, 6 |
    recv_mpst_g_from_f, RoleF, 7 |
    recv_mpst_g_from_h, RoleH, 8 |
    recv_mpst_g_from_i, RoleI, 9 |
    recv_mpst_g_from_j, RoleJ, 10 |
    recv_mpst_g_from_k, RoleK, 11 |
    recv_mpst_g_from_l, RoleL, 12 |
    recv_mpst_g_from_m, RoleM, 13 |
    recv_mpst_g_from_n, RoleN, 14 |
    recv_mpst_g_from_o, RoleO, 15 |
    recv_mpst_g_from_p, RoleP, 16 |
    recv_mpst_g_from_q, RoleQ, 17 |
    recv_mpst_g_from_r, RoleR, 18 |
    recv_mpst_g_from_s, RoleS, 19 |
    recv_mpst_g_from_t, RoleT, 20 | =>
    RoleG, SessionMpstTwentyOne, 21
);
// H
create_recv_mpst_session_bundle!(
    recv_mpst_h_from_a, RoleA, 2 |
    recv_mpst_h_from_b, RoleB, 3 |
    recv_mpst_h_from_c, RoleC, 4 |
    recv_mpst_h_from_d, RoleD, 5 |
    recv_mpst_h_from_e, RoleE, 6 |
    recv_mpst_h_from_f, RoleF, 7 |
    recv_mpst_h_from_g, RoleG, 8 |
    recv_mpst_h_from_i, RoleI, 9 |
    recv_mpst_h_from_j, RoleJ, 10 |
    recv_mpst_h_from_k, RoleK, 11 |
    recv_mpst_h_from_l, RoleL, 12 |
    recv_mpst_h_from_m, RoleM, 13 |
    recv_mpst_h_from_n, RoleN, 14 |
    recv_mpst_h_from_o, RoleO, 15 |
    recv_mpst_h_from_p, RoleP, 16 |
    recv_mpst_h_from_q, RoleQ, 17 |
    recv_mpst_h_from_r, RoleR, 18 |
    recv_mpst_h_from_s, RoleS, 19 |
    recv_mpst_h_from_t, RoleT, 20 | =>
    RoleH, SessionMpstTwentyOne, 21
);
// I
create_recv_mpst_session_bundle!(
    recv_mpst_i_from_a, RoleA, 2 |
    recv_mpst_i_from_b, RoleB, 3 |
    recv_mpst_i_from_c, RoleC, 4 |
    recv_mpst_i_from_d, RoleD, 5 |
    recv_mpst_i_from_e, RoleE, 6 |
    recv_mpst_i_from_f, RoleF, 7 |
    recv_mpst_i_from_g, RoleG, 8 |
    recv_mpst_i_from_h, RoleH, 9 |
    recv_mpst_i_from_j, RoleJ, 10 |
    recv_mpst_i_from_k, RoleK, 11 |
    recv_mpst_i_from_l, RoleL, 12 |
    recv_mpst_i_from_m, RoleM, 13 |
    recv_mpst_i_from_n, RoleN, 14 |
    recv_mpst_i_from_o, RoleO, 15 |
    recv_mpst_i_from_p, RoleP, 16 |
    recv_mpst_i_from_q, RoleQ, 17 |
    recv_mpst_i_from_r, RoleR, 18 |
    recv_mpst_i_from_s, RoleS, 19 |
    recv_mpst_i_from_t, RoleT, 20 | =>
    RoleI, SessionMpstTwentyOne, 21
);
// J
create_recv_mpst_session_bundle!(
    recv_mpst_j_from_a, RoleA, 2 |
    recv_mpst_j_from_b, RoleB, 3 |
    recv_mpst_j_from_c, RoleC, 4 |
    recv_mpst_j_from_d, RoleD, 5 |
    recv_mpst_j_from_e, RoleE, 6 |
    recv_mpst_j_from_f, RoleF, 7 |
    recv_mpst_j_from_g, RoleG, 8 |
    recv_mpst_j_from_h, RoleH, 9 |
    recv_mpst_j_from_i, RoleI, 10 |
    recv_mpst_j_from_k, RoleK, 11 |
    recv_mpst_j_from_l, RoleL, 12 |
    recv_mpst_j_from_m, RoleM, 13 |
    recv_mpst_j_from_n, RoleN, 14 |
    recv_mpst_j_from_o, RoleO, 15 |
    recv_mpst_j_from_p, RoleP, 16 |
    recv_mpst_j_from_q, RoleQ, 17 |
    recv_mpst_j_from_r, RoleR, 18 |
    recv_mpst_j_from_s, RoleS, 19 |
    recv_mpst_j_from_t, RoleT, 20 | =>
    RoleJ, SessionMpstTwentyOne, 21
);
// K
create_recv_mpst_session_bundle!(
    recv_mpst_k_from_a, RoleA, 2 |
    recv_mpst_k_from_b, RoleB, 3 |
    recv_mpst_k_from_c, RoleC, 4 |
    recv_mpst_k_from_d, RoleD, 5 |
    recv_mpst_k_from_e, RoleE, 6 |
    recv_mpst_k_from_f, RoleF, 7 |
    recv_mpst_k_from_g, RoleG, 8 |
    recv_mpst_k_from_h, RoleH, 9 |
    recv_mpst_k_from_i, RoleI, 10 |
    recv_mpst_k_from_j, RoleJ, 11 |
    recv_mpst_k_from_l, RoleL, 12 |
    recv_mpst_k_from_m, RoleM, 13 |
    recv_mpst_k_from_n, RoleN, 14 |
    recv_mpst_k_from_o, RoleO, 15 |
    recv_mpst_k_from_p, RoleP, 16 |
    recv_mpst_k_from_q, RoleQ, 17 |
    recv_mpst_k_from_r, RoleR, 18 |
    recv_mpst_k_from_s, RoleS, 19 |
    recv_mpst_k_from_t, RoleT, 20 | =>
    RoleK, SessionMpstTwentyOne, 21
);
// L
create_recv_mpst_session_bundle!(
    recv_mpst_l_from_a, RoleA, 2 |
    recv_mpst_l_from_b, RoleB, 3 |
    recv_mpst_l_from_c, RoleC, 4 |
    recv_mpst_l_from_d, RoleD, 5 |
    recv_mpst_l_from_e, RoleE, 6 |
    recv_mpst_l_from_f, RoleF, 7 |
    recv_mpst_l_from_g, RoleG, 8 |
    recv_mpst_l_from_h, RoleH, 9 |
    recv_mpst_l_from_i, RoleI, 10 |
    recv_mpst_l_from_j, RoleJ, 11 |
    recv_mpst_l_from_k, RoleK, 12 |
    recv_mpst_l_from_m, RoleM, 13 |
    recv_mpst_l_from_n, RoleN, 14 |
    recv_mpst_l_from_o, RoleO, 15 |
    recv_mpst_l_from_p, RoleP, 16 |
    recv_mpst_l_from_q, RoleQ, 17 |
    recv_mpst_l_from_r, RoleR, 18 |
    recv_mpst_l_from_s, RoleS, 19 |
    recv_mpst_l_from_t, RoleT, 20 | =>
    RoleL, SessionMpstTwentyOne, 21
);
// M
create_recv_mpst_session_bundle!(
    recv_mpst_m_from_a, RoleA, 2 |
    recv_mpst_m_from_b, RoleB, 3 |
    recv_mpst_m_from_c, RoleC, 4 |
    recv_mpst_m_from_d, RoleD, 5 |
    recv_mpst_m_from_e, RoleE, 6 |
    recv_mpst_m_from_f, RoleF, 7 |
    recv_mpst_m_from_g, RoleG, 8 |
    recv_mpst_m_from_h, RoleH, 9 |
    recv_mpst_m_from_i, RoleI, 10 |
    recv_mpst_m_from_j, RoleJ, 11 |
    recv_mpst_m_from_k, RoleK, 12 |
    recv_mpst_m_from_l, RoleL, 13 |
    recv_mpst_m_from_n, RoleN, 14 |
    recv_mpst_m_from_o, RoleO, 15 |
    recv_mpst_m_from_p, RoleP, 16 |
    recv_mpst_m_from_q, RoleQ, 17 |
    recv_mpst_m_from_r, RoleR, 18 |
    recv_mpst_m_from_s, RoleS, 19 |
    recv_mpst_m_from_t, RoleT, 20 | =>
    RoleM, SessionMpstTwentyOne, 21
);
// N
create_recv_mpst_session_bundle!(
    recv_mpst_n_from_a, RoleA, 2 |
    recv_mpst_n_from_b, RoleB, 3 |
    recv_mpst_n_from_c, RoleC, 4 |
    recv_mpst_n_from_d, RoleD, 5 |
    recv_mpst_n_from_e, RoleE, 6 |
    recv_mpst_n_from_f, RoleF, 7 |
    recv_mpst_n_from_g, RoleG, 8 |
    recv_mpst_n_from_h, RoleH, 9 |
    recv_mpst_n_from_i, RoleI, 10 |
    recv_mpst_n_from_j, RoleJ, 11 |
    recv_mpst_n_from_k, RoleK, 12 |
    recv_mpst_n_from_l, RoleL, 13 |
    recv_mpst_n_from_m, RoleM, 14 |
    recv_mpst_n_from_o, RoleO, 15 |
    recv_mpst_n_from_p, RoleP, 16 |
    recv_mpst_n_from_q, RoleQ, 17 |
    recv_mpst_n_from_r, RoleR, 18 |
    recv_mpst_n_from_s, RoleS, 19 |
    recv_mpst_n_from_t, RoleT, 20 | =>
    RoleN, SessionMpstTwentyOne, 21
);
// O
create_recv_mpst_session_bundle!(
    recv_mpst_o_from_a, RoleA, 2 |
    recv_mpst_o_from_b, RoleB, 3 |
    recv_mpst_o_from_c, RoleC, 4 |
    recv_mpst_o_from_d, RoleD, 5 |
    recv_mpst_o_from_e, RoleE, 6 |
    recv_mpst_o_from_f, RoleF, 7 |
    recv_mpst_o_from_g, RoleG, 8 |
    recv_mpst_o_from_h, RoleH, 9 |
    recv_mpst_o_from_i, RoleI, 10 |
    recv_mpst_o_from_j, RoleJ, 11 |
    recv_mpst_o_from_k, RoleK, 12 |
    recv_mpst_o_from_l, RoleL, 13 |
    recv_mpst_o_from_m, RoleM, 14 |
    recv_mpst_o_from_n, RoleN, 15 |
    recv_mpst_o_from_p, RoleP, 16 |
    recv_mpst_o_from_q, RoleQ, 17 |
    recv_mpst_o_from_r, RoleR, 18 |
    recv_mpst_o_from_s, RoleS, 19 |
    recv_mpst_o_from_t, RoleT, 20 | =>
    RoleO, SessionMpstTwentyOne, 21
);
// P
create_recv_mpst_session_bundle!(
    recv_mpst_p_from_a, RoleA, 2 |
    recv_mpst_p_from_b, RoleB, 3 |
    recv_mpst_p_from_c, RoleC, 4 |
    recv_mpst_p_from_d, RoleD, 5 |
    recv_mpst_p_from_e, RoleE, 6 |
    recv_mpst_p_from_f, RoleF, 7 |
    recv_mpst_p_from_g, RoleG, 8 |
    recv_mpst_p_from_h, RoleH, 9 |
    recv_mpst_p_from_i, RoleI, 10 |
    recv_mpst_p_from_j, RoleJ, 11 |
    recv_mpst_p_from_k, RoleK, 12 |
    recv_mpst_p_from_l, RoleL, 13 |
    recv_mpst_p_from_m, RoleM, 14 |
    recv_mpst_p_from_n, RoleN, 15 |
    recv_mpst_p_from_o, RoleO, 16 |
    recv_mpst_p_from_q, RoleQ, 17 |
    recv_mpst_p_from_r, RoleR, 18 |
    recv_mpst_p_from_s, RoleS, 19 |
    recv_mpst_p_from_t, RoleT, 20 | =>
    RoleP, SessionMpstTwentyOne, 21
);
// Q
create_recv_mpst_session_bundle!(
    recv_mpst_q_from_a, RoleA, 2 |
    recv_mpst_q_from_b, RoleB, 3 |
    recv_mpst_q_from_c, RoleC, 4 |
    recv_mpst_q_from_d, RoleD, 5 |
    recv_mpst_q_from_e, RoleE, 6 |
    recv_mpst_q_from_f, RoleF, 7 |
    recv_mpst_q_from_g, RoleG, 8 |
    recv_mpst_q_from_h, RoleH, 9 |
    recv_mpst_q_from_i, RoleI, 10 |
    recv_mpst_q_from_j, RoleJ, 11 |
    recv_mpst_q_from_k, RoleK, 12 |
    recv_mpst_q_from_l, RoleL, 13 |
    recv_mpst_q_from_m, RoleM, 14 |
    recv_mpst_q_from_n, RoleN, 15 |
    recv_mpst_q_from_o, RoleO, 16 |
    recv_mpst_q_from_p, RoleP, 17 |
    recv_mpst_q_from_r, RoleR, 18 |
    recv_mpst_q_from_s, RoleS, 19 |
    recv_mpst_q_from_t, RoleT, 20 | =>
    RoleQ, SessionMpstTwentyOne, 21
);
// R
create_recv_mpst_session_bundle!(
    recv_mpst_r_from_a, RoleA, 2 |
    recv_mpst_r_from_b, RoleB, 3 |
    recv_mpst_r_from_c, RoleC, 4 |
    recv_mpst_r_from_d, RoleD, 5 |
    recv_mpst_r_from_e, RoleE, 6 |
    recv_mpst_r_from_f, RoleF, 7 |
    recv_mpst_r_from_g, RoleG, 8 |
    recv_mpst_r_from_h, RoleH, 9 |
    recv_mpst_r_from_i, RoleI, 10 |
    recv_mpst_r_from_j, RoleJ, 11 |
    recv_mpst_r_from_k, RoleK, 12 |
    recv_mpst_r_from_l, RoleL, 13 |
    recv_mpst_r_from_m, RoleM, 14 |
    recv_mpst_r_from_n, RoleN, 15 |
    recv_mpst_r_from_o, RoleO, 16 |
    recv_mpst_r_from_p, RoleP, 17 |
    recv_mpst_r_from_q, RoleQ, 18 |
    recv_mpst_r_from_s, RoleS, 19 |
    recv_mpst_r_from_t, RoleT, 20 | =>
    RoleR, SessionMpstTwentyOne, 21
);
// S
create_recv_mpst_session_bundle!(
    recv_mpst_s_from_a, RoleA, 2 |
    recv_mpst_s_from_b, RoleB, 3 |
    recv_mpst_s_from_c, RoleC, 4 |
    recv_mpst_s_from_d, RoleD, 5 |
    recv_mpst_s_from_e, RoleE, 6 |
    recv_mpst_s_from_f, RoleF, 7 |
    recv_mpst_s_from_g, RoleG, 8 |
    recv_mpst_s_from_h, RoleH, 9 |
    recv_mpst_s_from_i, RoleI, 10 |
    recv_mpst_s_from_j, RoleJ, 11 |
    recv_mpst_s_from_k, RoleK, 12 |
    recv_mpst_s_from_l, RoleL, 13 |
    recv_mpst_s_from_m, RoleM, 14 |
    recv_mpst_s_from_n, RoleN, 15 |
    recv_mpst_s_from_o, RoleO, 16 |
    recv_mpst_s_from_p, RoleP, 17 |
    recv_mpst_s_from_q, RoleQ, 18 |
    recv_mpst_s_from_r, RoleR, 19 |
    recv_mpst_s_from_t, RoleT, 20 | =>
    RoleS, SessionMpstTwentyOne, 21
);
// T
create_recv_mpst_session_bundle!(
    recv_mpst_t_from_a, RoleA, 2 |
    recv_mpst_t_from_b, RoleB, 3 |
    recv_mpst_t_from_c, RoleC, 4 |
    recv_mpst_t_from_d, RoleD, 5 |
    recv_mpst_t_from_e, RoleE, 6 |
    recv_mpst_t_from_f, RoleF, 7 |
    recv_mpst_t_from_g, RoleG, 8 |
    recv_mpst_t_from_h, RoleH, 9 |
    recv_mpst_t_from_i, RoleI, 10 |
    recv_mpst_t_from_j, RoleJ, 11 |
    recv_mpst_t_from_k, RoleK, 12 |
    recv_mpst_t_from_l, RoleL, 13 |
    recv_mpst_t_from_m, RoleM, 14 |
    recv_mpst_t_from_n, RoleN, 15 |
    recv_mpst_t_from_o, RoleO, 16 |
    recv_mpst_t_from_p, RoleP, 17 |
    recv_mpst_t_from_q, RoleQ, 18 |
    recv_mpst_t_from_r, RoleR, 19 |
    recv_mpst_t_from_s, RoleS, 20 | =>
    RoleT, SessionMpstTwentyOne, 21
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
enum Branching0fromTtoA {
    More(
        SessionMpstTwentyOne<
            End,
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
        SessionMpstTwentyOne<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursAtoT = Recv<(End, Branching0fromTtoA), End>;
// B
enum Branching0fromTtoB {
    More(
        SessionMpstTwentyOne<
            End,
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
        SessionMpstTwentyOne<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursBtoT = Recv<(End, Branching0fromTtoB), End>;
// C
enum Branching0fromTtoC {
    More(
        SessionMpstTwentyOne<
            End,
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
        SessionMpstTwentyOne<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursCtoT = Recv<(End, Branching0fromTtoC), End>;
// D
enum Branching0fromTtoD {
    More(
        SessionMpstTwentyOne<
            End,
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
        SessionMpstTwentyOne<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursDtoT = Recv<(End, Branching0fromTtoD), End>;
// E
enum Branching0fromTtoE {
    More(
        SessionMpstTwentyOne<
            End,
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
        SessionMpstTwentyOne<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursEtoT = Recv<(End, Branching0fromTtoE), End>;
// F
enum Branching0fromTtoF {
    More(
        SessionMpstTwentyOne<
            End,
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
        SessionMpstTwentyOne<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursFtoT = Recv<(End, Branching0fromTtoF), End>;
// G
enum Branching0fromTtoG {
    More(
        SessionMpstTwentyOne<
            End,
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
        SessionMpstTwentyOne<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursGtoT = Recv<(End, Branching0fromTtoG), End>;
// H
enum Branching0fromTtoH {
    More(
        SessionMpstTwentyOne<
            End,
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
        SessionMpstTwentyOne<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursHtoT = Recv<(End, Branching0fromTtoH), End>;
// I
enum Branching0fromTtoI {
    More(
        SessionMpstTwentyOne<
            End,
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
        SessionMpstTwentyOne<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursItoT = Recv<(End, Branching0fromTtoI), End>;
// J
enum Branching0fromTtoJ {
    More(
        SessionMpstTwentyOne<
            End,
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
        SessionMpstTwentyOne<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursJtoT = Recv<(End, Branching0fromTtoJ), End>;
// K
enum Branching0fromTtoK {
    More(
        SessionMpstTwentyOne<
            End,
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
        SessionMpstTwentyOne<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursKtoT = Recv<(End, Branching0fromTtoK), End>;
// L
enum Branching0fromTtoL {
    More(
        SessionMpstTwentyOne<
            End,
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
        SessionMpstTwentyOne<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursLtoT = Recv<(End, Branching0fromTtoL), End>;
// M
enum Branching0fromTtoM {
    More(
        SessionMpstTwentyOne<
            End,
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
        SessionMpstTwentyOne<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursMtoT = Recv<(End, Branching0fromTtoM), End>;
// N
enum Branching0fromTtoN {
    More(
        SessionMpstTwentyOne<
            End,
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
        SessionMpstTwentyOne<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursNtoT = Recv<(End, Branching0fromTtoN), End>;
// O
enum Branching0fromTtoO {
    More(
        SessionMpstTwentyOne<
            End,
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
        SessionMpstTwentyOne<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursOtoT = Recv<(End, Branching0fromTtoO), End>;
// P
enum Branching0fromTtoP {
    More(
        SessionMpstTwentyOne<
            End,
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
        SessionMpstTwentyOne<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursPtoT = Recv<(End, Branching0fromTtoP), End>;
// Q
enum Branching0fromTtoQ {
    More(
        SessionMpstTwentyOne<
            End,
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
        SessionMpstTwentyOne<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursQtoT = Recv<(End, Branching0fromTtoQ), End>;
// R
enum Branching0fromTtoR {
    More(
        SessionMpstTwentyOne<
            End,
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
        SessionMpstTwentyOne<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursRtoT = Recv<(End, Branching0fromTtoR), End>;
// S
enum Branching0fromTtoS {
    More(
        SessionMpstTwentyOne<
            End,
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
        SessionMpstTwentyOne<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type RecursStoT = Recv<(End, Branching0fromTtoS), End>;
// T
type Choose0fromTtoA = <RecursAtoT as Session>::Dual;
type Choose0fromTtoB = <RecursBtoT as Session>::Dual;
type Choose0fromTtoC = <RecursCtoT as Session>::Dual;
type Choose0fromTtoD = <RecursDtoT as Session>::Dual;
type Choose0fromTtoE = <RecursEtoT as Session>::Dual;
type Choose0fromTtoF = <RecursFtoT as Session>::Dual;
type Choose0fromTtoG = <RecursGtoT as Session>::Dual;
type Choose0fromTtoH = <RecursHtoT as Session>::Dual;
type Choose0fromTtoI = <RecursItoT as Session>::Dual;
type Choose0fromTtoJ = <RecursJtoT as Session>::Dual;
type Choose0fromTtoK = <RecursKtoT as Session>::Dual;
type Choose0fromTtoL = <RecursLtoT as Session>::Dual;
type Choose0fromTtoM = <RecursMtoT as Session>::Dual;
type Choose0fromTtoN = <RecursNtoT as Session>::Dual;
type Choose0fromTtoO = <RecursOtoT as Session>::Dual;
type Choose0fromTtoP = <RecursPtoT as Session>::Dual;
type Choose0fromTtoQ = <RecursQtoT as Session>::Dual;
type Choose0fromTtoR = <RecursRtoT as Session>::Dual;
type Choose0fromTtoS = <RecursStoT as Session>::Dual;
type EndpointDoneT = SessionMpstTwentyOne<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointMoreT = SessionMpstTwentyOne<
    End,
    Send<(), Recv<(), Choose0fromTtoA>>,
    Send<(), Recv<(), Choose0fromTtoB>>,
    Send<(), Recv<(), Choose0fromTtoC>>,
    Send<(), Recv<(), Choose0fromTtoD>>,
    Send<(), Recv<(), Choose0fromTtoE>>,
    Send<(), Recv<(), Choose0fromTtoF>>,
    Send<(), Recv<(), Choose0fromTtoG>>,
    Send<(), Recv<(), Choose0fromTtoH>>,
    Send<(), Recv<(), Choose0fromTtoI>>,
    Send<(), Recv<(), Choose0fromTtoJ>>,
    Send<(), Recv<(), Choose0fromTtoK>>,
    Send<(), Recv<(), Choose0fromTtoL>>,
    Send<(), Recv<(), Choose0fromTtoM>>,
    Send<(), Recv<(), Choose0fromTtoN>>,
    Send<(), Recv<(), Choose0fromTtoO>>,
    Send<(), Recv<(), Choose0fromTtoP>>,
    Send<(), Recv<(), Choose0fromTtoQ>>,
    Send<(), Recv<(), Choose0fromTtoR>>,
    Send<(), Recv<(), Choose0fromTtoS>>,
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
                                                        R2N<R2O<R2P<R2Q<R2R<R2S<RoleBroadcast>>>>>>,
                                                    >,
                                                >,
                                            >,
                                        >,
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

// Creating the MP sessions
type EndpointCentral = SessionMpstTwentyOne<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointA = SessionMpstTwentyOne<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointB = SessionMpstTwentyOne<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointC = SessionMpstTwentyOne<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointD = SessionMpstTwentyOne<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointE = SessionMpstTwentyOne<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointF = SessionMpstTwentyOne<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointG = SessionMpstTwentyOne<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointH = SessionMpstTwentyOne<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointI = SessionMpstTwentyOne<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointJ = SessionMpstTwentyOne<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointK = SessionMpstTwentyOne<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointL = SessionMpstTwentyOne<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointM = SessionMpstTwentyOne<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointN = SessionMpstTwentyOne<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointO = SessionMpstTwentyOne<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointP = SessionMpstTwentyOne<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointQ = SessionMpstTwentyOne<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointR = SessionMpstTwentyOne<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointS = SessionMpstTwentyOne<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointT = SessionMpstTwentyOne<
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
    done_from_t_to_all, more_from_t_to_all, =>
    Done, More, =>
    EndpointDoneT, EndpointMoreT, =>
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
    RoleCentral, RoleT, SessionMpstTwentyOne, 21, 21
);

fn endpoint_central(s: EndpointCentral) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, 21);
    Ok(())
}

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_a_from_t, {
        Branching0fromTtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoA::More(s) => {
            let (_, s) = recv_mpst_a_from_t(s)?;
            let s = send_mpst_a_to_t((), s)?;
            let (_, s) = recv_mpst_a_from_b(s)?;
            let s = send_mpst_a_to_b((), s)?;
            let (_, s) = recv_mpst_a_from_c(s)?;
            let s = send_mpst_a_to_c((), s)?;
            let (_, s) = recv_mpst_a_from_d(s)?;
            let s = send_mpst_a_to_d((), s)?;
            let (_, s) = recv_mpst_a_from_e(s)?;
            let s = send_mpst_a_to_e((), s)?;
            let (_, s) = recv_mpst_a_from_f(s)?;
            let s = send_mpst_a_to_f((), s)?;
            let (_, s) = recv_mpst_a_from_g(s)?;
            let s = send_mpst_a_to_g((), s)?;
            let (_, s) = recv_mpst_a_from_h(s)?;
            let s = send_mpst_a_to_h((), s)?;
            let (_, s) = recv_mpst_a_from_i(s)?;
            let s = send_mpst_a_to_i((), s)?;
            let (_, s) = recv_mpst_a_from_j(s)?;
            let s = send_mpst_a_to_j((), s)?;
            let (_, s) = recv_mpst_a_from_k(s)?;
            let s = send_mpst_a_to_k((), s)?;
            let (_, s) = recv_mpst_a_from_l(s)?;
            let s = send_mpst_a_to_l((), s)?;
            let (_, s) = recv_mpst_a_from_m(s)?;
            let s = send_mpst_a_to_m((), s)?;
            let (_, s) = recv_mpst_a_from_n(s)?;
            let s = send_mpst_a_to_n((), s)?;
            let (_, s) = recv_mpst_a_from_o(s)?;
            let s = send_mpst_a_to_o((), s)?;
            let (_, s) = recv_mpst_a_from_p(s)?;
            let s = send_mpst_a_to_p((), s)?;
            let (_, s) = recv_mpst_a_from_q(s)?;
            let s = send_mpst_a_to_q((), s)?;
            let (_, s) = recv_mpst_a_from_r(s)?;
            let s = send_mpst_a_to_r((), s)?;
            let (_, s) = recv_mpst_a_from_s(s)?;
            let s = send_mpst_a_to_s((), s)?;
            endpoint_a(s)
        }, })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_b_from_t, {
        Branching0fromTtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoB::More(s) => {
            let (_, s) = recv_mpst_b_from_t(s)?;
            let s = send_mpst_b_to_t((), s)?;
            let s = send_mpst_b_to_a((), s)?;
            let (_, s) = recv_mpst_b_from_a(s)?;
            let (_, s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_c((), s)?;
            let (_, s) = recv_mpst_b_from_d(s)?;
            let s = send_mpst_b_to_d((), s)?;
            let (_, s) = recv_mpst_b_from_e(s)?;
            let s = send_mpst_b_to_e((), s)?;
            let (_, s) = recv_mpst_b_from_f(s)?;
            let s = send_mpst_b_to_f((), s)?;
            let (_, s) = recv_mpst_b_from_g(s)?;
            let s = send_mpst_b_to_g((), s)?;
            let (_, s) = recv_mpst_b_from_h(s)?;
            let s = send_mpst_b_to_h((), s)?;
            let (_, s) = recv_mpst_b_from_i(s)?;
            let s = send_mpst_b_to_i((), s)?;
            let (_, s) = recv_mpst_b_from_j(s)?;
            let s = send_mpst_b_to_j((), s)?;
            let (_, s) = recv_mpst_b_from_k(s)?;
            let s = send_mpst_b_to_k((), s)?;
            let (_, s) = recv_mpst_b_from_l(s)?;
            let s = send_mpst_b_to_l((), s)?;
            let (_, s) = recv_mpst_b_from_m(s)?;
            let s = send_mpst_b_to_m((), s)?;
            let (_, s) = recv_mpst_b_from_n(s)?;
            let s = send_mpst_b_to_n((), s)?;
            let (_, s) = recv_mpst_b_from_o(s)?;
            let s = send_mpst_b_to_o((), s)?;
            let (_, s) = recv_mpst_b_from_p(s)?;
            let s = send_mpst_b_to_p((), s)?;
            let (_, s) = recv_mpst_b_from_q(s)?;
            let s = send_mpst_b_to_q((), s)?;
            let (_, s) = recv_mpst_b_from_r(s)?;
            let s = send_mpst_b_to_r((), s)?;
            let (_, s) = recv_mpst_b_from_s(s)?;
            let s = send_mpst_b_to_s((), s)?;
            endpoint_b(s)
        }, })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_c_from_t, {
        Branching0fromTtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoC::More(s) => {
            let (_, s) = recv_mpst_c_from_t(s)?;
            let s = send_mpst_c_to_t((), s)?;
            let s = send_mpst_c_to_a((), s)?;
            let (_, s) = recv_mpst_c_from_a(s)?;
            let s = send_mpst_c_to_b((), s)?;
            let (_, s) = recv_mpst_c_from_b(s)?;
            let (_, s) = recv_mpst_c_from_d(s)?;
            let s = send_mpst_c_to_d((), s)?;
            let (_, s) = recv_mpst_c_from_e(s)?;
            let s = send_mpst_c_to_e((), s)?;
            let (_, s) = recv_mpst_c_from_f(s)?;
            let s = send_mpst_c_to_f((), s)?;
            let (_, s) = recv_mpst_c_from_g(s)?;
            let s = send_mpst_c_to_g((), s)?;
            let (_, s) = recv_mpst_c_from_h(s)?;
            let s = send_mpst_c_to_h((), s)?;
            let (_, s) = recv_mpst_c_from_i(s)?;
            let s = send_mpst_c_to_i((), s)?;
            let (_, s) = recv_mpst_c_from_j(s)?;
            let s = send_mpst_c_to_j((), s)?;
            let (_, s) = recv_mpst_c_from_k(s)?;
            let s = send_mpst_c_to_k((), s)?;
            let (_, s) = recv_mpst_c_from_l(s)?;
            let s = send_mpst_c_to_l((), s)?;
            let (_, s) = recv_mpst_c_from_m(s)?;
            let s = send_mpst_c_to_m((), s)?;
            let (_, s) = recv_mpst_c_from_n(s)?;
            let s = send_mpst_c_to_n((), s)?;
            let (_, s) = recv_mpst_c_from_o(s)?;
            let s = send_mpst_c_to_o((), s)?;
            let (_, s) = recv_mpst_c_from_p(s)?;
            let s = send_mpst_c_to_p((), s)?;
            let (_, s) = recv_mpst_c_from_q(s)?;
            let s = send_mpst_c_to_q((), s)?;
            let (_, s) = recv_mpst_c_from_r(s)?;
            let s = send_mpst_c_to_r((), s)?;
            let (_, s) = recv_mpst_c_from_s(s)?;
            let s = send_mpst_c_to_s((), s)?;
            endpoint_c(s)
        }, })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_d_from_t, {
        Branching0fromTtoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoD::More(s) => {
            let (_, s) = recv_mpst_d_from_t(s)?;
            let s = send_mpst_d_to_t((), s)?;
            let s = send_mpst_d_to_a((), s)?;
            let (_, s) = recv_mpst_d_from_a(s)?;
            let s = send_mpst_d_to_b((), s)?;
            let (_, s) = recv_mpst_d_from_b(s)?;
            let s = send_mpst_d_to_c((), s)?;
            let (_, s) = recv_mpst_d_from_c(s)?;
            let (_, s) = recv_mpst_d_from_e(s)?;
            let s = send_mpst_d_to_e((), s)?;
            let (_, s) = recv_mpst_d_from_f(s)?;
            let s = send_mpst_d_to_f((), s)?;
            let (_, s) = recv_mpst_d_from_g(s)?;
            let s = send_mpst_d_to_g((), s)?;
            let (_, s) = recv_mpst_d_from_h(s)?;
            let s = send_mpst_d_to_h((), s)?;
            let (_, s) = recv_mpst_d_from_i(s)?;
            let s = send_mpst_d_to_i((), s)?;
            let (_, s) = recv_mpst_d_from_j(s)?;
            let s = send_mpst_d_to_j((), s)?;
            let (_, s) = recv_mpst_d_from_k(s)?;
            let s = send_mpst_d_to_k((), s)?;
            let (_, s) = recv_mpst_d_from_l(s)?;
            let s = send_mpst_d_to_l((), s)?;
            let (_, s) = recv_mpst_d_from_m(s)?;
            let s = send_mpst_d_to_m((), s)?;
            let (_, s) = recv_mpst_d_from_n(s)?;
            let s = send_mpst_d_to_n((), s)?;
            let (_, s) = recv_mpst_d_from_o(s)?;
            let s = send_mpst_d_to_o((), s)?;
            let (_, s) = recv_mpst_d_from_p(s)?;
            let s = send_mpst_d_to_p((), s)?;
            let (_, s) = recv_mpst_d_from_q(s)?;
            let s = send_mpst_d_to_q((), s)?;
            let (_, s) = recv_mpst_d_from_r(s)?;
            let s = send_mpst_d_to_r((), s)?;
            let (_, s) = recv_mpst_d_from_s(s)?;
            let s = send_mpst_d_to_s((), s)?;
            endpoint_d(s)
        }, })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_e_from_t, {
        Branching0fromTtoE::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoE::More(s) => {
            let (_, s) = recv_mpst_e_from_t(s)?;
            let s = send_mpst_e_to_t((), s)?;
            let s = send_mpst_e_to_a((), s)?;
            let (_, s) = recv_mpst_e_from_a(s)?;
            let s = send_mpst_e_to_b((), s)?;
            let (_, s) = recv_mpst_e_from_b(s)?;
            let s = send_mpst_e_to_c((), s)?;
            let (_, s) = recv_mpst_e_from_c(s)?;
            let s = send_mpst_e_to_d((), s)?;
            let (_, s) = recv_mpst_e_from_d(s)?;
            let (_, s) = recv_mpst_e_from_f(s)?;
            let s = send_mpst_e_to_f((), s)?;
            let (_, s) = recv_mpst_e_from_g(s)?;
            let s = send_mpst_e_to_g((), s)?;
            let (_, s) = recv_mpst_e_from_h(s)?;
            let s = send_mpst_e_to_h((), s)?;
            let (_, s) = recv_mpst_e_from_i(s)?;
            let s = send_mpst_e_to_i((), s)?;
            let (_, s) = recv_mpst_e_from_j(s)?;
            let s = send_mpst_e_to_j((), s)?;
            let (_, s) = recv_mpst_e_from_k(s)?;
            let s = send_mpst_e_to_k((), s)?;
            let (_, s) = recv_mpst_e_from_l(s)?;
            let s = send_mpst_e_to_l((), s)?;
            let (_, s) = recv_mpst_e_from_m(s)?;
            let s = send_mpst_e_to_m((), s)?;
            let (_, s) = recv_mpst_e_from_n(s)?;
            let s = send_mpst_e_to_n((), s)?;
            let (_, s) = recv_mpst_e_from_o(s)?;
            let s = send_mpst_e_to_o((), s)?;
            let (_, s) = recv_mpst_e_from_p(s)?;
            let s = send_mpst_e_to_p((), s)?;
            let (_, s) = recv_mpst_e_from_q(s)?;
            let s = send_mpst_e_to_q((), s)?;
            let (_, s) = recv_mpst_e_from_r(s)?;
            let s = send_mpst_e_to_r((), s)?;
            let (_, s) = recv_mpst_e_from_s(s)?;
            let s = send_mpst_e_to_s((), s)?;
            endpoint_e(s)
        }, })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_f_from_t, {
        Branching0fromTtoF::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoF::More(s) => {
            let (_, s) = recv_mpst_f_from_t(s)?;
            let s = send_mpst_f_to_t((), s)?;
            let s = send_mpst_f_to_a((), s)?;
            let (_, s) = recv_mpst_f_from_a(s)?;
            let s = send_mpst_f_to_b((), s)?;
            let (_, s) = recv_mpst_f_from_b(s)?;
            let s = send_mpst_f_to_c((), s)?;
            let (_, s) = recv_mpst_f_from_c(s)?;
            let s = send_mpst_f_to_d((), s)?;
            let (_, s) = recv_mpst_f_from_d(s)?;
            let s = send_mpst_f_to_e((), s)?;
            let (_, s) = recv_mpst_f_from_e(s)?;
            let (_, s) = recv_mpst_f_from_g(s)?;
            let s = send_mpst_f_to_g((), s)?;
            let (_, s) = recv_mpst_f_from_h(s)?;
            let s = send_mpst_f_to_h((), s)?;
            let (_, s) = recv_mpst_f_from_i(s)?;
            let s = send_mpst_f_to_i((), s)?;
            let (_, s) = recv_mpst_f_from_j(s)?;
            let s = send_mpst_f_to_j((), s)?;
            let (_, s) = recv_mpst_f_from_k(s)?;
            let s = send_mpst_f_to_k((), s)?;
            let (_, s) = recv_mpst_f_from_l(s)?;
            let s = send_mpst_f_to_l((), s)?;
            let (_, s) = recv_mpst_f_from_m(s)?;
            let s = send_mpst_f_to_m((), s)?;
            let (_, s) = recv_mpst_f_from_n(s)?;
            let s = send_mpst_f_to_n((), s)?;
            let (_, s) = recv_mpst_f_from_o(s)?;
            let s = send_mpst_f_to_o((), s)?;
            let (_, s) = recv_mpst_f_from_p(s)?;
            let s = send_mpst_f_to_p((), s)?;
            let (_, s) = recv_mpst_f_from_q(s)?;
            let s = send_mpst_f_to_q((), s)?;
            let (_, s) = recv_mpst_f_from_r(s)?;
            let s = send_mpst_f_to_r((), s)?;
            let (_, s) = recv_mpst_f_from_s(s)?;
            let s = send_mpst_f_to_s((), s)?;
            endpoint_f(s)
        }, })
}

fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_g_from_t, {
        Branching0fromTtoG::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoG::More(s) => {
            let (_, s) = recv_mpst_g_from_t(s)?;
            let s = send_mpst_g_to_t((), s)?;
            let s = send_mpst_g_to_a((), s)?;
            let (_, s) = recv_mpst_g_from_a(s)?;
            let s = send_mpst_g_to_b((), s)?;
            let (_, s) = recv_mpst_g_from_b(s)?;
            let s = send_mpst_g_to_c((), s)?;
            let (_, s) = recv_mpst_g_from_c(s)?;
            let s = send_mpst_g_to_d((), s)?;
            let (_, s) = recv_mpst_g_from_d(s)?;
            let s = send_mpst_g_to_e((), s)?;
            let (_, s) = recv_mpst_g_from_e(s)?;
            let s = send_mpst_g_to_f((), s)?;
            let (_, s) = recv_mpst_g_from_f(s)?;
            let (_, s) = recv_mpst_g_from_h(s)?;
            let s = send_mpst_g_to_h((), s)?;
            let (_, s) = recv_mpst_g_from_i(s)?;
            let s = send_mpst_g_to_i((), s)?;
            let (_, s) = recv_mpst_g_from_j(s)?;
            let s = send_mpst_g_to_j((), s)?;
            let (_, s) = recv_mpst_g_from_k(s)?;
            let s = send_mpst_g_to_k((), s)?;
            let (_, s) = recv_mpst_g_from_l(s)?;
            let s = send_mpst_g_to_l((), s)?;
            let (_, s) = recv_mpst_g_from_m(s)?;
            let s = send_mpst_g_to_m((), s)?;
            let (_, s) = recv_mpst_g_from_n(s)?;
            let s = send_mpst_g_to_n((), s)?;
            let (_, s) = recv_mpst_g_from_o(s)?;
            let s = send_mpst_g_to_o((), s)?;
            let (_, s) = recv_mpst_g_from_p(s)?;
            let s = send_mpst_g_to_p((), s)?;
            let (_, s) = recv_mpst_g_from_q(s)?;
            let s = send_mpst_g_to_q((), s)?;
            let (_, s) = recv_mpst_g_from_r(s)?;
            let s = send_mpst_g_to_r((), s)?;
            let (_, s) = recv_mpst_g_from_s(s)?;
            let s = send_mpst_g_to_s((), s)?;
            endpoint_g(s)
        }, })
}

fn endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_h_from_t, {
        Branching0fromTtoH::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoH::More(s) => {
            let (_, s) = recv_mpst_h_from_t(s)?;
            let s = send_mpst_h_to_t((), s)?;
            let s = send_mpst_h_to_a((), s)?;
            let (_, s) = recv_mpst_h_from_a(s)?;
            let s = send_mpst_h_to_b((), s)?;
            let (_, s) = recv_mpst_h_from_b(s)?;
            let s = send_mpst_h_to_c((), s)?;
            let (_, s) = recv_mpst_h_from_c(s)?;
            let s = send_mpst_h_to_d((), s)?;
            let (_, s) = recv_mpst_h_from_d(s)?;
            let s = send_mpst_h_to_e((), s)?;
            let (_, s) = recv_mpst_h_from_e(s)?;
            let s = send_mpst_h_to_f((), s)?;
            let (_, s) = recv_mpst_h_from_f(s)?;
            let s = send_mpst_h_to_g((), s)?;
            let (_, s) = recv_mpst_h_from_g(s)?;
            let (_, s) = recv_mpst_h_from_i(s)?;
            let s = send_mpst_h_to_i((), s)?;
            let (_, s) = recv_mpst_h_from_j(s)?;
            let s = send_mpst_h_to_j((), s)?;
            let (_, s) = recv_mpst_h_from_k(s)?;
            let s = send_mpst_h_to_k((), s)?;
            let (_, s) = recv_mpst_h_from_l(s)?;
            let s = send_mpst_h_to_l((), s)?;
            let (_, s) = recv_mpst_h_from_m(s)?;
            let s = send_mpst_h_to_m((), s)?;
            let (_, s) = recv_mpst_h_from_n(s)?;
            let s = send_mpst_h_to_n((), s)?;
            let (_, s) = recv_mpst_h_from_o(s)?;
            let s = send_mpst_h_to_o((), s)?;
            let (_, s) = recv_mpst_h_from_p(s)?;
            let s = send_mpst_h_to_p((), s)?;
            let (_, s) = recv_mpst_h_from_q(s)?;
            let s = send_mpst_h_to_q((), s)?;
            let (_, s) = recv_mpst_h_from_r(s)?;
            let s = send_mpst_h_to_r((), s)?;
            let (_, s) = recv_mpst_h_from_s(s)?;
            let s = send_mpst_h_to_s((), s)?;
            endpoint_h(s)
        }, })
}

fn endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_i_from_t, {
        Branching0fromTtoI::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoI::More(s) => {
            let (_, s) = recv_mpst_i_from_t(s)?;
            let s = send_mpst_i_to_t((), s)?;
            let s = send_mpst_i_to_a((), s)?;
            let (_, s) = recv_mpst_i_from_a(s)?;
            let s = send_mpst_i_to_b((), s)?;
            let (_, s) = recv_mpst_i_from_b(s)?;
            let s = send_mpst_i_to_c((), s)?;
            let (_, s) = recv_mpst_i_from_c(s)?;
            let s = send_mpst_i_to_d((), s)?;
            let (_, s) = recv_mpst_i_from_d(s)?;
            let s = send_mpst_i_to_e((), s)?;
            let (_, s) = recv_mpst_i_from_e(s)?;
            let s = send_mpst_i_to_f((), s)?;
            let (_, s) = recv_mpst_i_from_f(s)?;
            let s = send_mpst_i_to_g((), s)?;
            let (_, s) = recv_mpst_i_from_g(s)?;
            let s = send_mpst_i_to_h((), s)?;
            let (_, s) = recv_mpst_i_from_h(s)?;
            let (_, s) = recv_mpst_i_from_j(s)?;
            let s = send_mpst_i_to_j((), s)?;
            let (_, s) = recv_mpst_i_from_k(s)?;
            let s = send_mpst_i_to_k((), s)?;
            let (_, s) = recv_mpst_i_from_l(s)?;
            let s = send_mpst_i_to_l((), s)?;
            let (_, s) = recv_mpst_i_from_m(s)?;
            let s = send_mpst_i_to_m((), s)?;
            let (_, s) = recv_mpst_i_from_n(s)?;
            let s = send_mpst_i_to_n((), s)?;
            let (_, s) = recv_mpst_i_from_o(s)?;
            let s = send_mpst_i_to_o((), s)?;
            let (_, s) = recv_mpst_i_from_p(s)?;
            let s = send_mpst_i_to_p((), s)?;
            let (_, s) = recv_mpst_i_from_q(s)?;
            let s = send_mpst_i_to_q((), s)?;
            let (_, s) = recv_mpst_i_from_r(s)?;
            let s = send_mpst_i_to_r((), s)?;
            let (_, s) = recv_mpst_i_from_s(s)?;
            let s = send_mpst_i_to_s((), s)?;
            endpoint_i(s)
        }, })
}

fn endpoint_j(s: EndpointJ) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_j_from_t, {
        Branching0fromTtoJ::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoJ::More(s) => {
            let (_, s) = recv_mpst_j_from_t(s)?;
            let s = send_mpst_j_to_t((), s)?;
            let s = send_mpst_j_to_a((), s)?;
            let (_, s) = recv_mpst_j_from_a(s)?;
            let s = send_mpst_j_to_b((), s)?;
            let (_, s) = recv_mpst_j_from_b(s)?;
            let s = send_mpst_j_to_c((), s)?;
            let (_, s) = recv_mpst_j_from_c(s)?;
            let s = send_mpst_j_to_d((), s)?;
            let (_, s) = recv_mpst_j_from_d(s)?;
            let s = send_mpst_j_to_e((), s)?;
            let (_, s) = recv_mpst_j_from_e(s)?;
            let s = send_mpst_j_to_f((), s)?;
            let (_, s) = recv_mpst_j_from_f(s)?;
            let s = send_mpst_j_to_g((), s)?;
            let (_, s) = recv_mpst_j_from_g(s)?;
            let s = send_mpst_j_to_h((), s)?;
            let (_, s) = recv_mpst_j_from_h(s)?;
            let s = send_mpst_j_to_i((), s)?;
            let (_, s) = recv_mpst_j_from_i(s)?;
            let (_, s) = recv_mpst_j_from_k(s)?;
            let s = send_mpst_j_to_k((), s)?;
            let (_, s) = recv_mpst_j_from_l(s)?;
            let s = send_mpst_j_to_l((), s)?;
            let (_, s) = recv_mpst_j_from_m(s)?;
            let s = send_mpst_j_to_m((), s)?;
            let (_, s) = recv_mpst_j_from_n(s)?;
            let s = send_mpst_j_to_n((), s)?;
            let (_, s) = recv_mpst_j_from_o(s)?;
            let s = send_mpst_j_to_o((), s)?;
            let (_, s) = recv_mpst_j_from_p(s)?;
            let s = send_mpst_j_to_p((), s)?;
            let (_, s) = recv_mpst_j_from_q(s)?;
            let s = send_mpst_j_to_q((), s)?;
            let (_, s) = recv_mpst_j_from_r(s)?;
            let s = send_mpst_j_to_r((), s)?;
            let (_, s) = recv_mpst_j_from_s(s)?;
            let s = send_mpst_j_to_s((), s)?;
            endpoint_j(s)
        }, })
}

fn endpoint_k(s: EndpointK) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_k_from_t, {
        Branching0fromTtoK::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoK::More(s) => {
            let (_, s) = recv_mpst_k_from_t(s)?;
            let s = send_mpst_k_to_t((), s)?;
            let s = send_mpst_k_to_a((), s)?;
            let (_, s) = recv_mpst_k_from_a(s)?;
            let s = send_mpst_k_to_b((), s)?;
            let (_, s) = recv_mpst_k_from_b(s)?;
            let s = send_mpst_k_to_c((), s)?;
            let (_, s) = recv_mpst_k_from_c(s)?;
            let s = send_mpst_k_to_d((), s)?;
            let (_, s) = recv_mpst_k_from_d(s)?;
            let s = send_mpst_k_to_e((), s)?;
            let (_, s) = recv_mpst_k_from_e(s)?;
            let s = send_mpst_k_to_f((), s)?;
            let (_, s) = recv_mpst_k_from_f(s)?;
            let s = send_mpst_k_to_g((), s)?;
            let (_, s) = recv_mpst_k_from_g(s)?;
            let s = send_mpst_k_to_h((), s)?;
            let (_, s) = recv_mpst_k_from_h(s)?;
            let s = send_mpst_k_to_i((), s)?;
            let (_, s) = recv_mpst_k_from_i(s)?;
            let s = send_mpst_k_to_j((), s)?;
            let (_, s) = recv_mpst_k_from_j(s)?;
            let (_, s) = recv_mpst_k_from_l(s)?;
            let s = send_mpst_k_to_l((), s)?;
            let (_, s) = recv_mpst_k_from_m(s)?;
            let s = send_mpst_k_to_m((), s)?;
            let (_, s) = recv_mpst_k_from_n(s)?;
            let s = send_mpst_k_to_n((), s)?;
            let (_, s) = recv_mpst_k_from_o(s)?;
            let s = send_mpst_k_to_o((), s)?;
            let (_, s) = recv_mpst_k_from_p(s)?;
            let s = send_mpst_k_to_p((), s)?;
            let (_, s) = recv_mpst_k_from_q(s)?;
            let s = send_mpst_k_to_q((), s)?;
            let (_, s) = recv_mpst_k_from_r(s)?;
            let s = send_mpst_k_to_r((), s)?;
            let (_, s) = recv_mpst_k_from_s(s)?;
            let s = send_mpst_k_to_s((), s)?;
            endpoint_k(s)
        }, })
}

fn endpoint_l(s: EndpointL) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_l_from_t, {
        Branching0fromTtoL::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoL::More(s) => {
            let (_, s) = recv_mpst_l_from_t(s)?;
            let s = send_mpst_l_to_t((), s)?;
            let s = send_mpst_l_to_a((), s)?;
            let (_, s) = recv_mpst_l_from_a(s)?;
            let s = send_mpst_l_to_b((), s)?;
            let (_, s) = recv_mpst_l_from_b(s)?;
            let s = send_mpst_l_to_c((), s)?;
            let (_, s) = recv_mpst_l_from_c(s)?;
            let s = send_mpst_l_to_d((), s)?;
            let (_, s) = recv_mpst_l_from_d(s)?;
            let s = send_mpst_l_to_e((), s)?;
            let (_, s) = recv_mpst_l_from_e(s)?;
            let s = send_mpst_l_to_f((), s)?;
            let (_, s) = recv_mpst_l_from_f(s)?;
            let s = send_mpst_l_to_g((), s)?;
            let (_, s) = recv_mpst_l_from_g(s)?;
            let s = send_mpst_l_to_h((), s)?;
            let (_, s) = recv_mpst_l_from_h(s)?;
            let s = send_mpst_l_to_i((), s)?;
            let (_, s) = recv_mpst_l_from_i(s)?;
            let s = send_mpst_l_to_j((), s)?;
            let (_, s) = recv_mpst_l_from_j(s)?;
            let s = send_mpst_l_to_k((), s)?;
            let (_, s) = recv_mpst_l_from_k(s)?;
            let (_, s) = recv_mpst_l_from_m(s)?;
            let s = send_mpst_l_to_m((), s)?;
            let (_, s) = recv_mpst_l_from_n(s)?;
            let s = send_mpst_l_to_n((), s)?;
            let (_, s) = recv_mpst_l_from_o(s)?;
            let s = send_mpst_l_to_o((), s)?;
            let (_, s) = recv_mpst_l_from_p(s)?;
            let s = send_mpst_l_to_p((), s)?;
            let (_, s) = recv_mpst_l_from_q(s)?;
            let s = send_mpst_l_to_q((), s)?;
            let (_, s) = recv_mpst_l_from_r(s)?;
            let s = send_mpst_l_to_r((), s)?;
            let (_, s) = recv_mpst_l_from_s(s)?;
            let s = send_mpst_l_to_s((), s)?;
            endpoint_l(s)
        }, })
}

fn endpoint_m(s: EndpointM) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_m_from_t, {
        Branching0fromTtoM::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoM::More(s) => {
            let (_, s) = recv_mpst_m_from_t(s)?;
            let s = send_mpst_m_to_t((), s)?;
            let s = send_mpst_m_to_a((), s)?;
            let (_, s) = recv_mpst_m_from_a(s)?;
            let s = send_mpst_m_to_b((), s)?;
            let (_, s) = recv_mpst_m_from_b(s)?;
            let s = send_mpst_m_to_c((), s)?;
            let (_, s) = recv_mpst_m_from_c(s)?;
            let s = send_mpst_m_to_d((), s)?;
            let (_, s) = recv_mpst_m_from_d(s)?;
            let s = send_mpst_m_to_e((), s)?;
            let (_, s) = recv_mpst_m_from_e(s)?;
            let s = send_mpst_m_to_f((), s)?;
            let (_, s) = recv_mpst_m_from_f(s)?;
            let s = send_mpst_m_to_g((), s)?;
            let (_, s) = recv_mpst_m_from_g(s)?;
            let s = send_mpst_m_to_h((), s)?;
            let (_, s) = recv_mpst_m_from_h(s)?;
            let s = send_mpst_m_to_i((), s)?;
            let (_, s) = recv_mpst_m_from_i(s)?;
            let s = send_mpst_m_to_j((), s)?;
            let (_, s) = recv_mpst_m_from_j(s)?;
            let s = send_mpst_m_to_k((), s)?;
            let (_, s) = recv_mpst_m_from_k(s)?;
            let s = send_mpst_m_to_l((), s)?;
            let (_, s) = recv_mpst_m_from_l(s)?;
            let (_, s) = recv_mpst_m_from_n(s)?;
            let s = send_mpst_m_to_n((), s)?;
            let (_, s) = recv_mpst_m_from_o(s)?;
            let s = send_mpst_m_to_o((), s)?;
            let (_, s) = recv_mpst_m_from_p(s)?;
            let s = send_mpst_m_to_p((), s)?;
            let (_, s) = recv_mpst_m_from_q(s)?;
            let s = send_mpst_m_to_q((), s)?;
            let (_, s) = recv_mpst_m_from_r(s)?;
            let s = send_mpst_m_to_r((), s)?;
            let (_, s) = recv_mpst_m_from_s(s)?;
            let s = send_mpst_m_to_s((), s)?;
            endpoint_m(s)
        }, })
}

fn endpoint_n(s: EndpointN) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_n_from_t, {
        Branching0fromTtoN::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoN::More(s) => {
            let (_, s) = recv_mpst_n_from_t(s)?;
            let s = send_mpst_n_to_t((), s)?;
            let s = send_mpst_n_to_a((), s)?;
            let (_, s) = recv_mpst_n_from_a(s)?;
            let s = send_mpst_n_to_b((), s)?;
            let (_, s) = recv_mpst_n_from_b(s)?;
            let s = send_mpst_n_to_c((), s)?;
            let (_, s) = recv_mpst_n_from_c(s)?;
            let s = send_mpst_n_to_d((), s)?;
            let (_, s) = recv_mpst_n_from_d(s)?;
            let s = send_mpst_n_to_e((), s)?;
            let (_, s) = recv_mpst_n_from_e(s)?;
            let s = send_mpst_n_to_f((), s)?;
            let (_, s) = recv_mpst_n_from_f(s)?;
            let s = send_mpst_n_to_g((), s)?;
            let (_, s) = recv_mpst_n_from_g(s)?;
            let s = send_mpst_n_to_h((), s)?;
            let (_, s) = recv_mpst_n_from_h(s)?;
            let s = send_mpst_n_to_i((), s)?;
            let (_, s) = recv_mpst_n_from_i(s)?;
            let s = send_mpst_n_to_j((), s)?;
            let (_, s) = recv_mpst_n_from_j(s)?;
            let s = send_mpst_n_to_k((), s)?;
            let (_, s) = recv_mpst_n_from_k(s)?;
            let s = send_mpst_n_to_l((), s)?;
            let (_, s) = recv_mpst_n_from_l(s)?;
            let s = send_mpst_n_to_m((), s)?;
            let (_, s) = recv_mpst_n_from_m(s)?;
            let (_, s) = recv_mpst_n_from_o(s)?;
            let s = send_mpst_n_to_o((), s)?;
            let (_, s) = recv_mpst_n_from_p(s)?;
            let s = send_mpst_n_to_p((), s)?;
            let (_, s) = recv_mpst_n_from_q(s)?;
            let s = send_mpst_n_to_q((), s)?;
            let (_, s) = recv_mpst_n_from_r(s)?;
            let s = send_mpst_n_to_r((), s)?;
            let (_, s) = recv_mpst_n_from_s(s)?;
            let s = send_mpst_n_to_s((), s)?;
            endpoint_n(s)
        }, })
}

fn endpoint_o(s: EndpointO) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_o_from_t, {
        Branching0fromTtoO::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoO::More(s) => {
            let (_, s) = recv_mpst_o_from_t(s)?;
            let s = send_mpst_o_to_t((), s)?;
            let s = send_mpst_o_to_a((), s)?;
            let (_, s) = recv_mpst_o_from_a(s)?;
            let s = send_mpst_o_to_b((), s)?;
            let (_, s) = recv_mpst_o_from_b(s)?;
            let s = send_mpst_o_to_c((), s)?;
            let (_, s) = recv_mpst_o_from_c(s)?;
            let s = send_mpst_o_to_d((), s)?;
            let (_, s) = recv_mpst_o_from_d(s)?;
            let s = send_mpst_o_to_e((), s)?;
            let (_, s) = recv_mpst_o_from_e(s)?;
            let s = send_mpst_o_to_f((), s)?;
            let (_, s) = recv_mpst_o_from_f(s)?;
            let s = send_mpst_o_to_g((), s)?;
            let (_, s) = recv_mpst_o_from_g(s)?;
            let s = send_mpst_o_to_h((), s)?;
            let (_, s) = recv_mpst_o_from_h(s)?;
            let s = send_mpst_o_to_i((), s)?;
            let (_, s) = recv_mpst_o_from_i(s)?;
            let s = send_mpst_o_to_j((), s)?;
            let (_, s) = recv_mpst_o_from_j(s)?;
            let s = send_mpst_o_to_k((), s)?;
            let (_, s) = recv_mpst_o_from_k(s)?;
            let s = send_mpst_o_to_l((), s)?;
            let (_, s) = recv_mpst_o_from_l(s)?;
            let s = send_mpst_o_to_m((), s)?;
            let (_, s) = recv_mpst_o_from_m(s)?;
            let s = send_mpst_o_to_n((), s)?;
            let (_, s) = recv_mpst_o_from_n(s)?;
            let (_, s) = recv_mpst_o_from_p(s)?;
            let s = send_mpst_o_to_p((), s)?;
            let (_, s) = recv_mpst_o_from_q(s)?;
            let s = send_mpst_o_to_q((), s)?;
            let (_, s) = recv_mpst_o_from_r(s)?;
            let s = send_mpst_o_to_r((), s)?;
            let (_, s) = recv_mpst_o_from_s(s)?;
            let s = send_mpst_o_to_s((), s)?;
            endpoint_o(s)
        }, })
}

fn endpoint_p(s: EndpointP) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_p_from_t, {
        Branching0fromTtoP::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoP::More(s) => {
            let (_, s) = recv_mpst_p_from_t(s)?;
            let s = send_mpst_p_to_t((), s)?;
            let s = send_mpst_p_to_a((), s)?;
            let (_, s) = recv_mpst_p_from_a(s)?;
            let s = send_mpst_p_to_b((), s)?;
            let (_, s) = recv_mpst_p_from_b(s)?;
            let s = send_mpst_p_to_c((), s)?;
            let (_, s) = recv_mpst_p_from_c(s)?;
            let s = send_mpst_p_to_d((), s)?;
            let (_, s) = recv_mpst_p_from_d(s)?;
            let s = send_mpst_p_to_e((), s)?;
            let (_, s) = recv_mpst_p_from_e(s)?;
            let s = send_mpst_p_to_f((), s)?;
            let (_, s) = recv_mpst_p_from_f(s)?;
            let s = send_mpst_p_to_g((), s)?;
            let (_, s) = recv_mpst_p_from_g(s)?;
            let s = send_mpst_p_to_h((), s)?;
            let (_, s) = recv_mpst_p_from_h(s)?;
            let s = send_mpst_p_to_i((), s)?;
            let (_, s) = recv_mpst_p_from_i(s)?;
            let s = send_mpst_p_to_j((), s)?;
            let (_, s) = recv_mpst_p_from_j(s)?;
            let s = send_mpst_p_to_k((), s)?;
            let (_, s) = recv_mpst_p_from_k(s)?;
            let s = send_mpst_p_to_l((), s)?;
            let (_, s) = recv_mpst_p_from_l(s)?;
            let s = send_mpst_p_to_m((), s)?;
            let (_, s) = recv_mpst_p_from_m(s)?;
            let s = send_mpst_p_to_n((), s)?;
            let (_, s) = recv_mpst_p_from_n(s)?;
            let s = send_mpst_p_to_o((), s)?;
            let (_, s) = recv_mpst_p_from_o(s)?;
            let (_, s) = recv_mpst_p_from_q(s)?;
            let s = send_mpst_p_to_q((), s)?;
            let (_, s) = recv_mpst_p_from_r(s)?;
            let s = send_mpst_p_to_r((), s)?;
            let (_, s) = recv_mpst_p_from_s(s)?;
            let s = send_mpst_p_to_s((), s)?;
            endpoint_p(s)
        }, })
}

fn endpoint_q(s: EndpointQ) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_q_from_t, {
        Branching0fromTtoQ::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoQ::More(s) => {
            let (_, s) = recv_mpst_q_from_t(s)?;
            let s = send_mpst_q_to_t((), s)?;
            let s = send_mpst_q_to_a((), s)?;
            let (_, s) = recv_mpst_q_from_a(s)?;
            let s = send_mpst_q_to_b((), s)?;
            let (_, s) = recv_mpst_q_from_b(s)?;
            let s = send_mpst_q_to_c((), s)?;
            let (_, s) = recv_mpst_q_from_c(s)?;
            let s = send_mpst_q_to_d((), s)?;
            let (_, s) = recv_mpst_q_from_d(s)?;
            let s = send_mpst_q_to_e((), s)?;
            let (_, s) = recv_mpst_q_from_e(s)?;
            let s = send_mpst_q_to_f((), s)?;
            let (_, s) = recv_mpst_q_from_f(s)?;
            let s = send_mpst_q_to_g((), s)?;
            let (_, s) = recv_mpst_q_from_g(s)?;
            let s = send_mpst_q_to_h((), s)?;
            let (_, s) = recv_mpst_q_from_h(s)?;
            let s = send_mpst_q_to_i((), s)?;
            let (_, s) = recv_mpst_q_from_i(s)?;
            let s = send_mpst_q_to_j((), s)?;
            let (_, s) = recv_mpst_q_from_j(s)?;
            let s = send_mpst_q_to_k((), s)?;
            let (_, s) = recv_mpst_q_from_k(s)?;
            let s = send_mpst_q_to_l((), s)?;
            let (_, s) = recv_mpst_q_from_l(s)?;
            let s = send_mpst_q_to_m((), s)?;
            let (_, s) = recv_mpst_q_from_m(s)?;
            let s = send_mpst_q_to_n((), s)?;
            let (_, s) = recv_mpst_q_from_n(s)?;
            let s = send_mpst_q_to_o((), s)?;
            let (_, s) = recv_mpst_q_from_o(s)?;
            let s = send_mpst_q_to_p((), s)?;
            let (_, s) = recv_mpst_q_from_p(s)?;
            let (_, s) = recv_mpst_q_from_r(s)?;
            let s = send_mpst_q_to_r((), s)?;
            let (_, s) = recv_mpst_q_from_s(s)?;
            let s = send_mpst_q_to_s((), s)?;
            endpoint_q(s)
        }, })
}

fn endpoint_r(s: EndpointR) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_r_from_t, {
        Branching0fromTtoR::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoR::More(s) => {
            let (_, s) = recv_mpst_r_from_t(s)?;
            let s = send_mpst_r_to_t((), s)?;
            let s = send_mpst_r_to_a((), s)?;
            let (_, s) = recv_mpst_r_from_a(s)?;
            let s = send_mpst_r_to_b((), s)?;
            let (_, s) = recv_mpst_r_from_b(s)?;
            let s = send_mpst_r_to_c((), s)?;
            let (_, s) = recv_mpst_r_from_c(s)?;
            let s = send_mpst_r_to_d((), s)?;
            let (_, s) = recv_mpst_r_from_d(s)?;
            let s = send_mpst_r_to_e((), s)?;
            let (_, s) = recv_mpst_r_from_e(s)?;
            let s = send_mpst_r_to_f((), s)?;
            let (_, s) = recv_mpst_r_from_f(s)?;
            let s = send_mpst_r_to_g((), s)?;
            let (_, s) = recv_mpst_r_from_g(s)?;
            let s = send_mpst_r_to_h((), s)?;
            let (_, s) = recv_mpst_r_from_h(s)?;
            let s = send_mpst_r_to_i((), s)?;
            let (_, s) = recv_mpst_r_from_i(s)?;
            let s = send_mpst_r_to_j((), s)?;
            let (_, s) = recv_mpst_r_from_j(s)?;
            let s = send_mpst_r_to_k((), s)?;
            let (_, s) = recv_mpst_r_from_k(s)?;
            let s = send_mpst_r_to_l((), s)?;
            let (_, s) = recv_mpst_r_from_l(s)?;
            let s = send_mpst_r_to_m((), s)?;
            let (_, s) = recv_mpst_r_from_m(s)?;
            let s = send_mpst_r_to_n((), s)?;
            let (_, s) = recv_mpst_r_from_n(s)?;
            let s = send_mpst_r_to_o((), s)?;
            let (_, s) = recv_mpst_r_from_o(s)?;
            let s = send_mpst_r_to_p((), s)?;
            let (_, s) = recv_mpst_r_from_p(s)?;
            let s = send_mpst_r_to_q((), s)?;
            let (_, s) = recv_mpst_r_from_q(s)?;
            let (_, s) = recv_mpst_r_from_s(s)?;
            let s = send_mpst_r_to_s((), s)?;
            endpoint_r(s)
        }, })
}

fn endpoint_s(s: EndpointS) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_s_from_t, {
        Branching0fromTtoS::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromTtoS::More(s) => {
            let (_, s) = recv_mpst_s_from_t(s)?;
            let s = send_mpst_s_to_t((), s)?;
            let s = send_mpst_s_to_a((), s)?;
            let (_, s) = recv_mpst_s_from_a(s)?;
            let s = send_mpst_s_to_b((), s)?;
            let (_, s) = recv_mpst_s_from_b(s)?;
            let s = send_mpst_s_to_c((), s)?;
            let (_, s) = recv_mpst_s_from_c(s)?;
            let s = send_mpst_s_to_d((), s)?;
            let (_, s) = recv_mpst_s_from_d(s)?;
            let s = send_mpst_s_to_e((), s)?;
            let (_, s) = recv_mpst_s_from_e(s)?;
            let s = send_mpst_s_to_f((), s)?;
            let (_, s) = recv_mpst_s_from_f(s)?;
            let s = send_mpst_s_to_g((), s)?;
            let (_, s) = recv_mpst_s_from_g(s)?;
            let s = send_mpst_s_to_h((), s)?;
            let (_, s) = recv_mpst_s_from_h(s)?;
            let s = send_mpst_s_to_i((), s)?;
            let (_, s) = recv_mpst_s_from_i(s)?;
            let s = send_mpst_s_to_j((), s)?;
            let (_, s) = recv_mpst_s_from_j(s)?;
            let s = send_mpst_s_to_k((), s)?;
            let (_, s) = recv_mpst_s_from_k(s)?;
            let s = send_mpst_s_to_l((), s)?;
            let (_, s) = recv_mpst_s_from_l(s)?;
            let s = send_mpst_s_to_m((), s)?;
            let (_, s) = recv_mpst_s_from_m(s)?;
            let s = send_mpst_s_to_n((), s)?;
            let (_, s) = recv_mpst_s_from_n(s)?;
            let s = send_mpst_s_to_o((), s)?;
            let (_, s) = recv_mpst_s_from_o(s)?;
            let s = send_mpst_s_to_p((), s)?;
            let (_, s) = recv_mpst_s_from_p(s)?;
            let s = send_mpst_s_to_q((), s)?;
            let (_, s) = recv_mpst_s_from_q(s)?;
            let s = send_mpst_s_to_r((), s)?;
            let (_, s) = recv_mpst_s_from_r(s)?;
            endpoint_s(s)
        }, })
}

fn endpoint_t(s: EndpointT) -> Result<(), Box<dyn Error>> {
    recurs_t(s, 100)
}

fn recurs_t(s: EndpointT, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_t_to_all(s)?;

            close_mpst_multi(s)
        }
        i => {
            let s = more_from_t_to_all(s)?;

            let s = send_mpst_t_to_a((), s)?;
            let (_, s) = recv_mpst_t_from_a(s)?;
            let s = send_mpst_t_to_b((), s)?;
            let (_, s) = recv_mpst_t_from_b(s)?;
            let s = send_mpst_t_to_c((), s)?;
            let (_, s) = recv_mpst_t_from_c(s)?;
            let s = send_mpst_t_to_d((), s)?;
            let (_, s) = recv_mpst_t_from_d(s)?;
            let s = send_mpst_t_to_e((), s)?;
            let (_, s) = recv_mpst_t_from_e(s)?;
            let s = send_mpst_t_to_f((), s)?;
            let (_, s) = recv_mpst_t_from_f(s)?;
            let s = send_mpst_t_to_g((), s)?;
            let (_, s) = recv_mpst_t_from_g(s)?;
            let s = send_mpst_t_to_h((), s)?;
            let (_, s) = recv_mpst_t_from_h(s)?;
            let s = send_mpst_t_to_i((), s)?;
            let (_, s) = recv_mpst_t_from_i(s)?;
            let s = send_mpst_t_to_j((), s)?;
            let (_, s) = recv_mpst_t_from_j(s)?;
            let s = send_mpst_t_to_k((), s)?;
            let (_, s) = recv_mpst_t_from_k(s)?;
            let s = send_mpst_t_to_l((), s)?;
            let (_, s) = recv_mpst_t_from_l(s)?;
            let s = send_mpst_t_to_m((), s)?;
            let (_, s) = recv_mpst_t_from_m(s)?;
            let s = send_mpst_t_to_n((), s)?;
            let (_, s) = recv_mpst_t_from_n(s)?;
            let s = send_mpst_t_to_o((), s)?;
            let (_, s) = recv_mpst_t_from_o(s)?;
            let s = send_mpst_t_to_p((), s)?;
            let (_, s) = recv_mpst_t_from_p(s)?;
            let s = send_mpst_t_to_q((), s)?;
            let (_, s) = recv_mpst_t_from_q(s)?;
            let s = send_mpst_t_to_r((), s)?;
            let (_, s) = recv_mpst_t_from_r(s)?;
            let s = send_mpst_t_to_s((), s)?;
            let (_, s) = recv_mpst_t_from_s(s)?;

            recurs_t(s, i - 1)
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
        endpoint_l,
        endpoint_m,
        endpoint_n,
        endpoint_o,
        endpoint_p,
        endpoint_q,
        endpoint_r,
        endpoint_s,
        endpoint_t,
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