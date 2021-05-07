#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    broadcast_cancel, bundle_struct_fork_close_multi,
    create_fn_choose_mpst_cancel_multi_to_all_bundle, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_check_cancel_bundle, offer_cancel_mpst,
};

use std::error::Error;
use std::time::Duration;

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
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstFour, 4);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
    RoleC, RoleCDual |
    RoleCentral, RoleCentralDual |
);

// Create new send functions
// A
create_send_check_cancel_bundle!(
    send_mpst_a_to_b, RoleB, 2 | =>
    RoleA, SessionMpstFour, 4
);
// B
create_send_check_cancel_bundle!(
    send_mpst_b_to_a, RoleA, 2 | =>
    RoleB, SessionMpstFour, 4
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 2 | =>
    RoleA, SessionMpstFour, 4
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 2 | =>
    RoleB, SessionMpstFour, 4
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a, RoleA, 2 | =>
    RoleC, SessionMpstFour, 4
);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;

// Types
// A
type Choose0fromAtoB = <RecursBtoA as Session>::Dual;
type Choose0fromAtoC = <RecursCtoA as Session>::Dual;
// B
enum Branching0fromAtoB {
    More(
        SessionMpstFour<
            End,
            Recv<(), Send<(), RecursBtoA>>,
            End,
            RoleA<RoleA<RoleA<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(SessionMpstFour<End, End, End, RoleEnd, NameB>),
}
type RecursBtoA = Recv<(End, Branching0fromAtoB), End>;
// C
enum Branching0fromAtoC {
    More(SessionMpstFour<End, RecursCtoA, End, RoleA<RoleEnd>, NameC>),
    Done(SessionMpstFour<End, End, End, RoleEnd, NameC>),
}
type RecursCtoA = Recv<(End, Branching0fromAtoC), End>;

// Creating the MP sessions

type EndpointDoneA = SessionMpstFour<End, End, End, RoleEnd, NameA>;
type EndpointForwardA = SessionMpstFour<
    End,
    Send<(), Recv<(), Choose0fromAtoB>>,
    Choose0fromAtoC,
    RoleB<RoleB<RoleBroadcast>>,
    NameA,
>;

type EndpointCentral = SessionMpstFour<End, End, End, RoleEnd, RoleCentral<RoleEnd>>;
type EndpointA = SessionMpstFour<End, Choose0fromAtoB, Choose0fromAtoC, RoleBroadcast, NameA>;
type EndpointB = SessionMpstFour<End, RecursBtoA, End, RoleA<RoleEnd>, NameB>;
type EndpointC = SessionMpstFour<End, RecursCtoA, End, RoleA<RoleEnd>, NameC>;

create_fn_choose_mpst_cancel_multi_to_all_bundle!(
    done_from_a_to_all, more_from_a_to_all, =>
    Done, More, =>
    EndpointDoneA, EndpointForwardA, =>
    Branching0fromAtoB, Branching0fromAtoC, =>
    RoleB, RoleC, =>
    RoleCentral, RoleA, SessionMpstFour, 4, 2
);

// Functions
fn endpoint_central(s: EndpointCentral) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, RoleCentral, 4);
    Ok(())
}

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    recurs_a(s, SIZE)
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

fn recurs_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_c_from_a, {
        Branching0fromAtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromAtoC::More(s) => {
            recurs_c(s)
        },
    })
}

fn all_mpst() -> Result<(), Box<dyn std::any::Any + std::marker::Send>> {
    let (thread_central, thread_a, thread_b, thread_c) = fork_mpst(
        black_box(endpoint_central),
        black_box(endpoint_a),
        black_box(recurs_b),
        black_box(recurs_c),
    );

    thread_central.join()?;
    thread_a.join()?;
    thread_b.join()?;
    thread_c.join()?;

    Ok(())
}

/////////////////////////

static SIZE: i64 = 110;

fn ping_pong_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("ping pong cancel protocol MPST {}", SIZE), |b| {
        b.iter(|| all_mpst())
    });
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(1800, 0))
}

criterion_group! {
    name = ping_pong;
    // config = long_warmup();
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = ping_pong_protocol_mpst
}
criterion_main!(ping_pong);
