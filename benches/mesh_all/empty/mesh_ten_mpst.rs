#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, create_fn_choose_mpst_multi_to_all_bundle,
    create_multiple_normal_name_short, create_multiple_normal_role_short,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, offer_mpst,
};

use std::error::Error;

// Create the new MeshedChannels for ten participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannels, 10);

// Create new roles
// normal
create_multiple_normal_role_short!(A, B, C, D, E, F, G, H, I, J);

// Create new names
create_multiple_normal_name_short!(A, B, C, D, E, F, G, H, I, J);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, 1 |
    send_mpst_a_to_c, RoleC, 2 |
    send_mpst_a_to_d, RoleD, 3 |
    send_mpst_a_to_e, RoleE, 4 |
    send_mpst_a_to_f, RoleF, 5 |
    send_mpst_a_to_g, RoleG, 6 |
    send_mpst_a_to_h, RoleH, 7 |
    send_mpst_a_to_i, RoleI, 8 |
    send_mpst_a_to_j, RoleJ, 9 | =>
    NameA, MeshedChannels, 10
);

// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a, RoleA, 1 |
    send_mpst_b_to_c, RoleC, 2 |
    send_mpst_b_to_d, RoleD, 3 |
    send_mpst_b_to_e, RoleE, 4 |
    send_mpst_b_to_f, RoleF, 5 |
    send_mpst_b_to_g, RoleG, 6 |
    send_mpst_b_to_h, RoleH, 7 |
    send_mpst_b_to_i, RoleI, 8 |
    send_mpst_b_to_j, RoleJ, 9 | =>
    NameB, MeshedChannels, 10
);

// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a, RoleA, 1 |
    send_mpst_c_to_b, RoleB, 2 |
    send_mpst_c_to_d, RoleD, 3 |
    send_mpst_c_to_e, RoleE, 4 |
    send_mpst_c_to_f, RoleF, 5 |
    send_mpst_c_to_g, RoleG, 6 |
    send_mpst_c_to_h, RoleH, 7 |
    send_mpst_c_to_i, RoleI, 8 |
    send_mpst_c_to_j, RoleJ, 9 | =>
    NameC, MeshedChannels, 10
);

// D
create_send_mpst_session_bundle!(
    send_mpst_d_to_a, RoleA, 1 |
    send_mpst_d_to_b, RoleB, 2 |
    send_mpst_d_to_c, RoleC, 3 |
    send_mpst_d_to_e, RoleE, 4 |
    send_mpst_d_to_f, RoleF, 5 |
    send_mpst_d_to_g, RoleG, 6 |
    send_mpst_d_to_h, RoleH, 7 |
    send_mpst_d_to_i, RoleI, 8 |
    send_mpst_d_to_j, RoleJ, 9 | =>
    NameD, MeshedChannels, 10
);

// E
create_send_mpst_session_bundle!(
    send_mpst_e_to_a, RoleA, 1 |
    send_mpst_e_to_b, RoleB, 2 |
    send_mpst_e_to_c, RoleC, 3 |
    send_mpst_e_to_d, RoleD, 4 |
    send_mpst_e_to_f, RoleF, 5 |
    send_mpst_e_to_g, RoleG, 6 |
    send_mpst_e_to_h, RoleH, 7 |
    send_mpst_e_to_i, RoleI, 8 |
    send_mpst_e_to_j, RoleJ, 9 | =>
    NameE, MeshedChannels, 10
);

// F
create_send_mpst_session_bundle!(
    send_mpst_f_to_a, RoleA, 1 |
    send_mpst_f_to_b, RoleB, 2 |
    send_mpst_f_to_c, RoleC, 3 |
    send_mpst_f_to_d, RoleD, 4 |
    send_mpst_f_to_e, RoleE, 5 |
    send_mpst_f_to_g, RoleG, 6 |
    send_mpst_f_to_h, RoleH, 7 |
    send_mpst_f_to_i, RoleI, 8 |
    send_mpst_f_to_j, RoleJ, 9 | =>
    NameF, MeshedChannels, 10
);

// G
create_send_mpst_session_bundle!(
    send_mpst_g_to_a, RoleA, 1 |
    send_mpst_g_to_b, RoleB, 2 |
    send_mpst_g_to_c, RoleC, 3 |
    send_mpst_g_to_d, RoleD, 4 |
    send_mpst_g_to_e, RoleE, 5 |
    send_mpst_g_to_f, RoleF, 6 |
    send_mpst_g_to_h, RoleH, 7 |
    send_mpst_g_to_i, RoleI, 8 |
    send_mpst_g_to_j, RoleJ, 9 | =>
    NameG, MeshedChannels, 10
);

// H
create_send_mpst_session_bundle!(
    send_mpst_h_to_a, RoleA, 1 |
    send_mpst_h_to_b, RoleB, 2 |
    send_mpst_h_to_c, RoleC, 3 |
    send_mpst_h_to_d, RoleD, 4 |
    send_mpst_h_to_e, RoleE, 5 |
    send_mpst_h_to_f, RoleF, 6 |
    send_mpst_h_to_g, RoleG, 7 |
    send_mpst_h_to_i, RoleI, 8 |
    send_mpst_h_to_j, RoleJ, 9 | =>
    NameH, MeshedChannels, 10
);

// I
create_send_mpst_session_bundle!(
    send_mpst_i_to_a, RoleA, 1 |
    send_mpst_i_to_b, RoleB, 2 |
    send_mpst_i_to_c, RoleC, 3 |
    send_mpst_i_to_d, RoleD, 4 |
    send_mpst_i_to_e, RoleE, 5 |
    send_mpst_i_to_f, RoleF, 6 |
    send_mpst_i_to_g, RoleG, 7 |
    send_mpst_i_to_h, RoleH, 8 |
    send_mpst_i_to_j, RoleJ, 9 | =>
    NameI, MeshedChannels, 10
);

// J
create_send_mpst_session_bundle!(
    send_mpst_j_to_a, RoleA, 1 |
    send_mpst_j_to_b, RoleB, 2 |
    send_mpst_j_to_c, RoleC, 3 |
    send_mpst_j_to_d, RoleD, 4 |
    send_mpst_j_to_e, RoleE, 5 |
    send_mpst_j_to_f, RoleF, 6 |
    send_mpst_j_to_g, RoleG, 7 |
    send_mpst_j_to_h, RoleH, 8 |
    send_mpst_j_to_i, RoleI, 9 | =>
    NameJ, MeshedChannels, 10
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 1 |
    recv_mpst_a_from_c, RoleC, 2 |
    recv_mpst_a_from_d, RoleD, 3 |
    recv_mpst_a_from_e, RoleE, 4 |
    recv_mpst_a_from_f, RoleF, 5 |
    recv_mpst_a_from_g, RoleG, 6 |
    recv_mpst_a_from_h, RoleH, 7 |
    recv_mpst_a_from_i, RoleI, 8 |
    recv_mpst_a_from_j, RoleJ, 9 | =>
    NameA, MeshedChannels, 10
);

// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 1 |
    recv_mpst_b_from_c, RoleC, 2 |
    recv_mpst_b_from_d, RoleD, 3 |
    recv_mpst_b_from_e, RoleE, 4 |
    recv_mpst_b_from_f, RoleF, 5 |
    recv_mpst_b_from_g, RoleG, 6 |
    recv_mpst_b_from_h, RoleH, 7 |
    recv_mpst_b_from_i, RoleI, 8 |
    recv_mpst_b_from_j, RoleJ, 9 | =>
    NameB, MeshedChannels, 10
);

// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a, RoleA, 1 |
    recv_mpst_c_from_b, RoleB, 2 |
    recv_mpst_c_from_d, RoleD, 3 |
    recv_mpst_c_from_e, RoleE, 4 |
    recv_mpst_c_from_f, RoleF, 5 |
    recv_mpst_c_from_g, RoleG, 6 |
    recv_mpst_c_from_h, RoleH, 7 |
    recv_mpst_c_from_i, RoleI, 8 |
    recv_mpst_c_from_j, RoleJ, 9 | =>
    NameC, MeshedChannels, 10
);

// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_a, RoleA, 1 |
    recv_mpst_d_from_b, RoleB, 2 |
    recv_mpst_d_from_c, RoleC, 3 |
    recv_mpst_d_from_e, RoleE, 4 |
    recv_mpst_d_from_f, RoleF, 5 |
    recv_mpst_d_from_g, RoleG, 6 |
    recv_mpst_d_from_h, RoleH, 7 |
    recv_mpst_d_from_i, RoleI, 8 |
    recv_mpst_d_from_j, RoleJ, 9 | =>
    NameD, MeshedChannels, 10
);

// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_a, RoleA, 1 |
    recv_mpst_e_from_b, RoleB, 2 |
    recv_mpst_e_from_c, RoleC, 3 |
    recv_mpst_e_from_d, RoleD, 4 |
    recv_mpst_e_from_f, RoleF, 5 |
    recv_mpst_e_from_g, RoleG, 6 |
    recv_mpst_e_from_h, RoleH, 7 |
    recv_mpst_e_from_i, RoleI, 8 |
    recv_mpst_e_from_j, RoleJ, 9 | =>
    NameE, MeshedChannels, 10
);

// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_from_a, RoleA, 1 |
    recv_mpst_f_from_b, RoleB, 2 |
    recv_mpst_f_from_c, RoleC, 3 |
    recv_mpst_f_from_d, RoleD, 4 |
    recv_mpst_f_from_e, RoleE, 5 |
    recv_mpst_f_from_g, RoleG, 6 |
    recv_mpst_f_from_h, RoleH, 7 |
    recv_mpst_f_from_i, RoleI, 8 |
    recv_mpst_f_from_j, RoleJ, 9 | =>
    NameF, MeshedChannels, 10
);

// G
create_recv_mpst_session_bundle!(
    recv_mpst_g_from_a, RoleA, 1 |
    recv_mpst_g_from_b, RoleB, 2 |
    recv_mpst_g_from_c, RoleC, 3 |
    recv_mpst_g_from_d, RoleD, 4 |
    recv_mpst_g_from_e, RoleE, 5 |
    recv_mpst_g_from_f, RoleF, 6 |
    recv_mpst_g_from_h, RoleH, 7 |
    recv_mpst_g_from_i, RoleI, 8 |
    recv_mpst_g_from_j, RoleJ, 9 | =>
    NameG, MeshedChannels, 10
);

// H
create_recv_mpst_session_bundle!(
    recv_mpst_h_from_a, RoleA, 1 |
    recv_mpst_h_from_b, RoleB, 2 |
    recv_mpst_h_from_c, RoleC, 3 |
    recv_mpst_h_from_d, RoleD, 4 |
    recv_mpst_h_from_e, RoleE, 5 |
    recv_mpst_h_from_f, RoleF, 6 |
    recv_mpst_h_from_g, RoleG, 7 |
    recv_mpst_h_from_i, RoleI, 8 |
    recv_mpst_h_from_j, RoleJ, 9 | =>
    NameH, MeshedChannels, 10
);

// I
create_recv_mpst_session_bundle!(
    recv_mpst_i_from_a, RoleA, 1 |
    recv_mpst_i_from_b, RoleB, 2 |
    recv_mpst_i_from_c, RoleC, 3 |
    recv_mpst_i_from_d, RoleD, 4 |
    recv_mpst_i_from_e, RoleE, 5 |
    recv_mpst_i_from_f, RoleF, 6 |
    recv_mpst_i_from_g, RoleG, 7 |
    recv_mpst_i_from_h, RoleH, 8 |
    recv_mpst_i_from_j, RoleJ, 9 | =>
    NameI, MeshedChannels, 10
);

// J
create_recv_mpst_session_bundle!(
    recv_mpst_j_from_a, RoleA, 1 |
    recv_mpst_j_from_b, RoleB, 2 |
    recv_mpst_j_from_c, RoleC, 3 |
    recv_mpst_j_from_d, RoleD, 4 |
    recv_mpst_j_from_e, RoleE, 5 |
    recv_mpst_j_from_f, RoleF, 6 |
    recv_mpst_j_from_g, RoleG, 7 |
    recv_mpst_j_from_h, RoleH, 8 |
    recv_mpst_j_from_i, RoleI, 9 | =>
    NameJ, MeshedChannels, 10
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

// A
enum Branching0fromJtoA {
    More(
        MeshedChannels<
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursAtoJ>>,
            R2J<R2B<R2C<R2D<R2E<R2F<R2G<R2H<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameA,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoJ = Recv<Branching0fromJtoA, End>;

// B
enum Branching0fromJtoB {
    More(
        MeshedChannels<
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursBtoJ>>,
            R2J<R2A<R2C<R2D<R2E<R2F<R2G<R2H<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameB,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoJ = Recv<Branching0fromJtoB, End>;

// C
enum Branching0fromJtoC {
    More(
        MeshedChannels<
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursCtoJ>>,
            R2J<R2A<R2B<R2D<R2E<R2F<R2G<R2H<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameC,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoJ = Recv<Branching0fromJtoC, End>;

// D
enum Branching0fromJtoD {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursDtoJ>>,
            R2J<R2A<R2B<R2C<R2E<R2F<R2G<R2H<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameD,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoJ = Recv<Branching0fromJtoD, End>;

// E
enum Branching0fromJtoE {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursEtoJ>>,
            R2J<R2A<R2B<R2C<R2D<R2F<R2G<R2H<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameE,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoJ = Recv<Branching0fromJtoE, End>;

// F
enum Branching0fromJtoF {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursFtoJ>>,
            R2J<R2A<R2B<R2C<R2D<R2E<R2G<R2H<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameF,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoJ = Recv<Branching0fromJtoF, End>;

// G
enum Branching0fromJtoG {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            Recv<(), Send<(), RecursGtoJ>>,
            R2J<R2A<R2B<R2C<R2D<R2E<R2F<R2H<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameG,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoJ = Recv<Branching0fromJtoG, End>;

// H
enum Branching0fromJtoH {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            Recv<(), Send<(), RecursHtoJ>>,
            R2J<R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameH,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, End, RoleEnd, NameH>),
}
type RecursHtoJ = Recv<Branching0fromJtoH, End>;

// I
enum Branching0fromJtoI {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            Recv<(), Send<(), RecursItoJ>>,
            R2J<R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2H<RoleJ<RoleEnd>>>>>>>>>>,
            NameI,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, End, RoleEnd, NameI>),
}
type RecursItoJ = Recv<Branching0fromJtoI, End>;

// J
type Choose0fromJtoA = Send<Branching0fromJtoA, End>;
type Choose0fromJtoB = Send<Branching0fromJtoB, End>;
type Choose0fromJtoC = Send<Branching0fromJtoC, End>;
type Choose0fromJtoD = Send<Branching0fromJtoD, End>;
type Choose0fromJtoE = Send<Branching0fromJtoE, End>;
type Choose0fromJtoF = Send<Branching0fromJtoF, End>;
type Choose0fromJtoG = Send<Branching0fromJtoG, End>;
type Choose0fromJtoH = Send<Branching0fromJtoH, End>;
type Choose0fromJtoI = Send<Branching0fromJtoI, End>;
type EndpointDoneJ = MeshedChannels<End, End, End, End, End, End, End, End, End, RoleEnd, NameJ>;
type EndpointMoreJ = MeshedChannels<
    Send<(), Recv<(), Choose0fromJtoA>>,
    Send<(), Recv<(), Choose0fromJtoB>>,
    Send<(), Recv<(), Choose0fromJtoC>>,
    Send<(), Recv<(), Choose0fromJtoD>>,
    Send<(), Recv<(), Choose0fromJtoE>>,
    Send<(), Recv<(), Choose0fromJtoF>>,
    Send<(), Recv<(), Choose0fromJtoG>>,
    Send<(), Recv<(), Choose0fromJtoH>>,
    Send<(), Recv<(), Choose0fromJtoI>>,
    R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2H<R2I<RoleBroadcast>>>>>>>>>,
    NameJ,
>;

// Creating the MP sessions
type EndpointA =
    MeshedChannels<End, End, End, End, End, End, End, End, RecursAtoJ, RoleJ<RoleEnd>, NameA>;
type EndpointB =
    MeshedChannels<End, End, End, End, End, End, End, End, RecursBtoJ, RoleJ<RoleEnd>, NameB>;
type EndpointC =
    MeshedChannels<End, End, End, End, End, End, End, End, RecursCtoJ, RoleJ<RoleEnd>, NameC>;
type EndpointD =
    MeshedChannels<End, End, End, End, End, End, End, End, RecursDtoJ, RoleJ<RoleEnd>, NameD>;
type EndpointE =
    MeshedChannels<End, End, End, End, End, End, End, End, RecursEtoJ, RoleJ<RoleEnd>, NameE>;
type EndpointF =
    MeshedChannels<End, End, End, End, End, End, End, End, RecursFtoJ, RoleJ<RoleEnd>, NameF>;
type EndpointG =
    MeshedChannels<End, End, End, End, End, End, End, End, RecursGtoJ, RoleJ<RoleEnd>, NameG>;
type EndpointH =
    MeshedChannels<End, End, End, End, End, End, End, End, RecursHtoJ, RoleJ<RoleEnd>, NameH>;
type EndpointI =
    MeshedChannels<End, End, End, End, End, End, End, End, RecursItoJ, RoleJ<RoleEnd>, NameI>;
type EndpointJ = MeshedChannels<
    Choose0fromJtoA,
    Choose0fromJtoB,
    Choose0fromJtoC,
    Choose0fromJtoD,
    Choose0fromJtoE,
    Choose0fromJtoF,
    Choose0fromJtoG,
    Choose0fromJtoH,
    Choose0fromJtoI,
    RoleBroadcast,
    NameJ,
>;

create_fn_choose_mpst_multi_to_all_bundle!(
    done_from_j_to_all, more_from_j_to_all, =>
    Done, More, =>
    EndpointDoneJ, EndpointMoreJ, =>
    Branching0fromJtoA,
    Branching0fromJtoB,
    Branching0fromJtoC,
    Branching0fromJtoD,
    Branching0fromJtoE,
    Branching0fromJtoF,
    Branching0fromJtoG,
    Branching0fromJtoH,
    Branching0fromJtoI, =>
    NameA,
    NameB,
    NameC,
    NameD,
    NameE,
    NameF,
    NameG,
    NameH,
    NameI, =>
    NameJ, MeshedChannels, 10
);

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_j, {
        Branching0fromJtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromJtoA::More(s) => {
            let (_, s) = recv_mpst_a_from_j(s)?;
            let s = send_mpst_a_to_j((), s);
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
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_j, {
        Branching0fromJtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromJtoB::More(s) => {
            let (_, s) = recv_mpst_b_from_j(s)?;
            let s = send_mpst_b_to_j((), s);
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
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_j, {
        Branching0fromJtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromJtoC::More(s) => {
            let (_, s) = recv_mpst_c_from_j(s)?;
            let s = send_mpst_c_to_j((), s);
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
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_from_j, {
        Branching0fromJtoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromJtoD::More(s) => {
            let (_, s) = recv_mpst_d_from_j(s)?;
            let s = send_mpst_d_to_j((), s);
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
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_e_from_j, {
        Branching0fromJtoE::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromJtoE::More(s) => {
            let (_, s) = recv_mpst_e_from_j(s)?;
            let s = send_mpst_e_to_j((), s);
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
            endpoint_e(s)
        },
    })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_f_from_j, {
        Branching0fromJtoF::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromJtoF::More(s) => {
            let (_, s) = recv_mpst_f_from_j(s)?;
            let s = send_mpst_f_to_j((), s);
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
            endpoint_f(s)
        },
    })
}

fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_g_from_j, {
        Branching0fromJtoG::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromJtoG::More(s) => {
            let (_, s) = recv_mpst_g_from_j(s)?;
            let s = send_mpst_g_to_j((), s);
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
            endpoint_g(s)
        },
    })
}

fn endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_h_from_j, {
        Branching0fromJtoH::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromJtoH::More(s) => {
            let (_, s) = recv_mpst_h_from_j(s)?;
            let s = send_mpst_h_to_j((), s);
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
            endpoint_h(s)
        },
    })
}

fn endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_i_from_j, {
        Branching0fromJtoI::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromJtoI::More(s) => {
            let (_, s) = recv_mpst_i_from_j(s)?;
            let s = send_mpst_i_to_j((), s);
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
            endpoint_i(s)
        },
    })
}

fn endpoint_j(s: EndpointJ) -> Result<(), Box<dyn Error>> {
    recurs_j(s, LOOPS)
}

fn recurs_j(s: EndpointJ, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_j_to_all(s);

            close_mpst_multi(s)
        }
        i => {
            let s = more_from_j_to_all(s);

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

            recurs_j(s, i - 1)
        }
    }
}

fn aux() {
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
}

/////////////////////////

static LOOPS: i64 = 0;

pub fn mesh_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("mesh ten empty {LOOPS}"), |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(10000);
    targets = mesh_protocol_mpst,
}

criterion_main! {
    bench
}
