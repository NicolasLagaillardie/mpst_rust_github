#![allow(dead_code)]

use crossbeam_channel::bounded;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose, create_fn_choose_mpst_multi_to_all_bundle,
    create_multiple_normal_role_short, create_recv_mpst_session_bundle,
    create_send_mpst_session_bundle, offer, offer_mpst,
};

use std::error::Error;
use std::thread::{spawn, JoinHandle};
use std::time::Duration;

// Create the new SessionMpst for eleven participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstEleven, 11);

// Create new roles
// normal
create_multiple_normal_role_short!(A, B, C, D, E, F, G, H, I, J, K);

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
    send_mpst_a_to_k, RoleK, next_k, 10 | =>
    RoleA, SessionMpstEleven, 11
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
    send_mpst_b_to_k, RoleK, next_k, 10 | =>
    RoleB, SessionMpstEleven, 11
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
    send_mpst_c_to_k, RoleK, next_k, 10 | =>
    RoleC, SessionMpstEleven, 11
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
    send_mpst_d_to_k, RoleK, next_k, 10 | =>
    RoleD, SessionMpstEleven, 11
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
    send_mpst_e_to_k, RoleK, next_k, 10 | =>
    RoleE, SessionMpstEleven, 11
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
    send_mpst_f_to_k, RoleK, next_k, 10 | =>
    RoleF, SessionMpstEleven, 11
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
    send_mpst_g_to_k, RoleK, next_k, 10 | =>
    RoleG, SessionMpstEleven, 11
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
    send_mpst_h_to_k, RoleK, next_k, 10 | =>
    RoleH, SessionMpstEleven, 11
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
    send_mpst_i_to_k, RoleK, next_k, 10 | =>
    RoleI, SessionMpstEleven, 11
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
    send_mpst_j_to_k, RoleK, next_k, 10 | =>
    RoleJ, SessionMpstEleven, 11
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
    send_mpst_k_to_j, RoleJ, next_j, 10 | =>
    RoleK, SessionMpstEleven, 11
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, next_b, 1 |
    recv_mpst_a_from_c, RoleC, next_c, 2 |
    recv_mpst_a_from_d, RoleD, next_d, 3 |
    recv_mpst_a_from_e, RoleE, next_e, 4 |
    recv_mpst_a_from_f, RoleF, next_f, 5 |
    recv_mpst_a_from_g, RoleG, next_g, 6 |
    recv_mpst_a_from_h, RoleH, next_h, 7 |
    recv_mpst_a_from_i, RoleI, next_i, 8 |
    recv_mpst_a_from_j, RoleJ, next_j, 9 |
    recv_mpst_a_from_k, RoleK, next_k, 10 | =>
    RoleA, SessionMpstEleven, 11
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, next_a, 1 |
    recv_mpst_b_from_c, RoleC, next_c, 2 |
    recv_mpst_b_from_d, RoleD, next_d, 3 |
    recv_mpst_b_from_e, RoleE, next_e, 4 |
    recv_mpst_b_from_f, RoleF, next_f, 5 |
    recv_mpst_b_from_g, RoleG, next_g, 6 |
    recv_mpst_b_from_h, RoleH, next_h, 7 |
    recv_mpst_b_from_i, RoleI, next_i, 8 |
    recv_mpst_b_from_j, RoleJ, next_j, 9 |
    recv_mpst_b_from_k, RoleK, next_k, 10 | =>
    RoleB, SessionMpstEleven, 11
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a, RoleA, next_a, 1 |
    recv_mpst_c_from_b, RoleB, next_b, 2 |
    recv_mpst_c_from_d, RoleD, next_d, 3 |
    recv_mpst_c_from_e, RoleE, next_e, 4 |
    recv_mpst_c_from_f, RoleF, next_f, 5 |
    recv_mpst_c_from_g, RoleG, next_g, 6 |
    recv_mpst_c_from_h, RoleH, next_h, 7 |
    recv_mpst_c_from_i, RoleI, next_i, 8 |
    recv_mpst_c_from_j, RoleJ, next_j, 9 |
    recv_mpst_c_from_k, RoleK, next_k, 10 | =>
    RoleC, SessionMpstEleven, 11
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_a, RoleA, next_a, 1 |
    recv_mpst_d_from_b, RoleB, next_b, 2 |
    recv_mpst_d_from_c, RoleC, next_c, 3 |
    recv_mpst_d_from_e, RoleE, next_e, 4 |
    recv_mpst_d_from_f, RoleF, next_f, 5 |
    recv_mpst_d_from_g, RoleG, next_g, 6 |
    recv_mpst_d_from_h, RoleH, next_h, 7 |
    recv_mpst_d_from_i, RoleI, next_i, 8 |
    recv_mpst_d_from_j, RoleJ, next_j, 9 |
    recv_mpst_d_from_k, RoleK, next_k, 10 | =>
    RoleD, SessionMpstEleven, 11
);
// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_a, RoleA, next_a, 1 |
    recv_mpst_e_from_b, RoleB, next_b, 2 |
    recv_mpst_e_from_c, RoleC, next_c, 3 |
    recv_mpst_e_from_d, RoleD, next_d, 4 |
    recv_mpst_e_from_f, RoleF, next_f, 5 |
    recv_mpst_e_from_g, RoleG, next_g, 6 |
    recv_mpst_e_from_h, RoleH, next_h, 7 |
    recv_mpst_e_from_i, RoleI, next_i, 8 |
    recv_mpst_e_from_j, RoleJ, next_j, 9 |
    recv_mpst_e_from_k, RoleK, next_k, 10 | =>
    RoleE, SessionMpstEleven, 11
);
// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_from_a, RoleA, next_a, 1 |
    recv_mpst_f_from_b, RoleB, next_b, 2 |
    recv_mpst_f_from_c, RoleC, next_c, 3 |
    recv_mpst_f_from_d, RoleD, next_d, 4 |
    recv_mpst_f_from_e, RoleE, next_e, 5 |
    recv_mpst_f_from_g, RoleG, next_g, 6 |
    recv_mpst_f_from_h, RoleH, next_h, 7 |
    recv_mpst_f_from_i, RoleI, next_i, 8 |
    recv_mpst_f_from_j, RoleJ, next_j, 9 |
    recv_mpst_f_from_k, RoleK, next_k, 10 | =>
    RoleF, SessionMpstEleven, 11
);
// G
create_recv_mpst_session_bundle!(
    recv_mpst_g_from_a, RoleA, next_a, 1 |
    recv_mpst_g_from_b, RoleB, next_b, 2 |
    recv_mpst_g_from_c, RoleC, next_c, 3 |
    recv_mpst_g_from_d, RoleD, next_d, 4 |
    recv_mpst_g_from_e, RoleE, next_e, 5 |
    recv_mpst_g_from_f, RoleF, next_f, 6 |
    recv_mpst_g_from_h, RoleH, next_h, 7 |
    recv_mpst_g_from_i, RoleI, next_i, 8 |
    recv_mpst_g_from_j, RoleJ, next_j, 9 |
    recv_mpst_g_from_k, RoleK, next_k, 10 | =>
    RoleG, SessionMpstEleven, 11
);
// H
create_recv_mpst_session_bundle!(
    recv_mpst_h_from_a, RoleA, next_a, 1 |
    recv_mpst_h_from_b, RoleB, next_b, 2 |
    recv_mpst_h_from_c, RoleC, next_c, 3 |
    recv_mpst_h_from_d, RoleD, next_d, 4 |
    recv_mpst_h_from_e, RoleE, next_e, 5 |
    recv_mpst_h_from_f, RoleF, next_f, 6 |
    recv_mpst_h_from_g, RoleG, next_g, 7 |
    recv_mpst_h_from_i, RoleI, next_i, 8 |
    recv_mpst_h_from_j, RoleJ, next_j, 9 |
    recv_mpst_h_from_k, RoleK, next_k, 10 | =>
    RoleH, SessionMpstEleven, 11
);
// I
create_recv_mpst_session_bundle!(
    recv_mpst_i_from_a, RoleA, next_a, 1 |
    recv_mpst_i_from_b, RoleB, next_b, 2 |
    recv_mpst_i_from_c, RoleC, next_c, 3 |
    recv_mpst_i_from_d, RoleD, next_d, 4 |
    recv_mpst_i_from_e, RoleE, next_e, 5 |
    recv_mpst_i_from_f, RoleF, next_f, 6 |
    recv_mpst_i_from_g, RoleG, next_g, 7 |
    recv_mpst_i_from_h, RoleH, next_h, 8 |
    recv_mpst_i_from_j, RoleJ, next_j, 9 |
    recv_mpst_i_from_k, RoleK, next_k, 10 | =>
    RoleI, SessionMpstEleven, 11
);
// J
create_recv_mpst_session_bundle!(
    recv_mpst_j_from_a, RoleA, next_a, 1 |
    recv_mpst_j_from_b, RoleB, next_b, 2 |
    recv_mpst_j_from_c, RoleC, next_c, 3 |
    recv_mpst_j_from_d, RoleD, next_d, 4 |
    recv_mpst_j_from_e, RoleE, next_e, 5 |
    recv_mpst_j_from_f, RoleF, next_f, 6 |
    recv_mpst_j_from_g, RoleG, next_g, 7 |
    recv_mpst_j_from_h, RoleH, next_h, 8 |
    recv_mpst_j_from_i, RoleI, next_i, 9 |
    recv_mpst_j_from_k, RoleK, next_k, 10 | =>
    RoleJ, SessionMpstEleven, 11
);
// K
create_recv_mpst_session_bundle!(
    recv_mpst_k_from_a, RoleA, next_a, 1 |
    recv_mpst_k_from_b, RoleB, next_b, 2 |
    recv_mpst_k_from_c, RoleC, next_c, 3 |
    recv_mpst_k_from_d, RoleD, next_d, 4 |
    recv_mpst_k_from_e, RoleE, next_e, 5 |
    recv_mpst_k_from_f, RoleF, next_f, 6 |
    recv_mpst_k_from_g, RoleG, next_g, 7 |
    recv_mpst_k_from_h, RoleH, next_h, 8 |
    recv_mpst_k_from_i, RoleI, next_i, 9 |
    recv_mpst_k_from_j, RoleJ, next_j, 10 | =>
    RoleK, SessionMpstEleven, 11
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
type Choose0fromKtoA = Send<Branching0fromKtoA, End>;
type Choose0fromKtoB = Send<Branching0fromKtoB, End>;
type Choose0fromKtoC = Send<Branching0fromKtoC, End>;
type Choose0fromKtoD = Send<Branching0fromKtoD, End>;
type Choose0fromKtoE = Send<Branching0fromKtoE, End>;
type Choose0fromKtoF = Send<Branching0fromKtoF, End>;
type Choose0fromKtoG = Send<Branching0fromKtoG, End>;
type Choose0fromKtoH = Send<Branching0fromKtoH, End>;
type Choose0fromKtoI = Send<Branching0fromKtoI, End>;
type Choose0fromKtoJ = Send<Branching0fromKtoJ, End>;
type EndpointDoneK =
    SessionMpstEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameK>;
type EndpointMoreK = SessionMpstEleven<
    Send<(), Recv<(), Choose0fromKtoA>>,
    Send<(), Recv<(), Choose0fromKtoB>>,
    Send<(), Recv<(), Choose0fromKtoC>>,
    Send<(), Recv<(), Choose0fromKtoD>>,
    Send<(), Recv<(), Choose0fromKtoE>>,
    Send<(), Recv<(), Choose0fromKtoF>>,
    Send<(), Recv<(), Choose0fromKtoG>>,
    Send<(), Recv<(), Choose0fromKtoH>>,
    Send<(), Recv<(), Choose0fromKtoI>>,
    Send<(), Recv<(), Choose0fromKtoJ>>,
    R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2H<R2I<R2J<StackRecurs>>>>>>>>>>,
    NameK,
>;

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
type StackRecurs = RoleA<RoleB<RoleC<RoleD<RoleE<RoleF<RoleG<RoleH<RoleI<RoleJ<RoleEnd>>>>>>>>>>;
type EndpointK = SessionMpstEleven<
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
    StackRecurs,
    NameK,
>;

create_fn_choose_mpst_multi_to_all_bundle!(
    done_from_k_to_all, more_from_k_to_all, =>
    Done, More, =>
    EndpointDoneK, EndpointMoreK, =>
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
    RoleK, SessionMpstEleven, 11, 11
);

fn simple_eleven_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_k, {
        Branching0fromKtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoA::More(s) => {
            let (_, s) = recv_mpst_a_from_k(s)?;
            let s = send_mpst_a_to_k((), s);
            let (_, s) = recv_mpst_a_from_b(s)?;
            let s = send_mpst_a_to_b((), s);
            let (_, s) = recv_mpst_a_from_c(s)?;
            let s = send_mpst_a_to_c((), s);
            let (_, s) = recv_mpst_a_from_d(s)?;
            let s = send_mpst_a_to_d((), s);
            let (_, s) = recv_mpst_a_from_e(s)?;
            let s = send_mpst_a_to_e((), s);
            let (_, s) = recv_mpst_a_from_f(s)?;
            let s = send_mpst_a_to_f((), s);
            let (_, s) = recv_mpst_a_from_g(s)?;
            let s = send_mpst_a_to_g((), s);
            let (_, s) = recv_mpst_a_from_h(s)?;
            let s = send_mpst_a_to_h((), s);
            let (_, s) = recv_mpst_a_from_i(s)?;
            let s = send_mpst_a_to_i((), s);
            let (_, s) = recv_mpst_a_from_j(s)?;
            let s = send_mpst_a_to_j((), s);
            simple_eleven_endpoint_a(s)
        },
    })
}

fn simple_eleven_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_k, {
        Branching0fromKtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoB::More(s) => {
            let (_, s) = recv_mpst_b_from_k(s)?;
            let s = send_mpst_b_to_k((), s);
            let s = send_mpst_b_to_a((), s);
            let (_, s) = recv_mpst_b_from_a(s)?;
            let (_, s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_c((), s);
            let (_, s) = recv_mpst_b_from_d(s)?;
            let s = send_mpst_b_to_d((), s);
            let (_, s) = recv_mpst_b_from_e(s)?;
            let s = send_mpst_b_to_e((), s);
            let (_, s) = recv_mpst_b_from_f(s)?;
            let s = send_mpst_b_to_f((), s);
            let (_, s) = recv_mpst_b_from_g(s)?;
            let s = send_mpst_b_to_g((), s);
            let (_, s) = recv_mpst_b_from_h(s)?;
            let s = send_mpst_b_to_h((), s);
            let (_, s) = recv_mpst_b_from_i(s)?;
            let s = send_mpst_b_to_i((), s);
            let (_, s) = recv_mpst_b_from_j(s)?;
            let s = send_mpst_b_to_j((), s);
            simple_eleven_endpoint_b(s)
        },
    })
}

fn simple_eleven_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_k, {
        Branching0fromKtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoC::More(s) => {
            let (_, s) = recv_mpst_c_from_k(s)?;
            let s = send_mpst_c_to_k((), s);
            let s = send_mpst_c_to_a((), s);
            let (_, s) = recv_mpst_c_from_a(s)?;
            let s = send_mpst_c_to_b((), s);
            let (_, s) = recv_mpst_c_from_b(s)?;
            let (_, s) = recv_mpst_c_from_d(s)?;
            let s = send_mpst_c_to_d((), s);
            let (_, s) = recv_mpst_c_from_e(s)?;
            let s = send_mpst_c_to_e((), s);
            let (_, s) = recv_mpst_c_from_f(s)?;
            let s = send_mpst_c_to_f((), s);
            let (_, s) = recv_mpst_c_from_g(s)?;
            let s = send_mpst_c_to_g((), s);
            let (_, s) = recv_mpst_c_from_h(s)?;
            let s = send_mpst_c_to_h((), s);
            let (_, s) = recv_mpst_c_from_i(s)?;
            let s = send_mpst_c_to_i((), s);
            let (_, s) = recv_mpst_c_from_j(s)?;
            let s = send_mpst_c_to_j((), s);
            simple_eleven_endpoint_c(s)
        },
    })
}

fn simple_eleven_endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_from_k, {
        Branching0fromKtoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoD::More(s) => {
            let (_, s) = recv_mpst_d_from_k(s)?;
            let s = send_mpst_d_to_k((), s);
            let s = send_mpst_d_to_a((), s);
            let (_, s) = recv_mpst_d_from_a(s)?;
            let s = send_mpst_d_to_b((), s);
            let (_, s) = recv_mpst_d_from_b(s)?;
            let s = send_mpst_d_to_c((), s);
            let (_, s) = recv_mpst_d_from_c(s)?;
            let (_, s) = recv_mpst_d_from_e(s)?;
            let s = send_mpst_d_to_e((), s);
            let (_, s) = recv_mpst_d_from_f(s)?;
            let s = send_mpst_d_to_f((), s);
            let (_, s) = recv_mpst_d_from_g(s)?;
            let s = send_mpst_d_to_g((), s);
            let (_, s) = recv_mpst_d_from_h(s)?;
            let s = send_mpst_d_to_h((), s);
            let (_, s) = recv_mpst_d_from_i(s)?;
            let s = send_mpst_d_to_i((), s);
            let (_, s) = recv_mpst_d_from_j(s)?;
            let s = send_mpst_d_to_j((), s);
            simple_eleven_endpoint_d(s)
        },
    })
}

fn simple_eleven_endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_e_from_k, {
        Branching0fromKtoE::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoE::More(s) => {
            let (_, s) = recv_mpst_e_from_k(s)?;
            let s = send_mpst_e_to_k((), s);
            let s = send_mpst_e_to_a((), s);
            let (_, s) = recv_mpst_e_from_a(s)?;
            let s = send_mpst_e_to_b((), s);
            let (_, s) = recv_mpst_e_from_b(s)?;
            let s = send_mpst_e_to_c((), s);
            let (_, s) = recv_mpst_e_from_c(s)?;
            let s = send_mpst_e_to_d((), s);
            let (_, s) = recv_mpst_e_from_d(s)?;
            let (_, s) = recv_mpst_e_from_f(s)?;
            let s = send_mpst_e_to_f((), s);
            let (_, s) = recv_mpst_e_from_g(s)?;
            let s = send_mpst_e_to_g((), s);
            let (_, s) = recv_mpst_e_from_h(s)?;
            let s = send_mpst_e_to_h((), s);
            let (_, s) = recv_mpst_e_from_i(s)?;
            let s = send_mpst_e_to_i((), s);
            let (_, s) = recv_mpst_e_from_j(s)?;
            let s = send_mpst_e_to_j((), s);
            simple_eleven_endpoint_e(s)
        },
    })
}

fn simple_eleven_endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_f_from_k, {
        Branching0fromKtoF::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoF::More(s) => {
            let (_, s) = recv_mpst_f_from_k(s)?;
            let s = send_mpst_f_to_k((), s);
            let s = send_mpst_f_to_a((), s);
            let (_, s) = recv_mpst_f_from_a(s)?;
            let s = send_mpst_f_to_b((), s);
            let (_, s) = recv_mpst_f_from_b(s)?;
            let s = send_mpst_f_to_c((), s);
            let (_, s) = recv_mpst_f_from_c(s)?;
            let s = send_mpst_f_to_d((), s);
            let (_, s) = recv_mpst_f_from_d(s)?;
            let s = send_mpst_f_to_e((), s);
            let (_, s) = recv_mpst_f_from_e(s)?;
            let (_, s) = recv_mpst_f_from_g(s)?;
            let s = send_mpst_f_to_g((), s);
            let (_, s) = recv_mpst_f_from_h(s)?;
            let s = send_mpst_f_to_h((), s);
            let (_, s) = recv_mpst_f_from_i(s)?;
            let s = send_mpst_f_to_i((), s);
            let (_, s) = recv_mpst_f_from_j(s)?;
            let s = send_mpst_f_to_j((), s);
            simple_eleven_endpoint_f(s)
        },
    })
}

fn simple_eleven_endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_g_from_k, {
        Branching0fromKtoG::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoG::More(s) => {
            let (_, s) = recv_mpst_g_from_k(s)?;
            let s = send_mpst_g_to_k((), s);
            let s = send_mpst_g_to_a((), s);
            let (_, s) = recv_mpst_g_from_a(s)?;
            let s = send_mpst_g_to_b((), s);
            let (_, s) = recv_mpst_g_from_b(s)?;
            let s = send_mpst_g_to_c((), s);
            let (_, s) = recv_mpst_g_from_c(s)?;
            let s = send_mpst_g_to_d((), s);
            let (_, s) = recv_mpst_g_from_d(s)?;
            let s = send_mpst_g_to_e((), s);
            let (_, s) = recv_mpst_g_from_e(s)?;
            let s = send_mpst_g_to_f((), s);
            let (_, s) = recv_mpst_g_from_f(s)?;
            let (_, s) = recv_mpst_g_from_h(s)?;
            let s = send_mpst_g_to_h((), s);
            let (_, s) = recv_mpst_g_from_i(s)?;
            let s = send_mpst_g_to_i((), s);
            let (_, s) = recv_mpst_g_from_j(s)?;
            let s = send_mpst_g_to_j((), s);
            simple_eleven_endpoint_g(s)
        },
    })
}

fn simple_eleven_endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_h_from_k, {
        Branching0fromKtoH::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoH::More(s) => {
            let (_, s) = recv_mpst_h_from_k(s)?;
            let s = send_mpst_h_to_k((), s);
            let s = send_mpst_h_to_a((), s);
            let (_, s) = recv_mpst_h_from_a(s)?;
            let s = send_mpst_h_to_b((), s);
            let (_, s) = recv_mpst_h_from_b(s)?;
            let s = send_mpst_h_to_c((), s);
            let (_, s) = recv_mpst_h_from_c(s)?;
            let s = send_mpst_h_to_d((), s);
            let (_, s) = recv_mpst_h_from_d(s)?;
            let s = send_mpst_h_to_e((), s);
            let (_, s) = recv_mpst_h_from_e(s)?;
            let s = send_mpst_h_to_f((), s);
            let (_, s) = recv_mpst_h_from_f(s)?;
            let s = send_mpst_h_to_g((), s);
            let (_, s) = recv_mpst_h_from_g(s)?;
            let (_, s) = recv_mpst_h_from_i(s)?;
            let s = send_mpst_h_to_i((), s);
            let (_, s) = recv_mpst_h_from_j(s)?;
            let s = send_mpst_h_to_j((), s);
            simple_eleven_endpoint_h(s)
        },
    })
}

fn simple_eleven_endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_i_from_k, {
        Branching0fromKtoI::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoI::More(s) => {
            let (_, s) = recv_mpst_i_from_k(s)?;
            let s = send_mpst_i_to_k((), s);
            let s = send_mpst_i_to_a((), s);
            let (_, s) = recv_mpst_i_from_a(s)?;
            let s = send_mpst_i_to_b((), s);
            let (_, s) = recv_mpst_i_from_b(s)?;
            let s = send_mpst_i_to_c((), s);
            let (_, s) = recv_mpst_i_from_c(s)?;
            let s = send_mpst_i_to_d((), s);
            let (_, s) = recv_mpst_i_from_d(s)?;
            let s = send_mpst_i_to_e((), s);
            let (_, s) = recv_mpst_i_from_e(s)?;
            let s = send_mpst_i_to_f((), s);
            let (_, s) = recv_mpst_i_from_f(s)?;
            let s = send_mpst_i_to_g((), s);
            let (_, s) = recv_mpst_i_from_g(s)?;
            let s = send_mpst_i_to_h((), s);
            let (_, s) = recv_mpst_i_from_h(s)?;
            let (_, s) = recv_mpst_i_from_j(s)?;
            let s = send_mpst_i_to_j((), s);
            simple_eleven_endpoint_i(s)
        },
    })
}

fn simple_eleven_endpoint_j(s: EndpointJ) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_j_from_k, {
        Branching0fromKtoJ::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoJ::More(s) => {
            let (_, s) = recv_mpst_j_from_k(s)?;
            let s = send_mpst_j_to_k((), s);
            let s = send_mpst_j_to_a((), s);
            let (_, s) = recv_mpst_j_from_a(s)?;
            let s = send_mpst_j_to_b((), s);
            let (_, s) = recv_mpst_j_from_b(s)?;
            let s = send_mpst_j_to_c((), s);
            let (_, s) = recv_mpst_j_from_c(s)?;
            let s = send_mpst_j_to_d((), s);
            let (_, s) = recv_mpst_j_from_d(s)?;
            let s = send_mpst_j_to_e((), s);
            let (_, s) = recv_mpst_j_from_e(s)?;
            let s = send_mpst_j_to_f((), s);
            let (_, s) = recv_mpst_j_from_f(s)?;
            let s = send_mpst_j_to_g((), s);
            let (_, s) = recv_mpst_j_from_g(s)?;
            let s = send_mpst_j_to_h((), s);
            let (_, s) = recv_mpst_j_from_h(s)?;
            let s = send_mpst_j_to_i((), s);
            let (_, s) = recv_mpst_j_from_i(s)?;
            simple_eleven_endpoint_j(s)
        },
    })
}

fn simple_eleven_endpoint_k(s: EndpointK) -> Result<(), Box<dyn Error>> {
    recurs_k(s, SIZE)
}

fn recurs_k(s: EndpointK, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_k_to_all(s);

            close_mpst_multi(s)
        }
        i => {
            let s = more_from_k_to_all(s);

            let s = send_mpst_k_to_a((), s);
            let (_, s) = recv_mpst_k_from_a(s)?;
            let s = send_mpst_k_to_b((), s);
            let (_, s) = recv_mpst_k_from_b(s)?;
            let s = send_mpst_k_to_c((), s);
            let (_, s) = recv_mpst_k_from_c(s)?;
            let s = send_mpst_k_to_d((), s);
            let (_, s) = recv_mpst_k_from_d(s)?;
            let s = send_mpst_k_to_e((), s);
            let (_, s) = recv_mpst_k_from_e(s)?;
            let s = send_mpst_k_to_f((), s);
            let (_, s) = recv_mpst_k_from_f(s)?;
            let s = send_mpst_k_to_g((), s);
            let (_, s) = recv_mpst_k_from_g(s)?;
            let s = send_mpst_k_to_h((), s);
            let (_, s) = recv_mpst_k_from_h(s)?;
            let s = send_mpst_k_to_i((), s);
            let (_, s) = recv_mpst_k_from_i(s)?;
            let s = send_mpst_k_to_j((), s);
            let (_, s) = recv_mpst_k_from_j(s)?;

            recurs_k(s, i - 1)
        }
    }
}

fn all_mpst() -> Result<(), Box<dyn std::any::Any + std::marker::Send>> {
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
        black_box(simple_eleven_endpoint_a),
        black_box(simple_eleven_endpoint_b),
        black_box(simple_eleven_endpoint_c),
        black_box(simple_eleven_endpoint_d),
        black_box(simple_eleven_endpoint_e),
        black_box(simple_eleven_endpoint_f),
        black_box(simple_eleven_endpoint_g),
        black_box(simple_eleven_endpoint_h),
        black_box(simple_eleven_endpoint_i),
        black_box(simple_eleven_endpoint_j),
        black_box(simple_eleven_endpoint_k),
    );

    thread_a.join()?;
    thread_b.join()?;
    thread_c.join()?;
    thread_d.join()?;
    thread_e.join()?;
    thread_f.join()?;
    thread_g.join()?;
    thread_h.join()?;
    thread_i.join()?;
    thread_j.join()?;
    thread_k.join()?;

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

fn all_binaries() -> Result<(), Box<dyn std::any::Any + std::marker::Send>> {
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

    main.join()?;

    Ok(())
}

/////////////////////////

type ReceivingSendingReceiving = crossbeam_channel::Receiver<SendingReceiving>;
type SendingReceivingSending = crossbeam_channel::Sender<ReceivingSending>;

type SendingReceiving = crossbeam_channel::Sender<Receiving>;
type ReceivingSending = crossbeam_channel::Receiver<Sending>;

type Receiving = crossbeam_channel::Receiver<()>;
type Sending = crossbeam_channel::Sender<()>;

fn all_crossbeam() -> Result<(), Box<dyn Error>> {
    let mut threads = Vec::new();

    for _ in 0..55 {
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

fn long_simple_protocol_mpst(c: &mut Criterion) {
    c.bench_function(
        &format!("long eleven empty simple protocol MPST {}", SIZE),
        |b| b.iter(|| all_mpst()),
    );
}

fn long_simple_protocol_binary(c: &mut Criterion) {
    c.bench_function(
        &format!("long eleven empty simple protocol binary {}", SIZE),
        |b| b.iter(|| all_binaries()),
    );
}

fn long_simple_protocol_crossbeam(c: &mut Criterion) {
    c.bench_function(
        &format!("long eleven empty simple protocol crossbeam {}", SIZE),
        |b| b.iter(|| all_crossbeam()),
    );
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(20, 0))
}

criterion_group! {
    name = long_eleven_empty_simple_protocols;
    config = long_warmup();
    targets = long_simple_protocol_mpst, long_simple_protocol_binary, long_simple_protocol_crossbeam
}
criterion_main!(long_eleven_empty_simple_protocols);
