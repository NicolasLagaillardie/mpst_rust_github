use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    broadcast_cancel, bundle_struct_fork_close_multi, choose_mpst_multi_cancel_to_all,
    create_normal_role, create_recv_mpst_session_bundle, create_send_check_cancel_bundle,
    offer_cancel_mpst, send_cancel,
};

use std::error::Error;

// Create new MeshedChannels for four participants
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsFour, 4);

// Create new roles
// normal
create_normal_role!(RoleA, RoleADual);
create_normal_role!(RoleB, RoleBDual);
create_normal_role!(RoleC, RoleCDual);
create_normal_role!(RoleD, RoleDDual);

// Create new send functions
// B
create_send_check_cancel_bundle!(
    send_check_b_to_c, RoleC, 2 |
    send_check_b_to_d, RoleD, 3 | =>
    RoleB, MeshedChannelsFour, 4
);
// C
create_send_check_cancel_bundle!(
    send_check_c_to_b, RoleB, 2 |
    send_check_c_to_d, RoleD, 3 | =>
    RoleC, MeshedChannelsFour, 4
);
// D
create_send_check_cancel_bundle!(
    send_check_d_to_b, RoleB, 2 |
    send_check_d_to_c, RoleC, 3 | =>
    RoleD, MeshedChannelsFour, 4
);

// Create new recv functions and related types
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_c, RoleC, 2 |
    recv_mpst_b_from_d, RoleD, 3 | =>
    RoleB, MeshedChannelsFour, 4
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_b, RoleB, 2 |
    recv_mpst_c_from_d, RoleD, 3 | =>
    RoleC, MeshedChannelsFour, 4
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_b, RoleB, 2 |
    recv_mpst_d_from_c, RoleC, 3 | =>
    RoleD, MeshedChannelsFour, 4
);

send_cancel!(cancel_mpst, RoleB, MeshedChannelsFour, 4, "Session dropped");

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;

// Types
// Send/Recv
type RS<S> = Recv<(), Send<(), S>>;
type SR<S> = Send<(), Recv<(), S>>;
// Roles
type R2B<R> = RoleB<RoleB<R>>;
type R2C<R> = RoleC<RoleC<R>>;
type R2D<R> = RoleD<RoleD<R>>;
// B
enum Branching0fromDtoB {
    More(MeshedChannelsFour<End, SR<End>, RS<RecursBtoD>, R2D<R2C<RoleD<RoleEnd>>>, NameB>),
    Done(MeshedChannelsFour<End, End, End, RoleEnd, NameB>),
}
type RecursBtoD = Recv<(End, Branching0fromDtoB), End>;
// C
enum Branching0fromDtoC {
    More(MeshedChannelsFour<End, RS<End>, RS<RecursCtoD>, R2D<R2B<RoleD<RoleEnd>>>, NameC>),
    Done(MeshedChannelsFour<End, End, End, RoleEnd, NameC>),
}
type RecursCtoD = Recv<(End, Branching0fromDtoC), End>;
// D
type Choose0fromDtoA = End;
type Choose0fromDtoB = Send<(End, Branching0fromDtoB), End>; // TODO: Remove the need of tuple with a
type Choose0fromDtoC = Send<(End, Branching0fromDtoC), End>; // End which is forwaded to A

// Creating the MP sessions
type EndpointA = MeshedChannelsFour<End, End, End, RoleEnd, NameA>;
type EndpointB = MeshedChannelsFour<End, End, RecursBtoD, RoleD<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsFour<End, End, RecursCtoD, RoleD<RoleEnd>, NameC>;
type EndpointD =
    MeshedChannelsFour<Choose0fromDtoA, Choose0fromDtoB, Choose0fromDtoC, RoleBroadcast, NameD>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, 4)
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_b_from_d, {
        Branching0fromDtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromDtoB::More(s) => {
            // let (_, s) = recv_mpst_b_from_d(s)?;
            // let s = send_check_b_to_d((), s)?;
            // let s = send_check_b_to_c((), s)?;
            // let (_, s) = recv_mpst_b_from_c(s)?;
            // endpoint_b(s)

            cancel_mpst(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_c_from_d, {
        Branching0fromDtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromDtoC::More(s) => {
            let (_, s) = recv_mpst_c_from_d(s)?;
            let s = send_check_c_to_d((), s)?;
            let (_, s) = recv_mpst_c_from_b(s)?;
            let s = send_check_c_to_b((), s)?;
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    recurs_d(s, SIZE)
}

fn recurs_d(s: EndpointD, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_multi_cancel_to_all!(
                s,
                Branching0fromDtoB::Done,
                Branching0fromDtoC::Done, =>
                RoleB,
                RoleC, =>
                RoleA,
                RoleD,
                MeshedChannelsFour,
                4
            );

            close_mpst_multi(s)
        }
        i => {
            let s = choose_mpst_multi_cancel_to_all!(
                s,
                Branching0fromDtoB::More,
                Branching0fromDtoC::More, =>
                RoleB,
                RoleC, =>
                RoleA,
                RoleD,
                MeshedChannelsFour,
                4
            );

            let s = send_check_d_to_b((), s)?;
            let (_, s) = recv_mpst_d_from_b(s)?;
            let s = send_check_d_to_c((), s)?;
            let (_, s) = recv_mpst_d_from_c(s)?;

            recurs_d(s, i - 1)
        }
    }
}

pub fn main() {
    let (thread_a, thread_b, thread_c, thread_d) =
        fork_mpst(endpoint_a, endpoint_b, endpoint_c, endpoint_d);

    assert!(thread_a.join().is_err());
    assert!(thread_b.join().is_err());
    assert!(thread_c.join().is_err());
    assert!(thread_d.join().is_err());
}

/////////////////////////

static SIZE: i64 = 15;
