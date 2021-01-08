#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::{close, fork_with_thread_id, recv, send, End, Recv, Send, Session};
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::{
    bundle_fork_multi, choose, choose_mpst_multi_to_all, close_mpst, create_normal_role,
    create_recv_mpst_session, create_send_mpst_session, create_sessionmpst, offer, offer_mpst,
};

use std::error::Error;
use std::thread::{spawn, JoinHandle};
use std::time::Duration;

// Create new SessionMpst for four participants
create_sessionmpst!(SessionMpstTen, 10);

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

// Create new send functions
// A
create_send_mpst_session!(
    send_mpst_a_to_b,
    RoleB,
    next_b,
    RoleA,
    SessionMpstTen,
    10,
    1
);
create_send_mpst_session!(
    send_mpst_a_to_c,
    RoleC,
    next_c,
    RoleA,
    SessionMpstTen,
    10,
    2
);
create_send_mpst_session!(
    send_mpst_a_to_d,
    RoleD,
    next_d,
    RoleA,
    SessionMpstTen,
    10,
    3
);
create_send_mpst_session!(
    send_mpst_a_to_e,
    RoleE,
    next_e,
    RoleA,
    SessionMpstTen,
    10,
    4
);
create_send_mpst_session!(
    send_mpst_a_to_f,
    RoleF,
    next_f,
    RoleA,
    SessionMpstTen,
    10,
    5
);
create_send_mpst_session!(
    send_mpst_a_to_g,
    RoleG,
    next_g,
    RoleA,
    SessionMpstTen,
    10,
    6
);
create_send_mpst_session!(
    send_mpst_a_to_h,
    RoleH,
    next_h,
    RoleA,
    SessionMpstTen,
    10,
    7
);
create_send_mpst_session!(
    send_mpst_a_to_i,
    RoleI,
    next_i,
    RoleA,
    SessionMpstTen,
    10,
    8
);
create_send_mpst_session!(
    send_mpst_a_to_j,
    RoleJ,
    next_j,
    RoleA,
    SessionMpstTen,
    10,
    9
);
// B
create_send_mpst_session!(
    send_mpst_b_to_a,
    RoleA,
    next_a,
    RoleB,
    SessionMpstTen,
    10,
    1
);
create_send_mpst_session!(
    send_mpst_b_to_c,
    RoleC,
    next_c,
    RoleB,
    SessionMpstTen,
    10,
    2
);
create_send_mpst_session!(
    send_mpst_b_to_d,
    RoleD,
    next_d,
    RoleB,
    SessionMpstTen,
    10,
    3
);
create_send_mpst_session!(
    send_mpst_b_to_e,
    RoleE,
    next_e,
    RoleB,
    SessionMpstTen,
    10,
    4
);
create_send_mpst_session!(
    send_mpst_b_to_f,
    RoleF,
    next_f,
    RoleB,
    SessionMpstTen,
    10,
    5
);
create_send_mpst_session!(
    send_mpst_b_to_g,
    RoleG,
    next_g,
    RoleB,
    SessionMpstTen,
    10,
    6
);
create_send_mpst_session!(
    send_mpst_b_to_h,
    RoleH,
    next_h,
    RoleB,
    SessionMpstTen,
    10,
    7
);
create_send_mpst_session!(
    send_mpst_b_to_i,
    RoleI,
    next_i,
    RoleB,
    SessionMpstTen,
    10,
    8
);
create_send_mpst_session!(
    send_mpst_b_to_j,
    RoleJ,
    next_j,
    RoleB,
    SessionMpstTen,
    10,
    9
);
// C
create_send_mpst_session!(
    send_mpst_c_to_a,
    RoleA,
    next_a,
    RoleC,
    SessionMpstTen,
    10,
    1
);
create_send_mpst_session!(
    send_mpst_c_to_b,
    RoleB,
    next_b,
    RoleC,
    SessionMpstTen,
    10,
    2
);
create_send_mpst_session!(
    send_mpst_c_to_d,
    RoleD,
    next_d,
    RoleC,
    SessionMpstTen,
    10,
    3
);
create_send_mpst_session!(
    send_mpst_c_to_e,
    RoleE,
    next_e,
    RoleC,
    SessionMpstTen,
    10,
    4
);
create_send_mpst_session!(
    send_mpst_c_to_f,
    RoleF,
    next_f,
    RoleC,
    SessionMpstTen,
    10,
    5
);
create_send_mpst_session!(
    send_mpst_c_to_g,
    RoleG,
    next_g,
    RoleC,
    SessionMpstTen,
    10,
    6
);
create_send_mpst_session!(
    send_mpst_c_to_h,
    RoleH,
    next_h,
    RoleC,
    SessionMpstTen,
    10,
    7
);
create_send_mpst_session!(
    send_mpst_c_to_i,
    RoleI,
    next_i,
    RoleC,
    SessionMpstTen,
    10,
    8
);
create_send_mpst_session!(
    send_mpst_c_to_j,
    RoleJ,
    next_j,
    RoleC,
    SessionMpstTen,
    10,
    9
);
// D
create_send_mpst_session!(
    send_mpst_d_to_a,
    RoleA,
    next_a,
    RoleD,
    SessionMpstTen,
    10,
    1
);
create_send_mpst_session!(
    send_mpst_d_to_b,
    RoleB,
    next_b,
    RoleD,
    SessionMpstTen,
    10,
    2
);
create_send_mpst_session!(
    send_mpst_d_to_c,
    RoleC,
    next_c,
    RoleD,
    SessionMpstTen,
    10,
    3
);
create_send_mpst_session!(
    send_mpst_d_to_e,
    RoleE,
    next_e,
    RoleD,
    SessionMpstTen,
    10,
    4
);
create_send_mpst_session!(
    send_mpst_d_to_f,
    RoleF,
    next_f,
    RoleD,
    SessionMpstTen,
    10,
    5
);
create_send_mpst_session!(
    send_mpst_d_to_g,
    RoleG,
    next_g,
    RoleD,
    SessionMpstTen,
    10,
    6
);
create_send_mpst_session!(
    send_mpst_d_to_h,
    RoleH,
    next_h,
    RoleD,
    SessionMpstTen,
    10,
    7
);
create_send_mpst_session!(
    send_mpst_d_to_i,
    RoleI,
    next_i,
    RoleD,
    SessionMpstTen,
    10,
    8
);
create_send_mpst_session!(
    send_mpst_d_to_j,
    RoleJ,
    next_j,
    RoleD,
    SessionMpstTen,
    10,
    9
);
// E
create_send_mpst_session!(
    send_mpst_e_to_a,
    RoleA,
    next_a,
    RoleE,
    SessionMpstTen,
    10,
    1
);
create_send_mpst_session!(
    send_mpst_e_to_b,
    RoleB,
    next_b,
    RoleE,
    SessionMpstTen,
    10,
    2
);
create_send_mpst_session!(
    send_mpst_e_to_c,
    RoleC,
    next_c,
    RoleE,
    SessionMpstTen,
    10,
    3
);
create_send_mpst_session!(
    send_mpst_e_to_d,
    RoleD,
    next_d,
    RoleE,
    SessionMpstTen,
    10,
    4
);
create_send_mpst_session!(
    send_mpst_e_to_f,
    RoleF,
    next_f,
    RoleE,
    SessionMpstTen,
    10,
    5
);
create_send_mpst_session!(
    send_mpst_e_to_g,
    RoleG,
    next_g,
    RoleE,
    SessionMpstTen,
    10,
    6
);
create_send_mpst_session!(
    send_mpst_e_to_h,
    RoleH,
    next_h,
    RoleE,
    SessionMpstTen,
    10,
    7
);
create_send_mpst_session!(
    send_mpst_e_to_i,
    RoleI,
    next_i,
    RoleE,
    SessionMpstTen,
    10,
    8
);
create_send_mpst_session!(
    send_mpst_e_to_j,
    RoleJ,
    next_j,
    RoleE,
    SessionMpstTen,
    10,
    9
);
// F
create_send_mpst_session!(
    send_mpst_f_to_a,
    RoleA,
    next_a,
    RoleF,
    SessionMpstTen,
    10,
    1
);
create_send_mpst_session!(
    send_mpst_f_to_b,
    RoleB,
    next_b,
    RoleF,
    SessionMpstTen,
    10,
    2
);
create_send_mpst_session!(
    send_mpst_f_to_c,
    RoleC,
    next_c,
    RoleF,
    SessionMpstTen,
    10,
    3
);
create_send_mpst_session!(
    send_mpst_f_to_d,
    RoleD,
    next_d,
    RoleF,
    SessionMpstTen,
    10,
    4
);
create_send_mpst_session!(
    send_mpst_f_to_e,
    RoleE,
    next_e,
    RoleF,
    SessionMpstTen,
    10,
    5
);
create_send_mpst_session!(
    send_mpst_f_to_g,
    RoleG,
    next_g,
    RoleF,
    SessionMpstTen,
    10,
    6
);
create_send_mpst_session!(
    send_mpst_f_to_h,
    RoleH,
    next_h,
    RoleF,
    SessionMpstTen,
    10,
    7
);
create_send_mpst_session!(
    send_mpst_f_to_i,
    RoleI,
    next_i,
    RoleF,
    SessionMpstTen,
    10,
    8
);
create_send_mpst_session!(
    send_mpst_f_to_j,
    RoleJ,
    next_j,
    RoleF,
    SessionMpstTen,
    10,
    9
);
// G
create_send_mpst_session!(
    send_mpst_g_to_a,
    RoleA,
    next_a,
    RoleG,
    SessionMpstTen,
    10,
    1
);
create_send_mpst_session!(
    send_mpst_g_to_b,
    RoleB,
    next_b,
    RoleG,
    SessionMpstTen,
    10,
    2
);
create_send_mpst_session!(
    send_mpst_g_to_c,
    RoleC,
    next_c,
    RoleG,
    SessionMpstTen,
    10,
    3
);
create_send_mpst_session!(
    send_mpst_g_to_d,
    RoleD,
    next_d,
    RoleG,
    SessionMpstTen,
    10,
    4
);
create_send_mpst_session!(
    send_mpst_g_to_e,
    RoleE,
    next_e,
    RoleG,
    SessionMpstTen,
    10,
    5
);
create_send_mpst_session!(
    send_mpst_g_to_f,
    RoleF,
    next_f,
    RoleG,
    SessionMpstTen,
    10,
    6
);
create_send_mpst_session!(
    send_mpst_g_to_h,
    RoleH,
    next_h,
    RoleG,
    SessionMpstTen,
    10,
    7
);
create_send_mpst_session!(
    send_mpst_g_to_i,
    RoleI,
    next_i,
    RoleG,
    SessionMpstTen,
    10,
    8
);
create_send_mpst_session!(
    send_mpst_g_to_j,
    RoleJ,
    next_j,
    RoleG,
    SessionMpstTen,
    10,
    9
);
// H
create_send_mpst_session!(
    send_mpst_h_to_a,
    RoleA,
    next_a,
    RoleH,
    SessionMpstTen,
    10,
    1
);
create_send_mpst_session!(
    send_mpst_h_to_b,
    RoleB,
    next_b,
    RoleH,
    SessionMpstTen,
    10,
    2
);
create_send_mpst_session!(
    send_mpst_h_to_c,
    RoleC,
    next_c,
    RoleH,
    SessionMpstTen,
    10,
    3
);
create_send_mpst_session!(
    send_mpst_h_to_d,
    RoleD,
    next_d,
    RoleH,
    SessionMpstTen,
    10,
    4
);
create_send_mpst_session!(
    send_mpst_h_to_e,
    RoleE,
    next_e,
    RoleH,
    SessionMpstTen,
    10,
    5
);
create_send_mpst_session!(
    send_mpst_h_to_f,
    RoleF,
    next_f,
    RoleH,
    SessionMpstTen,
    10,
    6
);
create_send_mpst_session!(
    send_mpst_h_to_g,
    RoleG,
    next_g,
    RoleH,
    SessionMpstTen,
    10,
    7
);
create_send_mpst_session!(
    send_mpst_h_to_i,
    RoleI,
    next_i,
    RoleH,
    SessionMpstTen,
    10,
    8
);
create_send_mpst_session!(
    send_mpst_h_to_j,
    RoleJ,
    next_j,
    RoleH,
    SessionMpstTen,
    10,
    9
);
// I
create_send_mpst_session!(
    send_mpst_i_to_a,
    RoleA,
    next_a,
    RoleI,
    SessionMpstTen,
    10,
    1
);
create_send_mpst_session!(
    send_mpst_i_to_b,
    RoleB,
    next_b,
    RoleI,
    SessionMpstTen,
    10,
    2
);
create_send_mpst_session!(
    send_mpst_i_to_c,
    RoleC,
    next_c,
    RoleI,
    SessionMpstTen,
    10,
    3
);
create_send_mpst_session!(
    send_mpst_i_to_d,
    RoleD,
    next_d,
    RoleI,
    SessionMpstTen,
    10,
    4
);
create_send_mpst_session!(
    send_mpst_i_to_e,
    RoleE,
    next_e,
    RoleI,
    SessionMpstTen,
    10,
    5
);
create_send_mpst_session!(
    send_mpst_i_to_f,
    RoleF,
    next_f,
    RoleI,
    SessionMpstTen,
    10,
    6
);
create_send_mpst_session!(
    send_mpst_i_to_g,
    RoleG,
    next_g,
    RoleI,
    SessionMpstTen,
    10,
    7
);
create_send_mpst_session!(
    send_mpst_i_to_h,
    RoleH,
    next_h,
    RoleI,
    SessionMpstTen,
    10,
    8
);
create_send_mpst_session!(
    send_mpst_i_to_j,
    RoleJ,
    next_j,
    RoleI,
    SessionMpstTen,
    10,
    9
);
// J
create_send_mpst_session!(
    send_mpst_j_to_a,
    RoleA,
    next_a,
    RoleJ,
    SessionMpstTen,
    10,
    1
);
create_send_mpst_session!(
    send_mpst_j_to_b,
    RoleB,
    next_b,
    RoleJ,
    SessionMpstTen,
    10,
    2
);
create_send_mpst_session!(
    send_mpst_j_to_c,
    RoleC,
    next_c,
    RoleJ,
    SessionMpstTen,
    10,
    3
);
create_send_mpst_session!(
    send_mpst_j_to_d,
    RoleD,
    next_d,
    RoleJ,
    SessionMpstTen,
    10,
    4
);
create_send_mpst_session!(
    send_mpst_j_to_e,
    RoleE,
    next_e,
    RoleJ,
    SessionMpstTen,
    10,
    5
);
create_send_mpst_session!(
    send_mpst_j_to_f,
    RoleF,
    next_f,
    RoleJ,
    SessionMpstTen,
    10,
    6
);
create_send_mpst_session!(
    send_mpst_j_to_g,
    RoleG,
    next_g,
    RoleJ,
    SessionMpstTen,
    10,
    7
);
create_send_mpst_session!(
    send_mpst_j_to_h,
    RoleH,
    next_h,
    RoleJ,
    SessionMpstTen,
    10,
    8
);
create_send_mpst_session!(
    send_mpst_j_to_i,
    RoleI,
    next_i,
    RoleJ,
    SessionMpstTen,
    10,
    9
);

// Create new recv functions and related types
// A
create_recv_mpst_session!(
    recv_mpst_a_to_b,
    RoleB,
    next_b,
    RoleA,
    SessionMpstTen,
    10,
    1
);
create_recv_mpst_session!(
    recv_mpst_a_to_c,
    RoleC,
    next_c,
    RoleA,
    SessionMpstTen,
    10,
    2
);
create_recv_mpst_session!(
    recv_mpst_a_to_d,
    RoleD,
    next_d,
    RoleA,
    SessionMpstTen,
    10,
    3
);
create_recv_mpst_session!(
    recv_mpst_a_to_e,
    RoleE,
    next_e,
    RoleA,
    SessionMpstTen,
    10,
    4
);
create_recv_mpst_session!(
    recv_mpst_a_to_f,
    RoleF,
    next_f,
    RoleA,
    SessionMpstTen,
    10,
    5
);
create_recv_mpst_session!(
    recv_mpst_a_to_g,
    RoleG,
    next_g,
    RoleA,
    SessionMpstTen,
    10,
    6
);
create_recv_mpst_session!(
    recv_mpst_a_to_h,
    RoleH,
    next_h,
    RoleA,
    SessionMpstTen,
    10,
    7
);
create_recv_mpst_session!(
    recv_mpst_a_to_i,
    RoleI,
    next_i,
    RoleA,
    SessionMpstTen,
    10,
    8
);
create_recv_mpst_session!(
    recv_mpst_a_to_j,
    RoleJ,
    next_j,
    RoleA,
    SessionMpstTen,
    10,
    9
);
// B
create_recv_mpst_session!(
    recv_mpst_b_to_a,
    RoleA,
    next_a,
    RoleB,
    SessionMpstTen,
    10,
    1
);
create_recv_mpst_session!(
    recv_mpst_b_to_c,
    RoleC,
    next_c,
    RoleB,
    SessionMpstTen,
    10,
    2
);
create_recv_mpst_session!(
    recv_mpst_b_to_d,
    RoleD,
    next_d,
    RoleB,
    SessionMpstTen,
    10,
    3
);
create_recv_mpst_session!(
    recv_mpst_b_to_e,
    RoleE,
    next_e,
    RoleB,
    SessionMpstTen,
    10,
    4
);
create_recv_mpst_session!(
    recv_mpst_b_to_f,
    RoleF,
    next_f,
    RoleB,
    SessionMpstTen,
    10,
    5
);
create_recv_mpst_session!(
    recv_mpst_b_to_g,
    RoleG,
    next_g,
    RoleB,
    SessionMpstTen,
    10,
    6
);
create_recv_mpst_session!(
    recv_mpst_b_to_h,
    RoleH,
    next_h,
    RoleB,
    SessionMpstTen,
    10,
    7
);
create_recv_mpst_session!(
    recv_mpst_b_to_i,
    RoleI,
    next_i,
    RoleB,
    SessionMpstTen,
    10,
    8
);
create_recv_mpst_session!(
    recv_mpst_b_to_j,
    RoleJ,
    next_j,
    RoleB,
    SessionMpstTen,
    10,
    9
);
// C
create_recv_mpst_session!(
    recv_mpst_c_to_a,
    RoleA,
    next_a,
    RoleC,
    SessionMpstTen,
    10,
    1
);
create_recv_mpst_session!(
    recv_mpst_c_to_b,
    RoleB,
    next_b,
    RoleC,
    SessionMpstTen,
    10,
    2
);
create_recv_mpst_session!(
    recv_mpst_c_to_d,
    RoleD,
    next_d,
    RoleC,
    SessionMpstTen,
    10,
    3
);
create_recv_mpst_session!(
    recv_mpst_c_to_e,
    RoleE,
    next_e,
    RoleC,
    SessionMpstTen,
    10,
    4
);
create_recv_mpst_session!(
    recv_mpst_c_to_f,
    RoleF,
    next_f,
    RoleC,
    SessionMpstTen,
    10,
    5
);
create_recv_mpst_session!(
    recv_mpst_c_to_g,
    RoleG,
    next_g,
    RoleC,
    SessionMpstTen,
    10,
    6
);
create_recv_mpst_session!(
    recv_mpst_c_to_h,
    RoleH,
    next_h,
    RoleC,
    SessionMpstTen,
    10,
    7
);
create_recv_mpst_session!(
    recv_mpst_c_to_i,
    RoleI,
    next_i,
    RoleC,
    SessionMpstTen,
    10,
    8
);
create_recv_mpst_session!(
    recv_mpst_c_to_j,
    RoleJ,
    next_j,
    RoleC,
    SessionMpstTen,
    10,
    9
);
// D
create_recv_mpst_session!(
    recv_mpst_d_to_a,
    RoleA,
    next_a,
    RoleD,
    SessionMpstTen,
    10,
    1
);
create_recv_mpst_session!(
    recv_mpst_d_to_b,
    RoleB,
    next_b,
    RoleD,
    SessionMpstTen,
    10,
    2
);
create_recv_mpst_session!(
    recv_mpst_d_to_c,
    RoleC,
    next_c,
    RoleD,
    SessionMpstTen,
    10,
    3
);
create_recv_mpst_session!(
    recv_mpst_d_to_e,
    RoleE,
    next_e,
    RoleD,
    SessionMpstTen,
    10,
    4
);
create_recv_mpst_session!(
    recv_mpst_d_to_f,
    RoleF,
    next_f,
    RoleD,
    SessionMpstTen,
    10,
    5
);
create_recv_mpst_session!(
    recv_mpst_d_to_g,
    RoleG,
    next_g,
    RoleD,
    SessionMpstTen,
    10,
    6
);
create_recv_mpst_session!(
    recv_mpst_d_to_h,
    RoleH,
    next_h,
    RoleD,
    SessionMpstTen,
    10,
    7
);
create_recv_mpst_session!(
    recv_mpst_d_to_i,
    RoleI,
    next_i,
    RoleD,
    SessionMpstTen,
    10,
    8
);
create_recv_mpst_session!(
    recv_mpst_d_to_j,
    RoleJ,
    next_j,
    RoleD,
    SessionMpstTen,
    10,
    9
);
// E
create_recv_mpst_session!(
    recv_mpst_e_to_a,
    RoleA,
    next_a,
    RoleE,
    SessionMpstTen,
    10,
    1
);
create_recv_mpst_session!(
    recv_mpst_e_to_b,
    RoleB,
    next_b,
    RoleE,
    SessionMpstTen,
    10,
    2
);
create_recv_mpst_session!(
    recv_mpst_e_to_c,
    RoleC,
    next_c,
    RoleE,
    SessionMpstTen,
    10,
    3
);
create_recv_mpst_session!(
    recv_mpst_e_to_d,
    RoleD,
    next_d,
    RoleE,
    SessionMpstTen,
    10,
    4
);
create_recv_mpst_session!(
    recv_mpst_e_to_f,
    RoleF,
    next_f,
    RoleE,
    SessionMpstTen,
    10,
    5
);
create_recv_mpst_session!(
    recv_mpst_e_to_g,
    RoleG,
    next_g,
    RoleE,
    SessionMpstTen,
    10,
    6
);
create_recv_mpst_session!(
    recv_mpst_e_to_h,
    RoleH,
    next_h,
    RoleE,
    SessionMpstTen,
    10,
    7
);
create_recv_mpst_session!(
    recv_mpst_e_to_i,
    RoleI,
    next_i,
    RoleE,
    SessionMpstTen,
    10,
    8
);
create_recv_mpst_session!(
    recv_mpst_e_to_j,
    RoleJ,
    next_j,
    RoleE,
    SessionMpstTen,
    10,
    9
);
// F
create_recv_mpst_session!(
    recv_mpst_f_to_a,
    RoleA,
    next_a,
    RoleF,
    SessionMpstTen,
    10,
    1
);
create_recv_mpst_session!(
    recv_mpst_f_to_b,
    RoleB,
    next_b,
    RoleF,
    SessionMpstTen,
    10,
    2
);
create_recv_mpst_session!(
    recv_mpst_f_to_c,
    RoleC,
    next_c,
    RoleF,
    SessionMpstTen,
    10,
    3
);
create_recv_mpst_session!(
    recv_mpst_f_to_d,
    RoleD,
    next_d,
    RoleF,
    SessionMpstTen,
    10,
    4
);
create_recv_mpst_session!(
    recv_mpst_f_to_e,
    RoleE,
    next_e,
    RoleF,
    SessionMpstTen,
    10,
    5
);
create_recv_mpst_session!(
    recv_mpst_f_to_g,
    RoleG,
    next_g,
    RoleF,
    SessionMpstTen,
    10,
    6
);
create_recv_mpst_session!(
    recv_mpst_f_to_h,
    RoleH,
    next_h,
    RoleF,
    SessionMpstTen,
    10,
    7
);
create_recv_mpst_session!(
    recv_mpst_f_to_i,
    RoleI,
    next_i,
    RoleF,
    SessionMpstTen,
    10,
    8
);
create_recv_mpst_session!(
    recv_mpst_f_to_j,
    RoleJ,
    next_j,
    RoleF,
    SessionMpstTen,
    10,
    9
);
// G
create_recv_mpst_session!(
    recv_mpst_g_to_a,
    RoleA,
    next_a,
    RoleG,
    SessionMpstTen,
    10,
    1
);
create_recv_mpst_session!(
    recv_mpst_g_to_b,
    RoleB,
    next_b,
    RoleG,
    SessionMpstTen,
    10,
    2
);
create_recv_mpst_session!(
    recv_mpst_g_to_c,
    RoleC,
    next_c,
    RoleG,
    SessionMpstTen,
    10,
    3
);
create_recv_mpst_session!(
    recv_mpst_g_to_d,
    RoleD,
    next_d,
    RoleG,
    SessionMpstTen,
    10,
    4
);
create_recv_mpst_session!(
    recv_mpst_g_to_e,
    RoleE,
    next_e,
    RoleG,
    SessionMpstTen,
    10,
    5
);
create_recv_mpst_session!(
    recv_mpst_g_to_f,
    RoleF,
    next_f,
    RoleG,
    SessionMpstTen,
    10,
    6
);
create_recv_mpst_session!(
    recv_mpst_g_to_h,
    RoleH,
    next_h,
    RoleG,
    SessionMpstTen,
    10,
    7
);
create_recv_mpst_session!(
    recv_mpst_g_to_i,
    RoleI,
    next_i,
    RoleG,
    SessionMpstTen,
    10,
    8
);
create_recv_mpst_session!(
    recv_mpst_g_to_j,
    RoleJ,
    next_j,
    RoleG,
    SessionMpstTen,
    10,
    9
);
// H
create_recv_mpst_session!(
    recv_mpst_h_to_a,
    RoleA,
    next_a,
    RoleH,
    SessionMpstTen,
    10,
    1
);
create_recv_mpst_session!(
    recv_mpst_h_to_b,
    RoleB,
    next_b,
    RoleH,
    SessionMpstTen,
    10,
    2
);
create_recv_mpst_session!(
    recv_mpst_h_to_c,
    RoleC,
    next_c,
    RoleH,
    SessionMpstTen,
    10,
    3
);
create_recv_mpst_session!(
    recv_mpst_h_to_d,
    RoleD,
    next_d,
    RoleH,
    SessionMpstTen,
    10,
    4
);
create_recv_mpst_session!(
    recv_mpst_h_to_e,
    RoleE,
    next_e,
    RoleH,
    SessionMpstTen,
    10,
    5
);
create_recv_mpst_session!(
    recv_mpst_h_to_f,
    RoleF,
    next_f,
    RoleH,
    SessionMpstTen,
    10,
    6
);
create_recv_mpst_session!(
    recv_mpst_h_to_g,
    RoleG,
    next_g,
    RoleH,
    SessionMpstTen,
    10,
    7
);
create_recv_mpst_session!(
    recv_mpst_h_to_i,
    RoleI,
    next_i,
    RoleH,
    SessionMpstTen,
    10,
    8
);
create_recv_mpst_session!(
    recv_mpst_h_to_j,
    RoleJ,
    next_j,
    RoleH,
    SessionMpstTen,
    10,
    9
);
// I
create_recv_mpst_session!(
    recv_mpst_i_to_a,
    RoleA,
    next_a,
    RoleI,
    SessionMpstTen,
    10,
    1
);
create_recv_mpst_session!(
    recv_mpst_i_to_b,
    RoleB,
    next_b,
    RoleI,
    SessionMpstTen,
    10,
    2
);
create_recv_mpst_session!(
    recv_mpst_i_to_c,
    RoleC,
    next_c,
    RoleI,
    SessionMpstTen,
    10,
    3
);
create_recv_mpst_session!(
    recv_mpst_i_to_d,
    RoleD,
    next_d,
    RoleI,
    SessionMpstTen,
    10,
    4
);
create_recv_mpst_session!(
    recv_mpst_i_to_e,
    RoleE,
    next_e,
    RoleI,
    SessionMpstTen,
    10,
    5
);
create_recv_mpst_session!(
    recv_mpst_i_to_f,
    RoleF,
    next_f,
    RoleI,
    SessionMpstTen,
    10,
    6
);
create_recv_mpst_session!(
    recv_mpst_i_to_g,
    RoleG,
    next_g,
    RoleI,
    SessionMpstTen,
    10,
    7
);
create_recv_mpst_session!(
    recv_mpst_i_to_h,
    RoleH,
    next_h,
    RoleI,
    SessionMpstTen,
    10,
    8
);
create_recv_mpst_session!(
    recv_mpst_i_to_j,
    RoleJ,
    next_j,
    RoleI,
    SessionMpstTen,
    10,
    9
);
// J
create_recv_mpst_session!(
    recv_mpst_j_to_a,
    RoleA,
    next_a,
    RoleJ,
    SessionMpstTen,
    10,
    1
);
create_recv_mpst_session!(
    recv_mpst_j_to_b,
    RoleB,
    next_b,
    RoleJ,
    SessionMpstTen,
    10,
    2
);
create_recv_mpst_session!(
    recv_mpst_j_to_c,
    RoleC,
    next_c,
    RoleJ,
    SessionMpstTen,
    10,
    3
);
create_recv_mpst_session!(
    recv_mpst_j_to_d,
    RoleD,
    next_d,
    RoleJ,
    SessionMpstTen,
    10,
    4
);
create_recv_mpst_session!(
    recv_mpst_j_to_e,
    RoleE,
    next_e,
    RoleJ,
    SessionMpstTen,
    10,
    5
);
create_recv_mpst_session!(
    recv_mpst_j_to_f,
    RoleF,
    next_f,
    RoleJ,
    SessionMpstTen,
    10,
    6
);
create_recv_mpst_session!(
    recv_mpst_j_to_g,
    RoleG,
    next_g,
    RoleJ,
    SessionMpstTen,
    10,
    7
);
create_recv_mpst_session!(
    recv_mpst_j_to_h,
    RoleH,
    next_h,
    RoleJ,
    SessionMpstTen,
    10,
    8
);
create_recv_mpst_session!(
    recv_mpst_j_to_i,
    RoleI,
    next_i,
    RoleJ,
    SessionMpstTen,
    10,
    9
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstTen, 10);

// Create fork function
bundle_fork_multi!(fork_mpst, fork_simple, SessionMpstTen, 10);

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

// Types
// Binary
// A
enum BranchingJforA {
    More(
        SessionMpstTen<
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursAtoJ>>,
            RoleJ<
                RoleJ<
                    RoleB<
                        RoleB<
                            RoleC<
                                RoleC<
                                    RoleD<
                                        RoleD<
                                            RoleE<
                                                RoleE<
                                                    RoleF<
                                                        RoleF<
                                                            RoleG<
                                                                RoleG<
                                                                    RoleH<
                                                                        RoleH<
                                                                            RoleI<
                                                                                RoleI<
                                                                                    RoleJ<RoleEnd>,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
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
    Done(SessionMpstTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoJ = Recv<BranchingJforA, End>;
// B
enum BranchingJforB {
    More(
        SessionMpstTen<
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursBtoJ>>,
            RoleJ<
                RoleJ<
                    RoleA<
                        RoleA<
                            RoleC<
                                RoleC<
                                    RoleD<
                                        RoleD<
                                            RoleE<
                                                RoleE<
                                                    RoleF<
                                                        RoleF<
                                                            RoleG<
                                                                RoleG<
                                                                    RoleH<
                                                                        RoleH<
                                                                            RoleI<
                                                                                RoleI<
                                                                                    RoleJ<RoleEnd>,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
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
    Done(SessionMpstTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoJ = Recv<BranchingJforB, End>;
// C
enum BranchingJforC {
    More(
        SessionMpstTen<
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursCtoJ>>,
            RoleJ<
                RoleJ<
                    RoleA<
                        RoleA<
                            RoleB<
                                RoleB<
                                    RoleD<
                                        RoleD<
                                            RoleE<
                                                RoleE<
                                                    RoleF<
                                                        RoleF<
                                                            RoleG<
                                                                RoleG<
                                                                    RoleH<
                                                                        RoleH<
                                                                            RoleI<
                                                                                RoleI<
                                                                                    RoleJ<RoleEnd>,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
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
    Done(SessionMpstTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoJ = Recv<BranchingJforC, End>;
// D
enum BranchingJforD {
    More(
        SessionMpstTen<
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursDtoJ>>,
            RoleJ<
                RoleJ<
                    RoleA<
                        RoleA<
                            RoleB<
                                RoleB<
                                    RoleC<
                                        RoleC<
                                            RoleE<
                                                RoleE<
                                                    RoleF<
                                                        RoleF<
                                                            RoleG<
                                                                RoleG<
                                                                    RoleH<
                                                                        RoleH<
                                                                            RoleI<
                                                                                RoleI<
                                                                                    RoleJ<RoleEnd>,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
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
    Done(SessionMpstTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoJ = Recv<BranchingJforD, End>;
// E
enum BranchingJforE {
    More(
        SessionMpstTen<
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursEtoJ>>,
            RoleJ<
                RoleJ<
                    RoleA<
                        RoleA<
                            RoleB<
                                RoleB<
                                    RoleC<
                                        RoleC<
                                            RoleD<
                                                RoleD<
                                                    RoleF<
                                                        RoleF<
                                                            RoleG<
                                                                RoleG<
                                                                    RoleH<
                                                                        RoleH<
                                                                            RoleI<
                                                                                RoleI<
                                                                                    RoleJ<RoleEnd>,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
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
    Done(SessionMpstTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoJ = Recv<BranchingJforE, End>;
// F
enum BranchingJforF {
    More(
        SessionMpstTen<
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursFtoJ>>,
            RoleJ<
                RoleJ<
                    RoleA<
                        RoleA<
                            RoleB<
                                RoleB<
                                    RoleC<
                                        RoleC<
                                            RoleD<
                                                RoleD<
                                                    RoleE<
                                                        RoleE<
                                                            RoleG<
                                                                RoleG<
                                                                    RoleH<
                                                                        RoleH<
                                                                            RoleI<
                                                                                RoleI<
                                                                                    RoleJ<RoleEnd>,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
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
    Done(SessionMpstTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoJ = Recv<BranchingJforF, End>;
// G
enum BranchingJforG {
    More(
        SessionMpstTen<
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursGtoJ>>,
            RoleJ<
                RoleJ<
                    RoleA<
                        RoleA<
                            RoleB<
                                RoleB<
                                    RoleC<
                                        RoleC<
                                            RoleD<
                                                RoleD<
                                                    RoleE<
                                                        RoleE<
                                                            RoleF<
                                                                RoleF<
                                                                    RoleH<
                                                                        RoleH<
                                                                            RoleI<
                                                                                RoleI<
                                                                                    RoleJ<RoleEnd>,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
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
    Done(SessionMpstTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoJ = Recv<BranchingJforG, End>;
// H
enum BranchingJforH {
    More(
        SessionMpstTen<
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursHtoJ>>,
            RoleJ<
                RoleJ<
                    RoleA<
                        RoleA<
                            RoleB<
                                RoleB<
                                    RoleC<
                                        RoleC<
                                            RoleD<
                                                RoleD<
                                                    RoleE<
                                                        RoleE<
                                                            RoleF<
                                                                RoleF<
                                                                    RoleG<
                                                                        RoleG<
                                                                            RoleI<
                                                                                RoleI<
                                                                                    RoleJ<RoleEnd>,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
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
    Done(SessionMpstTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameH>),
}
type RecursHtoJ = Recv<BranchingJforH, End>;
// I
enum BranchingJforI {
    More(
        SessionMpstTen<
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), RecursItoJ>>,
            RoleJ<
                RoleJ<
                    RoleA<
                        RoleA<
                            RoleB<
                                RoleB<
                                    RoleC<
                                        RoleC<
                                            RoleD<
                                                RoleD<
                                                    RoleE<
                                                        RoleE<
                                                            RoleF<
                                                                RoleF<
                                                                    RoleG<
                                                                        RoleG<
                                                                            RoleH<
                                                                                RoleH<
                                                                                    RoleJ<RoleEnd>,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
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
    Done(SessionMpstTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameI>),
}
type RecursItoJ = Recv<BranchingJforI, End>;
// J
type ChooseJforAtoJ = Send<BranchingJforA, End>;
type ChooseJforBtoJ = Send<BranchingJforB, End>;
type ChooseJforCtoJ = Send<BranchingJforC, End>;
type ChooseJforDtoJ = Send<BranchingJforD, End>;
type ChooseJforEtoJ = Send<BranchingJforE, End>;
type ChooseJforFtoJ = Send<BranchingJforF, End>;
type ChooseJforGtoJ = Send<BranchingJforG, End>;
type ChooseJforHtoJ = Send<BranchingJforH, End>;
type ChooseJforItoJ = Send<BranchingJforI, End>;

// Creating the MP sessions
type EndpointA =
    SessionMpstTen<End, End, End, End, End, End, End, End, RecursAtoJ, RoleJ<RoleEnd>, NameA>;
type EndpointB =
    SessionMpstTen<End, End, End, End, End, End, End, End, RecursBtoJ, RoleJ<RoleEnd>, NameB>;
type EndpointC =
    SessionMpstTen<End, End, End, End, End, End, End, End, RecursCtoJ, RoleJ<RoleEnd>, NameC>;
type EndpointD =
    SessionMpstTen<End, End, End, End, End, End, End, End, RecursDtoJ, RoleJ<RoleEnd>, NameD>;
type EndpointE =
    SessionMpstTen<End, End, End, End, End, End, End, End, RecursEtoJ, RoleJ<RoleEnd>, NameE>;
type EndpointF =
    SessionMpstTen<End, End, End, End, End, End, End, End, RecursFtoJ, RoleJ<RoleEnd>, NameF>;
type EndpointG =
    SessionMpstTen<End, End, End, End, End, End, End, End, RecursGtoJ, RoleJ<RoleEnd>, NameG>;
type EndpointH =
    SessionMpstTen<End, End, End, End, End, End, End, End, RecursHtoJ, RoleJ<RoleEnd>, NameH>;
type EndpointI =
    SessionMpstTen<End, End, End, End, End, End, End, End, RecursItoJ, RoleJ<RoleEnd>, NameI>;
type EndpointJ = SessionMpstTen<
    ChooseJforAtoJ,
    ChooseJforBtoJ,
    ChooseJforCtoJ,
    ChooseJforDtoJ,
    ChooseJforEtoJ,
    ChooseJforFtoJ,
    ChooseJforGtoJ,
    ChooseJforHtoJ,
    ChooseJforItoJ,
    RoleA<RoleB<RoleC<RoleD<RoleE<RoleF<RoleG<RoleH<RoleI<RoleEnd>>>>>>>>>,
    NameJ,
>;

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_to_j, {
          BranchingJforA::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingJforA::More(s) => {
            let (_, s) = recv_mpst_a_to_j(s)?;
            let s = send_mpst_a_to_j((), s);
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
            simple_five_endpoint_a(s)
        },
    })
}

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_to_j, {
          BranchingJforB::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingJforB::More(s) => {
            let (_, s) = recv_mpst_b_to_j(s)?;
            let s = send_mpst_b_to_j((), s);
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
            simple_five_endpoint_b(s)
        },
    })
}

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_to_j, {
          BranchingJforC::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingJforC::More(s) => {
            let (_, s) = recv_mpst_c_to_j(s)?;
            let s = send_mpst_c_to_j((), s);
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
            simple_five_endpoint_c(s)
        },
    })
}

fn simple_five_endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_to_j, {
          BranchingJforD::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingJforD::More(s) => {
            let (_, s) = recv_mpst_d_to_j(s)?;
            let s = send_mpst_d_to_j((), s);
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
            simple_five_endpoint_d(s)
        },
    })
}

fn simple_five_endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_e_to_j, {
          BranchingJforE::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingJforE::More(s) => {
            let (_, s) = recv_mpst_e_to_j(s)?;
            let s = send_mpst_e_to_j((), s);
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
            simple_five_endpoint_e(s)
        },
    })
}

fn simple_five_endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_f_to_j, {
          BranchingJforF::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingJforF::More(s) => {
            let (_, s) = recv_mpst_f_to_j(s)?;
            let s = send_mpst_f_to_j((), s);
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
            simple_five_endpoint_f(s)
        },
    })
}

fn simple_five_endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_g_to_j, {
          BranchingJforG::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingJforG::More(s) => {
            let (_, s) = recv_mpst_g_to_j(s)?;
            let s = send_mpst_g_to_j((), s);
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
            simple_five_endpoint_g(s)
        },
    })
}

fn simple_five_endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_h_to_j, {
          BranchingJforH::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingJforH::More(s) => {
            let (_, s) = recv_mpst_h_to_j(s)?;
            let s = send_mpst_h_to_j((), s);
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
            simple_five_endpoint_h(s)
        },
    })
}

fn simple_five_endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_i_to_j, {
          BranchingJforI::Done(s) => {
            close_mpst_multi(s)
        },
        BranchingJforI::More(s) => {
            let (_, s) = recv_mpst_i_to_j(s)?;
            let s = send_mpst_i_to_j((), s);
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
            simple_five_endpoint_i(s)
        },
    })
}

fn simple_five_endpoint_j(s: EndpointJ) -> Result<(), Box<dyn Error>> {
    recurs_j(s, SIZE)
}

fn recurs_j(s: EndpointJ, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_j_to_a,
                send_mpst_j_to_b,
                send_mpst_j_to_c,
                send_mpst_j_to_d,
                send_mpst_j_to_e,
                send_mpst_j_to_f,
                send_mpst_j_to_g,
                send_mpst_j_to_h,
                send_mpst_j_to_i, =>
                  BranchingJforA::Done,
                  BranchingJforB::Done,
                  BranchingJforC::Done,
                  BranchingJforD::Done,
                  BranchingJforE::Done,
                  BranchingJforF::Done,
                  BranchingJforG::Done,
                  BranchingJforH::Done,
                  BranchingJforI::Done, =>
                RoleA,
                RoleB,
                RoleC,
                RoleD,
                RoleE,
                RoleF,
                RoleG,
                RoleH,
                RoleI, =>
                RoleJ,
                 SessionMpstTen,
                10
            );

            close_mpst_multi(s)
        }
        i => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_j_to_a,
                send_mpst_j_to_b,
                send_mpst_j_to_c,
                send_mpst_j_to_d,
                send_mpst_j_to_e,
                send_mpst_j_to_f,
                send_mpst_j_to_g,
                send_mpst_j_to_h,
                send_mpst_j_to_i, =>
                  BranchingJforA::More,
                  BranchingJforB::More,
                  BranchingJforC::More,
                  BranchingJforD::More,
                  BranchingJforE::More,
                  BranchingJforF::More,
                  BranchingJforG::More,
                  BranchingJforH::More,
                  BranchingJforI::More, =>
                  RoleA,
                  RoleB,
                  RoleC,
                  RoleD,
                  RoleE,
                  RoleF,
                  RoleG,
                  RoleH,
                  RoleI, =>
                  RoleJ,
                 SessionMpstTen,
                10
            );

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

            recurs_j(s, i - 1)
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

    for _ in 0..45 {
        let (thread, s): (JoinHandle<()>, RecursB) = fork_with_thread_id(black_box(binary_a_to_b));

        threads.push(thread);
        sessions.push(s)
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

static SIZE: i64 = 100;

fn long_simple_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("long ten simple protocol MPST {}", SIZE), |b| {
        b.iter(|| all_mpst())
    });
}

fn long_simple_protocol_binary(c: &mut Criterion) {
    c.bench_function(&format!("long ten simple protocol binary {}", SIZE), |b| {
        b.iter(|| all_binaries())
    });
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(1500, 0))
}

criterion_group! {
    name = long_ten_simple_protocols;
    config = long_warmup();
    targets = long_simple_protocol_mpst, long_simple_protocol_binary
}
criterion_main!(long_ten_simple_protocols);
