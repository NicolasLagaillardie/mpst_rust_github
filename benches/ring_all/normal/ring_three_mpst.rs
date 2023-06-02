use crossbeam_channel::bounded;

use criterion::{black_box, Criterion};

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose, create_fn_choose_mpst_multi_to_all_bundle,
    create_multiple_normal_name_short, create_multiple_normal_role_short,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, offer, offer_mpst,
};

use std::error::Error;
use std::thread::{spawn, JoinHandle};

// use std::time::Duration;

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsThree, 3);

// Create new roles
// normal
create_multiple_normal_role_short!(A, B, C);

// Create new names
create_multiple_normal_name_short!(A, B, C);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, 1 | =>
    NameA, MeshedChannelsThree, 3
);

// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a, RoleA, 1 |
    send_mpst_b_to_c, RoleC, 2 | =>
    NameB, MeshedChannelsThree, 3
);

// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_b, RoleB, 2 | =>
    NameC, MeshedChannelsThree, 3
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 1 |
    recv_mpst_a_from_c, RoleC, 2 | =>
    NameA, MeshedChannelsThree, 3
);

// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 1 |
    recv_mpst_b_from_c, RoleC, 2 | =>
    NameB, MeshedChannelsThree, 3
);

// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_b, RoleB, 2 | =>
    NameC, MeshedChannelsThree, 3
);

// Types
// A
enum Branching0fromCtoA {
    Forward(MeshedChannelsThree<Send<(), End>, RecursAtoC, RoleB<RoleC<RoleEnd>>, NameA>),
    Backward(MeshedChannelsThree<Recv<(), End>, RecursAtoC, RoleB<RoleC<RoleEnd>>, NameA>),
    Done(MeshedChannelsThree<End, End, RoleEnd, NameA>),
}
type RecursAtoC = <Choose0fromCtoA as Session>::Dual;

// B
enum Branching0fromCtoB {
    Forward(
        MeshedChannelsThree<
            Recv<(), End>,
            Send<(), RecursBtoC>,
            RoleA<RoleC<RoleC<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannelsThree<
            Send<(), End>,
            Recv<(), RecursBtoC>,
            RoleC<RoleA<RoleC<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsThree<End, End, RoleEnd, NameB>),
}
type RecursBtoC = <Choose0fromCtoB as Session>::Dual;

// C
type Choose0fromCtoA = Send<Branching0fromCtoA, End>;
type Choose0fromCtoB = Send<Branching0fromCtoB, End>;
type EndpointDoneC = MeshedChannelsThree<End, End, RoleEnd, NameC>;
type EndpointForwardC =
    MeshedChannelsThree<Choose0fromCtoA, Recv<(), Choose0fromCtoB>, RoleB<RoleBroadcast>, NameC>;
type EndpointBackwardC =
    MeshedChannelsThree<Choose0fromCtoA, Send<(), Choose0fromCtoB>, RoleB<RoleBroadcast>, NameC>;

// Creating the MP sessions
type EndpointA = MeshedChannelsThree<End, RecursAtoC, RoleC<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsThree<End, RecursBtoC, RoleC<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsThree<Choose0fromCtoA, Choose0fromCtoB, RoleBroadcast, NameC>;

create_fn_choose_mpst_multi_to_all_bundle!(
    done_from_c_to_all, forward_from_c_to_all, backward_from_c_to_all, =>
    Done, Forward, Backward, =>
    EndpointDoneC, EndpointForwardC, EndpointBackwardC, =>
    Branching0fromCtoA, Branching0fromCtoB, =>
    NameA, NameB, =>
    NameC, MeshedChannelsThree, 3
);

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_c, {
        Branching0fromCtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromCtoA::Forward(s) => {
            let s = send_mpst_a_to_b((), s);
            endpoint_a(s)
        },
        Branching0fromCtoA::Backward(s) => {
            let (_, s) = recv_mpst_a_from_b(s)?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_c, {
        Branching0fromCtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromCtoB::Forward(s) => {
            let ((), s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_c((), s);
            endpoint_b(s)
        },
        Branching0fromCtoB::Backward(s) => {
            let ((), s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_a((), s);
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    recurs_c(s, LOOPS)
}

fn recurs_c(s: EndpointC, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_c_to_all(s);

            close_mpst_multi(s)
        }
        i if i % 2 == 0 => {
            let s = forward_from_c_to_all(s);

            let (_, s) = recv_mpst_c_from_b(s)?;

            recurs_c(s, i - 1)
        }
        i => {
            let s = backward_from_c_to_all(s);

            let s = send_mpst_c_to_b((), s);

            recurs_c(s, i - 1)
        }
    }
}

fn all_mpst() {
    let (thread_a, thread_b, thread_c) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
}



/////////////////////////

static LOOPS: i64 = 100;

pub fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("ring three protocol MPST {LOOPS}"), |b| {
        b.iter(all_mpst)
    });
}
