use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, create_fn_choose_mpst_multi_to_all_bundle,
    create_multiple_normal_role_short, create_recv_mpst_session_bundle,
    create_send_mpst_session_bundle, offer_mpst,
};

use std::error::Error;

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsTwo, 2);

// Create new roles
// normal
create_multiple_normal_role_short!(A, B);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, 1 | =>
    RoleA, MeshedChannelsTwo, 2
);
// B
create_send_mpst_session_bundle!(
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
enum Branching0fromBtoA {
    More(MeshedChannelsTwo<Recv<(), Send<(), RecursAtoB>>, RoleB<RoleB<RoleB<RoleEnd>>>, NameA>),
    Done(MeshedChannelsTwo<End, RoleEnd, NameA>),
}
type RecursAtoB = Recv<Branching0fromBtoA, End>;
// C
type Choose0fromBtoA = Send<Branching0fromBtoA, End>;
type EndpointDoneB = MeshedChannelsTwo<End, RoleEnd, NameB>;
type EndpointMoreB =
    MeshedChannelsTwo<Send<(), Recv<(), Choose0fromBtoA>>, RoleA<RoleA<RoleBroadcast>>, NameB>;

// Creating the MP sessions
type EndpointA = MeshedChannelsTwo<RecursAtoB, RoleB<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsTwo<Choose0fromBtoA, RoleBroadcast, NameB>;

create_fn_choose_mpst_multi_to_all_bundle!(
    done_from_b_to_all, more_from_b_to_all, =>
    Done, More, =>
    EndpointDoneB, EndpointMoreB, =>
    Branching0fromBtoA, =>
    RoleA, =>
    RoleB, MeshedChannelsTwo, 2
);

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_b, {
        Branching0fromBtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromBtoA::More(s) => {
            let (_, s) = recv_mpst_a_from_b(s)?;
            let s = send_mpst_a_to_b((), s);
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    recurs_b(s, 100)
}

fn recurs_b(s: EndpointB, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_b_to_all(s);

            close_mpst_multi(s)
        }
        i => {
            let s = more_from_b_to_all(s);

            let s = send_mpst_b_to_a((), s);
            let (_, s) = recv_mpst_b_from_a(s)?;

            recurs_b(s, i - 1)
        }
    }
}

fn main() {
    let (thread_a, thread_b) = fork_mpst(endpoint_a, endpoint_b);

    assert!(thread_a.join().is_ok());
    assert!(thread_b.join().is_ok());
}
