/// An example which mixes both the usual way of creating recv/send functions with create_recv_mpst_session_bundle/
/// create_send_mpst_session_bundle and the short way to call the code within those functions with recv_mpst/send_mpst
use mpstthree::binary::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::{
    bundle_fork_multi, choose_mpst_multi_to_all, close_mpst, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, create_sessionmpst,
    offer_mpst, recv_mpst, send_mpst,
};

use std::error::Error;

// Create new SessionMpst for three participants
create_sessionmpst!(SessionMpstThree, 3);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, next_a, RoleADual, next_a_dual |
    RoleB, next_b, RoleBDual, next_b_dual |
    RoleC, next_c, RoleCDual, next_c_dual |
);

// Create new send functions
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_c_to_b,
    RoleB,
    next_b,
    2, | =>
    RoleC,
    SessionMpstThree,
    3
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_to_c,
    RoleC,
    next_c,
    2, | =>
    RoleA,
    SessionMpstThree,
    3
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_to_c,
    RoleC,
    next_c,
    2, | =>
    RoleB,
    SessionMpstThree,
    3
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstThree, 3);

// Create fork function
bundle_fork_multi!(fork_mpst, fork_simple, SessionMpstThree, 3);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;

// Types
// Send/Recv
type RS = Recv<(), Send<(), End>>;
type SR = Send<(), Recv<(), End>>;
// Roles
type R2A<R> = RoleA<RoleA<R>>;
type R2B<R> = RoleB<RoleB<R>>;
type R2C<R> = RoleC<RoleC<R>>;
// A
enum Branching0fromCtoA {
    More(SessionMpstThree<RS, Recv<(), Send<(), RecursAtoC>>, R2C<R2B<RoleC<RoleEnd>>>, NameA>),
    Done(SessionMpstThree<End, End, RoleEnd, NameA>),
}
type RecursAtoC = Recv<Branching0fromCtoA, End>;
// B
enum Branching0fromCtoB {
    More(SessionMpstThree<SR, Recv<(), Send<(), RecursBtoC>>, R2C<R2A<RoleC<RoleEnd>>>, NameB>),
    Done(SessionMpstThree<End, End, RoleEnd, NameB>),
}
type RecursBtoC = Recv<Branching0fromCtoB, End>;
// C
type Choose0fromCtoA = Send<Branching0fromCtoA, End>;
type Choose0fromCtoB = Send<Branching0fromCtoB, End>;

// Creating the MP sessions
type EndpointA = SessionMpstThree<End, RecursAtoC, RoleC<RoleEnd>, NameA>;
type EndpointB = SessionMpstThree<End, RecursBtoC, RoleC<RoleEnd>, NameB>;
type EndpointC = SessionMpstThree<Choose0fromCtoA, Choose0fromCtoB, RoleA<RoleB<RoleEnd>>, NameC>;

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_to_c, {
        Branching0fromCtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromCtoA::More(s) => {
            let (_, s) = recv_mpst!(s, next_c, SessionMpstThree, 3, 2)()?;
            let s = send_mpst!(s, (), next_c, SessionMpstThree, 3, 2);
            let (_, s) = recv_mpst!(s, next_b, SessionMpstThree, 3, 1)()?;
            let s = send_mpst!(s, (), next_b, SessionMpstThree, 3, 1);
            simple_five_endpoint_a(s)
        },
    })
}

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_to_c, {
        Branching0fromCtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromCtoB::More(s) => {
            let (_, s) = recv_mpst!(s, next_c, SessionMpstThree, 3, 2)()?;
            let s = send_mpst!(s, (), next_c, SessionMpstThree, 3, 2);
            let s = send_mpst!(s, (), next_a, SessionMpstThree, 3, 1);
            let (_, s) = recv_mpst!(s, next_a, SessionMpstThree, 3, 1)()?;
            simple_five_endpoint_b(s)
        },
    })
}

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    recurs_c(s, SIZE)
}

fn recurs_c(s: EndpointC, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_c_to_a,
                send_mpst_c_to_b, =>
                Branching0fromCtoA::Done,
                Branching0fromCtoB::Done, =>
                RoleA,
                RoleB, =>
                RoleC,
                SessionMpstThree,
                3,
                3
            );

            close_mpst_multi(s)
        }
        i => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_c_to_a,
                send_mpst_c_to_b, =>
                Branching0fromCtoA::More,
                Branching0fromCtoB::More, =>
                RoleA,
                RoleB, =>
                RoleC,
                SessionMpstThree,
                3,
                3
            );

            let s = send_mpst!(s, (), next_a, SessionMpstThree, 3, 1);
            let (_, s) = recv_mpst!(s, next_a, SessionMpstThree, 3, 1)()?;
            let s = send_mpst!(s, (), next_b, SessionMpstThree, 3, 2);
            let (_, s) = recv_mpst!(s, next_b, SessionMpstThree, 3, 2)()?;

            recurs_c(s, i - 1)
        }
    }
}

fn all_mpst() -> Result<(), Box<dyn Error>> {
    let (thread_a, thread_b, thread_c) = fork_mpst(
        simple_five_endpoint_a,
        simple_five_endpoint_b,
        simple_five_endpoint_c,
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();

    Ok(())
}

/////////////////////////

static SIZE: i64 = 15;

fn main() {
    all_mpst().unwrap();
}
