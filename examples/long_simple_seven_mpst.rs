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
create_sessionmpst!(SessionMpstSeven, 7);

// Create new roles
// normal
create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
create_normal_role!(RoleD, next_d, RoleDDual, next_d_dual);
create_normal_role!(RoleE, next_e, RoleEDual, next_e_dual);
create_normal_role!(RoleF, next_f, RoleFDual, next_f_dual);
create_normal_role!(RoleG, next_g, RoleGDual, next_g_dual);

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
    6, | =>
    RoleA,
    SessionMpstSeven,
    7
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
    6, | =>
    RoleB,
    SessionMpstSeven,
    7
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
    6, | =>
    RoleC,
    SessionMpstSeven,
    7
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
    6, | =>
    RoleD,
    SessionMpstSeven,
    7
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
    6, | =>
    RoleE,
    SessionMpstSeven,
    7
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
    6, | =>
    RoleF,
    SessionMpstSeven,
    7
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
    6, | =>
    RoleG,
    SessionMpstSeven,
    7
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
    6, | =>
    RoleA,
    SessionMpstSeven,
    7
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
    6, | =>
    RoleB,
    SessionMpstSeven,
    7
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
    6, | =>
    RoleC,
    SessionMpstSeven,
    7
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
    6, | =>
    RoleD,
    SessionMpstSeven,
    7
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
    6, | =>
    RoleE,
    SessionMpstSeven,
    7
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
    6, | =>
    RoleF,
    SessionMpstSeven,
    7
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
    6, | =>
    RoleG,
    SessionMpstSeven,
    7
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstSeven, 7);

// Create fork function
bundle_fork_multi!(fork_mpst, fork_simple, SessionMpstSeven, 7);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;
type NameE = RoleE<RoleEnd>;
type NameF = RoleF<RoleEnd>;
type NameG = RoleG<RoleEnd>;

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
// A
enum BranchingGforA {
    More(
        SessionMpstSeven<
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursAtoG>>,
            R2G<R2B<R2C<R2D<R2E<R2F<RoleG<RoleEnd>>>>>>>,
            NameA,
        >,
    ),
    Done(SessionMpstSeven<End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoG = Recv<BranchingGforA, End>;
// B
enum BranchingGforB {
    More(
        SessionMpstSeven<
            SR,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursBtoG>>,
            R2G<R2A<R2C<R2D<R2E<R2F<RoleG<RoleEnd>>>>>>>,
            NameB,
        >,
    ),
    Done(SessionMpstSeven<End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoG = Recv<BranchingGforB, End>;
// C
enum BranchingGforC {
    More(
        SessionMpstSeven<
            SR,
            SR,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursCtoG>>,
            R2G<R2A<R2B<R2D<R2E<R2F<RoleG<RoleEnd>>>>>>>,
            NameC,
        >,
    ),
    Done(SessionMpstSeven<End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoG = Recv<BranchingGforC, End>;
// D
enum BranchingGforD {
    More(
        SessionMpstSeven<
            SR,
            SR,
            SR,
            RS,
            RS,
            Recv<(), Send<(), RecursDtoG>>,
            R2G<R2A<R2B<R2C<R2E<R2F<RoleG<RoleEnd>>>>>>>,
            NameD,
        >,
    ),
    Done(SessionMpstSeven<End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoG = Recv<BranchingGforD, End>;
// E
enum BranchingGforE {
    More(
        SessionMpstSeven<
            SR,
            SR,
            SR,
            SR,
            RS,
            Recv<(), Send<(), RecursEtoG>>,
            R2G<R2A<R2B<R2C<R2D<R2F<RoleG<RoleEnd>>>>>>>,
            NameE,
        >,
    ),
    Done(SessionMpstSeven<End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoG = Recv<BranchingGforE, End>;
// F
enum BranchingGforF {
    More(
        SessionMpstSeven<
            SR,
            SR,
            SR,
            SR,
            SR,
            Recv<(), Send<(), RecursFtoG>>,
            R2G<R2A<R2B<R2C<R2D<R2E<RoleG<RoleEnd>>>>>>>,
            NameF,
        >,
    ),
    Done(SessionMpstSeven<End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoG = Recv<BranchingGforF, End>;
// F
type ChooseGforAtoG = Send<BranchingGforA, End>;
type ChooseGforBtoG = Send<BranchingGforB, End>;
type ChooseGforCtoG = Send<BranchingGforC, End>;
type ChooseGforDtoG = Send<BranchingGforD, End>;
type ChooseGforEtoG = Send<BranchingGforE, End>;
type ChooseGforFtoG = Send<BranchingGforF, End>;

// Creating the MP sessions
type EndpointA = SessionMpstSeven<End, End, End, End, End, RecursAtoG, RoleG<RoleEnd>, NameA>;
type EndpointB = SessionMpstSeven<End, End, End, End, End, RecursBtoG, RoleG<RoleEnd>, NameB>;
type EndpointC = SessionMpstSeven<End, End, End, End, End, RecursCtoG, RoleG<RoleEnd>, NameC>;
type EndpointD = SessionMpstSeven<End, End, End, End, End, RecursDtoG, RoleG<RoleEnd>, NameD>;
type EndpointE = SessionMpstSeven<End, End, End, End, End, RecursEtoG, RoleG<RoleEnd>, NameE>;
type EndpointF = SessionMpstSeven<End, End, End, End, End, RecursFtoG, RoleG<RoleEnd>, NameF>;
type EndpointG = SessionMpstSeven<
    ChooseGforAtoG,
    ChooseGforBtoG,
    ChooseGforCtoG,
    ChooseGforDtoG,
    ChooseGforEtoG,
    ChooseGforFtoG,
    RoleA<RoleB<RoleC<RoleD<RoleE<RoleF<RoleEnd>>>>>>,
    NameG,
>;

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_to_g, {
          BranchingGforA::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingGforA::More(s) => {
            let (_, s) = recv_mpst_a_to_g(s)?;
            let s = send_mpst_a_to_g((), s);
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
            simple_five_endpoint_a(s)
        },
    })
}

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_to_g, {
          BranchingGforB::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingGforB::More(s) => {
            let (_, s) = recv_mpst_b_to_g(s)?;
            let s = send_mpst_b_to_g((), s);
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
            simple_five_endpoint_b(s)
        },
    })
}

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_to_g, {
          BranchingGforC::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingGforC::More(s) => {
            let (_, s) = recv_mpst_c_to_g(s)?;
            let s = send_mpst_c_to_g((), s);
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
            simple_five_endpoint_c(s)
        },
    })
}

fn simple_five_endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_to_g, {
          BranchingGforD::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingGforD::More(s) => {
            let (_, s) = recv_mpst_d_to_g(s)?;
            let s = send_mpst_d_to_g((), s);
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
            simple_five_endpoint_d(s)
        },
    })
}

fn simple_five_endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_e_to_g, {
          BranchingGforE::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingGforE::More(s) => {
            let (_, s) = recv_mpst_e_to_g(s)?;
            let s = send_mpst_e_to_g((), s);
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
            simple_five_endpoint_e(s)
        },
    })
}

fn simple_five_endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_f_to_g, {
          BranchingGforF::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingGforF::More(s) => {
            let (_, s) = recv_mpst_f_to_g(s)?;
            let s = send_mpst_f_to_g((), s);
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
            simple_five_endpoint_f(s)
        },
    })
}

fn simple_five_endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    recurs_g(s, SIZE)
}

fn recurs_g(s: EndpointG, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_g_to_a,
                send_mpst_g_to_b,
                send_mpst_g_to_c,
                send_mpst_g_to_d,
                send_mpst_g_to_e,
                send_mpst_g_to_f, =>
                BranchingGforA::Done,
                BranchingGforB::Done,
                BranchingGforC::Done,
                BranchingGforD::Done,
                BranchingGforE::Done,
                BranchingGforF::Done, =>
                RoleA,
                RoleB,
                RoleC,
                RoleD,
                RoleE,
                RoleF, =>
                RoleG,
                SessionMpstSeven,
                7,
                7
            );

            close_mpst_multi(s)
        }
        i => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_g_to_a,
                send_mpst_g_to_b,
                send_mpst_g_to_c,
                send_mpst_g_to_d,
                send_mpst_g_to_e,
                send_mpst_g_to_f,=>
                BranchingGforA::More,
                BranchingGforB::More,
                BranchingGforC::More,
                BranchingGforD::More,
                BranchingGforE::More,
                BranchingGforF::More, =>
                RoleA,
                RoleB,
                RoleC,
                RoleD,
                RoleE,
                RoleF, =>
                RoleG,
                SessionMpstSeven,
                7,
                7
            );

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

            recurs_g(s, i - 1)
        }
    }
}

fn all_mpst() -> Result<(), Box<dyn Error>> {
    let (thread_a, thread_b, thread_c, thread_d, thread_e, thread_f, thread_g) = fork_mpst(
        simple_five_endpoint_a,
        simple_five_endpoint_b,
        simple_five_endpoint_c,
        simple_five_endpoint_d,
        simple_five_endpoint_e,
        simple_five_endpoint_f,
        simple_five_endpoint_g,
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
    thread_f.join().unwrap();
    thread_g.join().unwrap();

    Ok(())
}

/////////////////////////

static SIZE: i64 = 15;

fn main() {
    all_mpst().unwrap();
}
