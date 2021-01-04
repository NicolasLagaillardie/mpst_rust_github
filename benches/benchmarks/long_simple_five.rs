use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::{close, fork_with_thread_id, recv, send, End, Recv, Send, Session};
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::{
    bundle_fork_multi, choose, choose_mpst_multi_to_all, close_mpst, create_normal_role,
    create_recv_mpst_session, create_send_mpst_session, create_sessionmpst, offer, offer_mpst,
};

use std::error::Error;
use std::thread::JoinHandle;
use std::time::Duration;

// Create new SessionMpst for four participants
create_sessionmpst!(SessionMpstFive, 5);

// Create new roles
// normal
create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
create_normal_role!(RoleD, next_d, RoleDDual, next_d_dual);
create_normal_role!(RoleE, next_e, RoleEDual, next_e_dual);

// Create new send functions
// A
create_send_mpst_session!(
    send_mpst_a_to_b,
    RoleB,
    next_b,
    RoleA,
    SessionMpstFive,
    5,
    1
);
create_send_mpst_session!(
    send_mpst_a_to_c,
    RoleC,
    next_c,
    RoleA,
    SessionMpstFive,
    5,
    2
);
create_send_mpst_session!(
    send_mpst_a_to_d,
    RoleD,
    next_d,
    RoleA,
    SessionMpstFive,
    5,
    3
);
create_send_mpst_session!(
    send_mpst_a_to_e,
    RoleE,
    next_e,
    RoleA,
    SessionMpstFive,
    5,
    4
);
// B
create_send_mpst_session!(
    send_mpst_b_to_a,
    RoleA,
    next_a,
    RoleB,
    SessionMpstFive,
    5,
    1
);
create_send_mpst_session!(
    send_mpst_b_to_c,
    RoleC,
    next_c,
    RoleB,
    SessionMpstFive,
    5,
    2
);
create_send_mpst_session!(
    send_mpst_b_to_d,
    RoleD,
    next_d,
    RoleB,
    SessionMpstFive,
    5,
    3
);
create_send_mpst_session!(
    send_mpst_b_to_e,
    RoleE,
    next_e,
    RoleB,
    SessionMpstFive,
    5,
    4
);
// C
create_send_mpst_session!(
    send_mpst_c_to_a,
    RoleA,
    next_a,
    RoleC,
    SessionMpstFive,
    5,
    1
);
create_send_mpst_session!(
    send_mpst_c_to_b,
    RoleB,
    next_b,
    RoleC,
    SessionMpstFive,
    5,
    2
);
create_send_mpst_session!(
    send_mpst_c_to_d,
    RoleD,
    next_d,
    RoleC,
    SessionMpstFive,
    5,
    3
);
create_send_mpst_session!(
    send_mpst_c_to_e,
    RoleE,
    next_e,
    RoleC,
    SessionMpstFive,
    5,
    4
);
// D
create_send_mpst_session!(
    send_mpst_d_to_a,
    RoleA,
    next_a,
    RoleD,
    SessionMpstFive,
    5,
    1
);
create_send_mpst_session!(
    send_mpst_d_to_b,
    RoleB,
    next_b,
    RoleD,
    SessionMpstFive,
    5,
    2
);
create_send_mpst_session!(
    send_mpst_d_to_c,
    RoleC,
    next_c,
    RoleD,
    SessionMpstFive,
    5,
    3
);
create_send_mpst_session!(
    send_mpst_d_to_e,
    RoleE,
    next_e,
    RoleD,
    SessionMpstFive,
    5,
    4
);
// E
create_send_mpst_session!(
    send_mpst_e_to_a,
    RoleA,
    next_a,
    RoleE,
    SessionMpstFive,
    5,
    1
);
create_send_mpst_session!(
    send_mpst_e_to_b,
    RoleB,
    next_b,
    RoleE,
    SessionMpstFive,
    5,
    2
);
create_send_mpst_session!(
    send_mpst_e_to_c,
    RoleC,
    next_c,
    RoleE,
    SessionMpstFive,
    5,
    3
);
create_send_mpst_session!(
    send_mpst_e_to_d,
    RoleD,
    next_d,
    RoleE,
    SessionMpstFive,
    5,
    4
);

// Create new recv functions and related types
// A
create_recv_mpst_session!(
    recv_mpst_a_to_b,
    RoleB,
    next_b,
    RoleA,
    SessionMpstFive,
    5,
    1
);
create_recv_mpst_session!(
    recv_mpst_a_to_c,
    RoleC,
    next_c,
    RoleA,
    SessionMpstFive,
    5,
    2
);
create_recv_mpst_session!(
    recv_mpst_a_to_d,
    RoleD,
    next_d,
    RoleA,
    SessionMpstFive,
    5,
    3
);
create_recv_mpst_session!(
    recv_mpst_a_to_e,
    RoleE,
    next_e,
    RoleA,
    SessionMpstFive,
    5,
    4
);
// B
create_recv_mpst_session!(
    recv_mpst_b_to_a,
    RoleA,
    next_a,
    RoleB,
    SessionMpstFive,
    5,
    1
);
create_recv_mpst_session!(
    recv_mpst_b_to_c,
    RoleC,
    next_c,
    RoleB,
    SessionMpstFive,
    5,
    2
);
create_recv_mpst_session!(
    recv_mpst_b_to_d,
    RoleD,
    next_d,
    RoleB,
    SessionMpstFive,
    5,
    3
);
create_recv_mpst_session!(
    recv_mpst_b_to_e,
    RoleE,
    next_e,
    RoleB,
    SessionMpstFive,
    5,
    4
);
// C
create_recv_mpst_session!(
    recv_mpst_c_to_a,
    RoleA,
    next_a,
    RoleC,
    SessionMpstFive,
    5,
    1
);
create_recv_mpst_session!(
    recv_mpst_c_to_b,
    RoleB,
    next_b,
    RoleC,
    SessionMpstFive,
    5,
    2
);
create_recv_mpst_session!(
    recv_mpst_c_to_d,
    RoleD,
    next_d,
    RoleC,
    SessionMpstFive,
    5,
    3
);
create_recv_mpst_session!(
    recv_mpst_c_to_e,
    RoleE,
    next_e,
    RoleC,
    SessionMpstFive,
    5,
    4
);
// D
create_recv_mpst_session!(
    recv_mpst_d_to_a,
    RoleA,
    next_a,
    RoleD,
    SessionMpstFive,
    5,
    1
);
create_recv_mpst_session!(
    recv_mpst_d_to_b,
    RoleB,
    next_b,
    RoleD,
    SessionMpstFive,
    5,
    2
);
create_recv_mpst_session!(
    recv_mpst_d_to_c,
    RoleC,
    next_c,
    RoleD,
    SessionMpstFive,
    5,
    3
);
create_recv_mpst_session!(
    recv_mpst_d_to_e,
    RoleE,
    next_e,
    RoleD,
    SessionMpstFive,
    5,
    4
);
// E
create_recv_mpst_session!(
    recv_mpst_e_to_a,
    RoleA,
    next_a,
    RoleE,
    SessionMpstFive,
    5,
    1
);
create_recv_mpst_session!(
    recv_mpst_e_to_b,
    RoleB,
    next_b,
    RoleE,
    SessionMpstFive,
    5,
    2
);
create_recv_mpst_session!(
    recv_mpst_e_to_c,
    RoleC,
    next_c,
    RoleE,
    SessionMpstFive,
    5,
    3
);
create_recv_mpst_session!(
    recv_mpst_e_to_d,
    RoleD,
    next_d,
    RoleE,
    SessionMpstFive,
    5,
    4
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstFive, 5);

// Create fork function
bundle_fork_multi!(fork_mpst, fork_simple, SessionMpstFive, 5);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;
type NameE = RoleE<RoleEnd>;

// Types
// Binary
// A
enum BranchingEforA {
    More(
        SessionMpstFive<
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursAtoE>>,
            RoleE<RoleE<RoleB<RoleB<RoleC<RoleC<RoleD<RoleD<RoleE<RoleEnd>>>>>>>>>,
            NameA,
        >,
    ),
    Done(SessionMpstFive<End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoE = Recv<BranchingEforA, End>;
// B
enum BranchingEforB {
    More(
        SessionMpstFive<
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursBtoE>>,
            RoleE<RoleE<RoleA<RoleA<RoleC<RoleC<RoleD<RoleD<RoleE<RoleEnd>>>>>>>>>,
            NameB,
        >,
    ),
    Done(SessionMpstFive<End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoE = Recv<BranchingEforB, End>;
// C
enum BranchingEforC {
    More(
        SessionMpstFive<
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursCtoE>>,
            RoleE<RoleE<RoleA<RoleA<RoleB<RoleB<RoleD<RoleD<RoleE<RoleEnd>>>>>>>>>,
            NameC,
        >,
    ),
    Done(SessionMpstFive<End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoE = Recv<BranchingEforC, End>;
// D
enum BranchingEforD {
    More(
        SessionMpstFive<
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), RecursDtoE>>,
            RoleE<RoleE<RoleA<RoleA<RoleB<RoleB<RoleC<RoleC<RoleE<RoleEnd>>>>>>>>>,
            NameD,
        >,
    ),
    Done(SessionMpstFive<End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoE = Recv<BranchingEforD, End>;
// E
type ChooseEforAtoE = Send<BranchingEforA, End>;
type ChooseEforBtoE = Send<BranchingEforB, End>;
type ChooseEforCtoE = Send<BranchingEforC, End>;
type ChooseEforDtoE = Send<BranchingEforD, End>;

// Creating the MP sessions
type EndpointA = SessionMpstFive<End, End, End, RecursAtoE, RoleE<RoleEnd>, NameA>;
type EndpointB = SessionMpstFive<End, End, End, RecursBtoE, RoleE<RoleEnd>, NameB>;
type EndpointC = SessionMpstFive<End, End, End, RecursCtoE, RoleE<RoleEnd>, NameC>;
type EndpointD = SessionMpstFive<End, End, End, RecursDtoE, RoleE<RoleEnd>, NameD>;
type EndpointE = SessionMpstFive<
    ChooseEforAtoE,
    ChooseEforBtoE,
    ChooseEforCtoE,
    ChooseEforDtoE,
    RoleA<RoleB<RoleC<RoleD<RoleEnd>>>>,
    NameE,
>;

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_to_e, {
         BranchingEforA::Done(s) => {
            close_mpst_multi(s)
        },
         BranchingEforA::More(s) => {
            let (_, s) = recv_mpst_a_to_e(s)?;
            let s = send_mpst_a_to_e((), s);
            let (_, s) = recv_mpst_a_to_b(s)?;
            let s = send_mpst_a_to_b((), s);
            let (_, s) = recv_mpst_a_to_c(s)?;
            let s = send_mpst_a_to_c((), s);
            let (_, s) = recv_mpst_a_to_d(s)?;
            let s = send_mpst_a_to_d((), s);
            simple_five_endpoint_a(s)
        },
    })
}

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_to_e, {
         BranchingEforB::Done(s) => {
            close_mpst_multi(s)
        },
         BranchingEforB::More(s) => {
            let (_, s) = recv_mpst_b_to_e(s)?;
            let s = send_mpst_b_to_e((), s);
            let s = send_mpst_b_to_a((), s);
            let (_, s) = recv_mpst_b_to_a(s)?;
            let (_, s) = recv_mpst_b_to_c(s)?;
            let s = send_mpst_b_to_c((), s);
            let (_, s) = recv_mpst_b_to_d(s)?;
            let s = send_mpst_b_to_d((), s);
            simple_five_endpoint_b(s)
        },
    })
}

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_to_e, {
         BranchingEforC::Done(s) => {
            close_mpst_multi(s)
        },
         BranchingEforC::More(s) => {
            let (_, s) = recv_mpst_c_to_e(s)?;
            let s = send_mpst_c_to_e((), s);
            let s = send_mpst_c_to_a((), s);
            let (_, s) = recv_mpst_c_to_a(s)?;
            let s = send_mpst_c_to_b((), s);
            let (_, s) = recv_mpst_c_to_b(s)?;
            let (_, s) = recv_mpst_c_to_d(s)?;
            let s = send_mpst_c_to_d((), s);
            simple_five_endpoint_c(s)
        },
    })
}

fn simple_five_endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_to_e, {
         BranchingEforD::Done(s) => {
            close_mpst_multi(s)
        },
         BranchingEforD::More(s) => {
            let (_, s) = recv_mpst_d_to_e(s)?;
            let s = send_mpst_d_to_e((), s);
            let s = send_mpst_d_to_a((), s);
            let (_, s) = recv_mpst_d_to_a(s)?;
            let s = send_mpst_d_to_b((), s);
            let (_, s) = recv_mpst_d_to_b(s)?;
            let s = send_mpst_d_to_c((), s);
            let (_, s) = recv_mpst_d_to_c(s)?;
            simple_five_endpoint_d(s)
        },
    })
}

fn simple_five_endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    recurs_d(s, SIZE)
}

fn recurs_d(s: EndpointE, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_e_to_a,
                send_mpst_e_to_b,
                send_mpst_e_to_c,
                send_mpst_e_to_d, =>
                 BranchingEforA::Done,
                 BranchingEforB::Done,
                 BranchingEforC::Done,
                 BranchingEforD::Done, =>
                RoleA,
                RoleB,
                RoleC,
                RoleD, =>
                RoleE,
                 SessionMpstFive,
                5
            );

            close_mpst_multi(s)
        }
        i => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_e_to_a,
                send_mpst_e_to_b,
                send_mpst_e_to_c,
                send_mpst_e_to_d, =>
                 BranchingEforA::More,
                 BranchingEforB::More,
                 BranchingEforC::More,
                 BranchingEforD::More, =>
                RoleA,
                RoleB,
                RoleC,
                RoleD, =>
                RoleE,
                 SessionMpstFive,
                5
            );

            let s = send_mpst_e_to_a((), s);
            let (_, s) = recv_mpst_e_to_a(s)?;
            let s = send_mpst_e_to_b((), s);
            let (_, s) = recv_mpst_e_to_b(s)?;
            let s = send_mpst_e_to_c((), s);
            let (_, s) = recv_mpst_e_to_c(s)?;
            let s = send_mpst_e_to_d((), s);
            let (_, s) = recv_mpst_e_to_d(s)?;

            recurs_d(s, i - 1)
        }
    }
}

fn all_mpst() -> Result<(), Box<dyn Error>> {
    let (thread_a, thread_b, thread_c, thread_d, thread_e) = fork_mpst(
        black_box(simple_five_endpoint_a),
        black_box(simple_five_endpoint_b),
        black_box(simple_five_endpoint_c),
        black_box(simple_five_endpoint_d),
        black_box(simple_five_endpoint_e),
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();

    Ok(())
}

/////////////////////////
// B
enum BinaryA {
    More(Recv<(), Send<(), RecursA>>),
    Done(End),
}
type RecursA = Recv<BinaryA, End>;
fn binary_b_to_c(s: RecursA) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        BinaryA::More(s) => {
            let (_, s) = recv(s)?;
            let s = send((), s);
            binary_b_to_c(s)
        },
        BinaryA::Done(s) => {
            close(s)
        },
    })
}

// C
type RecursB = <RecursA as Session>::Dual;
fn binary_c_to_b(s: RecursB, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose!(BinaryA::Done, s);
            close(s)
        }
        i => {
            let s = choose!(BinaryA::More, s);
            let s = send((), s);
            let (_, s) = recv(s)?;
            binary_c_to_b(s, i - 1)
        }
    }
}

fn all_binaries(index: i64) -> Result<(), Box<dyn Error>> {
    let (thread_b_to_c_0, s_b_to_c_0): (JoinHandle<()>, RecursB) =
        fork_with_thread_id(black_box(binary_b_to_c));

    ////////////////////

    let (thread_b_to_c_1, s_b_to_c_1): (JoinHandle<()>, RecursB) =
        fork_with_thread_id(black_box(binary_b_to_c));

    ////////////////////

    let (thread_b_to_c_2, s_b_to_c_2): (JoinHandle<()>, RecursB) =
        fork_with_thread_id(black_box(binary_b_to_c));

    ////////////////////

    let (thread_b_to_c_3, s_b_to_c_3): (JoinHandle<()>, RecursB) =
        fork_with_thread_id(black_box(binary_b_to_c));

    ////////////////////

    let (thread_b_to_c_4, s_b_to_c_4): (JoinHandle<()>, RecursB) =
        fork_with_thread_id(black_box(binary_b_to_c));

    ////////////////////

    let (thread_b_to_c_5, s_b_to_c_5): (JoinHandle<()>, RecursB) =
        fork_with_thread_id(black_box(binary_b_to_c));

    ////////////////////
    let (thread_b_to_c_6, s_b_to_c_6): (JoinHandle<()>, RecursB) =
        fork_with_thread_id(black_box(binary_b_to_c));

    ////////////////////

    let (thread_b_to_c_7, s_b_to_c_7): (JoinHandle<()>, RecursB) =
        fork_with_thread_id(black_box(binary_b_to_c));

    ////////////////////

    let (thread_b_to_c_8, s_b_to_c_8): (JoinHandle<()>, RecursB) =
        fork_with_thread_id(black_box(binary_b_to_c));

    ////////////////////

    let (thread_b_to_c_9, s_b_to_c_9): (JoinHandle<()>, RecursB) =
        fork_with_thread_id(black_box(binary_b_to_c));

    ////////////////////

    binary_c_to_b(black_box(s_b_to_c_0), index).unwrap();
    thread_b_to_c_0.join().unwrap();

    binary_c_to_b(black_box(s_b_to_c_1), index).unwrap();
    thread_b_to_c_1.join().unwrap();

    binary_c_to_b(black_box(s_b_to_c_2), index).unwrap();
    thread_b_to_c_2.join().unwrap();

    binary_c_to_b(black_box(s_b_to_c_3), index).unwrap();
    thread_b_to_c_3.join().unwrap();

    binary_c_to_b(black_box(s_b_to_c_4), index).unwrap();
    thread_b_to_c_4.join().unwrap();

    binary_c_to_b(black_box(s_b_to_c_5), index).unwrap();
    thread_b_to_c_5.join().unwrap();

    binary_c_to_b(black_box(s_b_to_c_6), index).unwrap();
    thread_b_to_c_6.join().unwrap();

    binary_c_to_b(black_box(s_b_to_c_7), index).unwrap();
    thread_b_to_c_7.join().unwrap();

    binary_c_to_b(black_box(s_b_to_c_8), index).unwrap();
    thread_b_to_c_8.join().unwrap();

    binary_c_to_b(black_box(s_b_to_c_9), index).unwrap();
    thread_b_to_c_9.join().unwrap();

    Ok(())
}

/////////////////////////

static SIZE: i64 = 1000;

fn long_simple_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("long four simple protocol MPST {}", SIZE), |b| {
        b.iter(|| all_mpst())
    });
}

fn long_simple_protocol_binary(c: &mut Criterion) {
    c.bench_function(&format!("long four simple protocol binary {}", SIZE), |b| {
        b.iter(|| all_binaries(SIZE))
    });
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(350, 0))
}

criterion_group! {
    name = long_five_simple_protocols;
    config = long_warmup();
    targets = long_simple_protocol_mpst, long_simple_protocol_binary
}
criterion_main!(long_five_simple_protocols);
