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
create_sessionmpst!(SessionMpstFive, 5);

// Create new roles
// normal
create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
create_normal_role!(RoleD, next_d, RoleDDual, next_d_dual);
create_normal_role!(RoleE, next_e, RoleEDual, next_e_dual);

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
    4, | =>
    RoleA,
    SessionMpstFive,
    5
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
    4, | =>
    RoleB,
    SessionMpstFive,
    5
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
    4, | =>
    RoleC,
    SessionMpstFive,
    5
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
    4, | =>
    RoleD,
    SessionMpstFive,
    5
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
    4, | =>
    RoleE,
    SessionMpstFive,
    5
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
    4, | =>
    RoleA,
    SessionMpstFive,
    5
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
    4, | =>
    RoleB,
    SessionMpstFive,
    5
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
    4, | =>
    RoleC,
    SessionMpstFive,
    5
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
    4, | =>
    RoleD,
    SessionMpstFive,
    5
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
    4, | =>
    RoleE,
    SessionMpstFive,
    5
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstFive, 5);

// Create fork function
bundle_fork_multi!(fork_mpst, fork_simple, SessionMpstFive, 5);

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
enum BranchingEforA {
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
type RecursAtoE = Recv<BranchingEforA, End>;
// B
enum BranchingEforB {
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
type RecursBtoE = Recv<BranchingEforB, End>;
// C
enum BranchingEforC {
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
type RecursCtoE = Recv<BranchingEforC, End>;
// D
enum BranchingEforD {
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
type RecursDtoE = Recv<BranchingEforD, End>;
// E
type ChooseEforAtoE = Send<BranchingEforA, End>;
type ChooseEforBtoE = Send<BranchingEforB, End>;
type ChooseEforCtoE = Send<BranchingEforC, End>;
type ChooseEforDtoE = Send<BranchingEforD, End>;

// Creating the MP sessions
type EndpointA = SessionMpstFive<End, End, End, RecursAtoE, RoleE<RoleEnd>, NameA>;
type EndpointB = SessionMpstFive<End, End, End, RecursBtoE, RoleE<RoleEnd>, NameB>;
type EndpointC = SessionMpstFive<End, End, End, RecursCtoE, RoleE<RoleEnd>, NameC>;
type EndpointD = SessionMpstFive<End, End, End, RecursDtoE, RoleE<RoleEnd>, NameD>;
type EndpointE = SessionMpstFive<
    ChooseEforAtoE,
    ChooseEforBtoE,
    ChooseEforCtoE,
    ChooseEforDtoE,
    RoleA<RoleB<RoleC<RoleD<RoleEnd>>>>,
    NameE,
>;

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_to_e, {
         BranchingEforA::Done(s) => {
            close_mpst_multi(s)
        },
         BranchingEforA::More(s) => {
            let (_, s) = recv_mpst_a_to_e(s)?;
            let s = send_mpst_a_to_e((), s);
            let (_, s) = recv_mpst_a_to_b(s)?;
            let s = send_mpst_a_to_b((), s);
            let (_, s) = recv_mpst_a_to_c(s)?;
            let s = send_mpst_a_to_c((), s);
            let (_, s) = recv_mpst_a_to_d(s)?;
            let s = send_mpst_a_to_d((), s);
            simple_five_endpoint_a(s)
        },
    })
}

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_to_e, {
         BranchingEforB::Done(s) => {
            close_mpst_multi(s)
        },
         BranchingEforB::More(s) => {
            let (_, s) = recv_mpst_b_to_e(s)?;
            let s = send_mpst_b_to_e((), s);
            let s = send_mpst_b_to_a((), s);
            let (_, s) = recv_mpst_b_to_a(s)?;
            let (_, s) = recv_mpst_b_to_c(s)?;
            let s = send_mpst_b_to_c((), s);
            let (_, s) = recv_mpst_b_to_d(s)?;
            let s = send_mpst_b_to_d((), s);
            simple_five_endpoint_b(s)
        },
    })
}

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_to_e, {
         BranchingEforC::Done(s) => {
            close_mpst_multi(s)
        },
         BranchingEforC::More(s) => {
            let (_, s) = recv_mpst_c_to_e(s)?;
            let s = send_mpst_c_to_e((), s);
            let s = send_mpst_c_to_a((), s);
            let (_, s) = recv_mpst_c_to_a(s)?;
            let s = send_mpst_c_to_b((), s);
            let (_, s) = recv_mpst_c_to_b(s)?;
            let (_, s) = recv_mpst_c_to_d(s)?;
            let s = send_mpst_c_to_d((), s);
            simple_five_endpoint_c(s)
        },
    })
}

fn simple_five_endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_to_e, {
         BranchingEforD::Done(s) => {
            close_mpst_multi(s)
        },
         BranchingEforD::More(s) => {
            let (_, s) = recv_mpst_d_to_e(s)?;
            let s = send_mpst_d_to_e((), s);
            let s = send_mpst_d_to_a((), s);
            let (_, s) = recv_mpst_d_to_a(s)?;
            let s = send_mpst_d_to_b((), s);
            let (_, s) = recv_mpst_d_to_b(s)?;
            let s = send_mpst_d_to_c((), s);
            let (_, s) = recv_mpst_d_to_c(s)?;
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
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_e_to_a,
                send_mpst_e_to_b,
                send_mpst_e_to_c,
                send_mpst_e_to_d, =>
                BranchingEforA::Done,
                BranchingEforB::Done,
                BranchingEforC::Done,
                BranchingEforD::Done, =>
                RoleA,
                RoleB,
                RoleC,
                RoleD, =>
                RoleE,
                SessionMpstFive,
                5,
                5
            );

            close_mpst_multi(s)
        }
        i => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_e_to_a,
                send_mpst_e_to_b,
                send_mpst_e_to_c,
                send_mpst_e_to_d, =>
                BranchingEforA::More,
                BranchingEforB::More,
                BranchingEforC::More,
                BranchingEforD::More, =>
                RoleA,
                RoleB,
                RoleC,
                RoleD, =>
                RoleE,
                SessionMpstFive,
                5,
                5
            );

            let s = send_mpst_e_to_a((), s);
            let (_, s) = recv_mpst_e_to_a(s)?;
            let s = send_mpst_e_to_b((), s);
            let (_, s) = recv_mpst_e_to_b(s)?;
            let s = send_mpst_e_to_c((), s);
            let (_, s) = recv_mpst_e_to_c(s)?;
            let s = send_mpst_e_to_d((), s);
            let (_, s) = recv_mpst_e_to_d(s)?;

            recurs_e(s, i - 1)
        }
    }
}

fn all_mpst() -> Result<(), Box<dyn Error>> {
    let (thread_a, thread_b, thread_c, thread_d, thread_e) = fork_mpst(
        simple_five_endpoint_a,
        simple_five_endpoint_b,
        simple_five_endpoint_c,
        simple_five_endpoint_d,
        simple_five_endpoint_e,
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();

    Ok(())
}

/////////////////////////

static SIZE: i64 = 15;

fn main() {
    all_mpst().unwrap();
}
