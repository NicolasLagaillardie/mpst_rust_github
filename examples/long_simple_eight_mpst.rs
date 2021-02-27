use mpstthree::binary::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    choose_mpst_multi_to_all, close_mpst, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, create_sessionmpst,
    fork_mpst_multi, offer_mpst,
};

use std::error::Error;

// Create new SessionMpst for four participants
create_sessionmpst!(SessionMpstEight, 8);

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
);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, next_b, 1 |
    send_mpst_a_to_c, RoleC, next_c, 2 |
    send_mpst_a_to_d, RoleD, next_d, 3 |
    send_mpst_a_to_e, RoleE, next_e, 4 |
    send_mpst_a_to_f, RoleF, next_f, 5 |
    send_mpst_a_to_g, RoleG, next_g, 6 |
    send_mpst_a_to_h, RoleH, next_h, 7 | =>
    RoleA, SessionMpstEight, 8
);
// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a, RoleA, next_a, 1 |
    send_mpst_b_to_c, RoleC, next_c, 2 |
    send_mpst_b_to_d, RoleD, next_d, 3 |
    send_mpst_b_to_e, RoleE, next_e, 4 |
    send_mpst_b_to_f, RoleF, next_f, 5 |
    send_mpst_b_to_g, RoleG, next_g, 6 |
    send_mpst_b_to_h, RoleH, next_h, 7 | =>
    RoleB, SessionMpstEight, 8
);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a, RoleA, next_a, 1 |
    send_mpst_c_to_b, RoleB, next_b, 2 |
    send_mpst_c_to_d, RoleD, next_d, 3 |
    send_mpst_c_to_e, RoleE, next_e, 4 |
    send_mpst_c_to_f, RoleF, next_f, 5 |
    send_mpst_c_to_g, RoleG, next_g, 6 |
    send_mpst_c_to_h, RoleH, next_h, 7 | =>
    RoleC, SessionMpstEight, 8
);
// D
create_send_mpst_session_bundle!(
    send_mpst_d_to_a, RoleA, next_a, 1 |
    send_mpst_d_to_b, RoleB, next_b, 2 |
    send_mpst_d_to_c, RoleC, next_c, 3 |
    send_mpst_d_to_e, RoleE, next_e, 4 |
    send_mpst_d_to_f, RoleF, next_f, 5 |
    send_mpst_d_to_g, RoleG, next_g, 6 |
    send_mpst_d_to_h, RoleH, next_h, 7 | =>
    RoleD, SessionMpstEight, 8
);
// E
create_send_mpst_session_bundle!(
    send_mpst_e_to_a, RoleA, next_a, 1 |
    send_mpst_e_to_b, RoleB, next_b, 2 |
    send_mpst_e_to_c, RoleC, next_c, 3 |
    send_mpst_e_to_d, RoleD, next_d, 4 |
    send_mpst_e_to_f, RoleF, next_f, 5 |
    send_mpst_e_to_g, RoleG, next_g, 6 |
    send_mpst_e_to_h, RoleH, next_h, 7 | =>
    RoleE, SessionMpstEight, 8
);
// F
create_send_mpst_session_bundle!(
    send_mpst_f_to_a, RoleA, next_a, 1 |
    send_mpst_f_to_b, RoleB, next_b, 2 |
    send_mpst_f_to_c, RoleC, next_c, 3 |
    send_mpst_f_to_d, RoleD, next_d, 4 |
    send_mpst_f_to_e, RoleE, next_e, 5 |
    send_mpst_f_to_g, RoleG, next_g, 6 |
    send_mpst_f_to_h, RoleH, next_h, 7 | =>
    RoleF, SessionMpstEight, 8
);
// G
create_send_mpst_session_bundle!(
    send_mpst_g_to_a, RoleA, next_a, 1 |
    send_mpst_g_to_b, RoleB, next_b, 2 |
    send_mpst_g_to_c, RoleC, next_c, 3 |
    send_mpst_g_to_d, RoleD, next_d, 4 |
    send_mpst_g_to_e, RoleE, next_e, 5 |
    send_mpst_g_to_f, RoleF, next_f, 6 |
    send_mpst_g_to_h, RoleH, next_h, 7 | =>
    RoleG, SessionMpstEight, 8
);
// H
create_send_mpst_session_bundle!(
    send_mpst_h_to_a, RoleA, next_a, 1 |
    send_mpst_h_to_b, RoleB, next_b, 2 |
    send_mpst_h_to_c, RoleC, next_c, 3 |
    send_mpst_h_to_d, RoleD, next_d, 4 |
    send_mpst_h_to_e, RoleE, next_e, 5 |
    send_mpst_h_to_f, RoleF, next_f, 6 |
    send_mpst_h_to_g, RoleG, next_g, 7 | =>
    RoleH, SessionMpstEight, 8
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_to_b, RoleB, next_b, 1 |
    recv_mpst_a_to_c, RoleC, next_c, 2 |
    recv_mpst_a_to_d, RoleD, next_d, 3 |
    recv_mpst_a_to_e, RoleE, next_e, 4 |
    recv_mpst_a_to_f, RoleF, next_f, 5 |
    recv_mpst_a_to_g, RoleG, next_g, 6 |
    recv_mpst_a_to_h, RoleH, next_h, 7 | =>
    RoleA, SessionMpstEight, 8
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_to_a, RoleA, next_a, 1 |
    recv_mpst_b_to_c, RoleC, next_c, 2 |
    recv_mpst_b_to_d, RoleD, next_d, 3 |
    recv_mpst_b_to_e, RoleE, next_e, 4 |
    recv_mpst_b_to_f, RoleF, next_f, 5 |
    recv_mpst_b_to_g, RoleG, next_g, 6 |
    recv_mpst_b_to_h, RoleH, next_h, 7 | =>
    RoleB, SessionMpstEight, 8
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_to_a, RoleA, next_a, 1 |
    recv_mpst_c_to_b, RoleB, next_b, 2 |
    recv_mpst_c_to_d, RoleD, next_d, 3 |
    recv_mpst_c_to_e, RoleE, next_e, 4 |
    recv_mpst_c_to_f, RoleF, next_f, 5 |
    recv_mpst_c_to_g, RoleG, next_g, 6 |
    recv_mpst_c_to_h, RoleH, next_h, 7 | =>
    RoleC, SessionMpstEight, 8
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_to_a, RoleA, next_a, 1 |
    recv_mpst_d_to_b, RoleB, next_b, 2 |
    recv_mpst_d_to_c, RoleC, next_c, 3 |
    recv_mpst_d_to_e, RoleE, next_e, 4 |
    recv_mpst_d_to_f, RoleF, next_f, 5 |
    recv_mpst_d_to_g, RoleG, next_g, 6 |
    recv_mpst_d_to_h, RoleH, next_h, 7 | =>
    RoleD, SessionMpstEight, 8
);
// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_to_a, RoleA, next_a, 1 |
    recv_mpst_e_to_b, RoleB, next_b, 2 |
    recv_mpst_e_to_c, RoleC, next_c, 3 |
    recv_mpst_e_to_d, RoleD, next_d, 4 |
    recv_mpst_e_to_f, RoleF, next_f, 5 |
    recv_mpst_e_to_g, RoleG, next_g, 6 |
    recv_mpst_e_to_h, RoleH, next_h, 7 | =>
    RoleE, SessionMpstEight, 8
);
// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_to_a, RoleA, next_a, 1 |
    recv_mpst_f_to_b, RoleB, next_b, 2 |
    recv_mpst_f_to_c, RoleC, next_c, 3 |
    recv_mpst_f_to_d, RoleD, next_d, 4 |
    recv_mpst_f_to_e, RoleE, next_e, 5 |
    recv_mpst_f_to_g, RoleG, next_g, 6 |
    recv_mpst_f_to_h, RoleH, next_h, 7 | =>
    RoleF, SessionMpstEight, 8
);
// G
create_recv_mpst_session_bundle!(
    recv_mpst_g_to_a, RoleA, next_a, 1 |
    recv_mpst_g_to_b, RoleB, next_b, 2 |
    recv_mpst_g_to_c, RoleC, next_c, 3 |
    recv_mpst_g_to_d, RoleD, next_d, 4 |
    recv_mpst_g_to_e, RoleE, next_e, 5 |
    recv_mpst_g_to_f, RoleF, next_f, 6 |
    recv_mpst_g_to_h, RoleH, next_h, 7 | =>
    RoleG, SessionMpstEight, 8
);
// H
create_recv_mpst_session_bundle!(
    recv_mpst_h_to_a, RoleA, next_a, 1 |
    recv_mpst_h_to_b, RoleB, next_b, 2 |
    recv_mpst_h_to_c, RoleC, next_c, 3 |
    recv_mpst_h_to_d, RoleD, next_d, 4 |
    recv_mpst_h_to_e, RoleE, next_e, 5 |
    recv_mpst_h_to_f, RoleF, next_f, 6 |
    recv_mpst_h_to_g, RoleG, next_g, 7 | =>
    RoleH, SessionMpstEight, 8
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstEight, 8);

// Create fork function
fork_mpst_multi!(fork_mpst, SessionMpstEight, 8);

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
// A
enum Branching0fromHtoA {
    More(
        SessionMpstEight<
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursAtoH>>,
            R2H<R2B<R2C<R2D<R2E<R2F<R2G<RoleH<RoleEnd>>>>>>>>,
            NameA,
        >,
    ),
    Done(SessionMpstEight<End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoH = Recv<Branching0fromHtoA, End>;
// B
enum Branching0fromHtoB {
    More(
        SessionMpstEight<
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursBtoH>>,
            R2H<R2A<R2C<R2D<R2E<R2F<R2G<RoleH<RoleEnd>>>>>>>>,
            NameB,
        >,
    ),
    Done(SessionMpstEight<End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoH = Recv<Branching0fromHtoB, End>;
// C
enum Branching0fromHtoC {
    More(
        SessionMpstEight<
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursCtoH>>,
            R2H<R2A<R2B<R2D<R2E<R2F<R2G<RoleH<RoleEnd>>>>>>>>,
            NameC,
        >,
    ),
    Done(SessionMpstEight<End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoH = Recv<Branching0fromHtoC, End>;
// D
enum Branching0fromHtoD {
    More(
        SessionMpstEight<
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursDtoH>>,
            R2H<R2A<R2B<R2C<R2E<R2F<R2G<RoleH<RoleEnd>>>>>>>>,
            NameD,
        >,
    ),
    Done(SessionMpstEight<End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoH = Recv<Branching0fromHtoD, End>;
// E
enum Branching0fromHtoE {
    More(
        SessionMpstEight<
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            Recv<(), Send<(), RecursEtoH>>,
            R2H<R2A<R2B<R2C<R2D<R2F<R2G<RoleH<RoleEnd>>>>>>>>,
            NameE,
        >,
    ),
    Done(SessionMpstEight<End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoH = Recv<Branching0fromHtoE, End>;
// F
enum Branching0fromHtoF {
    More(
        SessionMpstEight<
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            Recv<(), Send<(), RecursFtoH>>,
            R2H<R2A<R2B<R2C<R2D<R2E<R2G<RoleH<RoleEnd>>>>>>>>,
            NameF,
        >,
    ),
    Done(SessionMpstEight<End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoH = Recv<Branching0fromHtoF, End>;
// G
enum Branching0fromHtoG {
    More(
        SessionMpstEight<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            Recv<(), Send<(), RecursGtoH>>,
            R2H<R2A<R2B<R2C<R2D<R2E<R2F<RoleH<RoleEnd>>>>>>>>,
            NameG,
        >,
    ),
    Done(SessionMpstEight<End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoH = Recv<Branching0fromHtoG, End>;
// H
type Choose0fromHtoA = Send<Branching0fromHtoA, End>;
type Choose0fromHtoB = Send<Branching0fromHtoB, End>;
type Choose0fromHtoC = Send<Branching0fromHtoC, End>;
type Choose0fromHtoD = Send<Branching0fromHtoD, End>;
type Choose0fromHtoE = Send<Branching0fromHtoE, End>;
type Choose0fromHtoF = Send<Branching0fromHtoF, End>;
type Choose0fromHtoG = Send<Branching0fromHtoG, End>;

// Creating the MP sessions
type EndpointA = SessionMpstEight<End, End, End, End, End, End, RecursAtoH, RoleH<RoleEnd>, NameA>;
type EndpointB = SessionMpstEight<End, End, End, End, End, End, RecursBtoH, RoleH<RoleEnd>, NameB>;
type EndpointC = SessionMpstEight<End, End, End, End, End, End, RecursCtoH, RoleH<RoleEnd>, NameC>;
type EndpointD = SessionMpstEight<End, End, End, End, End, End, RecursDtoH, RoleH<RoleEnd>, NameD>;
type EndpointE = SessionMpstEight<End, End, End, End, End, End, RecursEtoH, RoleH<RoleEnd>, NameE>;
type EndpointF = SessionMpstEight<End, End, End, End, End, End, RecursFtoH, RoleH<RoleEnd>, NameF>;
type EndpointG = SessionMpstEight<End, End, End, End, End, End, RecursGtoH, RoleH<RoleEnd>, NameG>;
type EndpointH = SessionMpstEight<
    Choose0fromHtoA,
    Choose0fromHtoB,
    Choose0fromHtoC,
    Choose0fromHtoD,
    Choose0fromHtoE,
    Choose0fromHtoF,
    Choose0fromHtoG,
    RoleA<RoleB<RoleC<RoleD<RoleE<RoleF<RoleG<RoleEnd>>>>>>>,
    NameH,
>;

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_to_h, {
        Branching0fromHtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromHtoA::More(s) => {
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
        Branching0fromHtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromHtoB::More(s) => {
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
        Branching0fromHtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromHtoC::More(s) => {
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
        Branching0fromHtoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromHtoD::More(s) => {
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
        Branching0fromHtoE::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromHtoE::More(s) => {
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
        Branching0fromHtoF::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromHtoF::More(s) => {
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
        Branching0fromHtoG::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromHtoG::More(s) => {
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
                Branching0fromHtoA::Done,
                Branching0fromHtoB::Done,
                Branching0fromHtoC::Done,
                Branching0fromHtoD::Done,
                Branching0fromHtoE::Done,
                Branching0fromHtoF::Done,
                Branching0fromHtoG::Done, =>
                RoleA,
                RoleB,
                RoleC,
                RoleD,
                RoleE,
                RoleF,
                RoleG, =>
                RoleH,
                SessionMpstEight,
                8,
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
                Branching0fromHtoA::More,
                Branching0fromHtoB::More,
                Branching0fromHtoC::More,
                Branching0fromHtoD::More,
                Branching0fromHtoE::More,
                Branching0fromHtoF::More,
                Branching0fromHtoG::More, =>
                RoleA,
                RoleB,
                RoleC,
                RoleD,
                RoleE,
                RoleF,
                RoleG, =>
                RoleH,
                SessionMpstEight,
                8,
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
            simple_five_endpoint_a,
            simple_five_endpoint_b,
            simple_five_endpoint_c,
            simple_five_endpoint_d,
            simple_five_endpoint_e,
            simple_five_endpoint_f,
            simple_five_endpoint_g,
            simple_five_endpoint_h,
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

static SIZE: i64 = 15;

fn main() {
    all_mpst().unwrap();
}
