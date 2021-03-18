use mpstthree::binary::struct_trait::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, create_fn_choose_mpst_multi_to_all_bundle,
    create_multiple_normal_role_short, create_recv_mpst_session_bundle,
    create_send_mpst_session_bundle, offer_mpst,
};

use std::error::Error;

// Create the new SessionMpst for five participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstFive, 5);

// Create new roles
// normal
create_multiple_normal_role_short!(A, B, C, D, E);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, next_b, 1 |
    send_mpst_a_to_c, RoleC, next_c, 2 |
    send_mpst_a_to_d, RoleD, next_d, 3 |
    send_mpst_a_to_e, RoleE, next_e, 4 | =>
    RoleA, SessionMpstFive, 5
);
// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a, RoleA, next_a, 1 |
    send_mpst_b_to_c, RoleC, next_c, 2 |
    send_mpst_b_to_d, RoleD, next_d, 3 |
    send_mpst_b_to_e, RoleE, next_e, 4 | =>
    RoleB, SessionMpstFive, 5
);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a, RoleA, next_a, 1 |
    send_mpst_c_to_b, RoleB, next_b, 2 |
    send_mpst_c_to_d, RoleD, next_d, 3 |
    send_mpst_c_to_e, RoleE, next_e, 4 | =>
    RoleC, SessionMpstFive, 5
);
// D
create_send_mpst_session_bundle!(
    send_mpst_d_to_a, RoleA, next_a, 1 |
    send_mpst_d_to_b, RoleB, next_b, 2 |
    send_mpst_d_to_c, RoleC, next_c, 3 |
    send_mpst_d_to_e, RoleE, next_e, 4 | =>
    RoleD, SessionMpstFive, 5
);
// E
create_send_mpst_session_bundle!(
    send_mpst_e_to_a, RoleA, next_a, 1 |
    send_mpst_e_to_b, RoleB, next_b, 2 |
    send_mpst_e_to_c, RoleC, next_c, 3 |
    send_mpst_e_to_d, RoleD, next_d, 4 | =>
    RoleE, SessionMpstFive, 5
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, next_b, 1 |
    recv_mpst_a_from_c, RoleC, next_c, 2 |
    recv_mpst_a_from_d, RoleD, next_d, 3 |
    recv_mpst_a_from_e, RoleE, next_e, 4 | =>
    RoleA, SessionMpstFive, 5
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, next_a, 1 |
    recv_mpst_b_from_c, RoleC, next_c, 2 |
    recv_mpst_b_from_d, RoleD, next_d, 3 |
    recv_mpst_b_from_e, RoleE, next_e, 4 | =>
    RoleB, SessionMpstFive, 5
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a, RoleA, next_a, 1 |
    recv_mpst_c_from_b, RoleB, next_b, 2 |
    recv_mpst_c_from_d, RoleD, next_d, 3 |
    recv_mpst_c_from_e, RoleE, next_e, 4 | =>
    RoleC, SessionMpstFive, 5
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_a, RoleA, next_a, 1 |
    recv_mpst_d_from_b, RoleB, next_b, 2 |
    recv_mpst_d_from_c, RoleC, next_c, 3 |
    recv_mpst_d_from_e, RoleE, next_e, 4 | =>
    RoleD, SessionMpstFive, 5
);
// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_a, RoleA, next_a, 1 |
    recv_mpst_e_from_b, RoleB, next_b, 2 |
    recv_mpst_e_from_c, RoleC, next_c, 3 |
    recv_mpst_e_from_d, RoleD, next_d, 4 | =>
    RoleE, SessionMpstFive, 5
);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;
type NameE = RoleE<RoleEnd>;

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
// A
enum Branching0fromEtoA {
    More(
        SessionMpstFive<
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursAtoE>>,
            R2E<R2B<R2C<R2D<RoleE<RoleEnd>>>>>,
            NameA,
        >,
    ),
    Done(SessionMpstFive<End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoE = Recv<Branching0fromEtoA, End>;
// B
enum Branching0fromEtoB {
    More(
        SessionMpstFive<
            SR,
            RS,
            RS,
            Recv<(), Send<(), RecursBtoE>>,
            R2E<R2A<R2C<R2D<RoleE<RoleEnd>>>>>,
            NameB,
        >,
    ),
    Done(SessionMpstFive<End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoE = Recv<Branching0fromEtoB, End>;
// C
enum Branching0fromEtoC {
    More(
        SessionMpstFive<
            SR,
            SR,
            RS,
            Recv<(), Send<(), RecursCtoE>>,
            R2E<R2A<R2B<R2D<RoleE<RoleEnd>>>>>,
            NameC,
        >,
    ),
    Done(SessionMpstFive<End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoE = Recv<Branching0fromEtoC, End>;
// D
enum Branching0fromEtoD {
    More(
        SessionMpstFive<
            SR,
            SR,
            SR,
            Recv<(), Send<(), RecursDtoE>>,
            R2E<R2A<R2B<R2C<RoleE<RoleEnd>>>>>,
            NameD,
        >,
    ),
    Done(SessionMpstFive<End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoE = Recv<Branching0fromEtoD, End>;
// E
type Choose0fromEtoA = Send<Branching0fromEtoA, End>;
type Choose0fromEtoB = Send<Branching0fromEtoB, End>;
type Choose0fromEtoC = Send<Branching0fromEtoC, End>;
type Choose0fromEtoD = Send<Branching0fromEtoD, End>;
type EndpointDoneE = SessionMpstFive<End, End, End, End, RoleEnd, NameE>;
type EndpointMoreE = SessionMpstFive<
    Send<(), Recv<(), Choose0fromEtoA>>,
    Send<(), Recv<(), Choose0fromEtoB>>,
    Send<(), Recv<(), Choose0fromEtoC>>,
    Send<(), Recv<(), Choose0fromEtoD>>,
    R2A<R2B<R2C<R2D<StackRecurs>>>>,
    NameE,
>;

// Creating the MP sessions
type EndpointA = SessionMpstFive<End, End, End, RecursAtoE, RoleE<RoleEnd>, NameA>;
type EndpointB = SessionMpstFive<End, End, End, RecursBtoE, RoleE<RoleEnd>, NameB>;
type EndpointC = SessionMpstFive<End, End, End, RecursCtoE, RoleE<RoleEnd>, NameC>;
type EndpointD = SessionMpstFive<End, End, End, RecursDtoE, RoleE<RoleEnd>, NameD>;
type StackRecurs = RoleA<RoleB<RoleC<RoleD<RoleEnd>>>>;
type EndpointE = SessionMpstFive<
    Choose0fromEtoA,
    Choose0fromEtoB,
    Choose0fromEtoC,
    Choose0fromEtoD,
    StackRecurs,
    NameE,
>;

create_fn_choose_mpst_multi_to_all_bundle!(
    done_from_e_to_all, more_from_e_to_all, =>
    Done, More, =>
    EndpointDoneE, EndpointMoreE, =>
    Branching0fromEtoA,
    Branching0fromEtoB,
    Branching0fromEtoC,
    Branching0fromEtoD, =>
    RoleA,
    RoleB,
    RoleC,
    RoleD, =>
    RoleE, SessionMpstFive, 5, 5
);

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_e, {
        Branching0fromEtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromEtoA::More(s) => {
            let (_, s) = recv_mpst_a_from_e(s)?;
            let s = send_mpst_a_to_e((), s);
            let (_, s) = recv_mpst_a_from_b(s)?;
            let s = send_mpst_a_to_b((), s);
            let (_, s) = recv_mpst_a_from_c(s)?;
            let s = send_mpst_a_to_c((), s);
            let (_, s) = recv_mpst_a_from_d(s)?;
            let s = send_mpst_a_to_d((), s);
            simple_five_endpoint_a(s)
        },
    })
}

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_e, {
        Branching0fromEtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromEtoB::More(s) => {
            let (_, s) = recv_mpst_b_from_e(s)?;
            let s = send_mpst_b_to_e((), s);
            let s = send_mpst_b_to_a((), s);
            let (_, s) = recv_mpst_b_from_a(s)?;
            let (_, s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_c((), s);
            let (_, s) = recv_mpst_b_from_d(s)?;
            let s = send_mpst_b_to_d((), s);
            simple_five_endpoint_b(s)
        },
    })
}

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_e, {
        Branching0fromEtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromEtoC::More(s) => {
            let (_, s) = recv_mpst_c_from_e(s)?;
            let s = send_mpst_c_to_e((), s);
            let s = send_mpst_c_to_a((), s);
            let (_, s) = recv_mpst_c_from_a(s)?;
            let s = send_mpst_c_to_b((), s);
            let (_, s) = recv_mpst_c_from_b(s)?;
            let (_, s) = recv_mpst_c_from_d(s)?;
            let s = send_mpst_c_to_d((), s);
            simple_five_endpoint_c(s)
        },
    })
}

fn simple_five_endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_from_e, {
        Branching0fromEtoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromEtoD::More(s) => {
            let (_, s) = recv_mpst_d_from_e(s)?;
            let s = send_mpst_d_to_e((), s);
            let s = send_mpst_d_to_a((), s);
            let (_, s) = recv_mpst_d_from_a(s)?;
            let s = send_mpst_d_to_b((), s);
            let (_, s) = recv_mpst_d_from_b(s)?;
            let s = send_mpst_d_to_c((), s);
            let (_, s) = recv_mpst_d_from_c(s)?;
            simple_five_endpoint_d(s)
        },
    })
}

fn simple_five_endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    recurs_e(s, SIZE)
}

fn recurs_e(s: EndpointE, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_e_to_all(s);

            close_mpst_multi(s)
        }
        i => {
            let s = more_from_e_to_all(s);

            let s = send_mpst_e_to_a((), s);
            let (_, s) = recv_mpst_e_from_a(s)?;
            let s = send_mpst_e_to_b((), s);
            let (_, s) = recv_mpst_e_from_b(s)?;
            let s = send_mpst_e_to_c((), s);
            let (_, s) = recv_mpst_e_from_c(s)?;
            let s = send_mpst_e_to_d((), s);
            let (_, s) = recv_mpst_e_from_d(s)?;

            recurs_e(s, i - 1)
        }
    }
}

fn all_mpst() -> Result<(), Box<dyn std::any::Any + std::marker::Send>> {
    let (thread_a, thread_b, thread_c, thread_d, thread_e) = fork_mpst(
        simple_five_endpoint_a,
        simple_five_endpoint_b,
        simple_five_endpoint_c,
        simple_five_endpoint_d,
        simple_five_endpoint_e,
    );

    thread_a.join()?;
    thread_b.join()?;
    thread_c.join()?;
    thread_d.join()?;
    thread_e.join()?;

    Ok(())
}

/////////////////////////

static SIZE: i64 = 15;

fn main() {
    assert!(all_mpst().is_ok());
}
