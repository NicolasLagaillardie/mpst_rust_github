use mpstthree::binary::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose_mpst_multi_to_all, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, offer_mpst,
};

use std::error::Error;

// Create the new SessionMpst for four participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstFour, 4);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, next_a, RoleADual, next_a_dual |
    RoleB, next_b, RoleBDual, next_b_dual |
    RoleC, next_c, RoleCDual, next_c_dual |
    RoleD, next_d, RoleDDual, next_d_dual |
);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, next_b, 1 |
    send_mpst_a_to_c, RoleC, next_c, 2 |
    send_mpst_a_to_d, RoleD, next_d, 3 | =>
    RoleA, SessionMpstFour, 4
);
// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a, RoleA, next_a, 1 |
    send_mpst_b_to_c, RoleC, next_c, 2 |
    send_mpst_b_to_d, RoleD, next_d, 3 | =>
    RoleB, SessionMpstFour, 4
);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a, RoleA, next_a, 1 |
    send_mpst_c_to_b, RoleB, next_b, 2 |
    send_mpst_c_to_d, RoleD, next_d, 3 | =>
    RoleC, SessionMpstFour, 4
);
// D
create_send_mpst_session_bundle!(
    send_mpst_d_to_a, RoleA, next_a, 1 |
    send_mpst_d_to_b, RoleB, next_b, 2 |
    send_mpst_d_to_c, RoleC, next_c, 3 | =>
    RoleD, SessionMpstFour, 4
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_to_b, RoleB, next_b, 1 |
    recv_mpst_a_to_c, RoleC, next_c, 2 |
    recv_mpst_a_to_d, RoleD, next_d, 3 | =>
    RoleA, SessionMpstFour, 4
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_to_a, RoleA, next_a, 1 |
    recv_mpst_b_to_c, RoleC, next_c, 2 |
    recv_mpst_b_to_d, RoleD, next_d, 3 | =>
    RoleB, SessionMpstFour, 4
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_to_a, RoleA, next_a, 1 |
    recv_mpst_c_to_b, RoleB, next_b, 2 |
    recv_mpst_c_to_d, RoleD, next_d, 3 | =>
    RoleC, SessionMpstFour, 4
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_to_a, RoleA, next_a, 1 |
    recv_mpst_d_to_b, RoleB, next_b, 2 |
    recv_mpst_d_to_c, RoleC, next_c, 3 | =>
    RoleD, SessionMpstFour, 4
);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;

// Types
// Send/Recv
type RS = Recv<(), Send<(), End>>;
type SR = Send<(), Recv<(), End>>;
// Roles
type R2A<R> = RoleA<RoleA<R>>;
type R2B<R> = RoleB<RoleB<R>>;
type R2C<R> = RoleC<RoleC<R>>;
type R2D<R> = RoleD<RoleD<R>>;
// A
enum Branching0fromDtoA {
    More(
        SessionMpstFour<
            RS,
            RS,
            Recv<(), Send<(), RecursAtoD>>,
            R2D<R2B<R2C<RoleD<RoleEnd>>>>,
            NameA,
        >,
    ),
    Done(SessionMpstFour<End, End, End, RoleEnd, NameA>),
}
type RecursAtoD = Recv<Branching0fromDtoA, End>;
// B
enum Branching0fromDtoB {
    More(
        SessionMpstFour<
            SR,
            RS,
            Recv<(), Send<(), RecursBtoD>>,
            R2D<R2A<R2C<RoleD<RoleEnd>>>>,
            NameB,
        >,
    ),
    Done(SessionMpstFour<End, End, End, RoleEnd, NameB>),
}
type RecursBtoD = Recv<Branching0fromDtoB, End>;
// C
enum Branching0fromDtoC {
    More(
        SessionMpstFour<
            SR,
            SR,
            Recv<(), Send<(), RecursCtoD>>,
            R2D<R2A<R2B<RoleD<RoleEnd>>>>,
            NameC,
        >,
    ),
    Done(SessionMpstFour<End, End, End, RoleEnd, NameC>),
}
type RecursCtoD = Recv<Branching0fromDtoC, End>;
// D
type Choose0fromDtoA = Send<Branching0fromDtoA, End>;
type Choose0fromDtoB = Send<Branching0fromDtoB, End>;
type Choose0fromDtoC = Send<Branching0fromDtoC, End>;

// Creating the MP sessions
type EndpointA = SessionMpstFour<End, End, RecursAtoD, RoleD<RoleEnd>, NameA>;
type EndpointB = SessionMpstFour<End, End, RecursBtoD, RoleD<RoleEnd>, NameB>;
type EndpointC = SessionMpstFour<End, End, RecursCtoD, RoleD<RoleEnd>, NameC>;
type EndpointD = SessionMpstFour<
    Choose0fromDtoA,
    Choose0fromDtoB,
    Choose0fromDtoC,
    RoleA<RoleB<RoleC<RoleEnd>>>,
    NameD,
>;

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_to_d, {
        Branching0fromDtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromDtoA::More(s) => {
            let (_, s) = recv_mpst_a_to_d(s)?;
            let s = send_mpst_a_to_d((), s);
            let (_, s) = recv_mpst_a_to_b(s)?;
            let s = send_mpst_a_to_b((), s);
            let (_, s) = recv_mpst_a_to_c(s)?;
            let s = send_mpst_a_to_c((), s);
            simple_five_endpoint_a(s)
        },
    })
}

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_to_d, {
        Branching0fromDtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromDtoB::More(s) => {
            let (_, s) = recv_mpst_b_to_d(s)?;
            let s = send_mpst_b_to_d((), s);
            let s = send_mpst_b_to_a((), s);
            let (_, s) = recv_mpst_b_to_a(s)?;
            let (_, s) = recv_mpst_b_to_c(s)?;
            let s = send_mpst_b_to_c((), s);
            simple_five_endpoint_b(s)
        },
    })
}

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_to_d, {
        Branching0fromDtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromDtoC::More(s) => {
            let (_, s) = recv_mpst_c_to_d(s)?;
            let s = send_mpst_c_to_d((), s);
            let s = send_mpst_c_to_a((), s);
            let (_, s) = recv_mpst_c_to_a(s)?;
            let s = send_mpst_c_to_b((), s);
            let (_, s) = recv_mpst_c_to_b(s)?;
            simple_five_endpoint_c(s)
        },
    })
}

fn simple_five_endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    recurs_d(s, SIZE)
}

fn recurs_d(s: EndpointD, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_d_to_a,
                send_mpst_d_to_b,
                send_mpst_d_to_c, =>
                Branching0fromDtoA::Done,
                Branching0fromDtoB::Done,
                Branching0fromDtoC::Done, =>
                RoleA,
                RoleB,
                RoleC, =>
                RoleD,
                SessionMpstFour,
                4,
                4
            );

            close_mpst_multi(s)
        }
        i => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_d_to_a,
                send_mpst_d_to_b,
                send_mpst_d_to_c, =>
                Branching0fromDtoA::More,
                Branching0fromDtoB::More,
                Branching0fromDtoC::More, =>
                RoleA,
                RoleB,
                RoleC, =>
                RoleD,
                SessionMpstFour,
                4,
                4
            );

            let s = send_mpst_d_to_a((), s);
            let (_, s) = recv_mpst_d_to_a(s)?;
            let s = send_mpst_d_to_b((), s);
            let (_, s) = recv_mpst_d_to_b(s)?;
            let s = send_mpst_d_to_c((), s);
            let (_, s) = recv_mpst_d_to_c(s)?;

            recurs_d(s, i - 1)
        }
    }
}

fn all_mpst() -> Result<(), Box<dyn Error>> {
    let (thread_a, thread_b, thread_c, thread_d) = fork_mpst(
        simple_five_endpoint_a,
        simple_five_endpoint_b,
        simple_five_endpoint_c,
        simple_five_endpoint_d,
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();

    Ok(())
}

/////////////////////////

static SIZE: i64 = 15;

fn main() {
    all_mpst().unwrap();
}
