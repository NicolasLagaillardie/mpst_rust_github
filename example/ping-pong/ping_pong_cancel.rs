use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    broadcast_cancel, bundle_struct_fork_close_multi,
    create_fn_choose_mpst_cancel_multi_to_all_bundle, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_check_cancel_bundle, offer_cancel_mpst,
};

use std::error::Error;

// global protocol ping_pong(role A, role B)
// {
//     rec PP
//     {
//         choice at A
//         {
//             ping(()) from A to B;
//             pong(()) from B to A;
//             continue PP;
//         }
//         or
//         {
//             stop() from A to B;
//         }
//     }
// }

// Create the new SessionMpst for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstFour, 3);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
    RoleCentral, RoleCentralDual |
);

// Create new send functions
// A
create_send_check_cancel_bundle!(
    send_mpst_a_to_b, RoleB, 2 | =>
    RoleA, SessionMpstFour, 3
);
// B
create_send_check_cancel_bundle!(
    send_mpst_b_to_a, RoleA, 2 | =>
    RoleB, SessionMpstFour, 3
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 2 | =>
    RoleA, SessionMpstFour, 3
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 2 | =>
    RoleB, SessionMpstFour, 3
);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;

// Types
// A
type Choose0fromAtoB = <RecursBtoA as Session>::Dual;

// B
enum Branching0fromAtoB {
    More(SessionMpstFour<End, Recv<(), Send<(), RecursBtoA>>, ThreeRoleA, NameB>),
    Done(SessionMpstFour<End, End, RoleEnd, NameB>),
}
type ThreeRoleA = RoleA<RoleA<RoleA<RoleEnd>>>;
type RecursBtoA = Recv<(End, Branching0fromAtoB), End>;

// Creating the MP sessions

type EndpointDoneA = SessionMpstFour<End, End, RoleEnd, NameA>;
type EndpointForwardA =
    SessionMpstFour<End, Send<(), Recv<(), Choose0fromAtoB>>, RoleB<RoleB<RoleBroadcast>>, NameA>;

type EndpointCentral = SessionMpstFour<End, End, RoleEnd, RoleCentral<RoleEnd>>;
type EndpointA = SessionMpstFour<End, Choose0fromAtoB, RoleBroadcast, NameA>;
type EndpointB = SessionMpstFour<End, RecursBtoA, RoleA<RoleEnd>, NameB>;

create_fn_choose_mpst_cancel_multi_to_all_bundle!(
    done_from_a_to_all, more_from_a_to_all, =>
    Done, More, =>
    EndpointDoneA, EndpointForwardA, =>
    Branching0fromAtoB, =>
    RoleB, =>
    RoleCentral, RoleA, SessionMpstFour, 3, 2
);

// Functions
fn endpoint_central(s: EndpointCentral) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, 3);
    Ok(())
}

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    recurs_a(s, 1)
}

fn recurs_a(s: EndpointA, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_a_to_all(s)?;

            close_mpst_multi(s)
        }
        i => {
            let s = more_from_a_to_all(s)?;

            let s = send_mpst_a_to_b((), s)?;
            let ((), s) = recv_mpst_a_from_b(s)?;

            recurs_a(s, i - 1)
        }
    }
}

fn recurs_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_b_from_a, {
        Branching0fromAtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromAtoB::More(s) => {
            let ((), s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_a((), s)?;
            recurs_b(s)
        },
    })
}

fn main() {
    let (thread_central, thread_a, thread_b) = fork_mpst(endpoint_central, endpoint_a, recurs_b);

    thread_central.join().unwrap();
    thread_a.join().unwrap();
    thread_b.join().unwrap();
}
