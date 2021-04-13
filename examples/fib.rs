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
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstTwo, 2);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, 1 | =>
    RoleA, SessionMpstTwo, 2
);
// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a, RoleA, 1 | =>
    RoleB, SessionMpstTwo, 2
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 1 | =>
    RoleA, SessionMpstTwo, 2
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 1 | =>
    RoleB, SessionMpstTwo, 2
);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;

// Types
// A
type Choose0fromAtoB<N> = <RecursBtoA<N> as Session>::Dual;
// B
enum Branching0fromAtoB<N: marker::Send> {
    More(SessionMpstTwo<Recv<N, Send<N, RecursBtoA<N>>>, RoleA<RoleA<RoleA<RoleEnd>>>, NameB>),
    Done(SessionMpstTwo<End, RoleEnd, NameB>),
}
type RecursBtoA<N> = Recv<Branching0fromAtoB<N>, End>;

// Creating the MP sessions
type EndpointA<N> = SessionMpstTwo<Choose0fromAtoB<N>, RoleBroadcast, NameA>;
type EndpointB<N> = SessionMpstTwo<RecursBtoA<N>, RoleA<RoleEnd>, NameB>;

// Functions
fn simple_three_endpoint_a(s: EndpointA<i64>) -> Result<(), Box<dyn Error>> {
    recurs_a(s, SIZE, 1)
}

fn recurs_a(s: EndpointA<i64>, index: i64, old: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromAtoB::<i64>::Done, =>
                RoleB, =>
                RoleA,
                SessionMpstTwo,
                2,
                1
            );

            close_mpst_multi(s)
        }
        i => {
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromAtoB::<i64>::More, =>
                RoleB, =>
                RoleA,
                SessionMpstTwo,
                2,
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

fn all_mpst() -> Result<(), Box<dyn std::any::Any + std::marker::Send>> {
    let (thread_a, thread_b) = fork_mpst(simple_three_endpoint_a, simple_three_endpoint_b);

    thread_a.join()?;
    thread_b.join()?;

    Ok(())
}

static SIZE: i64 = 10;

fn main() {
    assert!(all_mpst().is_ok());
}