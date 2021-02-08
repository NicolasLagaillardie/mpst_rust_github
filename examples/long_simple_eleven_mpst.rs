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
// A
enum Branching0fromKtoA {
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
type RecursAtoK = Recv<Branching0fromKtoA, End>;
// B
enum Branching0fromKtoB {
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
type RecursBtoK = Recv<Branching0fromKtoB, End>;
// C
enum Branching0fromKtoC {
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
type RecursCtoK = Recv<Branching0fromKtoC, End>;
// D
enum Branching0fromKtoD {
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
type RecursDtoK = Recv<Branching0fromKtoD, End>;
// E
enum Branching0fromKtoE {
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
type RecursEtoK = Recv<Branching0fromKtoE, End>;
// F
enum Branching0fromKtoF {
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
type RecursFtoK = Recv<Branching0fromKtoF, End>;
// G
enum Branching0fromKtoG {
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
type RecursGtoK = Recv<Branching0fromKtoG, End>;
// H
enum Branching0fromKtoH {
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
type RecursHtoK = Recv<Branching0fromKtoH, End>;
// I
enum Branching0fromKtoI {
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
type RecursItoK = Recv<Branching0fromKtoI, End>;
// J
enum Branching0fromKtoJ {
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
type RecursJtoK = Recv<Branching0fromKtoJ, End>;
// K
type ChooseKforAtoK = Send<Branching0fromKtoA, End>;
type ChooseKforBtoK = Send<Branching0fromKtoB, End>;
type ChooseKforCtoK = Send<Branching0fromKtoC, End>;
type ChooseKforDtoK = Send<Branching0fromKtoD, End>;
type ChooseKforEtoK = Send<Branching0fromKtoE, End>;
type ChooseKforFtoK = Send<Branching0fromKtoF, End>;
type ChooseKforGtoK = Send<Branching0fromKtoG, End>;
type ChooseKforHtoK = Send<Branching0fromKtoH, End>;
type ChooseKforItoK = Send<Branching0fromKtoI, End>;
type ChooseKforJtoK = Send<Branching0fromKtoJ, End>;

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
          Branching0fromKtoA::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromKtoA::More(s) => {
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
          Branching0fromKtoB::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromKtoB::More(s) => {
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
          Branching0fromKtoC::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromKtoC::More(s) => {
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
          Branching0fromKtoD::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromKtoD::More(s) => {
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
          Branching0fromKtoE::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromKtoE::More(s) => {
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
          Branching0fromKtoF::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromKtoF::More(s) => {
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
          Branching0fromKtoG::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromKtoG::More(s) => {
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
          Branching0fromKtoH::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromKtoH::More(s) => {
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
          Branching0fromKtoI::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoI::More(s) => {
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
          Branching0fromKtoJ::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoJ::More(s) => {
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
                Branching0fromKtoA::Done,
                Branching0fromKtoB::Done,
                Branching0fromKtoC::Done,
                Branching0fromKtoD::Done,
                Branching0fromKtoE::Done,
                Branching0fromKtoF::Done,
                Branching0fromKtoG::Done,
                Branching0fromKtoH::Done,
                Branching0fromKtoI::Done,
                Branching0fromKtoJ::Done, =>
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
                11,
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
                Branching0fromKtoA::More,
                Branching0fromKtoB::More,
                Branching0fromKtoC::More,
                Branching0fromKtoD::More,
                Branching0fromKtoE::More,
                Branching0fromKtoF::More,
                Branching0fromKtoG::More,
                Branching0fromKtoH::More,
                Branching0fromKtoI::More,
                Branching0fromKtoJ::More, =>
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
                11,
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

static SIZE: i64 = 15;

fn main() {
    all_mpst().unwrap();
}
