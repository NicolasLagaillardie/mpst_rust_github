#![allow(dead_code)]

use crossbeam_channel::bounded;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::{close, fork_with_thread_id, recv, send, End, Recv, Send, Session};
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::{
    bundle_fork_multi, choose, choose_mpst_multi_to_all, close_mpst, create_normal_role,
    create_recv_mpst_session, create_recv_mpst_session_bundle, create_send_mpst_session,
    create_send_mpst_session_bundle, create_sessionmpst, offer, offer_mpst,
};

use std::error::Error;
use std::thread::{spawn, JoinHandle};
use std::time::Duration;

// Create new SessionMpst for four participants
create_sessionmpst!(SessionMpstSix, 6);

// Create new roles
// normal
create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
create_normal_role!(RoleD, next_d, RoleDDual, next_d_dual);
create_normal_role!(RoleE, next_e, RoleEDual, next_e_dual);
create_normal_role!(RoleF, next_f, RoleFDual, next_f_dual);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b,
    RoleB,
    next_b,
    1, |
    send_mpst_a_to_c,
    RoleC,
    next_c,
    2, |
    send_mpst_a_to_d,
    RoleD,
    next_d,
    3, |
    send_mpst_a_to_e,
    RoleE,
    next_e,
    4, |
    send_mpst_a_to_f,
    RoleF,
    next_f,
    5, | =>
    RoleA,
    SessionMpstSix,
    6
);
// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_b_to_c,
    RoleC,
    next_c,
    2, |
    send_mpst_b_to_d,
    RoleD,
    next_d,
    3, |
    send_mpst_b_to_e,
    RoleE,
    next_e,
    4, |
    send_mpst_b_to_f,
    RoleF,
    next_f,
    5, | =>
    RoleB,
    SessionMpstSix,
    6
);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_c_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_c_to_d,
    RoleD,
    next_d,
    3, |
    send_mpst_c_to_e,
    RoleE,
    next_e,
    4, |
    send_mpst_c_to_f,
    RoleF,
    next_f,
    5, | =>
    RoleC,
    SessionMpstSix,
    6
);
// D
create_send_mpst_session_bundle!(
    send_mpst_d_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_d_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_d_to_c,
    RoleC,
    next_c,
    3, |
    send_mpst_d_to_e,
    RoleE,
    next_e,
    4, |
    send_mpst_d_to_f,
    RoleF,
    next_f,
    5, | =>
    RoleD,
    SessionMpstSix,
    6
);
// E
create_send_mpst_session_bundle!(
    send_mpst_e_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_e_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_e_to_c,
    RoleC,
    next_c,
    3, |
    send_mpst_e_to_d,
    RoleD,
    next_d,
    4, |
    send_mpst_e_to_f,
    RoleF,
    next_f,
    5, | =>
    RoleE,
    SessionMpstSix,
    6
);
// F
create_send_mpst_session_bundle!(
    send_mpst_f_to_a,
    RoleA,
    next_a,
    1, |
    send_mpst_f_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_f_to_c,
    RoleC,
    next_c,
    3, |
    send_mpst_f_to_d,
    RoleD,
    next_d,
    4, |
    send_mpst_f_to_e,
    RoleE,
    next_e,
    5, | =>
    RoleF,
    SessionMpstSix,
    6
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_to_b,
    RoleB,
    next_b,
    1, |
    recv_mpst_a_to_c,
    RoleC,
    next_c,
    2, |
    recv_mpst_a_to_d,
    RoleD,
    next_d,
    3, |
    recv_mpst_a_to_e,
    RoleE,
    next_e,
    4, |
    recv_mpst_a_to_f,
    RoleF,
    next_f,
    5, | =>
    RoleA,
    SessionMpstSix,
    6
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_b_to_c,
    RoleC,
    next_c,
    2, |
    recv_mpst_b_to_d,
    RoleD,
    next_d,
    3, |
    recv_mpst_b_to_e,
    RoleE,
    next_e,
    4, |
    recv_mpst_b_to_f,
    RoleF,
    next_f,
    5, | =>
    RoleB,
    SessionMpstSix,
    6
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_c_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_c_to_d,
    RoleD,
    next_d,
    3, |
    recv_mpst_c_to_e,
    RoleE,
    next_e,
    4, |
    recv_mpst_c_to_f,
    RoleF,
    next_f,
    5, | =>
    RoleC,
    SessionMpstSix,
    6
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_d_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_d_to_c,
    RoleC,
    next_c,
    3, |
    recv_mpst_d_to_e,
    RoleE,
    next_e,
    4, |
    recv_mpst_d_to_f,
    RoleF,
    next_f,
    5, | =>
    RoleD,
    SessionMpstSix,
    6
);
// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_e_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_e_to_c,
    RoleC,
    next_c,
    3, |
    recv_mpst_e_to_d,
    RoleD,
    next_d,
    4, |
    recv_mpst_e_to_f,
    RoleF,
    next_f,
    5, | =>
    RoleE,
    SessionMpstSix,
    6
);
// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_f_to_b,
    RoleB,
    next_b,
    2, |
    recv_mpst_f_to_c,
    RoleC,
    next_c,
    3, |
    recv_mpst_f_to_d,
    RoleD,
    next_d,
    4, |
    recv_mpst_f_to_e,
    RoleE,
    next_e,
    5, | =>
    RoleF,
    SessionMpstSix,
    6
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstSix, 6);

// Create fork function
bundle_fork_multi!(fork_mpst, fork_simple, SessionMpstSix, 6);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;
type NameE = RoleE<RoleEnd>;
type NameF = RoleF<RoleEnd>;

// Types
// Send/Recv
type RS = Recv<(), Send<(), End>>;
type SR = Send<(), Recv<(), End>>;
// Roles
type R2A<R> = RoleA<RoleA<R>>;
type R2B<R> = RoleB<RoleB<R>>;
type R2C<R> = RoleC<RoleC<R>>;
type R2D<R> = RoleD<RoleD<R>>;
type R2E<R> = RoleE<RoleE<R>>;
type R2F<R> = RoleF<RoleF<R>>;
// Binary
// A
enum BranchingFforA {
    More(
        SessionMpstSix<
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursAtoF>>,
            R2F<R2B<R2C<R2D<R2E<RoleF<RoleEnd>>>>>>,
            NameA,
        >,
    ),
    Done(SessionMpstSix<End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoF = Recv<BranchingFforA, End>;
// B
enum BranchingFforB {
    More(
        SessionMpstSix<
            SR,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursBtoF>>,
            R2F<R2A<R2C<R2D<R2E<RoleF<RoleEnd>>>>>>,
            NameB,
        >,
    ),
    Done(SessionMpstSix<End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoF = Recv<BranchingFforB, End>;
// C
enum BranchingFforC {
    More(
        SessionMpstSix<
            SR,
            SR,
            RS,
            RS,
            Recv<(), Send<(), RecursCtoF>>,
            R2F<R2A<R2B<R2D<R2E<RoleF<RoleEnd>>>>>>,
            NameC,
        >,
    ),
    Done(SessionMpstSix<End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoF = Recv<BranchingFforC, End>;
// D
enum BranchingFforD {
    More(
        SessionMpstSix<
            SR,
            SR,
            SR,
            RS,
            Recv<(), Send<(), RecursDtoF>>,
            R2F<R2A<R2B<R2C<R2E<RoleF<RoleEnd>>>>>>,
            NameD,
        >,
    ),
    Done(SessionMpstSix<End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoF = Recv<BranchingFforD, End>;
// E
enum BranchingFforE {
    More(
        SessionMpstSix<
            SR,
            SR,
            SR,
            SR,
            Recv<(), Send<(), RecursEtoF>>,
            R2F<R2A<R2B<R2C<R2D<RoleF<RoleEnd>>>>>>,
            NameE,
        >,
    ),
    Done(SessionMpstSix<End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoF = Recv<BranchingFforE, End>;
// F
type ChooseFforAtoF = Send<BranchingFforA, End>;
type ChooseFforBtoF = Send<BranchingFforB, End>;
type ChooseFforCtoF = Send<BranchingFforC, End>;
type ChooseFforDtoF = Send<BranchingFforD, End>;
type ChooseFforEtoF = Send<BranchingFforE, End>;

// Creating the MP sessions
type EndpointA = SessionMpstSix<End, End, End, End, RecursAtoF, RoleF<RoleEnd>, NameA>;
type EndpointB = SessionMpstSix<End, End, End, End, RecursBtoF, RoleF<RoleEnd>, NameB>;
type EndpointC = SessionMpstSix<End, End, End, End, RecursCtoF, RoleF<RoleEnd>, NameC>;
type EndpointD = SessionMpstSix<End, End, End, End, RecursDtoF, RoleF<RoleEnd>, NameD>;
type EndpointE = SessionMpstSix<End, End, End, End, RecursEtoF, RoleF<RoleEnd>, NameE>;
type EndpointF = SessionMpstSix<
    ChooseFforAtoF,
    ChooseFforBtoF,
    ChooseFforCtoF,
    ChooseFforDtoF,
    ChooseFforEtoF,
    RoleA<RoleB<RoleC<RoleD<RoleE<RoleEnd>>>>>,
    NameF,
>;

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_to_f, {
          BranchingFforA::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingFforA::More(s) => {
            let (_, s) = recv_mpst_a_to_f(s)?;
            let s = send_mpst_a_to_f((), s);
            let (_, s) = recv_mpst_a_to_b(s)?;
            let s = send_mpst_a_to_b((), s);
            let (_, s) = recv_mpst_a_to_c(s)?;
            let s = send_mpst_a_to_c((), s);
            let (_, s) = recv_mpst_a_to_d(s)?;
            let s = send_mpst_a_to_d((), s);
            let (_, s) = recv_mpst_a_to_e(s)?;
            let s = send_mpst_a_to_e((), s);
            simple_five_endpoint_a(s)
        },
    })
}

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_to_f, {
          BranchingFforB::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingFforB::More(s) => {
            let (_, s) = recv_mpst_b_to_f(s)?;
            let s = send_mpst_b_to_f((), s);
            let s = send_mpst_b_to_a((), s);
            let (_, s) = recv_mpst_b_to_a(s)?;
            let (_, s) = recv_mpst_b_to_c(s)?;
            let s = send_mpst_b_to_c((), s);
            let (_, s) = recv_mpst_b_to_d(s)?;
            let s = send_mpst_b_to_d((), s);
            let (_, s) = recv_mpst_b_to_e(s)?;
            let s = send_mpst_b_to_e((), s);
            simple_five_endpoint_b(s)
        },
    })
}

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_to_f, {
          BranchingFforC::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingFforC::More(s) => {
            let (_, s) = recv_mpst_c_to_f(s)?;
            let s = send_mpst_c_to_f((), s);
            let s = send_mpst_c_to_a((), s);
            let (_, s) = recv_mpst_c_to_a(s)?;
            let s = send_mpst_c_to_b((), s);
            let (_, s) = recv_mpst_c_to_b(s)?;
            let (_, s) = recv_mpst_c_to_d(s)?;
            let s = send_mpst_c_to_d((), s);
            let (_, s) = recv_mpst_c_to_e(s)?;
            let s = send_mpst_c_to_e((), s);
            simple_five_endpoint_c(s)
        },
    })
}

fn simple_five_endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_to_f, {
          BranchingFforD::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingFforD::More(s) => {
            let (_, s) = recv_mpst_d_to_f(s)?;
            let s = send_mpst_d_to_f((), s);
            let s = send_mpst_d_to_a((), s);
            let (_, s) = recv_mpst_d_to_a(s)?;
            let s = send_mpst_d_to_b((), s);
            let (_, s) = recv_mpst_d_to_b(s)?;
            let s = send_mpst_d_to_c((), s);
            let (_, s) = recv_mpst_d_to_c(s)?;
            let (_, s) = recv_mpst_d_to_e(s)?;
            let s = send_mpst_d_to_e((), s);
            simple_five_endpoint_d(s)
        },
    })
}

fn simple_five_endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_e_to_f, {
          BranchingFforE::Done(s) => {
            close_mpst_multi(s)
        },
          BranchingFforE::More(s) => {
            let (_, s) = recv_mpst_e_to_f(s)?;
            let s = send_mpst_e_to_f((), s);
            let s = send_mpst_e_to_a((), s);
            let (_, s) = recv_mpst_e_to_a(s)?;
            let s = send_mpst_e_to_b((), s);
            let (_, s) = recv_mpst_e_to_b(s)?;
            let s = send_mpst_e_to_c((), s);
            let (_, s) = recv_mpst_e_to_c(s)?;
            let s = send_mpst_e_to_d((), s);
            let (_, s) = recv_mpst_e_to_d(s)?;
            simple_five_endpoint_e(s)
        },
    })
}

fn simple_five_endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    recurs_f(s, SIZE)
}

fn recurs_f(s: EndpointF, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_f_to_a,
                send_mpst_f_to_b,
                send_mpst_f_to_c,
                send_mpst_f_to_d,
                send_mpst_f_to_e, =>
                  BranchingFforA::Done,
                  BranchingFforB::Done,
                  BranchingFforC::Done,
                  BranchingFforD::Done,
                  BranchingFforE::Done, =>
                RoleA,
                RoleB,
                RoleC,
                RoleD,
                RoleE, =>
                RoleF,
                 SessionMpstSix,
                6
            );

            close_mpst_multi(s)
        }
        i => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_f_to_a,
                send_mpst_f_to_b,
                send_mpst_f_to_c,
                send_mpst_f_to_d,
                send_mpst_f_to_e, =>
                  BranchingFforA::More,
                  BranchingFforB::More,
                  BranchingFforC::More,
                  BranchingFforD::More,
                  BranchingFforE::More, =>
                RoleA,
                RoleB,
                RoleC,
                RoleD,
                RoleE, =>
                RoleF,
                 SessionMpstSix,
                6
            );

            let s = send_mpst_f_to_a((), s);
            let (_, s) = recv_mpst_f_to_a(s)?;
            let s = send_mpst_f_to_b((), s);
            let (_, s) = recv_mpst_f_to_b(s)?;
            let s = send_mpst_f_to_c((), s);
            let (_, s) = recv_mpst_f_to_c(s)?;
            let s = send_mpst_f_to_d((), s);
            let (_, s) = recv_mpst_f_to_d(s)?;
            let s = send_mpst_f_to_e((), s);
            let (_, s) = recv_mpst_f_to_e(s)?;

            recurs_f(s, i - 1)
        }
    }
}

fn all_mpst() -> Result<(), Box<dyn Error>> {
    let (thread_a, thread_b, thread_c, thread_d, thread_e, thread_f) = fork_mpst(
        black_box(simple_five_endpoint_a),
        black_box(simple_five_endpoint_b),
        black_box(simple_five_endpoint_c),
        black_box(simple_five_endpoint_d),
        black_box(simple_five_endpoint_e),
        black_box(simple_five_endpoint_f),
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
    thread_f.join().unwrap();

    Ok(())
}

/////////////////////////
// A
enum BinaryA {
    More(Recv<(), Send<(), RecursA>>),
    Done(End),
}
type RecursA = Recv<BinaryA, End>;
fn binary_a_to_b(s: RecursA) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        BinaryA::Done(s) => {
            close(s)
        },
        BinaryA::More(s) => {
            let (_, s) = recv(s)?;
            let s = send((), s);
            binary_a_to_b(s)
        },
    })
}

// B
type RecursB = <RecursA as Session>::Dual;
fn binary_b_to_a(s: Send<(), Recv<(), RecursB>>) -> Result<RecursB, Box<dyn Error>> {
    let s = send((), s);
    let (_, s) = recv(s)?;
    Ok(s)
}

fn all_binaries() -> Result<(), Box<dyn Error>> {
    let mut threads = Vec::new();
    let mut sessions = Vec::new();

    for _ in 0..15 {
        let (thread, s): (JoinHandle<()>, RecursB) = fork_with_thread_id(black_box(binary_a_to_b));

        threads.push(thread);
        sessions.push(s);
    }

    let main = spawn(move || {
        for _ in 0..SIZE {
            sessions = sessions
                .into_iter()
                .map(|s| binary_b_to_a(choose!(BinaryA::More, s)).unwrap())
                .collect::<Vec<_>>();
        }

        sessions
            .into_iter()
            .for_each(|s| close(choose!(BinaryA::Done, s)).unwrap());

        threads.into_iter().for_each(|elt| elt.join().unwrap());
    });

    main.join().unwrap();

    Ok(())
}

/////////////////////////

fn all_crossbeam() -> Result<(), Box<dyn Error>> {
    let mut threads = Vec::new();

    for _ in 0..15 {
        let main = spawn(move || {
            for _ in 0..SIZE {
                let (sender_1, receiver_1) = bounded::<()>(1);
                sender_1.send(()).unwrap_or(());
                receiver_1.recv().unwrap_or(());

                let (sender_2, receiver_2) = bounded::<()>(1);
                sender_2.send(()).unwrap_or(());
                receiver_2.recv().unwrap_or(());
            }

            // "Close" connection
            let (sender_close_1, receiver_close_1) = bounded::<()>(1);
            sender_close_1.send(()).unwrap_or(());

            let (sender_close_2, receiver_close_2) = bounded::<()>(1);
            sender_close_2.send(()).unwrap_or(());

            receiver_close_1.recv().unwrap_or(());
            receiver_close_2.recv().unwrap_or(());
        });

        threads.push(main);
    }

    threads.into_iter().for_each(|elt| elt.join().unwrap());

    Ok(())
}

/////////////////////////

static SIZE: i64 = 100;

fn long_simple_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("long six simple protocol MPST {}", SIZE), |b| {
        b.iter(|| all_mpst())
    });
}

fn long_simple_protocol_binary(c: &mut Criterion) {
    c.bench_function(&format!("long six simple protocol binary {}", SIZE), |b| {
        b.iter(|| all_binaries())
    });
}

fn long_simple_protocol_crossbeam(c: &mut Criterion) {
    c.bench_function(
        &format!("long six simple protocol crossbeam {}", SIZE),
        |b| b.iter(|| all_crossbeam()),
    );
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(1800, 0))
}

criterion_group! {
    name = long_six_simple_protocols;
    config = long_warmup();
    targets = long_simple_protocol_mpst, long_simple_protocol_binary, long_simple_protocol_crossbeam
}
criterion_main!(long_six_simple_protocols);
