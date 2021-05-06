/// An example which mixes both the usual way of creating recv/send functions
/// with create_recv_mpst_session_bundle/create_send_mpst_session_bundle and the short way to
/// call the code within those functions with recv_mpst/send_mpst
use mpstthree::binary::struct_trait::{End, Recv, Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_impl, create_fn_choose_mpst_multi_to_all_bundle, create_recv_mpst_session_bundle,
    fork_mpst_multi, offer_mpst,
};

use std::error::Error;

// Create new roles
bundle_impl!(SessionMpstThree => A, B, C);

fork_mpst_multi!(fork_mpst, SessionMpstThree, 3);

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
            s.close()
        },
        Branching0fromCtoA::More(s) => {
            let (_, s) = s.recv()?;
            let (_, s) = s.send(()).recv()?;
            let s = s.send(());
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_c, {
        Branching0fromCtoB::Done(s) => {
            s.close()
        },
        Branching0fromCtoB::More(s) => {
            let (_, s) = s.recv()?;
            let (_, s) = s.send(()).send(()).recv()?;
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    recurs_c(s, SIZE)
}

fn recurs_c(s: EndpointC, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => done_from_c_to_all(s).close(),
        i => {
            let (_, s) = more_from_c_to_all(s).send(()).recv()?;
            let (_, s) = s.send(()).recv()?;

            recurs_c(s, i - 1)
        }
    }
}

pub fn shorten_main() {
    let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);

    assert!(thread_a.join().is_ok());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_ok());
}

/////////////////////////

static SIZE: i64 = 15;
