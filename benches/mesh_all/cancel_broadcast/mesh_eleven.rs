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

// Create the new MeshedChannels for eleven participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsTwelve, 12);

// Create new roles
// normal
create_multiple_normal_role_short!(Central, A, B, C, D, E, F, G, H, I, J, K);

// Create new names
create_multiple_normal_name_short!(Central, A, B, C, D, E, F, G, H, I, J, K);

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
    send_mpst_a_to_k, RoleK, 11 | =>
    NameA, MeshedChannelsTwelve, 12
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
    send_mpst_b_to_k, RoleK, 11 | =>
    NameB, MeshedChannelsTwelve, 12
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
    send_mpst_c_to_k, RoleK, 11 | =>
    NameC, MeshedChannelsTwelve, 12
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
    send_mpst_d_to_k, RoleK, 11 | =>
    NameD, MeshedChannelsTwelve, 12
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
    send_mpst_e_to_k, RoleK, 11 | =>
    NameE, MeshedChannelsTwelve, 12
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
    send_mpst_f_to_k, RoleK, 11 | =>
    NameF, MeshedChannelsTwelve, 12
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
    send_mpst_g_to_k, RoleK, 11 | =>
    NameG, MeshedChannelsTwelve, 12
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
    send_mpst_h_to_k, RoleK, 11 | =>
    NameH, MeshedChannelsTwelve, 12
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
    send_mpst_i_to_k, RoleK, 11 | =>
    NameI, MeshedChannelsTwelve, 12
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
    send_mpst_j_to_k, RoleK, 11 | =>
    NameJ, MeshedChannelsTwelve, 12
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
    send_mpst_k_to_j, RoleJ, 11 | =>
    NameK, MeshedChannelsTwelve, 12
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
    recv_mpst_a_from_k, RoleK, 11 | =>
    NameA, MeshedChannelsTwelve, 12
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
    recv_mpst_b_from_k, RoleK, 11 | =>
    NameB, MeshedChannelsTwelve, 12
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
    recv_mpst_c_from_k, RoleK, 11 | =>
    NameC, MeshedChannelsTwelve, 12
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
    recv_mpst_d_from_k, RoleK, 11 | =>
    NameD, MeshedChannelsTwelve, 12
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
    recv_mpst_e_from_k, RoleK, 11 | =>
    NameE, MeshedChannelsTwelve, 12
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
    recv_mpst_f_from_k, RoleK, 11 | =>
    NameF, MeshedChannelsTwelve, 12
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
    recv_mpst_g_from_k, RoleK, 11 | =>
    NameG, MeshedChannelsTwelve, 12
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
    recv_mpst_h_from_k, RoleK, 11 | =>
    NameH, MeshedChannelsTwelve, 12
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
    recv_mpst_i_from_k, RoleK, 11 | =>
    NameI, MeshedChannelsTwelve, 12
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
    recv_mpst_j_from_k, RoleK, 11 | =>
    NameJ, MeshedChannelsTwelve, 12
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
    recv_mpst_k_from_j, RoleJ, 11 | =>
    NameK, MeshedChannelsTwelve, 12
);

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
        MeshedChannelsTwelve<
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
            Recv<(), Send<(), RecursAtoK>>,
            R2K<R2B<R2C<R2D<R2E<R2F<R2G<R2H<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameA,
        >,
    ),
    Done(
        MeshedChannelsTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameA>,
    ),
}
type RecursAtoK = Recv<(End, Branching0fromKtoA), End>;

// B
enum Branching0fromKtoB {
    More(
        MeshedChannelsTwelve<
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
            Recv<(), Send<(), RecursBtoK>>,
            R2K<R2A<R2C<R2D<R2E<R2F<R2G<R2H<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameB,
        >,
    ),
    Done(
        MeshedChannelsTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameB>,
    ),
}
type RecursBtoK = Recv<(End, Branching0fromKtoB), End>;

// C
enum Branching0fromKtoC {
    More(
        MeshedChannelsTwelve<
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
            Recv<(), Send<(), RecursCtoK>>,
            R2K<R2A<R2B<R2D<R2E<R2F<R2G<R2H<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameC,
        >,
    ),
    Done(
        MeshedChannelsTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameC>,
    ),
}
type RecursCtoK = Recv<(End, Branching0fromKtoC), End>;

// D
enum Branching0fromKtoD {
    More(
        MeshedChannelsTwelve<
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
            Recv<(), Send<(), RecursDtoK>>,
            R2K<R2A<R2B<R2C<R2E<R2F<R2G<R2H<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameD,
        >,
    ),
    Done(
        MeshedChannelsTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameD>,
    ),
}
type RecursDtoK = Recv<(End, Branching0fromKtoD), End>;

// E
enum Branching0fromKtoE {
    More(
        MeshedChannelsTwelve<
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
            Recv<(), Send<(), RecursEtoK>>,
            R2K<R2A<R2B<R2C<R2D<R2F<R2G<R2H<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameE,
        >,
    ),
    Done(
        MeshedChannelsTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameE>,
    ),
}
type RecursEtoK = Recv<(End, Branching0fromKtoE), End>;

// F
enum Branching0fromKtoF {
    More(
        MeshedChannelsTwelve<
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
            Recv<(), Send<(), RecursFtoK>>,
            R2K<R2A<R2B<R2C<R2D<R2E<R2G<R2H<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameF,
        >,
    ),
    Done(
        MeshedChannelsTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameF>,
    ),
}
type RecursFtoK = Recv<(End, Branching0fromKtoF), End>;

// G
enum Branching0fromKtoG {
    More(
        MeshedChannelsTwelve<
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
            Recv<(), Send<(), RecursGtoK>>,
            R2K<R2A<R2B<R2C<R2D<R2E<R2F<R2H<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameG,
        >,
    ),
    Done(
        MeshedChannelsTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameG>,
    ),
}
type RecursGtoK = Recv<(End, Branching0fromKtoG), End>;

// H
enum Branching0fromKtoH {
    More(
        MeshedChannelsTwelve<
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
            Recv<(), Send<(), RecursHtoK>>,
            R2K<R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameH,
        >,
    ),
    Done(
        MeshedChannelsTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameH>,
    ),
}
type RecursHtoK = Recv<(End, Branching0fromKtoH), End>;

// I
enum Branching0fromKtoI {
    More(
        MeshedChannelsTwelve<
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
            Recv<(), Send<(), RecursItoK>>,
            R2K<R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2H<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameI,
        >,
    ),
    Done(
        MeshedChannelsTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameI>,
    ),
}
type RecursItoK = Recv<(End, Branching0fromKtoI), End>;

// J
enum Branching0fromKtoJ {
    More(
        MeshedChannelsTwelve<
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
            Recv<(), Send<(), RecursJtoK>>,
            R2K<R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2H<R2I<RoleK<RoleEnd>>>>>>>>>>>,
            NameJ,
        >,
    ),
    Done(
        MeshedChannelsTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameJ>,
    ),
}
type RecursJtoK = Recv<(End, Branching0fromKtoJ), End>;

// K
type Choose0fromKtoA = <RecursAtoK as Session>::Dual;
type Choose0fromKtoB = <RecursBtoK as Session>::Dual;
type Choose0fromKtoC = <RecursCtoK as Session>::Dual;
type Choose0fromKtoD = <RecursDtoK as Session>::Dual;
type Choose0fromKtoE = <RecursEtoK as Session>::Dual;
type Choose0fromKtoF = <RecursFtoK as Session>::Dual;
type Choose0fromKtoG = <RecursGtoK as Session>::Dual;
type Choose0fromKtoH = <RecursHtoK as Session>::Dual;
type Choose0fromKtoI = <RecursItoK as Session>::Dual;
type Choose0fromKtoJ = <RecursJtoK as Session>::Dual;
type EndpointDoneK =
    MeshedChannelsTwelve<End, End, End, End, End, End, End, End, End, End, End, RoleEnd, NameK>;
type EndpointMoreK = MeshedChannelsTwelve<
    End,
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
    R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2H<R2I<R2J<RoleBroadcast>>>>>>>>>>,
    NameK,
>;

// Creating the MP sessions
type EndpointCentral = MeshedChannelsTwelve<
    End,
    End,
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
type EndpointA = MeshedChannelsTwelve<
    End,
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
type EndpointB = MeshedChannelsTwelve<
    End,
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
type EndpointC = MeshedChannelsTwelve<
    End,
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
type EndpointD = MeshedChannelsTwelve<
    End,
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
type EndpointE = MeshedChannelsTwelve<
    End,
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
type EndpointF = MeshedChannelsTwelve<
    End,
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
type EndpointG = MeshedChannelsTwelve<
    End,
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
type EndpointH = MeshedChannelsTwelve<
    End,
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
type EndpointI = MeshedChannelsTwelve<
    End,
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
type EndpointJ = MeshedChannelsTwelve<
    End,
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
type EndpointK = MeshedChannelsTwelve<
    End,
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
    RoleBroadcast,
    NameK,
>;

create_fn_choose_mpst_cancel_multi_to_all_bundle!(
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
    NameA,
    NameB,
    NameC,
    NameD,
    NameE,
    NameF,
    NameG,
    NameH,
    NameI,
    NameJ, =>
    NameCentral, NameK, MeshedChannelsTwelve, 12
);

fn endpoint_central(s: EndpointCentral) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, 12)
}

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_a_from_k, {
        Branching0fromKtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoA::More(s) => {
            let (_, s) = recv_mpst_a_from_k(s)?;
            let s = send_mpst_a_to_k((), s)?;
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
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_b_from_k, {
        Branching0fromKtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoB::More(s) => {
            let (_, s) = recv_mpst_b_from_k(s)?;
            let s = send_mpst_b_to_k((), s)?;
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
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_c_from_k, {
        Branching0fromKtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoC::More(s) => {
            let (_, s) = recv_mpst_c_from_k(s)?;
            let s = send_mpst_c_to_k((), s)?;
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
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_d_from_k, {
        Branching0fromKtoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoD::More(s) => {
            let (_, s) = recv_mpst_d_from_k(s)?;
            let s = send_mpst_d_to_k((), s)?;
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
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_e_from_k, {
        Branching0fromKtoE::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoE::More(s) => {
            let (_, s) = recv_mpst_e_from_k(s)?;
            let s = send_mpst_e_to_k((), s)?;
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
            endpoint_e(s)
        },
    })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_f_from_k, {
        Branching0fromKtoF::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoF::More(s) => {
            let (_, s) = recv_mpst_f_from_k(s)?;
            let s = send_mpst_f_to_k((), s)?;
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
            endpoint_f(s)
        },
    })
}

fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_g_from_k, {
        Branching0fromKtoG::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoG::More(s) => {
            let (_, s) = recv_mpst_g_from_k(s)?;
            let s = send_mpst_g_to_k((), s)?;
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
            endpoint_g(s)
        },
    })
}

fn endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_h_from_k, {
        Branching0fromKtoH::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoH::More(s) => {
            let (_, s) = recv_mpst_h_from_k(s)?;
            let s = send_mpst_h_to_k((), s)?;
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
            endpoint_h(s)
        },
    })
}

fn endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_i_from_k, {
        Branching0fromKtoI::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoI::More(s) => {
            let (_, s) = recv_mpst_i_from_k(s)?;
            let s = send_mpst_i_to_k((), s)?;
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
            endpoint_i(s)
        },
    })
}

fn endpoint_j(s: EndpointJ) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_j_from_k, {
        Branching0fromKtoJ::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromKtoJ::More(s) => {
            let (_, s) = recv_mpst_j_from_k(s)?;
            let s = send_mpst_j_to_k((), s)?;
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
            endpoint_j(s)
        },
    })
}

fn endpoint_k(s: EndpointK) -> Result<(), Box<dyn Error>> {
    recurs_k(s, LOOPS)
}

fn recurs_k(s: EndpointK, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_k_to_all(s)?;

            close_mpst_multi(s)
        }
        i => {
            let s = more_from_k_to_all(s)?;

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

            recurs_k(s, i - 1)
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
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn mesh_protocol_mpst(c: &mut Criterion) {
    c.bench_function(
        &format!("mesh eleven cancel broadcast protocol {LOOPS}"),
        |b| b.iter(aux),
    );
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(20000);
    targets = mesh_protocol_mpst,
}

criterion_main! {
    bench
}
