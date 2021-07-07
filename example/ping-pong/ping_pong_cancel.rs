use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi_cancel, choose_mpst_multi_to_all, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_mpst_cancel_bundle, offer_mpst,
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

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi_cancel!(close_mpst_multi, fork_mpst, MeshedChannelsTwo, 2);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
);

// Create new send functions
// A
create_send_mpst_cancel_bundle!(
    send_mpst_a_to_b, RoleB, 1 | =>
    RoleA, MeshedChannelsTwo, 2
);
// B
create_send_mpst_cancel_bundle!(
    send_mpst_b_to_a, RoleA, 1 | =>
    RoleB, MeshedChannelsTwo, 2
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 1 | =>
    RoleA, MeshedChannelsTwo, 2
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 1 | =>
    RoleB, MeshedChannelsTwo, 2
);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;

// Types
// A
type Choose0fromAtoB = <RecursBtoA as Session>::Dual;

// B
enum Branching0fromAtoB {
    More(MeshedChannelsTwo<Recv<(), Send<(), RecursBtoA>>, ThreeRoleA, NameB>),
    Done(MeshedChannelsTwo<End, RoleEnd, NameB>),
}
type ThreeRoleA = RoleA<RoleA<RoleA<RoleEnd>>>;
type RecursBtoA = Recv<Branching0fromAtoB, End>;

// Creating the MP sessions
type EndpointA = MeshedChannelsTwo<Choose0fromAtoB, RoleBroadcast, NameA>;
type EndpointB = MeshedChannelsTwo<RecursBtoA, RoleA<RoleEnd>, NameB>;

// Functions
fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    recurs_a(s, 1)
}

fn recurs_a(s: EndpointA, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromAtoB::Done, =>
                RoleB, =>
                RoleA,
                MeshedChannelsTwo,
                1
            );

            close_mpst_multi(s)
        }
        i => {
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromAtoB::More, =>
                RoleB, =>
                RoleA,
                MeshedChannelsTwo,
                1
            );

            let s = send_mpst_a_to_b((), s)?;
            let ((), s) = recv_mpst_a_from_b(s)?;

            recurs_a(s, i - 1)
        }
    }
}

fn recurs_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_a, {
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
    let (thread_a, thread_b) = fork_mpst(endpoint_a, recurs_b);

    thread_a.join().unwrap();
    thread_b.join().unwrap();
}
