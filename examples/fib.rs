use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose_mpst_multi_to_all, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, offer_mpst,
};

use mpstthree::role::broadcast::RoleBroadcast;
use std::error::Error;
use std::marker;

// global protocol Fibonacci(role A, role B)
// {
//     rec Fib
//     {
//         choice at A
//         {
//             fibonacci(Long) from A to B;
//             fibonacci(Long) from B to A;
//             continue Fib;
//         }
//         or
//         {
//             stop() from A to B;
//         }
//     }
// }

// Create the new SessionMpst for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstThree, 3);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
    RoleC, RoleCDual |
);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, 1 |
    send_mpst_a_to_c, RoleC, 2 | =>
    RoleA, SessionMpstThree, 3
);
// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a, RoleA, 1 | =>
    RoleB, SessionMpstThree, 3
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 1 | =>
    RoleA, SessionMpstThree, 3
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 1 | =>
    RoleB, SessionMpstThree, 3
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a, RoleA, 1 | =>
    RoleC, SessionMpstThree, 3
);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;

// Types
// A
type Choose0fromAtoB<N> = <RecursBtoA<N> as Session>::Dual;
type Choose0fromAtoC = <RecursCtoA as Session>::Dual;
// B
enum Branching0fromAtoB<N: marker::Send> {
    More(
        SessionMpstThree<Recv<N, Send<N, RecursBtoA<N>>>, End, RoleA<RoleA<RoleA<RoleEnd>>>, NameB>,
    ),
    Done(SessionMpstThree<End, End, RoleEnd, NameB>),
}
type RecursBtoA<N> = Recv<Branching0fromAtoB<N>, End>;
// C
enum Branching0fromAtoC {
    More(SessionMpstThree<RecursCtoA, End, RoleA<RoleEnd>, NameC>),
    Done(SessionMpstThree<End, End, RoleEnd, NameC>),
}
type RecursCtoA = Recv<Branching0fromAtoC, End>;

// Creating the MP sessions
type EndpointA<N> = SessionMpstThree<Choose0fromAtoB<N>, Choose0fromAtoC, RoleBroadcast, NameA>;
type EndpointB<N> = SessionMpstThree<RecursBtoA<N>, End, RoleA<RoleEnd>, NameB>;
type EndpointC = SessionMpstThree<RecursCtoA, End, RoleA<RoleEnd>, NameC>;

// Functions
fn simple_three_endpoint_a(s: EndpointA<i64>) -> Result<(), Box<dyn Error>> {
    recurs_a(s, SIZE, 1)
}

fn recurs_a(s: EndpointA<i64>, index: i64, old: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromAtoB::<i64>::Done,
                Branching0fromAtoC::Done, =>
                RoleB,
                RoleC, =>
                RoleA,
                SessionMpstThree,
                3,
                1
            );

            close_mpst_multi(s)
        }
        i => {
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromAtoB::<i64>::More,
                Branching0fromAtoC::More, =>
                RoleB,
                RoleC, =>
                RoleA,
                SessionMpstThree,
                3,
                1
            );

            let s = send_mpst_a_to_b(old, s);
            let (new, s) = recv_mpst_a_from_b(s)?;

            recurs_a(s, i - 1, new)
        }
    }
}

fn simple_three_endpoint_b(s: EndpointB<i64>) -> Result<(), Box<dyn Error>> {
    recurs_b(s, 0)
}

fn recurs_b(s: EndpointB<i64>, old: i64) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_a, {
        Branching0fromAtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromAtoB::More(s) => {
            let (new, s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_a(new + old, s);
            recurs_b(s, new + old)
        },
    })
}

fn simple_three_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_a, {
        Branching0fromAtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromAtoC::More(s) => {
            simple_three_endpoint_c(s)
        },
    })
}

fn all_mpst() -> Result<(), Box<dyn std::any::Any + std::marker::Send>> {
    let (thread_a, thread_b, thread_c) = fork_mpst(
        simple_three_endpoint_a,
        simple_three_endpoint_b,
        simple_three_endpoint_c,
    );

    thread_a.join()?;
    thread_b.join()?;
    thread_c.join()?;

    Ok(())
}

static SIZE: i64 = 10;

fn main() {
    assert!(all_mpst().is_ok());
}
