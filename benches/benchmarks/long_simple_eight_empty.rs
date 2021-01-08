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
create_sessionmpst!(SessionMpstEight, 8);

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

// Create new send functions
// A
create_send_mpst_session!(
    send_mpst_a_to_b,
    RoleB,
    next_b,
    RoleA,
    SessionMpstEight,
    8,
    1
);
create_send_mpst_session!(
    send_mpst_a_to_c,
    RoleC,
    next_c,
    RoleA,
    SessionMpstEight,
    8,
    2
);
create_send_mpst_session!(
    send_mpst_a_to_d,
    RoleD,
    next_d,
    RoleA,
    SessionMpstEight,
    8,
    3
);
create_send_mpst_session!(
    send_mpst_a_to_e,
    RoleE,
    next_e,
    RoleA,
    SessionMpstEight,
    8,
    4
);
create_send_mpst_session!(
    send_mpst_a_to_f,
    RoleF,
    next_f,
    RoleA,
    SessionMpstEight,
    8,
    5
);
create_send_mpst_session!(
    send_mpst_a_to_g,
    RoleG,
    next_g,
    RoleA,
    SessionMpstEight,
    8,
    6
);
create_send_mpst_session!(
    send_mpst_a_to_h,
    RoleH,
    next_h,
    RoleA,
    SessionMpstEight,
    8,
    7
);
// B
create_send_mpst_session!(
    send_mpst_b_to_a,
    RoleA,
    next_a,
    RoleB,
    SessionMpstEight,
    8,
    1
);
create_send_mpst_session!(
    send_mpst_b_to_c,
    RoleC,
    next_c,
    RoleB,
    SessionMpstEight,
    8,
    2
);
create_send_mpst_session!(
    send_mpst_b_to_d,
    RoleD,
    next_d,
    RoleB,
    SessionMpstEight,
    8,
    3
);
create_send_mpst_session!(
    send_mpst_b_to_e,
    RoleE,
    next_e,
    RoleB,
    SessionMpstEight,
    8,
    4
);
create_send_mpst_session!(
    send_mpst_b_to_f,
    RoleF,
    next_f,
    RoleB,
    SessionMpstEight,
    8,
    5
);
create_send_mpst_session!(
    send_mpst_b_to_g,
    RoleG,
    next_g,
    RoleB,
    SessionMpstEight,
    8,
    6
);
create_send_mpst_session!(
    send_mpst_b_to_h,
    RoleH,
    next_h,
    RoleB,
    SessionMpstEight,
    8,
    7
);
// C
create_send_mpst_session!(
    send_mpst_c_to_a,
    RoleA,
    next_a,
    RoleC,
    SessionMpstEight,
    8,
    1
);
create_send_mpst_session!(
    send_mpst_c_to_b,
    RoleB,
    next_b,
    RoleC,
    SessionMpstEight,
    8,
    2
);
create_send_mpst_session!(
    send_mpst_c_to_d,
    RoleD,
    next_d,
    RoleC,
    SessionMpstEight,
    8,
    3
);
create_send_mpst_session!(
    send_mpst_c_to_e,
    RoleE,
    next_e,
    RoleC,
    SessionMpstEight,
    8,
    4
);
create_send_mpst_session!(
    send_mpst_c_to_f,
    RoleF,
    next_f,
    RoleC,
    SessionMpstEight,
    8,
    5
);
create_send_mpst_session!(
    send_mpst_c_to_g,
    RoleG,
    next_g,
    RoleC,
    SessionMpstEight,
    8,
    6
);
create_send_mpst_session!(
    send_mpst_c_to_h,
    RoleH,
    next_h,
    RoleC,
    SessionMpstEight,
    8,
    7
);
// D
create_send_mpst_session!(
    send_mpst_d_to_a,
    RoleA,
    next_a,
    RoleD,
    SessionMpstEight,
    8,
    1
);
create_send_mpst_session!(
    send_mpst_d_to_b,
    RoleB,
    next_b,
    RoleD,
    SessionMpstEight,
    8,
    2
);
create_send_mpst_session!(
    send_mpst_d_to_c,
    RoleC,
    next_c,
    RoleD,
    SessionMpstEight,
    8,
    3
);
create_send_mpst_session!(
    send_mpst_d_to_e,
    RoleE,
    next_e,
    RoleD,
    SessionMpstEight,
    8,
    4
);
create_send_mpst_session!(
    send_mpst_d_to_f,
    RoleF,
    next_f,
    RoleD,
    SessionMpstEight,
    8,
    5
);
create_send_mpst_session!(
    send_mpst_d_to_g,
    RoleG,
    next_g,
    RoleD,
    SessionMpstEight,
    8,
    6
);
create_send_mpst_session!(
    send_mpst_d_to_h,
    RoleH,
    next_h,
    RoleD,
    SessionMpstEight,
    8,
    7
);
// E
create_send_mpst_session!(
    send_mpst_e_to_a,
    RoleA,
    next_a,
    RoleE,
    SessionMpstEight,
    8,
    1
);
create_send_mpst_session!(
    send_mpst_e_to_b,
    RoleB,
    next_b,
    RoleE,
    SessionMpstEight,
    8,
    2
);
create_send_mpst_session!(
    send_mpst_e_to_c,
    RoleC,
    next_c,
    RoleE,
    SessionMpstEight,
    8,
    3
);
create_send_mpst_session!(
    send_mpst_e_to_d,
    RoleD,
    next_d,
    RoleE,
    SessionMpstEight,
    8,
    4
);
create_send_mpst_session!(
    send_mpst_e_to_f,
    RoleF,
    next_f,
    RoleE,
    SessionMpstEight,
    8,
    5
);
create_send_mpst_session!(
    send_mpst_e_to_g,
    RoleG,
    next_g,
    RoleE,
    SessionMpstEight,
    8,
    6
);
create_send_mpst_session!(
    send_mpst_e_to_h,
    RoleH,
    next_h,
    RoleE,
    SessionMpstEight,
    8,
    7
);
// F
create_send_mpst_session!(
    send_mpst_f_to_a,
    RoleA,
    next_a,
    RoleF,
    SessionMpstEight,
    8,
    1
);
create_send_mpst_session!(
    send_mpst_f_to_b,
    RoleB,
    next_b,
    RoleF,
    SessionMpstEight,
    8,
    2
);
create_send_mpst_session!(
    send_mpst_f_to_c,
    RoleC,
    next_c,
    RoleF,
    SessionMpstEight,
    8,
    3
);
create_send_mpst_session!(
    send_mpst_f_to_d,
    RoleD,
    next_d,
    RoleF,
    SessionMpstEight,
    8,
    4
);
create_send_mpst_session!(
    send_mpst_f_to_e,
    RoleE,
    next_e,
    RoleF,
    SessionMpstEight,
    8,
    5
);
create_send_mpst_session!(
    send_mpst_f_to_g,
    RoleG,
    next_g,
    RoleF,
    SessionMpstEight,
    8,
    6
);
create_send_mpst_session!(
    send_mpst_f_to_h,
    RoleH,
    next_h,
    RoleF,
    SessionMpstEight,
    8,
    7
);
// G
create_send_mpst_session!(
    send_mpst_g_to_a,
    RoleA,
    next_a,
    RoleG,
    SessionMpstEight,
    8,
    1
);
create_send_mpst_session!(
    send_mpst_g_to_b,
    RoleB,
    next_b,
    RoleG,
    SessionMpstEight,
    8,
    2
);
create_send_mpst_session!(
    send_mpst_g_to_c,
    RoleC,
    next_c,
    RoleG,
    SessionMpstEight,
    8,
    3
);
create_send_mpst_session!(
    send_mpst_g_to_d,
    RoleD,
    next_d,
    RoleG,
    SessionMpstEight,
    8,
    4
);
create_send_mpst_session!(
    send_mpst_g_to_e,
    RoleE,
    next_e,
    RoleG,
    SessionMpstEight,
    8,
    5
);
create_send_mpst_session!(
    send_mpst_g_to_f,
    RoleF,
    next_f,
    RoleG,
    SessionMpstEight,
    8,
    6
);
create_send_mpst_session!(
    send_mpst_g_to_h,
    RoleH,
    next_h,
    RoleG,
    SessionMpstEight,
    8,
    7
);
// H
create_send_mpst_session!(
    send_mpst_h_to_a,
    RoleA,
    next_a,
    RoleH,
    SessionMpstEight,
    8,
    1
);
create_send_mpst_session!(
    send_mpst_h_to_b,
    RoleB,
    next_b,
    RoleH,
    SessionMpstEight,
    8,
    2
);
create_send_mpst_session!(
    send_mpst_h_to_c,
    RoleC,
    next_c,
    RoleH,
    SessionMpstEight,
    8,
    3
);
create_send_mpst_session!(
    send_mpst_h_to_d,
    RoleD,
    next_d,
    RoleH,
    SessionMpstEight,
    8,
    4
);
create_send_mpst_session!(
    send_mpst_h_to_e,
    RoleE,
    next_e,
    RoleH,
    SessionMpstEight,
    8,
    5
);
create_send_mpst_session!(
    send_mpst_h_to_f,
    RoleF,
    next_f,
    RoleH,
    SessionMpstEight,
    8,
    6
);
create_send_mpst_session!(
    send_mpst_h_to_g,
    RoleG,
    next_g,
    RoleH,
    SessionMpstEight,
    8,
    7
);

// Create new recv functions and related types
// A
create_recv_mpst_session!(
    recv_mpst_a_to_b,
    RoleB,
    next_b,
    RoleA,
    SessionMpstEight,
    8,
    1
);
create_recv_mpst_session!(
    recv_mpst_a_to_c,
    RoleC,
    next_c,
    RoleA,
    SessionMpstEight,
    8,
    2
);
create_recv_mpst_session!(
    recv_mpst_a_to_d,
    RoleD,
    next_d,
    RoleA,
    SessionMpstEight,
    8,
    3
);
create_recv_mpst_session!(
    recv_mpst_a_to_e,
    RoleE,
    next_e,
    RoleA,
    SessionMpstEight,
    8,
    4
);
create_recv_mpst_session!(
    recv_mpst_a_to_f,
    RoleF,
    next_f,
    RoleA,
    SessionMpstEight,
    8,
    5
);
create_recv_mpst_session!(
    recv_mpst_a_to_g,
    RoleG,
    next_g,
    RoleA,
    SessionMpstEight,
    8,
    6
);
create_recv_mpst_session!(
    recv_mpst_a_to_h,
    RoleH,
    next_h,
    RoleA,
    SessionMpstEight,
    8,
    7
);
// B
create_recv_mpst_session!(
    recv_mpst_b_to_a,
    RoleA,
    next_a,
    RoleB,
    SessionMpstEight,
    8,
    1
);
create_recv_mpst_session!(
    recv_mpst_b_to_c,
    RoleC,
    next_c,
    RoleB,
    SessionMpstEight,
    8,
    2
);
create_recv_mpst_session!(
    recv_mpst_b_to_d,
    RoleD,
    next_d,
    RoleB,
    SessionMpstEight,
    8,
    3
);
create_recv_mpst_session!(
    recv_mpst_b_to_e,
    RoleE,
    next_e,
    RoleB,
    SessionMpstEight,
    8,
    4
);
create_recv_mpst_session!(
    recv_mpst_b_to_f,
    RoleF,
    next_f,
    RoleB,
    SessionMpstEight,
    8,
    5
);
create_recv_mpst_session!(
    recv_mpst_b_to_g,
    RoleG,
    next_g,
    RoleB,
    SessionMpstEight,
    8,
    6
);
create_recv_mpst_session!(
    recv_mpst_b_to_h,
    RoleH,
    next_h,
    RoleB,
    SessionMpstEight,
    8,
    7
);
// C
create_recv_mpst_session!(
    recv_mpst_c_to_a,
    RoleA,
    next_a,
    RoleC,
    SessionMpstEight,
    8,
    1
);
create_recv_mpst_session!(
    recv_mpst_c_to_b,
    RoleB,
    next_b,
    RoleC,
    SessionMpstEight,
    8,
    2
);
create_recv_mpst_session!(
    recv_mpst_c_to_d,
    RoleD,
    next_d,
    RoleC,
    SessionMpstEight,
    8,
    3
);
create_recv_mpst_session!(
    recv_mpst_c_to_e,
    RoleE,
    next_e,
    RoleC,
    SessionMpstEight,
    8,
    4
);
create_recv_mpst_session!(
    recv_mpst_c_to_f,
    RoleF,
    next_f,
    RoleC,
    SessionMpstEight,
    8,
    5
);
create_recv_mpst_session!(
    recv_mpst_c_to_g,
    RoleG,
    next_g,
    RoleC,
    SessionMpstEight,
    8,
    6
);
create_recv_mpst_session!(
    recv_mpst_c_to_h,
    RoleH,
    next_h,
    RoleC,
    SessionMpstEight,
    8,
    7
);
// D
create_recv_mpst_session!(
    recv_mpst_d_to_a,
    RoleA,
    next_a,
    RoleD,
    SessionMpstEight,
    8,
    1
);
create_recv_mpst_session!(
    recv_mpst_d_to_b,
    RoleB,
    next_b,
    RoleD,
    SessionMpstEight,
    8,
    2
);
create_recv_mpst_session!(
    recv_mpst_d_to_c,
    RoleC,
    next_c,
    RoleD,
    SessionMpstEight,
    8,
    3
);
create_recv_mpst_session!(
    recv_mpst_d_to_e,
    RoleE,
    next_e,
    RoleD,
    SessionMpstEight,
    8,
    4
);
create_recv_mpst_session!(
    recv_mpst_d_to_f,
    RoleF,
    next_f,
    RoleD,
    SessionMpstEight,
    8,
    5
);
create_recv_mpst_session!(
    recv_mpst_d_to_g,
    RoleG,
    next_g,
    RoleD,
    SessionMpstEight,
    8,
    6
);
create_recv_mpst_session!(
    recv_mpst_d_to_h,
    RoleH,
    next_h,
    RoleD,
    SessionMpstEight,
    8,
    7
);
// E
create_recv_mpst_session!(
    recv_mpst_e_to_a,
    RoleA,
    next_a,
    RoleE,
    SessionMpstEight,
    8,
    1
);
create_recv_mpst_session!(
    recv_mpst_e_to_b,
    RoleB,
    next_b,
    RoleE,
    SessionMpstEight,
    8,
    2
);
create_recv_mpst_session!(
    recv_mpst_e_to_c,
    RoleC,
    next_c,
    RoleE,
    SessionMpstEight,
    8,
    3
);
create_recv_mpst_session!(
    recv_mpst_e_to_d,
    RoleD,
    next_d,
    RoleE,
    SessionMpstEight,
    8,
    4
);
create_recv_mpst_session!(
    recv_mpst_e_to_f,
    RoleF,
    next_f,
    RoleE,
    SessionMpstEight,
    8,
    5
);
create_recv_mpst_session!(
    recv_mpst_e_to_g,
    RoleG,
    next_g,
    RoleE,
    SessionMpstEight,
    8,
    6
);
create_recv_mpst_session!(
    recv_mpst_e_to_h,
    RoleH,
    next_h,
    RoleE,
    SessionMpstEight,
    8,
    7
);
// F
create_recv_mpst_session!(
    recv_mpst_f_to_a,
    RoleA,
    next_a,
    RoleF,
    SessionMpstEight,
    8,
    1
);
create_recv_mpst_session!(
    recv_mpst_f_to_b,
    RoleB,
    next_b,
    RoleF,
    SessionMpstEight,
    8,
    2
);
create_recv_mpst_session!(
    recv_mpst_f_to_c,
    RoleC,
    next_c,
    RoleF,
    SessionMpstEight,
    8,
    3
);
create_recv_mpst_session!(
    recv_mpst_f_to_d,
    RoleD,
    next_d,
    RoleF,
    SessionMpstEight,
    8,
    4
);
create_recv_mpst_session!(
    recv_mpst_f_to_e,
    RoleE,
    next_e,
    RoleF,
    SessionMpstEight,
    8,
    5
);
create_recv_mpst_session!(
    recv_mpst_f_to_g,
    RoleG,
    next_g,
    RoleF,
    SessionMpstEight,
    8,
    6
);
create_recv_mpst_session!(
    recv_mpst_f_to_h,
    RoleH,
    next_h,
    RoleF,
    SessionMpstEight,
    8,
    7
);
// G
create_recv_mpst_session!(
    recv_mpst_g_to_a,
    RoleA,
    next_a,
    RoleG,
    SessionMpstEight,
    8,
    1
);
create_recv_mpst_session!(
    recv_mpst_g_to_b,
    RoleB,
    next_b,
    RoleG,
    SessionMpstEight,
    8,
    2
);
create_recv_mpst_session!(
    recv_mpst_g_to_c,
    RoleC,
    next_c,
    RoleG,
    SessionMpstEight,
    8,
    3
);
create_recv_mpst_session!(
    recv_mpst_g_to_d,
    RoleD,
    next_d,
    RoleG,
    SessionMpstEight,
    8,
    4
);
create_recv_mpst_session!(
    recv_mpst_g_to_e,
    RoleE,
    next_e,
    RoleG,
    SessionMpstEight,
    8,
    5
);
create_recv_mpst_session!(
    recv_mpst_g_to_f,
    RoleF,
    next_f,
    RoleG,
    SessionMpstEight,
    8,
    6
);
create_recv_mpst_session!(
    recv_mpst_g_to_h,
    RoleH,
    next_h,
    RoleG,
    SessionMpstEight,
    8,
    7
);
// H
create_recv_mpst_session!(
    recv_mpst_h_to_a,
    RoleA,
    next_a,
    RoleH,
    SessionMpstEight,
    8,
    1
);
create_recv_mpst_session!(
    recv_mpst_h_to_b,
    RoleB,
    next_b,
    RoleH,
    SessionMpstEight,
    8,
    2
);
create_recv_mpst_session!(
    recv_mpst_h_to_c,
    RoleC,
    next_c,
    RoleH,
    SessionMpstEight,
    8,
    3
);
create_recv_mpst_session!(
    recv_mpst_h_to_d,
    RoleD,
    next_d,
    RoleH,
    SessionMpstEight,
    8,
    4
);
create_recv_mpst_session!(
    recv_mpst_h_to_e,
    RoleE,
    next_e,
    RoleH,
    SessionMpstEight,
    8,
    5
);
create_recv_mpst_session!(
    recv_mpst_h_to_f,
    RoleF,
    next_f,
    RoleH,
    SessionMpstEight,
    8,
    6
);
create_recv_mpst_session!(
    recv_mpst_h_to_g,
    RoleG,
    next_g,
    RoleH,
    SessionMpstEight,
    8,
    7
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstEight, 8);

// Create fork function
bundle_fork_multi!(fork_mpst, fork_simple, SessionMpstEight, 8);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;
type NameE = RoleE<RoleEnd>;
type NameF = RoleF<RoleEnd>;
type NameG = RoleG<RoleEnd>;
type NameH = RoleH<RoleEnd>;

// Types
// Binary
// A
enum BranchingHforA {
    More(
        SessionMpstEight<
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursAtoH>>,
            RoleH<
                RoleH<
                    RoleB<
                        RoleB<
                            RoleC<
                                RoleC<
                                    RoleD<
                                        RoleD<
                                            RoleE<
                                                RoleE<RoleF<RoleF<RoleG<RoleG<RoleH<RoleEnd>>>>>>,
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
    Done(SessionMpstEight<End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoH = Recv<BranchingHforA, End>;
// B
enum BranchingHforB {
    More(
        SessionMpstEight<
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursBtoH>>,
            RoleH<
                RoleH<
                    RoleA<
                        RoleA<
                            RoleC<
                                RoleC<
                                    RoleD<
                                        RoleD<
                                            RoleE<
                                                RoleE<RoleF<RoleF<RoleG<RoleG<RoleH<RoleEnd>>>>>>,
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
    Done(SessionMpstEight<End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoH = Recv<BranchingHforB, End>;
// C
enum BranchingHforC {
    More(
        SessionMpstEight<
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursCtoH>>,
            RoleH<
                RoleH<
                    RoleA<
                        RoleA<
                            RoleB<
                                RoleB<
                                    RoleD<
                                        RoleD<
                                            RoleE<
                                                RoleE<RoleF<RoleF<RoleG<RoleG<RoleH<RoleEnd>>>>>>,
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
    Done(SessionMpstEight<End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoH = Recv<BranchingHforC, End>;
// D
enum BranchingHforD {
    More(
        SessionMpstEight<
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursDtoH>>,
            RoleH<
                RoleH<
                    RoleA<
                        RoleA<
                            RoleB<
                                RoleB<
                                    RoleC<
                                        RoleC<
                                            RoleE<
                                                RoleE<RoleF<RoleF<RoleG<RoleG<RoleH<RoleEnd>>>>>>,
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
    Done(SessionMpstEight<End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoH = Recv<BranchingHforD, End>;
// E
enum BranchingHforE {
    More(
        SessionMpstEight<
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursEtoH>>,
            RoleH<
                RoleH<
                    RoleA<
                        RoleA<
                            RoleB<
                                RoleB<
                                    RoleC<
                                        RoleC<
                                            RoleD<
                                                RoleD<RoleF<RoleF<RoleG<RoleG<RoleH<RoleEnd>>>>>>,
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
    Done(SessionMpstEight<End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoH = Recv<BranchingHforE, End>;
// F
enum BranchingHforF {
    More(
        SessionMpstEight<
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursFtoH>>,
            RoleH<
                RoleH<
                    RoleA<
                        RoleA<
                            RoleB<
                                RoleB<
                                    RoleC<
                                        RoleC<
                                            RoleD<
                                                RoleD<RoleE<RoleE<RoleG<RoleG<RoleH<RoleEnd>>>>>>,
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
    Done(SessionMpstEight<End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoH = Recv<BranchingHforF, End>;
// G
enum BranchingHforG {
    More(
        SessionMpstEight<
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), RecursGtoH>>,
            RoleH<
                RoleH<
                    RoleA<
                        RoleA<
                            RoleB<
                                RoleB<
                                    RoleC<
                                        RoleC<
                                            RoleD<
                                                RoleD<RoleE<RoleE<RoleF<RoleF<RoleH<RoleEnd>>>>>>,
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
    Done(SessionMpstEight<End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoH = Recv<BranchingHforG, End>;
// H
type ChooseHforAtoH = Send<BranchingHforA, End>;
type ChooseHforBtoH = Send<BranchingHforB, End>;
type ChooseHforCtoH = Send<BranchingHforC, End>;
type ChooseHforDtoH = Send<BranchingHforD, End>;
type ChooseHforEtoH = Send<BranchingHforE, End>;
type ChooseHforFtoH = Send<BranchingHforF, End>;
type ChooseHforGtoH = Send<BranchingHforG, End>;

// Creating the MP sessions
type EndpointA = SessionMpstEight<End, End, End, End, End, End, RecursAtoH, RoleH<RoleEnd>, NameA>;
type EndpointB = SessionMpstEight<End, End, End, End, End, End, RecursBtoH, RoleH<RoleEnd>, NameB>;
type EndpointC = SessionMpstEight<End, End, End, End, End, End, RecursCtoH, RoleH<RoleEnd>, NameC>;
type EndpointD = SessionMpstEight<End, End, End, End, End, End, RecursDtoH, RoleH<RoleEnd>, NameD>;
type EndpointE = SessionMpstEight<End, End, End, End, End, End, RecursEtoH, RoleH<RoleEnd>, NameE>;
type EndpointF = SessionMpstEight<End, End, End, End, End, End, RecursFtoH, RoleH<RoleEnd>, NameF>;
type EndpointG = SessionMpstEight<End, End, End, End, End, End, RecursGtoH, RoleH<RoleEnd>, NameG>;
type EndpointH = SessionMpstEight<
    ChooseHforAtoH,
    ChooseHforBtoH,
    ChooseHforCtoH,
    ChooseHforDtoH,
    ChooseHforEtoH,
    ChooseHforFtoH,
    ChooseHforGtoH,
    RoleA<RoleB<RoleC<RoleD<RoleE<RoleF<RoleG<RoleEnd>>>>>>>,
    NameH,
>;

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_to_h, {
          BranchingHforA::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingHforA::More(s) => {
            let (_, s) = recv_mpst_a_to_h(s)?;
            let s = send_mpst_a_to_h((), s);
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
            simple_five_endpoint_a(s)
        },
    })
}

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_to_h, {
          BranchingHforB::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingHforB::More(s) => {
            let (_, s) = recv_mpst_b_to_h(s)?;
            let s = send_mpst_b_to_h((), s);
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
            simple_five_endpoint_b(s)
        },
    })
}

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_to_h, {
          BranchingHforC::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingHforC::More(s) => {
            let (_, s) = recv_mpst_c_to_h(s)?;
            let s = send_mpst_c_to_h((), s);
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
            simple_five_endpoint_c(s)
        },
    })
}

fn simple_five_endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_to_h, {
          BranchingHforD::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingHforD::More(s) => {
            let (_, s) = recv_mpst_d_to_h(s)?;
            let s = send_mpst_d_to_h((), s);
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
            simple_five_endpoint_d(s)
        },
    })
}

fn simple_five_endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_e_to_h, {
          BranchingHforE::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingHforE::More(s) => {
            let (_, s) = recv_mpst_e_to_h(s)?;
            let s = send_mpst_e_to_h((), s);
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
            simple_five_endpoint_e(s)
        },
    })
}

fn simple_five_endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_f_to_h, {
          BranchingHforF::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingHforF::More(s) => {
            let (_, s) = recv_mpst_f_to_h(s)?;
            let s = send_mpst_f_to_h((), s);
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
            simple_five_endpoint_f(s)
        },
    })
}

fn simple_five_endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_g_to_h, {
          BranchingHforG::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingHforG::More(s) => {
            let (_, s) = recv_mpst_g_to_h(s)?;
            let s = send_mpst_g_to_h((), s);
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
            simple_five_endpoint_g(s)
        },
    })
}

fn simple_five_endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    recurs_h(s, SIZE)
}

fn recurs_h(s: EndpointH, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_h_to_a,
                send_mpst_h_to_b,
                send_mpst_h_to_c,
                send_mpst_h_to_d,
                send_mpst_h_to_e,
                send_mpst_h_to_f,
                send_mpst_h_to_g, =>
                  BranchingHforA::Done,
                  BranchingHforB::Done,
                  BranchingHforC::Done,
                  BranchingHforD::Done,
                  BranchingHforE::Done,
                  BranchingHforF::Done,
                  BranchingHforG::Done, =>
                RoleA,
                RoleB,
                RoleC,
                RoleD,
                RoleE,
                RoleF,
                RoleG, =>
                RoleH,
                 SessionMpstEight,
                8
            );

            close_mpst_multi(s)
        }
        i => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_h_to_a,
                send_mpst_h_to_b,
                send_mpst_h_to_c,
                send_mpst_h_to_d,
                send_mpst_h_to_e,
                send_mpst_h_to_f,
                send_mpst_h_to_g, =>
                  BranchingHforA::More,
                  BranchingHforB::More,
                  BranchingHforC::More,
                  BranchingHforD::More,
                  BranchingHforE::More,
                  BranchingHforF::More,
                  BranchingHforG::More, =>
                  RoleA,
                  RoleB,
                  RoleC,
                  RoleD,
                  RoleE,
                  RoleF,
                  RoleG, =>
                  RoleH,
                 SessionMpstEight,
                8
            );

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

            recurs_h(s, i - 1)
        }
    }
}

fn all_mpst() -> Result<(), Box<dyn Error>> {
    let (thread_a, thread_b, thread_c, thread_d, thread_e, thread_f, thread_g, thread_h) =
        fork_mpst(
            black_box(simple_five_endpoint_a),
            black_box(simple_five_endpoint_b),
            black_box(simple_five_endpoint_c),
            black_box(simple_five_endpoint_d),
            black_box(simple_five_endpoint_e),
            black_box(simple_five_endpoint_f),
            black_box(simple_five_endpoint_g),
            black_box(simple_five_endpoint_h),
        );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
    thread_f.join().unwrap();
    thread_g.join().unwrap();
    thread_h.join().unwrap();

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

    for _ in 0..28 {
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

static SIZE: i64 = 0;

fn long_simple_protocol_mpst(c: &mut Criterion) {
    c.bench_function(
        &format!("long eight empty simple protocol MPST {}", SIZE),
        |b| b.iter(|| all_mpst()),
    );
}

fn long_simple_protocol_binary(c: &mut Criterion) {
    c.bench_function(
        &format!("long eight empty simple protocol binary {}", SIZE),
        |b| b.iter(|| all_binaries()),
    );
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(10, 0))
}

criterion_group! {
    name = long_eight_empty_simple_protocols;
    config = long_warmup();
    targets = long_simple_protocol_mpst, long_simple_protocol_binary
}
criterion_main!(long_eight_empty_simple_protocols);
