#![allow(dead_code)]

use crossbeam_channel::bounded;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::{close, fork_with_thread_id, recv, send, End, Recv, Send, Session};
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::{
    bundle_fork_multi, choose, choose_mpst_multi_to_all, close_mpst, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, create_sessionmpst, offer,
    offer_mpst,
};

use std::error::Error;
use std::thread::{spawn, JoinHandle};
use std::time::Duration;

// Create new SessionMpst for four participants
create_sessionmpst!(SessionMpstSeven, 7);

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
    6 | =>
    RoleA,
    SessionMpstSeven,
    7
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
    6 | =>
    RoleB,
    SessionMpstSeven,
    7
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
    6 | =>
    RoleC,
    SessionMpstSeven,
    7
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
    6 | =>
    RoleD,
    SessionMpstSeven,
    7
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
    6 | =>
    RoleE,
    SessionMpstSeven,
    7
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
    6 | =>
    RoleF,
    SessionMpstSeven,
    7
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
    6 | =>
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
    6 | =>
    RoleA,
    SessionMpstSeven,
    7
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
    6 | =>
    RoleB,
    SessionMpstSeven,
    7
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
    6 | =>
    RoleC,
    SessionMpstSeven,
    7
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
    6 | =>
    RoleD,
    SessionMpstSeven,
    7
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
    6 | =>
    RoleE,
    SessionMpstSeven,
    7
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
    6 | =>
    RoleF,
    SessionMpstSeven,
    7
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
    6 | =>
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
enum Branching0fromGtoA
{
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
type RecursAtoG = Recv<Branching0fromGtoA, End>;
// B
enum Branching0fromGtoB
{
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
type RecursBtoG = Recv<Branching0fromGtoB, End>;
// C
enum Branching0fromGtoC
{
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
type RecursCtoG = Recv<Branching0fromGtoC, End>;
// D
enum Branching0fromGtoD
{
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
type RecursDtoG = Recv<Branching0fromGtoD, End>;
// E
enum Branching0fromGtoE
{
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
type RecursEtoG = Recv<Branching0fromGtoE, End>;
// F
enum Branching0fromGtoF
{
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
type RecursFtoG = Recv<Branching0fromGtoF, End>;
// F
type Choose0fromGtoA = Send<Branching0fromGtoA, End>;
type Choose0fromGtoB = Send<Branching0fromGtoB, End>;
type Choose0fromGtoC = Send<Branching0fromGtoC, End>;
type Choose0fromGtoD = Send<Branching0fromGtoD, End>;
type Choose0fromGtoE = Send<Branching0fromGtoE, End>;
type Choose0fromGtoF = Send<Branching0fromGtoF, End>;

// Creating the MP sessions
type EndpointA = SessionMpstSeven<End, End, End, End, End, RecursAtoG, RoleG<RoleEnd>, NameA>;
type EndpointB = SessionMpstSeven<End, End, End, End, End, RecursBtoG, RoleG<RoleEnd>, NameB>;
type EndpointC = SessionMpstSeven<End, End, End, End, End, RecursCtoG, RoleG<RoleEnd>, NameC>;
type EndpointD = SessionMpstSeven<End, End, End, End, End, RecursDtoG, RoleG<RoleEnd>, NameD>;
type EndpointE = SessionMpstSeven<End, End, End, End, End, RecursEtoG, RoleG<RoleEnd>, NameE>;
type EndpointF = SessionMpstSeven<End, End, End, End, End, RecursFtoG, RoleG<RoleEnd>, NameF>;
type EndpointG = SessionMpstSeven<
    Choose0fromGtoA,
    Choose0fromGtoB,
    Choose0fromGtoC,
    Choose0fromGtoD,
    Choose0fromGtoE,
    Choose0fromGtoF,
    RoleA<RoleB<RoleC<RoleD<RoleE<RoleF<RoleEnd>>>>>>,
    NameG,
>;

// Functions
fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_a_to_g, {
          Branching0fromGtoA::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromGtoA::More(s) => {
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

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_b_to_g, {
          Branching0fromGtoB::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromGtoB::More(s) => {
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

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_c_to_g, {
          Branching0fromGtoC::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromGtoC::More(s) => {
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

fn simple_five_endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_d_to_g, {
          Branching0fromGtoD::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromGtoD::More(s) => {
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

fn simple_five_endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_e_to_g, {
          Branching0fromGtoE::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromGtoE::More(s) => {
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

fn simple_five_endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>>
{
    offer_mpst!(s, recv_mpst_f_to_g, {
          Branching0fromGtoF::Done(s) => {
            close_mpst_multi(s)
        },
          Branching0fromGtoF::More(s) => {
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

fn simple_five_endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>>
{
    recurs_g(s, SIZE)
}

fn recurs_g(s: EndpointG, index: i64) -> Result<(), Box<dyn Error>>
{
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
                Branching0fromGtoA::Done,
                Branching0fromGtoB::Done,
                Branching0fromGtoC::Done,
                Branching0fromGtoD::Done,
                Branching0fromGtoE::Done,
                Branching0fromGtoF::Done, =>
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
                Branching0fromGtoA::More,
                Branching0fromGtoB::More,
                Branching0fromGtoC::More,
                Branching0fromGtoD::More,
                Branching0fromGtoE::More,
                Branching0fromGtoF::More, =>
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

fn all_mpst() -> Result<(), Box<dyn Error>>
{
    let (thread_a, thread_b, thread_c, thread_d, thread_e, thread_f, thread_g) = fork_mpst(
        black_box(simple_five_endpoint_a),
        black_box(simple_five_endpoint_b),
        black_box(simple_five_endpoint_c),
        black_box(simple_five_endpoint_d),
        black_box(simple_five_endpoint_e),
        black_box(simple_five_endpoint_f),
        black_box(simple_five_endpoint_g),
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
// A
enum BinaryA
{
    More(Recv<(), Send<(), RecursA>>),
    Done(End),
}
type RecursA = Recv<BinaryA, End>;
fn binary_a_to_b(s: RecursA) -> Result<(), Box<dyn Error>>
{
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
fn binary_b_to_a(s: Send<(), Recv<(), RecursB>>) -> Result<RecursB, Box<dyn Error>>
{
    let s = send((), s);
    let (_, s) = recv(s)?;
    Ok(s)
}

fn all_binaries() -> Result<(), Box<dyn Error>>
{
    let mut threads = Vec::new();
    let mut sessions = Vec::new();

    for _ in 0..21 {
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

fn all_crossbeam() -> Result<(), Box<dyn Error>>
{
    let mut threads = Vec::new();

    for _ in 0..21 {
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

fn long_simple_protocol_mpst(c: &mut Criterion)
{
    c.bench_function(&format!("long seven simple protocol MPST {}", SIZE), |b| {
        b.iter(|| all_mpst())
    });
}

fn long_simple_protocol_binary(c: &mut Criterion)
{
    c.bench_function(
        &format!("long seven simple protocol binary {}", SIZE),
        |b| b.iter(|| all_binaries()),
    );
}

fn long_simple_protocol_crossbeam(c: &mut Criterion)
{
    c.bench_function(
        &format!("long seven simple protocol crossbeam {}", SIZE),
        |b| b.iter(|| all_crossbeam()),
    );
}

fn long_warmup() -> Criterion
{
    Criterion::default().measurement_time(Duration::new(1800, 0))
}

criterion_group! {
    name = long_seven_simple_protocols;
    config = long_warmup();
    targets = long_simple_protocol_mpst, long_simple_protocol_binary, long_simple_protocol_crossbeam
}
criterion_main!(long_seven_simple_protocols);
