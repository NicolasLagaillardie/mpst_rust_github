#![allow(clippy::type_complexity, clippy::too_many_arguments, clippy::large_enum_variant)]
    
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    broadcast_cancel, bundle_struct_fork_close_multi,
    create_fn_choose_mpst_cancel_multi_to_all_bundle, create_multiple_normal_role_short,
    create_recv_mpst_session_bundle, create_send_check_cancel_bundle, offer_cancel_mpst,
};

use std::error::Error;

// Create the new MeshedChannels for four participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsFive, 5);

// Create new roles
// normal
create_multiple_normal_role_short!(Central, A, B, C, D);

// Create new send functions
// A
create_send_check_cancel_bundle!(
    send_mpst_a_to_b, RoleB, 2 | =>
    RoleA, MeshedChannelsFive, 5
);
// B
create_send_check_cancel_bundle!(
    send_mpst_b_to_a, RoleA, 2 |
    send_mpst_b_to_c, RoleC, 3 | =>
    RoleB, MeshedChannelsFive, 5
);
// C
create_send_check_cancel_bundle!(
    send_mpst_c_to_b, RoleB, 3 |
    send_mpst_c_to_d, RoleD, 4 | =>
    RoleC, MeshedChannelsFive, 5
);
// D
create_send_check_cancel_bundle!(
    send_mpst_d_to_c, RoleC, 4 | =>
    RoleD, MeshedChannelsFive, 5
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 2 |
    recv_mpst_a_from_d, RoleD, 4 | =>
    RoleA, MeshedChannelsFive, 5
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 2 |
    recv_mpst_b_from_c, RoleC, 3 |
    recv_mpst_b_from_d, RoleD, 4 | =>
    RoleB, MeshedChannelsFive, 5
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_b, RoleB, 3 |
    recv_mpst_c_from_d, RoleD, 4 | =>
    RoleC, MeshedChannelsFive, 5
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_c, RoleC, 4 | =>
    RoleD, MeshedChannelsFive, 5
);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;

// Types
// A
enum Branching0fromDtoA {
    Forward(MeshedChannelsFive<End, Send<(), End>, End, RecursAtoD, RoleB<RoleD<RoleEnd>>, NameA>),
    Backward(MeshedChannelsFive<End, Recv<(), End>, End, RecursAtoD, RoleB<RoleD<RoleEnd>>, NameA>),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoD = <Choose0fromDtoA as Session>::Dual;
// B
enum Branching0fromDtoB {
    Forward(
        MeshedChannelsFive<
            End,
            Recv<(), End>,
            Send<(), End>,
            RecursBtoD,
            RoleA<RoleC<RoleD<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannelsFive<
            End,
            Send<(), End>,
            Recv<(), End>,
            RecursBtoD,
            RoleC<RoleA<RoleD<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoD = <Choose0fromDtoB as Session>::Dual;
// C
enum Branching0fromDtoC {
    Forward(
        MeshedChannelsFive<
            End,
            End,
            Recv<(), End>,
            Send<(), RecursCtoD>,
            RoleB<RoleD<RoleD<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannelsFive<
            End,
            End,
            Send<(), End>,
            Recv<(), RecursCtoD>,
            RoleD<RoleB<RoleD<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoD = <Choose0fromDtoC as Session>::Dual;
// D
type Choose0fromDtoA = Send<(End, Branching0fromDtoA), End>;
type Choose0fromDtoB = Send<(End, Branching0fromDtoB), End>;
type Choose0fromDtoC = Send<(End, Branching0fromDtoC), End>;
type EndpointDoneD = MeshedChannelsFive<End, End, End, End, RoleEnd, NameD>;
type EndpointForwardD = MeshedChannelsFive<
    End,
    Choose0fromDtoA,
    Choose0fromDtoB,
    Recv<(), Choose0fromDtoC>,
    RoleC<RoleBroadcast>,
    NameD,
>;
type EndpointBackwardD = MeshedChannelsFive<
    End,
    Choose0fromDtoA,
    Choose0fromDtoB,
    Send<(), Choose0fromDtoC>,
    RoleC<RoleBroadcast>,
    NameD,
>;

// Creating the MP sessions
type EndpointCentral = MeshedChannelsFive<End, End, End, End, RoleEnd, RoleCentral<RoleEnd>>;
type EndpointA = MeshedChannelsFive<End, End, End, RecursAtoD, RoleD<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsFive<End, End, End, RecursBtoD, RoleD<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsFive<End, End, End, RecursCtoD, RoleD<RoleEnd>, NameC>;
type EndpointD =
    MeshedChannelsFive<End, Choose0fromDtoA, Choose0fromDtoB, Choose0fromDtoC, RoleBroadcast, NameD>;

create_fn_choose_mpst_cancel_multi_to_all_bundle!(
    done_from_d_to_all, forward_from_d_to_all, backward_from_d_to_all, =>
    Done, Forward, Backward, =>
    EndpointDoneD, EndpointForwardD, EndpointBackwardD, =>
    Branching0fromDtoA,
    Branching0fromDtoB,
    Branching0fromDtoC, =>
    RoleA, RoleB, RoleC, =>
    RoleCentral, RoleD, MeshedChannelsFive, 5
);

fn endpoint_central(s: EndpointCentral) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, 5)
}

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_a_from_d, {
        Branching0fromDtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromDtoA::Forward(s) => {
            let s = send_mpst_a_to_b((), s)?;
            endpoint_a(s)
        },
        Branching0fromDtoA::Backward(s) => {
            let (_, s) = recv_mpst_a_from_b(s)?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_b_from_d, {
        Branching0fromDtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromDtoB::Forward(s) => {
            let ((), s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_c((), s)?;
            endpoint_b(s)
        },
        Branching0fromDtoB::Backward(s) => {
            let ((), s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_a((), s)?;
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_c_from_d, {
        Branching0fromDtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromDtoC::Forward(s) => {
            let ((), s) = recv_mpst_c_from_b(s)?;
            let s = send_mpst_c_to_d((), s)?;
            endpoint_c(s)
        },
        Branching0fromDtoC::Backward(s) => {
            let ((), s) = recv_mpst_c_from_d(s)?;
            let s = send_mpst_c_to_b((), s)?;
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    recurs_d(s, 100)
}

fn recurs_d(s: EndpointD, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_d_to_all(s)?;

            close_mpst_multi(s)
        }
        i if i % 2 == 0 => {
            let s = forward_from_d_to_all(s)?;

            let (_, s) = recv_mpst_d_from_c(s)?;

            recurs_d(s, i - 1)
        }
        i => {
            let s = backward_from_d_to_all(s)?;

            let s = send_mpst_d_to_c((), s)?;

            recurs_d(s, i - 1)
        }
    }
}

fn main() {
    let (thread_central, thread_a, thread_b, thread_c, thread_d) = fork_mpst(
        endpoint_central,
        endpoint_a,
        endpoint_b,
        endpoint_c,
        endpoint_d,
    );

    assert!(thread_central.join().is_ok());
    assert!(thread_a.join().is_ok());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_ok());
    assert!(thread_d.join().is_ok());
}
