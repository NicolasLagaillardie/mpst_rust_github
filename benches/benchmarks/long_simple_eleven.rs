#![allow(dead_code)]

use crossbeam_channel::bounded;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::{close, fork_with_thread_id, recv, send, End, Recv, Send, Session};
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::{
    bundle_fork_multi, choose, choose_mpst_multi_to_all, close_mpst, create_normal_role,
    create_recv_mpst_session, create_recv_mpst_session_bundle, create_send_mpst_session,
    create_send_mpst_session_bundle, create_sessionmpst, offer, offer_mpst,
};

use std::error::Error;
use std::thread::{spawn, JoinHandle};
use std::time::Duration;

// Create new SessionMpst for four participants
create_sessionmpst!(SessionMpstEleven, 11);

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
    10, | =>
    RoleA,
    SessionMpstEleven,
    11
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
    10, | =>
    RoleB,
    SessionMpstEleven,
    11
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
    10, | =>
    RoleC,
    SessionMpstEleven,
    11
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
    10, | =>
    RoleD,
    SessionMpstEleven,
    11
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
    10, | =>
    RoleE,
    SessionMpstEleven,
    11
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
    10, | =>
    RoleF,
    SessionMpstEleven,
    11
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
    10, | =>
    RoleG,
    SessionMpstEleven,
    11
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
    10, | =>
    RoleH,
    SessionMpstEleven,
    11
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
    10, | =>
    RoleI,
    SessionMpstEleven,
    11
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
    10, | =>
    RoleJ,
    SessionMpstEleven,
    11
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
    10, | =>
    RoleK,
    SessionMpstEleven,
    11
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
    10, | =>
    RoleA,
    SessionMpstEleven,
    11
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
    10, | =>
    RoleB,
    SessionMpstEleven,
    11
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
    10, | =>
    RoleC,
    SessionMpstEleven,
    11
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
    10, | =>
    RoleD,
    SessionMpstEleven,
    11
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
    10, | =>
    RoleE,
    SessionMpstEleven,
    11
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
    10, | =>
    RoleF,
    SessionMpstEleven,
    11
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
    10, | =>
    RoleG,
    SessionMpstEleven,
    11
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
    10, | =>
    RoleH,
    SessionMpstEleven,
    11
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
    10, | =>
    RoleI,
    SessionMpstEleven,
    11
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
    10, | =>
    RoleJ,
    SessionMpstEleven,
    11
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
    10, | =>
    RoleK,
    SessionMpstEleven,
    11
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstEleven, 11);

// Create fork function
bundle_fork_multi!(fork_mpst, fork_simple, SessionMpstEleven, 11);

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
// Binary
// A
enum BranchingHforA {
    More(
        SessionMpstEleven<
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursAtoK>>,
            R2K<R2B<R2C<R2D<R2E<R2F<R2G<R2H<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameA,
        >,
    ),
    Done(SessionMpstEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoK = Recv<BranchingHforA, End>;
// B
enum BranchingHforB {
    More(
        SessionMpstEleven<
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursBtoK>>,
            R2K<R2A<R2C<R2D<R2E<R2F<R2G<R2H<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameB,
        >,
    ),
    Done(SessionMpstEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoK = Recv<BranchingHforB, End>;
// C
enum BranchingHforC {
    More(
        SessionMpstEleven<
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursCtoK>>,
            R2K<R2A<R2B<R2D<R2E<R2F<R2G<R2H<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameC,
        >,
    ),
    Done(SessionMpstEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoK = Recv<BranchingHforC, End>;
// D
enum BranchingHforD {
    More(
        SessionMpstEleven<
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursDtoK>>,
            R2K<R2A<R2B<R2C<R2E<R2F<R2G<R2H<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameD,
        >,
    ),
    Done(SessionMpstEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoK = Recv<BranchingHforD, End>;
// E
enum BranchingHforE {
    More(
        SessionMpstEleven<
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursEtoK>>,
            R2K<R2A<R2B<R2C<R2D<R2F<R2G<R2H<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameE,
        >,
    ),
    Done(SessionMpstEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoK = Recv<BranchingHforE, End>;
// F
enum BranchingHforF {
    More(
        SessionMpstEleven<
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursFtoK>>,
            R2K<R2A<R2B<R2C<R2D<R2E<R2G<R2H<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameF,
        >,
    ),
    Done(SessionMpstEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoK = Recv<BranchingHforF, End>;
// G
enum BranchingHforG {
    More(
        SessionMpstEleven<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursGtoK>>,
            R2K<R2A<R2B<R2C<R2D<R2E<R2F<R2H<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameG,
        >,
    ),
    Done(SessionMpstEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoK = Recv<BranchingHforG, End>;
// H
enum BranchingHforH {
    More(
        SessionMpstEleven<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            Recv<(), Send<(), RecursHtoK>>,
            R2K<R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameH,
        >,
    ),
    Done(SessionMpstEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameH>),
}
type RecursHtoK = Recv<BranchingHforH, End>;
// I
enum BranchingHforI {
    More(
        SessionMpstEleven<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            Recv<(), Send<(), RecursItoK>>,
            R2K<R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2H<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameI,
        >,
    ),
    Done(SessionMpstEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameI>),
}
type RecursItoK = Recv<BranchingHforI, End>;
// J
enum BranchingHforJ {
    More(
        SessionMpstEleven<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            Recv<(), Send<(), RecursJtoK>>,
            R2K<R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2H<R2I<RoleK<RoleEnd>>>>>>>>>>>,
            NameJ,
        >,
    ),
    Done(SessionMpstEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameJ>),
}
type RecursJtoK = Recv<BranchingHforJ, End>;
// J
type ChooseKforAtoK = Send<BranchingHforA, End>;
type ChooseKforBtoK = Send<BranchingHforB, End>;
type ChooseKforCtoK = Send<BranchingHforC, End>;
type ChooseKforDtoK = Send<BranchingHforD, End>;
type ChooseKforEtoK = Send<BranchingHforE, End>;
type ChooseKforFtoK = Send<BranchingHforF, End>;
type ChooseKforGtoK = Send<BranchingHforG, End>;
type ChooseKforHtoK = Send<BranchingHforH, End>;
type ChooseKforItoK = Send<BranchingHforI, End>;
type ChooseKforJtoK = Send<BranchingHforJ, End>;

// Creating the MP sessions
type EndpointA = SessionMpstEleven<
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
type EndpointB = SessionMpstEleven<
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
type EndpointC = SessionMpstEleven<
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
type EndpointD = SessionMpstEleven<
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
type EndpointE = SessionMpstEleven<
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
type EndpointF = SessionMpstEleven<
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
type EndpointG = SessionMpstEleven<
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
type EndpointH = SessionMpstEleven<
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
type EndpointI = SessionMpstEleven<
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
type EndpointJ = SessionMpstEleven<
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
type EndpointK = SessionMpstEleven<
    ChooseKforAtoK,
    ChooseKforBtoK,
    ChooseKforCtoK,
    ChooseKforDtoK,
    ChooseKforEtoK,
    ChooseKforFtoK,
    ChooseKforGtoK,
    ChooseKforHtoK,
    ChooseKforItoK,
    ChooseKforJtoK,
    RoleA<RoleB<RoleC<RoleD<RoleE<RoleF<RoleG<RoleH<RoleI<RoleJ<RoleEnd>>>>>>>>>>,
    NameK,
>;

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_to_k, {
          BranchingHforA::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingHforA::More(s) => {
            let (_, s) = recv_mpst_a_to_k(s)?;
            let s = send_mpst_a_to_k((), s);
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
            simple_five_endpoint_a(s)
        },
    })
}

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_to_k, {
          BranchingHforB::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingHforB::More(s) => {
            let (_, s) = recv_mpst_b_to_k(s)?;
            let s = send_mpst_b_to_k((), s);
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
            simple_five_endpoint_b(s)
        },
    })
}

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_to_k, {
          BranchingHforC::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingHforC::More(s) => {
            let (_, s) = recv_mpst_c_to_k(s)?;
            let s = send_mpst_c_to_k((), s);
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
            simple_five_endpoint_c(s)
        },
    })
}

fn simple_five_endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_to_k, {
          BranchingHforD::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingHforD::More(s) => {
            let (_, s) = recv_mpst_d_to_k(s)?;
            let s = send_mpst_d_to_k((), s);
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
            simple_five_endpoint_d(s)
        },
    })
}

fn simple_five_endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_e_to_k, {
          BranchingHforE::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingHforE::More(s) => {
            let (_, s) = recv_mpst_e_to_k(s)?;
            let s = send_mpst_e_to_k((), s);
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
            simple_five_endpoint_e(s)
        },
    })
}

fn simple_five_endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_f_to_k, {
          BranchingHforF::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingHforF::More(s) => {
            let (_, s) = recv_mpst_f_to_k(s)?;
            let s = send_mpst_f_to_k((), s);
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
            simple_five_endpoint_f(s)
        },
    })
}

fn simple_five_endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_g_to_k, {
          BranchingHforG::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingHforG::More(s) => {
            let (_, s) = recv_mpst_g_to_k(s)?;
            let s = send_mpst_g_to_k((), s);
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
            simple_five_endpoint_g(s)
        },
    })
}

fn simple_five_endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_h_to_k, {
          BranchingHforH::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingHforH::More(s) => {
            let (_, s) = recv_mpst_h_to_k(s)?;
            let s = send_mpst_h_to_k((), s);
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
            simple_five_endpoint_h(s)
        },
    })
}

fn simple_five_endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_i_to_k, {
          BranchingHforI::Done(s) => {
            close_mpst_multi(s)
        },
        BranchingHforI::More(s) => {
            let (_, s) = recv_mpst_i_to_k(s)?;
            let s = send_mpst_i_to_k((), s);
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
            simple_five_endpoint_i(s)
        },
    })
}

fn simple_five_endpoint_j(s: EndpointJ) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_j_to_k, {
          BranchingHforJ::Done(s) => {
            close_mpst_multi(s)
        },
        BranchingHforJ::More(s) => {
            let (_, s) = recv_mpst_j_to_k(s)?;
            let s = send_mpst_j_to_k((), s);
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
            simple_five_endpoint_j(s)
        },
    })
}

fn simple_five_endpoint_k(s: EndpointK) -> Result<(), Box<dyn Error>> {
    recurs_k(s, SIZE)
}

fn recurs_k(s: EndpointK, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_k_to_a,
                send_mpst_k_to_b,
                send_mpst_k_to_c,
                send_mpst_k_to_d,
                send_mpst_k_to_e,
                send_mpst_k_to_f,
                send_mpst_k_to_g,
                send_mpst_k_to_h,
                send_mpst_k_to_i,
                send_mpst_k_to_j, =>
                  BranchingHforA::Done,
                  BranchingHforB::Done,
                  BranchingHforC::Done,
                  BranchingHforD::Done,
                  BranchingHforE::Done,
                  BranchingHforF::Done,
                  BranchingHforG::Done,
                  BranchingHforH::Done,
                  BranchingHforI::Done,
                  BranchingHforJ::Done, =>
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
                RoleK,
                 SessionMpstEleven,
                11
            );

            close_mpst_multi(s)
        }
        i => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_k_to_a,
                send_mpst_k_to_b,
                send_mpst_k_to_c,
                send_mpst_k_to_d,
                send_mpst_k_to_e,
                send_mpst_k_to_f,
                send_mpst_k_to_g,
                send_mpst_k_to_h,
                send_mpst_k_to_i,
                send_mpst_k_to_j, =>
                  BranchingHforA::More,
                  BranchingHforB::More,
                  BranchingHforC::More,
                  BranchingHforD::More,
                  BranchingHforE::More,
                  BranchingHforF::More,
                  BranchingHforG::More,
                  BranchingHforH::More,
                  BranchingHforI::More,
                  BranchingHforJ::More, =>
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
                  RoleK,
                 SessionMpstEleven,
                11
            );

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

            recurs_k(s, i - 1)
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
    ) = fork_mpst(
        black_box(simple_five_endpoint_a),
        black_box(simple_five_endpoint_b),
        black_box(simple_five_endpoint_c),
        black_box(simple_five_endpoint_d),
        black_box(simple_five_endpoint_e),
        black_box(simple_five_endpoint_f),
        black_box(simple_five_endpoint_g),
        black_box(simple_five_endpoint_h),
        black_box(simple_five_endpoint_i),
        black_box(simple_five_endpoint_j),
        black_box(simple_five_endpoint_k),
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

    Ok(())
}

/////////////////////////
// A
enum BinaryA {
    More(Recv<(), Send<(), RecursA>>),
    Done(End),
}
type RecursA = Recv<BinaryA, End>;
fn binary_a_to_b(s: RecursA) -> Result<(), Box<dyn Error>> {
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
fn binary_b_to_a(s: Send<(), Recv<(), RecursB>>) -> Result<RecursB, Box<dyn Error>> {
    let s = send((), s);
    let (_, s) = recv(s)?;
    Ok(s)
}

fn all_binaries() -> Result<(), Box<dyn Error>> {
    let mut threads = Vec::new();
    let mut sessions = Vec::new();

    for _ in 0..55 {
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

fn all_crossbeam() -> Result<(), Box<dyn Error>> {
    let mut threads = Vec::new();

    for _ in 0..55 {
        let main = spawn(move || {
            for _ in 0..SIZE {
                let (sender_1, receiver_1) = bounded::<()>(1);
                sender_1.send(()).unwrap_or(());
                receiver_1.recv().unwrap_or(());

                let (sender_2, receiver_2) = bounded::<()>(1);
                sender_2.send(()).unwrap_or(());
                receiver_2.recv().unwrap_or(());
            }

            // "Close" connection
            let (sender_close_1, receiver_close_1) = bounded::<()>(1);
            sender_close_1.send(()).unwrap_or(());

            let (sender_close_2, receiver_close_2) = bounded::<()>(1);
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

static SIZE: i64 = 100;

fn long_simple_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("long eleven simple protocol MPST {}", SIZE), |b| {
        b.iter(|| all_mpst())
    });
}

fn long_simple_protocol_binary(c: &mut Criterion) {
    c.bench_function(
        &format!("long eleven simple protocol binary {}", SIZE),
        |b| b.iter(|| all_binaries()),
    );
}

fn long_simple_protocol_crossbeam(c: &mut Criterion) {
    c.bench_function(
        &format!("long eleven simple protocol crossbeam {}", SIZE),
        |b| b.iter(|| all_crossbeam()),
    );
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(1800, 0))
}

criterion_group! {
    name = long_eleven_simple_protocols;
    config = long_warmup();
    targets = long_simple_protocol_mpst, long_simple_protocol_binary, long_simple_protocol_crossbeam
}
criterion_main!(long_eleven_simple_protocols);
