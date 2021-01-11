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
create_sessionmpst!(SessionMpstNine, 9);

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

// Create new send functions
// A
create_send_mpst_session!(
    send_mpst_a_to_b,
    RoleB,
    next_b,
    RoleA,
    SessionMpstNine,
    9,
    1
);
create_send_mpst_session!(
    send_mpst_a_to_c,
    RoleC,
    next_c,
    RoleA,
    SessionMpstNine,
    9,
    2
);
create_send_mpst_session!(
    send_mpst_a_to_d,
    RoleD,
    next_d,
    RoleA,
    SessionMpstNine,
    9,
    3
);
create_send_mpst_session!(
    send_mpst_a_to_e,
    RoleE,
    next_e,
    RoleA,
    SessionMpstNine,
    9,
    4
);
create_send_mpst_session!(
    send_mpst_a_to_f,
    RoleF,
    next_f,
    RoleA,
    SessionMpstNine,
    9,
    5
);
create_send_mpst_session!(
    send_mpst_a_to_g,
    RoleG,
    next_g,
    RoleA,
    SessionMpstNine,
    9,
    6
);
create_send_mpst_session!(
    send_mpst_a_to_h,
    RoleH,
    next_h,
    RoleA,
    SessionMpstNine,
    9,
    7
);
create_send_mpst_session!(
    send_mpst_a_to_i,
    RoleI,
    next_i,
    RoleA,
    SessionMpstNine,
    9,
    8
);
// B
create_send_mpst_session!(
    send_mpst_b_to_a,
    RoleA,
    next_a,
    RoleB,
    SessionMpstNine,
    9,
    1
);
create_send_mpst_session!(
    send_mpst_b_to_c,
    RoleC,
    next_c,
    RoleB,
    SessionMpstNine,
    9,
    2
);
create_send_mpst_session!(
    send_mpst_b_to_d,
    RoleD,
    next_d,
    RoleB,
    SessionMpstNine,
    9,
    3
);
create_send_mpst_session!(
    send_mpst_b_to_e,
    RoleE,
    next_e,
    RoleB,
    SessionMpstNine,
    9,
    4
);
create_send_mpst_session!(
    send_mpst_b_to_f,
    RoleF,
    next_f,
    RoleB,
    SessionMpstNine,
    9,
    5
);
create_send_mpst_session!(
    send_mpst_b_to_g,
    RoleG,
    next_g,
    RoleB,
    SessionMpstNine,
    9,
    6
);
create_send_mpst_session!(
    send_mpst_b_to_h,
    RoleH,
    next_h,
    RoleB,
    SessionMpstNine,
    9,
    7
);
create_send_mpst_session!(
    send_mpst_b_to_i,
    RoleI,
    next_i,
    RoleB,
    SessionMpstNine,
    9,
    8
);
// C
create_send_mpst_session!(
    send_mpst_c_to_a,
    RoleA,
    next_a,
    RoleC,
    SessionMpstNine,
    9,
    1
);
create_send_mpst_session!(
    send_mpst_c_to_b,
    RoleB,
    next_b,
    RoleC,
    SessionMpstNine,
    9,
    2
);
create_send_mpst_session!(
    send_mpst_c_to_d,
    RoleD,
    next_d,
    RoleC,
    SessionMpstNine,
    9,
    3
);
create_send_mpst_session!(
    send_mpst_c_to_e,
    RoleE,
    next_e,
    RoleC,
    SessionMpstNine,
    9,
    4
);
create_send_mpst_session!(
    send_mpst_c_to_f,
    RoleF,
    next_f,
    RoleC,
    SessionMpstNine,
    9,
    5
);
create_send_mpst_session!(
    send_mpst_c_to_g,
    RoleG,
    next_g,
    RoleC,
    SessionMpstNine,
    9,
    6
);
create_send_mpst_session!(
    send_mpst_c_to_h,
    RoleH,
    next_h,
    RoleC,
    SessionMpstNine,
    9,
    7
);
create_send_mpst_session!(
    send_mpst_c_to_i,
    RoleI,
    next_i,
    RoleC,
    SessionMpstNine,
    9,
    8
);
// D
create_send_mpst_session!(
    send_mpst_d_to_a,
    RoleA,
    next_a,
    RoleD,
    SessionMpstNine,
    9,
    1
);
create_send_mpst_session!(
    send_mpst_d_to_b,
    RoleB,
    next_b,
    RoleD,
    SessionMpstNine,
    9,
    2
);
create_send_mpst_session!(
    send_mpst_d_to_c,
    RoleC,
    next_c,
    RoleD,
    SessionMpstNine,
    9,
    3
);
create_send_mpst_session!(
    send_mpst_d_to_e,
    RoleE,
    next_e,
    RoleD,
    SessionMpstNine,
    9,
    4
);
create_send_mpst_session!(
    send_mpst_d_to_f,
    RoleF,
    next_f,
    RoleD,
    SessionMpstNine,
    9,
    5
);
create_send_mpst_session!(
    send_mpst_d_to_g,
    RoleG,
    next_g,
    RoleD,
    SessionMpstNine,
    9,
    6
);
create_send_mpst_session!(
    send_mpst_d_to_h,
    RoleH,
    next_h,
    RoleD,
    SessionMpstNine,
    9,
    7
);
create_send_mpst_session!(
    send_mpst_d_to_i,
    RoleI,
    next_i,
    RoleD,
    SessionMpstNine,
    9,
    8
);
// E
create_send_mpst_session!(
    send_mpst_e_to_a,
    RoleA,
    next_a,
    RoleE,
    SessionMpstNine,
    9,
    1
);
create_send_mpst_session!(
    send_mpst_e_to_b,
    RoleB,
    next_b,
    RoleE,
    SessionMpstNine,
    9,
    2
);
create_send_mpst_session!(
    send_mpst_e_to_c,
    RoleC,
    next_c,
    RoleE,
    SessionMpstNine,
    9,
    3
);
create_send_mpst_session!(
    send_mpst_e_to_d,
    RoleD,
    next_d,
    RoleE,
    SessionMpstNine,
    9,
    4
);
create_send_mpst_session!(
    send_mpst_e_to_f,
    RoleF,
    next_f,
    RoleE,
    SessionMpstNine,
    9,
    5
);
create_send_mpst_session!(
    send_mpst_e_to_g,
    RoleG,
    next_g,
    RoleE,
    SessionMpstNine,
    9,
    6
);
create_send_mpst_session!(
    send_mpst_e_to_h,
    RoleH,
    next_h,
    RoleE,
    SessionMpstNine,
    9,
    7
);
create_send_mpst_session!(
    send_mpst_e_to_i,
    RoleI,
    next_i,
    RoleE,
    SessionMpstNine,
    9,
    8
);
// F
create_send_mpst_session!(
    send_mpst_f_to_a,
    RoleA,
    next_a,
    RoleF,
    SessionMpstNine,
    9,
    1
);
create_send_mpst_session!(
    send_mpst_f_to_b,
    RoleB,
    next_b,
    RoleF,
    SessionMpstNine,
    9,
    2
);
create_send_mpst_session!(
    send_mpst_f_to_c,
    RoleC,
    next_c,
    RoleF,
    SessionMpstNine,
    9,
    3
);
create_send_mpst_session!(
    send_mpst_f_to_d,
    RoleD,
    next_d,
    RoleF,
    SessionMpstNine,
    9,
    4
);
create_send_mpst_session!(
    send_mpst_f_to_e,
    RoleE,
    next_e,
    RoleF,
    SessionMpstNine,
    9,
    5
);
create_send_mpst_session!(
    send_mpst_f_to_g,
    RoleG,
    next_g,
    RoleF,
    SessionMpstNine,
    9,
    6
);
create_send_mpst_session!(
    send_mpst_f_to_h,
    RoleH,
    next_h,
    RoleF,
    SessionMpstNine,
    9,
    7
);
create_send_mpst_session!(
    send_mpst_f_to_i,
    RoleI,
    next_i,
    RoleF,
    SessionMpstNine,
    9,
    8
);
// G
create_send_mpst_session!(
    send_mpst_g_to_a,
    RoleA,
    next_a,
    RoleG,
    SessionMpstNine,
    9,
    1
);
create_send_mpst_session!(
    send_mpst_g_to_b,
    RoleB,
    next_b,
    RoleG,
    SessionMpstNine,
    9,
    2
);
create_send_mpst_session!(
    send_mpst_g_to_c,
    RoleC,
    next_c,
    RoleG,
    SessionMpstNine,
    9,
    3
);
create_send_mpst_session!(
    send_mpst_g_to_d,
    RoleD,
    next_d,
    RoleG,
    SessionMpstNine,
    9,
    4
);
create_send_mpst_session!(
    send_mpst_g_to_e,
    RoleE,
    next_e,
    RoleG,
    SessionMpstNine,
    9,
    5
);
create_send_mpst_session!(
    send_mpst_g_to_f,
    RoleF,
    next_f,
    RoleG,
    SessionMpstNine,
    9,
    6
);
create_send_mpst_session!(
    send_mpst_g_to_h,
    RoleH,
    next_h,
    RoleG,
    SessionMpstNine,
    9,
    7
);
create_send_mpst_session!(
    send_mpst_g_to_i,
    RoleI,
    next_i,
    RoleG,
    SessionMpstNine,
    9,
    8
);
// H
create_send_mpst_session!(
    send_mpst_h_to_a,
    RoleA,
    next_a,
    RoleH,
    SessionMpstNine,
    9,
    1
);
create_send_mpst_session!(
    send_mpst_h_to_b,
    RoleB,
    next_b,
    RoleH,
    SessionMpstNine,
    9,
    2
);
create_send_mpst_session!(
    send_mpst_h_to_c,
    RoleC,
    next_c,
    RoleH,
    SessionMpstNine,
    9,
    3
);
create_send_mpst_session!(
    send_mpst_h_to_d,
    RoleD,
    next_d,
    RoleH,
    SessionMpstNine,
    9,
    4
);
create_send_mpst_session!(
    send_mpst_h_to_e,
    RoleE,
    next_e,
    RoleH,
    SessionMpstNine,
    9,
    5
);
create_send_mpst_session!(
    send_mpst_h_to_f,
    RoleF,
    next_f,
    RoleH,
    SessionMpstNine,
    9,
    6
);
create_send_mpst_session!(
    send_mpst_h_to_g,
    RoleG,
    next_g,
    RoleH,
    SessionMpstNine,
    9,
    7
);
create_send_mpst_session!(
    send_mpst_h_to_i,
    RoleI,
    next_i,
    RoleH,
    SessionMpstNine,
    9,
    8
);
// I
create_send_mpst_session!(
    send_mpst_i_to_a,
    RoleA,
    next_a,
    RoleI,
    SessionMpstNine,
    9,
    1
);
create_send_mpst_session!(
    send_mpst_i_to_b,
    RoleB,
    next_b,
    RoleI,
    SessionMpstNine,
    9,
    2
);
create_send_mpst_session!(
    send_mpst_i_to_c,
    RoleC,
    next_c,
    RoleI,
    SessionMpstNine,
    9,
    3
);
create_send_mpst_session!(
    send_mpst_i_to_d,
    RoleD,
    next_d,
    RoleI,
    SessionMpstNine,
    9,
    4
);
create_send_mpst_session!(
    send_mpst_i_to_e,
    RoleE,
    next_e,
    RoleI,
    SessionMpstNine,
    9,
    5
);
create_send_mpst_session!(
    send_mpst_i_to_f,
    RoleF,
    next_f,
    RoleI,
    SessionMpstNine,
    9,
    6
);
create_send_mpst_session!(
    send_mpst_i_to_g,
    RoleG,
    next_g,
    RoleI,
    SessionMpstNine,
    9,
    7
);
create_send_mpst_session!(
    send_mpst_i_to_h,
    RoleH,
    next_h,
    RoleI,
    SessionMpstNine,
    9,
    8
);

// Create new recv functions and related types
// A
create_recv_mpst_session!(
    recv_mpst_a_to_b,
    RoleB,
    next_b,
    RoleA,
    SessionMpstNine,
    9,
    1
);
create_recv_mpst_session!(
    recv_mpst_a_to_c,
    RoleC,
    next_c,
    RoleA,
    SessionMpstNine,
    9,
    2
);
create_recv_mpst_session!(
    recv_mpst_a_to_d,
    RoleD,
    next_d,
    RoleA,
    SessionMpstNine,
    9,
    3
);
create_recv_mpst_session!(
    recv_mpst_a_to_e,
    RoleE,
    next_e,
    RoleA,
    SessionMpstNine,
    9,
    4
);
create_recv_mpst_session!(
    recv_mpst_a_to_f,
    RoleF,
    next_f,
    RoleA,
    SessionMpstNine,
    9,
    5
);
create_recv_mpst_session!(
    recv_mpst_a_to_g,
    RoleG,
    next_g,
    RoleA,
    SessionMpstNine,
    9,
    6
);
create_recv_mpst_session!(
    recv_mpst_a_to_h,
    RoleH,
    next_h,
    RoleA,
    SessionMpstNine,
    9,
    7
);
create_recv_mpst_session!(
    recv_mpst_a_to_i,
    RoleI,
    next_i,
    RoleA,
    SessionMpstNine,
    9,
    8
);
// B
create_recv_mpst_session!(
    recv_mpst_b_to_a,
    RoleA,
    next_a,
    RoleB,
    SessionMpstNine,
    9,
    1
);
create_recv_mpst_session!(
    recv_mpst_b_to_c,
    RoleC,
    next_c,
    RoleB,
    SessionMpstNine,
    9,
    2
);
create_recv_mpst_session!(
    recv_mpst_b_to_d,
    RoleD,
    next_d,
    RoleB,
    SessionMpstNine,
    9,
    3
);
create_recv_mpst_session!(
    recv_mpst_b_to_e,
    RoleE,
    next_e,
    RoleB,
    SessionMpstNine,
    9,
    4
);
create_recv_mpst_session!(
    recv_mpst_b_to_f,
    RoleF,
    next_f,
    RoleB,
    SessionMpstNine,
    9,
    5
);
create_recv_mpst_session!(
    recv_mpst_b_to_g,
    RoleG,
    next_g,
    RoleB,
    SessionMpstNine,
    9,
    6
);
create_recv_mpst_session!(
    recv_mpst_b_to_h,
    RoleH,
    next_h,
    RoleB,
    SessionMpstNine,
    9,
    7
);
create_recv_mpst_session!(
    recv_mpst_b_to_i,
    RoleI,
    next_i,
    RoleB,
    SessionMpstNine,
    9,
    8
);
// C
create_recv_mpst_session!(
    recv_mpst_c_to_a,
    RoleA,
    next_a,
    RoleC,
    SessionMpstNine,
    9,
    1
);
create_recv_mpst_session!(
    recv_mpst_c_to_b,
    RoleB,
    next_b,
    RoleC,
    SessionMpstNine,
    9,
    2
);
create_recv_mpst_session!(
    recv_mpst_c_to_d,
    RoleD,
    next_d,
    RoleC,
    SessionMpstNine,
    9,
    3
);
create_recv_mpst_session!(
    recv_mpst_c_to_e,
    RoleE,
    next_e,
    RoleC,
    SessionMpstNine,
    9,
    4
);
create_recv_mpst_session!(
    recv_mpst_c_to_f,
    RoleF,
    next_f,
    RoleC,
    SessionMpstNine,
    9,
    5
);
create_recv_mpst_session!(
    recv_mpst_c_to_g,
    RoleG,
    next_g,
    RoleC,
    SessionMpstNine,
    9,
    6
);
create_recv_mpst_session!(
    recv_mpst_c_to_h,
    RoleH,
    next_h,
    RoleC,
    SessionMpstNine,
    9,
    7
);
create_recv_mpst_session!(
    recv_mpst_c_to_i,
    RoleI,
    next_i,
    RoleC,
    SessionMpstNine,
    9,
    8
);
// D
create_recv_mpst_session!(
    recv_mpst_d_to_a,
    RoleA,
    next_a,
    RoleD,
    SessionMpstNine,
    9,
    1
);
create_recv_mpst_session!(
    recv_mpst_d_to_b,
    RoleB,
    next_b,
    RoleD,
    SessionMpstNine,
    9,
    2
);
create_recv_mpst_session!(
    recv_mpst_d_to_c,
    RoleC,
    next_c,
    RoleD,
    SessionMpstNine,
    9,
    3
);
create_recv_mpst_session!(
    recv_mpst_d_to_e,
    RoleE,
    next_e,
    RoleD,
    SessionMpstNine,
    9,
    4
);
create_recv_mpst_session!(
    recv_mpst_d_to_f,
    RoleF,
    next_f,
    RoleD,
    SessionMpstNine,
    9,
    5
);
create_recv_mpst_session!(
    recv_mpst_d_to_g,
    RoleG,
    next_g,
    RoleD,
    SessionMpstNine,
    9,
    6
);
create_recv_mpst_session!(
    recv_mpst_d_to_h,
    RoleH,
    next_h,
    RoleD,
    SessionMpstNine,
    9,
    7
);
create_recv_mpst_session!(
    recv_mpst_d_to_i,
    RoleI,
    next_i,
    RoleD,
    SessionMpstNine,
    9,
    8
);
// E
create_recv_mpst_session!(
    recv_mpst_e_to_a,
    RoleA,
    next_a,
    RoleE,
    SessionMpstNine,
    9,
    1
);
create_recv_mpst_session!(
    recv_mpst_e_to_b,
    RoleB,
    next_b,
    RoleE,
    SessionMpstNine,
    9,
    2
);
create_recv_mpst_session!(
    recv_mpst_e_to_c,
    RoleC,
    next_c,
    RoleE,
    SessionMpstNine,
    9,
    3
);
create_recv_mpst_session!(
    recv_mpst_e_to_d,
    RoleD,
    next_d,
    RoleE,
    SessionMpstNine,
    9,
    4
);
create_recv_mpst_session!(
    recv_mpst_e_to_f,
    RoleF,
    next_f,
    RoleE,
    SessionMpstNine,
    9,
    5
);
create_recv_mpst_session!(
    recv_mpst_e_to_g,
    RoleG,
    next_g,
    RoleE,
    SessionMpstNine,
    9,
    6
);
create_recv_mpst_session!(
    recv_mpst_e_to_h,
    RoleH,
    next_h,
    RoleE,
    SessionMpstNine,
    9,
    7
);
create_recv_mpst_session!(
    recv_mpst_e_to_i,
    RoleI,
    next_i,
    RoleE,
    SessionMpstNine,
    9,
    8
);
// F
create_recv_mpst_session!(
    recv_mpst_f_to_a,
    RoleA,
    next_a,
    RoleF,
    SessionMpstNine,
    9,
    1
);
create_recv_mpst_session!(
    recv_mpst_f_to_b,
    RoleB,
    next_b,
    RoleF,
    SessionMpstNine,
    9,
    2
);
create_recv_mpst_session!(
    recv_mpst_f_to_c,
    RoleC,
    next_c,
    RoleF,
    SessionMpstNine,
    9,
    3
);
create_recv_mpst_session!(
    recv_mpst_f_to_d,
    RoleD,
    next_d,
    RoleF,
    SessionMpstNine,
    9,
    4
);
create_recv_mpst_session!(
    recv_mpst_f_to_e,
    RoleE,
    next_e,
    RoleF,
    SessionMpstNine,
    9,
    5
);
create_recv_mpst_session!(
    recv_mpst_f_to_g,
    RoleG,
    next_g,
    RoleF,
    SessionMpstNine,
    9,
    6
);
create_recv_mpst_session!(
    recv_mpst_f_to_h,
    RoleH,
    next_h,
    RoleF,
    SessionMpstNine,
    9,
    7
);
create_recv_mpst_session!(
    recv_mpst_f_to_i,
    RoleI,
    next_i,
    RoleF,
    SessionMpstNine,
    9,
    8
);
// G
create_recv_mpst_session!(
    recv_mpst_g_to_a,
    RoleA,
    next_a,
    RoleG,
    SessionMpstNine,
    9,
    1
);
create_recv_mpst_session!(
    recv_mpst_g_to_b,
    RoleB,
    next_b,
    RoleG,
    SessionMpstNine,
    9,
    2
);
create_recv_mpst_session!(
    recv_mpst_g_to_c,
    RoleC,
    next_c,
    RoleG,
    SessionMpstNine,
    9,
    3
);
create_recv_mpst_session!(
    recv_mpst_g_to_d,
    RoleD,
    next_d,
    RoleG,
    SessionMpstNine,
    9,
    4
);
create_recv_mpst_session!(
    recv_mpst_g_to_e,
    RoleE,
    next_e,
    RoleG,
    SessionMpstNine,
    9,
    5
);
create_recv_mpst_session!(
    recv_mpst_g_to_f,
    RoleF,
    next_f,
    RoleG,
    SessionMpstNine,
    9,
    6
);
create_recv_mpst_session!(
    recv_mpst_g_to_h,
    RoleH,
    next_h,
    RoleG,
    SessionMpstNine,
    9,
    7
);
create_recv_mpst_session!(
    recv_mpst_g_to_i,
    RoleI,
    next_i,
    RoleG,
    SessionMpstNine,
    9,
    8
);
// H
create_recv_mpst_session!(
    recv_mpst_h_to_a,
    RoleA,
    next_a,
    RoleH,
    SessionMpstNine,
    9,
    1
);
create_recv_mpst_session!(
    recv_mpst_h_to_b,
    RoleB,
    next_b,
    RoleH,
    SessionMpstNine,
    9,
    2
);
create_recv_mpst_session!(
    recv_mpst_h_to_c,
    RoleC,
    next_c,
    RoleH,
    SessionMpstNine,
    9,
    3
);
create_recv_mpst_session!(
    recv_mpst_h_to_d,
    RoleD,
    next_d,
    RoleH,
    SessionMpstNine,
    9,
    4
);
create_recv_mpst_session!(
    recv_mpst_h_to_e,
    RoleE,
    next_e,
    RoleH,
    SessionMpstNine,
    9,
    5
);
create_recv_mpst_session!(
    recv_mpst_h_to_f,
    RoleF,
    next_f,
    RoleH,
    SessionMpstNine,
    9,
    6
);
create_recv_mpst_session!(
    recv_mpst_h_to_g,
    RoleG,
    next_g,
    RoleH,
    SessionMpstNine,
    9,
    7
);
create_recv_mpst_session!(
    recv_mpst_h_to_i,
    RoleI,
    next_i,
    RoleH,
    SessionMpstNine,
    9,
    8
);
// I
create_recv_mpst_session!(
    recv_mpst_i_to_a,
    RoleA,
    next_a,
    RoleI,
    SessionMpstNine,
    9,
    1
);
create_recv_mpst_session!(
    recv_mpst_i_to_b,
    RoleB,
    next_b,
    RoleI,
    SessionMpstNine,
    9,
    2
);
create_recv_mpst_session!(
    recv_mpst_i_to_c,
    RoleC,
    next_c,
    RoleI,
    SessionMpstNine,
    9,
    3
);
create_recv_mpst_session!(
    recv_mpst_i_to_d,
    RoleD,
    next_d,
    RoleI,
    SessionMpstNine,
    9,
    4
);
create_recv_mpst_session!(
    recv_mpst_i_to_e,
    RoleE,
    next_e,
    RoleI,
    SessionMpstNine,
    9,
    5
);
create_recv_mpst_session!(
    recv_mpst_i_to_f,
    RoleF,
    next_f,
    RoleI,
    SessionMpstNine,
    9,
    6
);
create_recv_mpst_session!(
    recv_mpst_i_to_g,
    RoleG,
    next_g,
    RoleI,
    SessionMpstNine,
    9,
    7
);
create_recv_mpst_session!(
    recv_mpst_i_to_h,
    RoleH,
    next_h,
    RoleI,
    SessionMpstNine,
    9,
    8
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstNine, 9);

// Create fork function
bundle_fork_multi!(fork_mpst, fork_simple, SessionMpstNine, 9);

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

// Types
// Binary
// A
enum BranchingIforA {
    More(
        SessionMpstNine<
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursAtoI>>,
            RoleI<
                RoleI<
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
                                                                RoleG<RoleH<RoleH<RoleI<RoleEnd>>>>,
                                                            >,
                                                        >,
                                                    >,
                                                >,
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
    Done(SessionMpstNine<End, End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoI = Recv<BranchingIforA, End>;
// B
enum BranchingIforB {
    More(
        SessionMpstNine<
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursBtoI>>,
            RoleI<
                RoleI<
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
                                                                RoleG<RoleH<RoleH<RoleI<RoleEnd>>>>,
                                                            >,
                                                        >,
                                                    >,
                                                >,
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
    Done(SessionMpstNine<End, End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoI = Recv<BranchingIforB, End>;
// C
enum BranchingIforC {
    More(
        SessionMpstNine<
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursCtoI>>,
            RoleI<
                RoleI<
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
                                                                RoleG<RoleH<RoleH<RoleI<RoleEnd>>>>,
                                                            >,
                                                        >,
                                                    >,
                                                >,
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
    Done(SessionMpstNine<End, End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoI = Recv<BranchingIforC, End>;
// D
enum BranchingIforD {
    More(
        SessionMpstNine<
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursDtoI>>,
            RoleI<
                RoleI<
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
                                                                RoleG<RoleH<RoleH<RoleI<RoleEnd>>>>,
                                                            >,
                                                        >,
                                                    >,
                                                >,
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
    Done(SessionMpstNine<End, End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoI = Recv<BranchingIforD, End>;
// E
enum BranchingIforE {
    More(
        SessionMpstNine<
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursEtoI>>,
            RoleI<
                RoleI<
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
                                                                RoleG<RoleH<RoleH<RoleI<RoleEnd>>>>,
                                                            >,
                                                        >,
                                                    >,
                                                >,
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
    Done(SessionMpstNine<End, End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoI = Recv<BranchingIforE, End>;
// F
enum BranchingIforF {
    More(
        SessionMpstNine<
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursFtoI>>,
            RoleI<
                RoleI<
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
                                                                RoleG<RoleH<RoleH<RoleI<RoleEnd>>>>,
                                                            >,
                                                        >,
                                                    >,
                                                >,
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
    Done(SessionMpstNine<End, End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoI = Recv<BranchingIforF, End>;
// G
enum BranchingIforG {
    More(
        SessionMpstNine<
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursGtoI>>,
            RoleI<
                RoleI<
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
                                                                RoleF<RoleH<RoleH<RoleI<RoleEnd>>>>,
                                                            >,
                                                        >,
                                                    >,
                                                >,
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
    Done(SessionMpstNine<End, End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoI = Recv<BranchingIforG, End>;
// H
enum BranchingIforH {
    More(
        SessionMpstNine<
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), RecursHtoI>>,
            RoleI<
                RoleI<
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
                                                                RoleF<RoleG<RoleG<RoleI<RoleEnd>>>>,
                                                            >,
                                                        >,
                                                    >,
                                                >,
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
    Done(SessionMpstNine<End, End, End, End, End, End, End, End, RoleEnd, NameH>),
}
type RecursHtoI = Recv<BranchingIforH, End>;
// I
type ChooseIforAtoI = Send<BranchingIforA, End>;
type ChooseIforBtoI = Send<BranchingIforB, End>;
type ChooseIforCtoI = Send<BranchingIforC, End>;
type ChooseIforDtoI = Send<BranchingIforD, End>;
type ChooseIforEtoI = Send<BranchingIforE, End>;
type ChooseIforFtoI = Send<BranchingIforF, End>;
type ChooseIforGtoI = Send<BranchingIforG, End>;
type ChooseIforHtoI = Send<BranchingIforH, End>;

// Creating the MP sessions
type EndpointA =
    SessionMpstNine<End, End, End, End, End, End, End, RecursAtoI, RoleI<RoleEnd>, NameA>;
type EndpointB =
    SessionMpstNine<End, End, End, End, End, End, End, RecursBtoI, RoleI<RoleEnd>, NameB>;
type EndpointC =
    SessionMpstNine<End, End, End, End, End, End, End, RecursCtoI, RoleI<RoleEnd>, NameC>;
type EndpointD =
    SessionMpstNine<End, End, End, End, End, End, End, RecursDtoI, RoleI<RoleEnd>, NameD>;
type EndpointE =
    SessionMpstNine<End, End, End, End, End, End, End, RecursEtoI, RoleI<RoleEnd>, NameE>;
type EndpointF =
    SessionMpstNine<End, End, End, End, End, End, End, RecursFtoI, RoleI<RoleEnd>, NameF>;
type EndpointG =
    SessionMpstNine<End, End, End, End, End, End, End, RecursGtoI, RoleI<RoleEnd>, NameG>;
type EndpointH =
    SessionMpstNine<End, End, End, End, End, End, End, RecursHtoI, RoleI<RoleEnd>, NameH>;
type EndpointI = SessionMpstNine<
    ChooseIforAtoI,
    ChooseIforBtoI,
    ChooseIforCtoI,
    ChooseIforDtoI,
    ChooseIforEtoI,
    ChooseIforFtoI,
    ChooseIforGtoI,
    ChooseIforHtoI,
    RoleA<RoleB<RoleC<RoleD<RoleE<RoleF<RoleG<RoleH<RoleEnd>>>>>>>>,
    NameI,
>;

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_to_i, {
          BranchingIforA::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingIforA::More(s) => {
            let (_, s) = recv_mpst_a_to_i(s)?;
            let s = send_mpst_a_to_i((), s);
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
            simple_five_endpoint_a(s)
        },
    })
}

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_to_i, {
          BranchingIforB::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingIforB::More(s) => {
            let (_, s) = recv_mpst_b_to_i(s)?;
            let s = send_mpst_b_to_i((), s);
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
            simple_five_endpoint_b(s)
        },
    })
}

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_to_i, {
          BranchingIforC::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingIforC::More(s) => {
            let (_, s) = recv_mpst_c_to_i(s)?;
            let s = send_mpst_c_to_i((), s);
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
            simple_five_endpoint_c(s)
        },
    })
}

fn simple_five_endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_to_i, {
          BranchingIforD::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingIforD::More(s) => {
            let (_, s) = recv_mpst_d_to_i(s)?;
            let s = send_mpst_d_to_i((), s);
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
            simple_five_endpoint_d(s)
        },
    })
}

fn simple_five_endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_e_to_i, {
          BranchingIforE::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingIforE::More(s) => {
            let (_, s) = recv_mpst_e_to_i(s)?;
            let s = send_mpst_e_to_i((), s);
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
            simple_five_endpoint_e(s)
        },
    })
}

fn simple_five_endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_f_to_i, {
          BranchingIforF::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingIforF::More(s) => {
            let (_, s) = recv_mpst_f_to_i(s)?;
            let s = send_mpst_f_to_i((), s);
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
            simple_five_endpoint_f(s)
        },
    })
}

fn simple_five_endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_g_to_i, {
          BranchingIforG::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingIforG::More(s) => {
            let (_, s) = recv_mpst_g_to_i(s)?;
            let s = send_mpst_g_to_i((), s);
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
            simple_five_endpoint_g(s)
        },
    })
}

fn simple_five_endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_h_to_i, {
          BranchingIforH::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingIforH::More(s) => {
            let (_, s) = recv_mpst_h_to_i(s)?;
            let s = send_mpst_h_to_i((), s);
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
            simple_five_endpoint_h(s)
        },
    })
}

fn simple_five_endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>> {
    recurs_i(s, SIZE)
}

fn recurs_i(s: EndpointI, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_i_to_a,
                send_mpst_i_to_b,
                send_mpst_i_to_c,
                send_mpst_i_to_d,
                send_mpst_i_to_e,
                send_mpst_i_to_f,
                send_mpst_i_to_g,
                send_mpst_i_to_h, =>
                  BranchingIforA::Done,
                  BranchingIforB::Done,
                  BranchingIforC::Done,
                  BranchingIforD::Done,
                  BranchingIforE::Done,
                  BranchingIforF::Done,
                  BranchingIforG::Done,
                  BranchingIforH::Done, =>
                RoleA,
                RoleB,
                RoleC,
                RoleD,
                RoleE,
                RoleF,
                RoleG,
                RoleH, =>
                RoleI,
                 SessionMpstNine,
                9
            );

            close_mpst_multi(s)
        }
        i => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_i_to_a,
                send_mpst_i_to_b,
                send_mpst_i_to_c,
                send_mpst_i_to_d,
                send_mpst_i_to_e,
                send_mpst_i_to_f,
                send_mpst_i_to_g,
                send_mpst_i_to_h,=>
                  BranchingIforA::More,
                  BranchingIforB::More,
                  BranchingIforC::More,
                  BranchingIforD::More,
                  BranchingIforE::More,
                  BranchingIforF::More,
                  BranchingIforG::More,
                  BranchingIforH::More, =>
                  RoleA,
                  RoleB,
                  RoleC,
                  RoleD,
                  RoleE,
                  RoleF,
                  RoleG,
                  RoleH, =>
                  RoleI,
                 SessionMpstNine,
                9
            );

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

            recurs_i(s, i - 1)
        }
    }
}

fn all_mpst() -> Result<(), Box<dyn Error>> {
    let (thread_a, thread_b, thread_c, thread_d, thread_e, thread_f, thread_g, thread_h, thread_i) =
        fork_mpst(
            black_box(simple_five_endpoint_a),
            black_box(simple_five_endpoint_b),
            black_box(simple_five_endpoint_c),
            black_box(simple_five_endpoint_d),
            black_box(simple_five_endpoint_e),
            black_box(simple_five_endpoint_f),
            black_box(simple_five_endpoint_g),
            black_box(simple_five_endpoint_h),
            black_box(simple_five_endpoint_i),
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

    for _ in 0..36 {
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

static SIZE: i64 = 0;

fn long_simple_protocol_mpst(c: &mut Criterion) {
    c.bench_function(
        &format!("long nine empty simple protocol MPST {}", SIZE),
        |b| b.iter(|| all_mpst()),
    );
}

fn long_simple_protocol_binary(c: &mut Criterion) {
    c.bench_function(
        &format!("long nine empty simple protocol binary {}", SIZE),
        |b| b.iter(|| all_binaries()),
    );
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(10, 0))
}

criterion_group! {
    name = long_nine_empty_simple_protocols;
    config = long_warmup();
    targets = long_simple_protocol_mpst, long_simple_protocol_binary
}
criterion_main!(long_nine_empty_simple_protocols);
