#![allow(dead_code)]

use crossbeam_channel::bounded;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::{close, fork_with_thread_id, recv, send, End, Recv, Send, Session};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose, choose_mpst_multi_to_all, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, offer, offer_mpst,
};

use std::error::Error;
use std::thread::{spawn, JoinHandle};
use std::time::Duration;

// Create the new SessionMpst for eight participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstEight, 8);

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

// Functions
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

type ReceivingSendingReceiving = crossbeam_channel::Receiver<SendingReceiving>;
type SendingReceivingSending = crossbeam_channel::Sender<ReceivingSending>;

type SendingReceiving = crossbeam_channel::Sender<Receiving>;
type ReceivingSending = crossbeam_channel::Receiver<Sending>;

type Receiving = crossbeam_channel::Receiver<()>;
type Sending = crossbeam_channel::Sender<()>;

fn all_crossbeam() -> Result<(), Box<dyn Error>> {
    let mut threads = Vec::new();

    for _ in 0..28 {
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

static SIZE: i64 = 100;

fn long_simple_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("long eight simple protocol MPST {}", SIZE), |b| {
        b.iter(|| all_mpst())
    });
}

fn long_simple_protocol_binary(c: &mut Criterion) {
    c.bench_function(
        &format!("long eight simple protocol binary {}", SIZE),
        |b| b.iter(|| all_binaries()),
    );
}

fn long_simple_protocol_crossbeam(c: &mut Criterion) {
    c.bench_function(
        &format!("long eight simple protocol crossbeam {}", SIZE),
        |b| b.iter(|| all_crossbeam()),
    );
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(1800, 0))
}

criterion_group! {
    name = long_eight_simple_protocols;
    config = long_warmup();
    targets = long_simple_protocol_mpst, long_simple_protocol_binary, long_simple_protocol_crossbeam
}
criterion_main!(long_eight_simple_protocols);
