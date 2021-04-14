/// An example which mixes both the usual way of creating recv/send functions
/// with create_recv_mpst_session_bundle/create_send_mpst_session_bundle and the short way to
/// call the code within those functions with recv_mpst/send_mpst
use mpstthree::binary::struct_trait::{End, Recv, Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, create_fn_choose_mpst_multi_to_all_bundle,
    create_multiple_normal_role_short, create_recv_mpst_session_bundle, offer_mpst, recv_mpst,
    send_mpst,
};

use std::error::Error;

// Create the new SessionMpst for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstThree, 3);

// Create new roles
create_multiple_normal_role_short!(A, B, C);

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
type EndpointDoneC = SessionMpstThree<End, End, RoleEnd, NameC>;
type EndpointMoreC = SessionMpstThree<
    Send<(), Recv<(), Choose0fromCtoA>>,
    Send<(), Recv<(), Choose0fromCtoB>>,
    R2A<R2B<RoleBroadcast>>,
    NameC,
>;

// Creating the MP sessions
type EndpointA = SessionMpstThree<End, RecursAtoC, RoleC<RoleEnd>, NameA>;
type EndpointB = SessionMpstThree<End, RecursBtoC, RoleC<RoleEnd>, NameB>;
type EndpointC = SessionMpstThree<Choose0fromCtoA, Choose0fromCtoB, RoleBroadcast, NameC>;

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_c, RoleC, 2 | =>
    RoleA, SessionMpstThree, 3
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_c, RoleC, 2 | =>
    RoleB, SessionMpstThree, 3
);

create_fn_choose_mpst_multi_to_all_bundle!(
    done_from_c_to_all, more_from_c_to_all, =>
    Done, More, =>
    EndpointDoneC, EndpointMoreC, =>
    Branching0fromCtoA, Branching0fromCtoB, =>
    RoleA, RoleB, =>
    RoleC, SessionMpstThree, 3, 3
);

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_c, {
        Branching0fromCtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromCtoA::More(s) => {
            let (_, s) = recv_mpst!(s, RoleC, RoleA, SessionMpstThree, 3, 2)()?;
            let s = send_mpst!(s, (), RoleC, RoleA, SessionMpstThree, 3, 2);
            let (_, s) = recv_mpst!(s, RoleB, RoleA, SessionMpstThree, 3, 1)()?;
            let s = send_mpst!(s, (), RoleB, RoleA, SessionMpstThree, 3, 1);
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_c, {
        Branching0fromCtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromCtoB::More(s) => {
            let (_, s) = recv_mpst!(s, RoleC, RoleB, SessionMpstThree, 3, 2)()?;
            let s = send_mpst!(s, (), RoleC, RoleB, SessionMpstThree, 3, 2);
            let s = send_mpst!(s, (), RoleA, RoleB, SessionMpstThree, 3, 1);
            let (_, s) = recv_mpst!(s, RoleA, RoleB, SessionMpstThree, 3, 1)()?;
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    recurs_c(s, SIZE)
}

fn recurs_c(s: EndpointC, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_c_to_all(s);

            close_mpst_multi(s)
        }
        i => {
            let s = more_from_c_to_all(s);

            let s = send_mpst!(s, (), RoleA, RoleC, SessionMpstThree, 3, 1); // Generates much more lines in the end than a single use of
                                                                             // create_send_mpst_http_bundle, but faster to write for users
            let (_, s) = recv_mpst!(s, RoleA, RoleC, SessionMpstThree, 3, 1)()?;
            let s = send_mpst!(s, (), RoleB, RoleC, SessionMpstThree, 3, 2);
            let (_, s) = recv_mpst!(s, RoleB, RoleC, SessionMpstThree, 3, 2)()?;

            recurs_c(s, i - 1)
        }
    }
}

fn all_mpst() -> Result<(), Box<dyn std::any::Any + std::marker::Send>> {
    let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);

    thread_a.join()?;
    thread_b.join()?;
    thread_c.join()?;

    Ok(())
}

/////////////////////////

static SIZE: i64 = 15;

fn main() {
    assert!(all_mpst().is_ok());
}
