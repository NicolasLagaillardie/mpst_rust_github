use mpstthree::binary::struct_trait::{End, Recv, Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    broadcast_cancel, bundle_struct_fork_close_multi, choose_mpst_multi_cancel_to_all,
    create_normal_role, create_recv_mpst_session_bundle, create_send_check_cancel_bundle,
    offer_cancel_mpst, send_cancel,
};

use rand::random;
use std::error::Error;

// Create new SessionMpst for three participants
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstThree, 3);

// Create new roles
// normal
create_normal_role!(RoleA, RoleADual);
create_normal_role!(RoleB, RoleBDual);
create_normal_role!(RoleC, RoleCDual);

// Create new send functions
// B
create_send_check_cancel_bundle!(
    send_check_b_to_c, RoleC, 2 | =>
    RoleB, SessionMpstThree, 3
);
// C
create_send_check_cancel_bundle!(
    send_check_c_to_b, RoleB, 2 | =>
    RoleC, SessionMpstThree, 3
);

// Create new recv functions and related types
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_c, RoleC, 2 | =>
    RoleB, SessionMpstThree, 3
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_b, RoleB, 2 | =>
    RoleC, SessionMpstThree, 3
);

send_cancel!(cancel_mpst, RoleC, SessionMpstThree, 3, "Session dropped");

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;

// Types
// B
enum Branching0fromCtoB {
    More(SessionMpstThree<End, Send<i32, RecursBtoD>, RoleC<RoleC<RoleEnd>>, NameB>),
    Done(SessionMpstThree<End, End, RoleEnd, NameB>),
}
type RecursBtoD = Recv<(End, Branching0fromCtoB), End>;
// D
type Choose0fromCtoA = End;
type Choose0fromCtoB = Send<(End, Branching0fromCtoB), End>; // TODO: Remove the need of tuple with an End which is forwaded to A

// Creating the MP sessions
type EndpointA = SessionMpstThree<End, End, RoleEnd, NameA>;
type EndpointB = SessionMpstThree<End, RecursBtoD, RoleC<RoleEnd>, NameB>;
type EndpointC = SessionMpstThree<Choose0fromCtoA, Choose0fromCtoB, RoleBroadcast, NameC>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, RoleA, 3);
    Ok(())
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_b_from_c, {
        Branching0fromCtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromCtoB::More(s) => {
            let s = send_check_b_to_c(random(), s)?;
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
            let s = choose_mpst_multi_cancel_to_all!(
                s,
                Branching0fromCtoB::Done, =>
                RoleB, =>
                RoleA,
                RoleC,
                SessionMpstThree,
                3,
                3
            );

            close_mpst_multi(s)
        }
        5 => cancel_mpst(s),
        i => {
            let s = choose_mpst_multi_cancel_to_all!(
                s,
                Branching0fromCtoB::More, =>
                RoleB, =>
                RoleA,
                RoleC,
                SessionMpstThree,
                3,
                3
            );

            let (_, s) = recv_mpst_c_from_b(s)?;

            recurs_c(s, i - 1)
        }
    }
}

pub fn main() {
    let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);

    assert!(thread_a.join().is_err());
    assert!(thread_b.join().is_err());
    assert!(thread_c.join().is_err());
}

/////////////////////////

static SIZE: i64 = 15;
