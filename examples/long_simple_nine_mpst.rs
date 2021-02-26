use mpstthree::binary::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::{
    bundle_fork_multi, choose_mpst_multi_to_all, close_mpst, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, create_sessionmpst,
    offer_mpst,
};

use std::error::Error;

// Create new SessionMpst for four participants
create_sessionmpst!(SessionMpstNine, 9);

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
);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b,
    RoleB,
    next_b,
    1 |
    send_mpst_a_to_c,
    RoleC,
    next_c,
    2 |
    send_mpst_a_to_d,
    RoleD,
    next_d,
    3 |
    send_mpst_a_to_e,
    RoleE,
    next_e,
    4 |
    send_mpst_a_to_f,
    RoleF,
    next_f,
    5 |
    send_mpst_a_to_g,
    RoleG,
    next_g,
    6 |
    send_mpst_a_to_h,
    RoleH,
    next_h,
    7 |
    send_mpst_a_to_i,
    RoleI,
    next_i,
    8 | =>
    RoleA,
    SessionMpstNine,
    9
);
// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a,
    RoleA,
    next_a,
    1 |
    send_mpst_b_to_c,
    RoleC,
    next_c,
    2 |
    send_mpst_b_to_d,
    RoleD,
    next_d,
    3 |
    send_mpst_b_to_e,
    RoleE,
    next_e,
    4 |
    send_mpst_b_to_f,
    RoleF,
    next_f,
    5 |
    send_mpst_b_to_g,
    RoleG,
    next_g,
    6 |
    send_mpst_b_to_h,
    RoleH,
    next_h,
    7 |
    send_mpst_b_to_i,
    RoleI,
    next_i,
    8 | =>
    RoleB,
    SessionMpstNine,
    9
);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a,
    RoleA,
    next_a,
    1 |
    send_mpst_c_to_b,
    RoleB,
    next_b,
    2 |
    send_mpst_c_to_d,
    RoleD,
    next_d,
    3 |
    send_mpst_c_to_e,
    RoleE,
    next_e,
    4 |
    send_mpst_c_to_f,
    RoleF,
    next_f,
    5 |
    send_mpst_c_to_g,
    RoleG,
    next_g,
    6 |
    send_mpst_c_to_h,
    RoleH,
    next_h,
    7 |
    send_mpst_c_to_i,
    RoleI,
    next_i,
    8 | =>
    RoleC,
    SessionMpstNine,
    9
);
// D
create_send_mpst_session_bundle!(
    send_mpst_d_to_a,
    RoleA,
    next_a,
    1 |
    send_mpst_d_to_b,
    RoleB,
    next_b,
    2 |
    send_mpst_d_to_c,
    RoleC,
    next_c,
    3 |
    send_mpst_d_to_e,
    RoleE,
    next_e,
    4 |
    send_mpst_d_to_f,
    RoleF,
    next_f,
    5 |
    send_mpst_d_to_g,
    RoleG,
    next_g,
    6 |
    send_mpst_d_to_h,
    RoleH,
    next_h,
    7 |
    send_mpst_d_to_i,
    RoleI,
    next_i,
    8 | =>
    RoleD,
    SessionMpstNine,
    9
);
// E
create_send_mpst_session_bundle!(
    send_mpst_e_to_a,
    RoleA,
    next_a,
    1 |
    send_mpst_e_to_b,
    RoleB,
    next_b,
    2 |
    send_mpst_e_to_c,
    RoleC,
    next_c,
    3 |
    send_mpst_e_to_d,
    RoleD,
    next_d,
    4 |
    send_mpst_e_to_f,
    RoleF,
    next_f,
    5 |
    send_mpst_e_to_g,
    RoleG,
    next_g,
    6 |
    send_mpst_e_to_h,
    RoleH,
    next_h,
    7 |
    send_mpst_e_to_i,
    RoleI,
    next_i,
    8 | =>
    RoleE,
    SessionMpstNine,
    9
);
// F
create_send_mpst_session_bundle!(
    send_mpst_f_to_a,
    RoleA,
    next_a,
    1 |
    send_mpst_f_to_b,
    RoleB,
    next_b,
    2 |
    send_mpst_f_to_c,
    RoleC,
    next_c,
    3 |
    send_mpst_f_to_d,
    RoleD,
    next_d,
    4 |
    send_mpst_f_to_e,
    RoleE,
    next_e,
    5 |
    send_mpst_f_to_g,
    RoleG,
    next_g,
    6 |
    send_mpst_f_to_h,
    RoleH,
    next_h,
    7 |
    send_mpst_f_to_i,
    RoleI,
    next_i,
    8 | =>
    RoleF,
    SessionMpstNine,
    9
);
// G
create_send_mpst_session_bundle!(
    send_mpst_g_to_a,
    RoleA,
    next_a,
    1 |
    send_mpst_g_to_b,
    RoleB,
    next_b,
    2 |
    send_mpst_g_to_c,
    RoleC,
    next_c,
    3 |
    send_mpst_g_to_d,
    RoleD,
    next_d,
    4 |
    send_mpst_g_to_e,
    RoleE,
    next_e,
    5 |
    send_mpst_g_to_f,
    RoleF,
    next_f,
    6 |
    send_mpst_g_to_h,
    RoleH,
    next_h,
    7 |
    send_mpst_g_to_i,
    RoleI,
    next_i,
    8 | =>
    RoleG,
    SessionMpstNine,
    9
);
// H
create_send_mpst_session_bundle!(
    send_mpst_h_to_a,
    RoleA,
    next_a,
    1 |
    send_mpst_h_to_b,
    RoleB,
    next_b,
    2 |
    send_mpst_h_to_c,
    RoleC,
    next_c,
    3 |
    send_mpst_h_to_d,
    RoleD,
    next_d,
    4 |
    send_mpst_h_to_e,
    RoleE,
    next_e,
    5 |
    send_mpst_h_to_f,
    RoleF,
    next_f,
    6 |
    send_mpst_h_to_g,
    RoleG,
    next_g,
    7 |
    send_mpst_h_to_i,
    RoleI,
    next_i,
    8 | =>
    RoleH,
    SessionMpstNine,
    9
);
// I
create_send_mpst_session_bundle!(
    send_mpst_i_to_a,
    RoleA,
    next_a,
    1 |
    send_mpst_i_to_b,
    RoleB,
    next_b,
    2 |
    send_mpst_i_to_c,
    RoleC,
    next_c,
    3 |
    send_mpst_i_to_d,
    RoleD,
    next_d,
    4 |
    send_mpst_i_to_e,
    RoleE,
    next_e,
    5 |
    send_mpst_i_to_f,
    RoleF,
    next_f,
    6 |
    send_mpst_i_to_g,
    RoleG,
    next_g,
    7 |
    send_mpst_i_to_h,
    RoleH,
    next_h,
    8 | =>
    RoleI,
    SessionMpstNine,
    9
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_to_b,
    RoleB,
    next_b,
    1 |
    recv_mpst_a_to_c,
    RoleC,
    next_c,
    2 |
    recv_mpst_a_to_d,
    RoleD,
    next_d,
    3 |
    recv_mpst_a_to_e,
    RoleE,
    next_e,
    4 |
    recv_mpst_a_to_f,
    RoleF,
    next_f,
    5 |
    recv_mpst_a_to_g,
    RoleG,
    next_g,
    6 |
    recv_mpst_a_to_h,
    RoleH,
    next_h,
    7 |
    recv_mpst_a_to_i,
    RoleI,
    next_i,
    8 | =>
    RoleA,
    SessionMpstNine,
    9
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_to_a,
    RoleA,
    next_a,
    1 |
    recv_mpst_b_to_c,
    RoleC,
    next_c,
    2 |
    recv_mpst_b_to_d,
    RoleD,
    next_d,
    3 |
    recv_mpst_b_to_e,
    RoleE,
    next_e,
    4 |
    recv_mpst_b_to_f,
    RoleF,
    next_f,
    5 |
    recv_mpst_b_to_g,
    RoleG,
    next_g,
    6 |
    recv_mpst_b_to_h,
    RoleH,
    next_h,
    7 |
    recv_mpst_b_to_i,
    RoleI,
    next_i,
    8 | =>
    RoleB,
    SessionMpstNine,
    9
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_to_a,
    RoleA,
    next_a,
    1 |
    recv_mpst_c_to_b,
    RoleB,
    next_b,
    2 |
    recv_mpst_c_to_d,
    RoleD,
    next_d,
    3 |
    recv_mpst_c_to_e,
    RoleE,
    next_e,
    4 |
    recv_mpst_c_to_f,
    RoleF,
    next_f,
    5 |
    recv_mpst_c_to_g,
    RoleG,
    next_g,
    6 |
    recv_mpst_c_to_h,
    RoleH,
    next_h,
    7 |
    recv_mpst_c_to_i,
    RoleI,
    next_i,
    8 | =>
    RoleC,
    SessionMpstNine,
    9
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_to_a,
    RoleA,
    next_a,
    1 |
    recv_mpst_d_to_b,
    RoleB,
    next_b,
    2 |
    recv_mpst_d_to_c,
    RoleC,
    next_c,
    3 |
    recv_mpst_d_to_e,
    RoleE,
    next_e,
    4 |
    recv_mpst_d_to_f,
    RoleF,
    next_f,
    5 |
    recv_mpst_d_to_g,
    RoleG,
    next_g,
    6 |
    recv_mpst_d_to_h,
    RoleH,
    next_h,
    7 |
    recv_mpst_d_to_i,
    RoleI,
    next_i,
    8 | =>
    RoleD,
    SessionMpstNine,
    9
);
// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_to_a,
    RoleA,
    next_a,
    1 |
    recv_mpst_e_to_b,
    RoleB,
    next_b,
    2 |
    recv_mpst_e_to_c,
    RoleC,
    next_c,
    3 |
    recv_mpst_e_to_d,
    RoleD,
    next_d,
    4 |
    recv_mpst_e_to_f,
    RoleF,
    next_f,
    5 |
    recv_mpst_e_to_g,
    RoleG,
    next_g,
    6 |
    recv_mpst_e_to_h,
    RoleH,
    next_h,
    7 |
    recv_mpst_e_to_i,
    RoleI,
    next_i,
    8 | =>
    RoleE,
    SessionMpstNine,
    9
);
// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_to_a,
    RoleA,
    next_a,
    1 |
    recv_mpst_f_to_b,
    RoleB,
    next_b,
    2 |
    recv_mpst_f_to_c,
    RoleC,
    next_c,
    3 |
    recv_mpst_f_to_d,
    RoleD,
    next_d,
    4 |
    recv_mpst_f_to_e,
    RoleE,
    next_e,
    5 |
    recv_mpst_f_to_g,
    RoleG,
    next_g,
    6 |
    recv_mpst_f_to_h,
    RoleH,
    next_h,
    7 |
    recv_mpst_f_to_i,
    RoleI,
    next_i,
    8 | =>
    RoleF,
    SessionMpstNine,
    9
);
// G
create_recv_mpst_session_bundle!(
    recv_mpst_g_to_a,
    RoleA,
    next_a,
    1 |
    recv_mpst_g_to_b,
    RoleB,
    next_b,
    2 |
    recv_mpst_g_to_c,
    RoleC,
    next_c,
    3 |
    recv_mpst_g_to_d,
    RoleD,
    next_d,
    4 |
    recv_mpst_g_to_e,
    RoleE,
    next_e,
    5 |
    recv_mpst_g_to_f,
    RoleF,
    next_f,
    6 |
    recv_mpst_g_to_h,
    RoleH,
    next_h,
    7 |
    recv_mpst_g_to_i,
    RoleI,
    next_i,
    8 | =>
    RoleG,
    SessionMpstNine,
    9
);
// H
create_recv_mpst_session_bundle!(
    recv_mpst_h_to_a,
    RoleA,
    next_a,
    1 |
    recv_mpst_h_to_b,
    RoleB,
    next_b,
    2 |
    recv_mpst_h_to_c,
    RoleC,
    next_c,
    3 |
    recv_mpst_h_to_d,
    RoleD,
    next_d,
    4 |
    recv_mpst_h_to_e,
    RoleE,
    next_e,
    5 |
    recv_mpst_h_to_f,
    RoleF,
    next_f,
    6 |
    recv_mpst_h_to_g,
    RoleG,
    next_g,
    7 |
    recv_mpst_h_to_i,
    RoleI,
    next_i,
    8 | =>
    RoleH,
    SessionMpstNine,
    9
);
// I
create_recv_mpst_session_bundle!(
    recv_mpst_i_to_a,
    RoleA,
    next_a,
    1 |
    recv_mpst_i_to_b,
    RoleB,
    next_b,
    2 |
    recv_mpst_i_to_c,
    RoleC,
    next_c,
    3 |
    recv_mpst_i_to_d,
    RoleD,
    next_d,
    4 |
    recv_mpst_i_to_e,
    RoleE,
    next_e,
    5 |
    recv_mpst_i_to_f,
    RoleF,
    next_f,
    6 |
    recv_mpst_i_to_g,
    RoleG,
    next_g,
    7 |
    recv_mpst_i_to_h,
    RoleH,
    next_h,
    8 | =>
    RoleI,
    SessionMpstNine,
    9
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
// A
enum Branching0fromItoA
{
    More(
        SessionMpstNine<
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursAtoI>>,
            R2I<R2B<R2C<R2D<R2E<R2F<R2G<R2H<RoleI<RoleEnd>>>>>>>>>,
            NameA,
        >,
    ),
    Done(SessionMpstNine<End, End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoI = Recv<Branching0fromItoA, End>;
// B
enum Branching0fromItoB
{
    More(
        SessionMpstNine<
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursBtoI>>,
            R2I<R2A<R2C<R2D<R2E<R2F<R2G<R2H<RoleI<RoleEnd>>>>>>>>>,
            NameB,
        >,
    ),
    Done(SessionMpstNine<End, End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoI = Recv<Branching0fromItoB, End>;
// C
enum Branching0fromItoC
{
    More(
        SessionMpstNine<
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursCtoI>>,
            R2I<R2A<R2B<R2D<R2E<R2F<R2G<R2H<RoleI<RoleEnd>>>>>>>>>,
            NameC,
        >,
    ),
    Done(SessionMpstNine<End, End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoI = Recv<Branching0fromItoC, End>;
// D
enum Branching0fromItoD
{
    More(
        SessionMpstNine<
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursDtoI>>,
            R2I<R2A<R2B<R2C<R2E<R2F<R2G<R2H<RoleI<RoleEnd>>>>>>>>>,
            NameD,
        >,
    ),
    Done(SessionMpstNine<End, End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoI = Recv<Branching0fromItoD, End>;
// E
enum Branching0fromItoE
{
    More(
        SessionMpstNine<
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursEtoI>>,
            R2I<R2A<R2B<R2C<R2D<R2F<R2G<R2H<RoleI<RoleEnd>>>>>>>>>,
            NameE,
        >,
    ),
    Done(SessionMpstNine<End, End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoI = Recv<Branching0fromItoE, End>;
// F
enum Branching0fromItoF
{
    More(
        SessionMpstNine<
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            Recv<(), Send<(), RecursFtoI>>,
            R2I<R2A<R2B<R2C<R2D<R2E<R2G<R2H<RoleI<RoleEnd>>>>>>>>>,
            NameF,
        >,
    ),
    Done(SessionMpstNine<End, End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoI = Recv<Branching0fromItoF, End>;
// G
enum Branching0fromItoG
{
    More(
        SessionMpstNine<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            Recv<(), Send<(), RecursGtoI>>,
            R2I<R2A<R2B<R2C<R2D<R2E<R2F<R2H<RoleI<RoleEnd>>>>>>>>>,
            NameG,
        >,
    ),
    Done(SessionMpstNine<End, End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoI = Recv<Branching0fromItoG, End>;
// H
enum Branching0fromItoH
{
    More(
        SessionMpstNine<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            Recv<(), Send<(), RecursHtoI>>,
            R2I<R2A<R2B<R2C<R2D<R2E<R2F<R2G<RoleI<RoleEnd>>>>>>>>>,
            NameH,
        >,
    ),
    Done(SessionMpstNine<End, End, End, End, End, End, End, End, RoleEnd, NameH>),
}
type RecursHtoI = Recv<Branching0fromItoH, End>;
// I
type Choose0fromItoA = Send<Branching0fromItoA, End>;
type Choose0fromItoB = Send<Branching0fromItoB, End>;
type Choose0fromItoC = Send<Branching0fromItoC, End>;
type Choose0fromItoD = Send<Branching0fromItoD, End>;
type Choose0fromItoE = Send<Branching0fromItoE, End>;
type Choose0fromItoF = Send<Branching0fromItoF, End>;
type Choose0fromItoG = Send<Branching0fromItoG, End>;
type Choose0fromItoH = Send<Branching0fromItoH, End>;

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
    Choose0fromItoA,
    Choose0fromItoB,
    Choose0fromItoC,
    Choose0fromItoD,
    Choose0fromItoE,
    Choose0fromItoF,
    Choose0fromItoG,
    Choose0fromItoH,
    RoleA<RoleB<RoleC<RoleD<RoleE<RoleF<RoleG<RoleH<RoleEnd>>>>>>>>,
    NameI,
>;

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_a_to_i, {
          Branching0fromItoA::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromItoA::More(s) => {
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

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_b_to_i, {
          Branching0fromItoB::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromItoB::More(s) => {
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

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_c_to_i, {
          Branching0fromItoC::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromItoC::More(s) => {
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

fn simple_five_endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_d_to_i, {
          Branching0fromItoD::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromItoD::More(s) => {
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

fn simple_five_endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_e_to_i, {
          Branching0fromItoE::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromItoE::More(s) => {
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

fn simple_five_endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_f_to_i, {
          Branching0fromItoF::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromItoF::More(s) => {
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

fn simple_five_endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_g_to_i, {
          Branching0fromItoG::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromItoG::More(s) => {
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

fn simple_five_endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_h_to_i, {
          Branching0fromItoH::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromItoH::More(s) => {
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

fn simple_five_endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>>
{
    recurs_i(s, SIZE)
}

fn recurs_i(s: EndpointI, index: i64) -> Result<(), Box<dyn Error>>
{
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
                Branching0fromItoA::Done,
                Branching0fromItoB::Done,
                Branching0fromItoC::Done,
                Branching0fromItoD::Done,
                Branching0fromItoE::Done,
                Branching0fromItoF::Done,
                Branching0fromItoG::Done,
                Branching0fromItoH::Done, =>
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
                9,
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
                Branching0fromItoA::More,
                Branching0fromItoB::More,
                Branching0fromItoC::More,
                Branching0fromItoD::More,
                Branching0fromItoE::More,
                Branching0fromItoF::More,
                Branching0fromItoG::More,
                Branching0fromItoH::More, =>
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
                9,
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

fn all_mpst() -> Result<(), Box<dyn Error>>
{
    let (thread_a, thread_b, thread_c, thread_d, thread_e, thread_f, thread_g, thread_h, thread_i) =
        fork_mpst(
            simple_five_endpoint_a,
            simple_five_endpoint_b,
            simple_five_endpoint_c,
            simple_five_endpoint_d,
            simple_five_endpoint_e,
            simple_five_endpoint_f,
            simple_five_endpoint_g,
            simple_five_endpoint_h,
            simple_five_endpoint_i,
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

static SIZE: i64 = 15;

fn main()
{
    all_mpst().unwrap();
}
