use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    broadcast_cancel, bundle_struct_fork_close_multi,
    create_fn_choose_mpst_cancel_multi_to_all_bundle, create_multiple_normal_role_short,
    create_recv_mpst_session_bundle, create_send_check_cancel_bundle, offer_cancel_mpst,
};

use std::error::Error;

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsFour, 4);

// Create new roles
// normal
create_multiple_normal_role_short!(Central, A, B, C);

// Create new send functions
// A
// A
create_send_check_cancel_bundle!(
    send_mpst_a_to_b, RoleB, 2 | =>
    RoleA, MeshedChannelsFour, 4
);
// B
create_send_check_cancel_bundle!(
    send_mpst_b_to_a, RoleA, 2 |
    send_mpst_b_to_c, RoleC, 3 | =>
    RoleB, MeshedChannelsFour, 4
);
// C
create_send_check_cancel_bundle!(
    send_mpst_c_to_b, RoleB, 3 | =>
    RoleC, MeshedChannelsFour, 4
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 2 |
    recv_mpst_a_from_c, RoleC, 3 | =>
    RoleA, MeshedChannelsFour, 4
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 2 |
    recv_mpst_b_from_c, RoleC, 3 | =>
    RoleB, MeshedChannelsFour, 4
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_b, RoleB, 3 | =>
    RoleC, MeshedChannelsFour, 4
);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;

// Types
// A
enum Branching0fromCtoA {
    Forward(MeshedChannelsFour<End, Send<(), End>, RecursAtoC, RoleB<RoleC<RoleEnd>>, NameA>),
    Backward(MeshedChannelsFour<End, Recv<(), End>, RecursAtoC, RoleB<RoleC<RoleEnd>>, NameA>),
    Done(MeshedChannelsFour<End, End, End, RoleEnd, NameA>),
}
type RecursAtoC = <Choose0fromCtoA as Session>::Dual;
// B
enum Branching0fromCtoB {
    Forward(
        MeshedChannelsFour<
            End,
            Recv<(), End>,
            Send<(), RecursBtoC>,
            RoleA<RoleC<RoleC<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannelsFour<
            End,
            Send<(), End>,
            Recv<(), RecursBtoC>,
            RoleC<RoleA<RoleC<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsFour<End, End, End, RoleEnd, NameB>),
}
type RecursBtoC = <Choose0fromCtoB as Session>::Dual;
// C
type Choose0fromCtoA = Send<(End, Branching0fromCtoA), End>;
type Choose0fromCtoB = Send<(End, Branching0fromCtoB), End>;
type EndpointDoneC = MeshedChannelsFour<End, End, End, RoleEnd, NameC>;
type EndpointForwardC =
    MeshedChannelsFour<End, Choose0fromCtoA, Recv<(), Choose0fromCtoB>, RoleB<RoleBroadcast>, NameC>;
type EndpointBackwardC =
    MeshedChannelsFour<End, Choose0fromCtoA, Send<(), Choose0fromCtoB>, RoleB<RoleBroadcast>, NameC>;

// Creating the MP sessions
type EndpointCentral = MeshedChannelsFour<End, End, End, RoleEnd, RoleCentral<RoleEnd>>;
type EndpointA = MeshedChannelsFour<End, End, RecursAtoC, RoleC<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsFour<End, End, RecursBtoC, RoleC<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsFour<End, Choose0fromCtoA, Choose0fromCtoB, RoleBroadcast, NameC>;

create_fn_choose_mpst_cancel_multi_to_all_bundle!(
    done_from_c_to_all, forward_from_c_to_all, backward_from_c_to_all, =>
    Done, Forward, Backward, =>
    EndpointDoneC, EndpointForwardC, EndpointBackwardC, =>
    Branching0fromCtoA, Branching0fromCtoB, =>
    RoleA, RoleB, =>
    RoleCentral, RoleC, MeshedChannelsFour, 4
);

fn endpoint_central(s: EndpointCentral) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, 4)
}

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_a_from_c, {
        Branching0fromCtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromCtoA::Forward(s) => {
            let s = send_mpst_a_to_b((), s)?;
            endpoint_a(s)
        },
        Branching0fromCtoA::Backward(s) => {
            let (_, s) = recv_mpst_a_from_b(s)?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_b_from_c, {
        Branching0fromCtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromCtoB::Forward(s) => {
            let ((), s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_c((), s)?;
            endpoint_b(s)
        },
        Branching0fromCtoB::Backward(s) => {
            let ((), s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_a((), s)?;
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    recurs_c(s, 100)
}

fn recurs_c(s: EndpointC, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_c_to_all(s)?;

            close_mpst_multi(s)
        }
        i if i % 2 == 0 => {
            let s = forward_from_c_to_all(s)?;

            let (_, s) = recv_mpst_c_from_b(s)?;

            recurs_c(s, i - 1)
        }
        i => {
            let s = backward_from_c_to_all(s)?;

            let s = send_mpst_c_to_b((), s)?;

            recurs_c(s, i - 1)
        }
    }
}

fn main() {
    let (thread_central, thread_a, thread_b, thread_c) =
        fork_mpst(endpoint_central, endpoint_a, endpoint_b, endpoint_c);

    assert!(thread_central.join().is_ok());
    assert!(thread_a.join().is_ok());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_ok());
}
