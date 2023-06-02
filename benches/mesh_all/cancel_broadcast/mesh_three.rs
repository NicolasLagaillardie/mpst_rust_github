use criterion::{black_box, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    broadcast_cancel, bundle_struct_fork_close_multi,
    create_fn_choose_mpst_cancel_multi_to_all_bundle, create_multiple_normal_name_short,
    create_multiple_normal_role_short, create_recv_mpst_session_bundle,
    create_send_check_cancel_bundle, offer_cancel_mpst,
};

use std::error::Error;

// use std::time::Duration;

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsFour, 4);

// Create new roles
// normal
create_multiple_normal_role_short!(Central, A, B, C);

// Create new names
create_multiple_normal_name_short!(Central, A, B, C);

// Create new send functions
// A
create_send_check_cancel_bundle!(
    send_mpst_a_to_b, RoleB, 2 |
    send_mpst_a_to_c, RoleC, 3 | =>
    NameA, MeshedChannelsFour, 4
);

// B
create_send_check_cancel_bundle!(
    send_mpst_b_to_a, RoleA, 2 |
    send_mpst_b_to_c, RoleC, 3 | =>
    NameB, MeshedChannelsFour, 4
);

// C
create_send_check_cancel_bundle!(
    send_mpst_c_to_a, RoleA, 2 |
    send_mpst_c_to_b, RoleB, 3 | =>
    NameC, MeshedChannelsFour, 4
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 2 |
    recv_mpst_a_from_c, RoleC, 3 | =>
    NameA, MeshedChannelsFour, 4
);

// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 2 |
    recv_mpst_b_from_c, RoleC, 3 | =>
    NameB, MeshedChannelsFour, 4
);

// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a, RoleA, 2 |
    recv_mpst_c_from_b, RoleB, 3 | =>
    NameC, MeshedChannelsFour, 4
);

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
    More(
        MeshedChannelsFour<
            End,
            RS,
            Recv<(), Send<(), RecursAtoC>>,
            R2C<R2B<RoleC<RoleEnd>>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsFour<End, End, End, RoleEnd, NameA>),
}
type RecursAtoC = Recv<(End, Branching0fromCtoA), End>;

// B
enum Branching0fromCtoB {
    More(
        MeshedChannelsFour<
            End,
            SR,
            Recv<(), Send<(), RecursBtoC>>,
            R2C<R2A<RoleC<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsFour<End, End, End, RoleEnd, NameB>),
}
type RecursBtoC = Recv<(End, Branching0fromCtoB), End>;

// C
type Choose0fromCtoA = <RecursAtoC as Session>::Dual;
type Choose0fromCtoB = <RecursBtoC as Session>::Dual;
type EndpointDoneC = MeshedChannelsFour<End, End, End, RoleEnd, NameC>;
type EndpointMoreC = MeshedChannelsFour<
    End,
    Send<(), Recv<(), Choose0fromCtoA>>,
    Send<(), Recv<(), Choose0fromCtoB>>,
    R2A<R2B<RoleBroadcast>>,
    NameC,
>;

// Creating the MP sessions
type EndpointCentral = MeshedChannelsFour<End, End, End, RoleEnd, NameCentral>;
type EndpointA = MeshedChannelsFour<End, End, RecursAtoC, RoleC<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsFour<End, End, RecursBtoC, RoleC<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsFour<End, Choose0fromCtoA, Choose0fromCtoB, RoleBroadcast, NameC>;

create_fn_choose_mpst_cancel_multi_to_all_bundle!(
    done_from_c_to_all, more_from_c_to_all, =>
    Done, More, =>
    EndpointDoneC, EndpointMoreC, =>
    Branching0fromCtoA, Branching0fromCtoB, =>
    NameA, NameB, =>
    NameCentral, NameC, MeshedChannelsFour, 4
);

fn endpoint_central(s: EndpointCentral) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, 4)
}

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_a_from_c, {
        Branching0fromCtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromCtoA::More(s) => {
            let (_, s) = recv_mpst_a_from_c(s)?;
            let s = send_mpst_a_to_c((), s)?;
            let (_, s) = recv_mpst_a_from_b(s)?;
            let s = send_mpst_a_to_b((), s)?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_b_from_c, {
        Branching0fromCtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromCtoB::More(s) => {
            let (_, s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_c((), s)?;
            let s = send_mpst_b_to_a((), s)?;
            let (_, s) = recv_mpst_b_from_a(s)?;
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
            let s = done_from_c_to_all(s)?;

            close_mpst_multi(s)
        }
        i => {
            let s = more_from_c_to_all(s)?;

            let s = send_mpst_c_to_a((), s)?;
            let (_, s) = recv_mpst_c_from_a(s)?;
            let s = send_mpst_c_to_b((), s)?;
            let (_, s) = recv_mpst_c_from_b(s)?;

            recurs_c(s, i - 1)
        }
    }
}

fn all_mpst() {
    let (thread_central, thread_a, thread_b, thread_c) = fork_mpst(
        black_box(endpoint_central),
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
    );

    thread_central.join().unwrap();
    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn mesh_protocol_mpst(c: &mut Criterion) {
    c.bench_function(
        &format!("mesh three cancel broadcast protocol MPST {LOOPS}"),
        |b| b.iter(all_mpst),
    );
}


/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.1).sample_size(10000);
    targets = mesh_protocol_mpst,
}

criterion_main! {
    bench
}

