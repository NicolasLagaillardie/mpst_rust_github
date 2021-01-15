use mpstthree::binary::{close, fork_with_thread_id, recv, send, End, Recv, Send, Session};
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::{
    bundle_fork_multi, choose, choose_mpst_multi_to_all, close_mpst, create_normal_role,
    create_recv_mpst_session, create_send_mpst_session, create_sessionmpst, offer, offer_mpst,
};

use std::error::Error;
use std::thread::{spawn, JoinHandle};

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
create_send_mpst_session!(send_mpst_a_to_b, RoleB, next_b, RoleA, SessionMpstSix, 6, 1);
create_send_mpst_session!(send_mpst_a_to_c, RoleC, next_c, RoleA, SessionMpstSix, 6, 2);
create_send_mpst_session!(send_mpst_a_to_d, RoleD, next_d, RoleA, SessionMpstSix, 6, 3);
create_send_mpst_session!(send_mpst_a_to_e, RoleE, next_e, RoleA, SessionMpstSix, 6, 4);
create_send_mpst_session!(send_mpst_a_to_f, RoleF, next_f, RoleA, SessionMpstSix, 6, 5);
// B
create_send_mpst_session!(send_mpst_b_to_a, RoleA, next_a, RoleB, SessionMpstSix, 6, 1);
create_send_mpst_session!(send_mpst_b_to_c, RoleC, next_c, RoleB, SessionMpstSix, 6, 2);
create_send_mpst_session!(send_mpst_b_to_d, RoleD, next_d, RoleB, SessionMpstSix, 6, 3);
create_send_mpst_session!(send_mpst_b_to_e, RoleE, next_e, RoleB, SessionMpstSix, 6, 4);
create_send_mpst_session!(send_mpst_b_to_f, RoleF, next_f, RoleB, SessionMpstSix, 6, 5);
// C
create_send_mpst_session!(send_mpst_c_to_a, RoleA, next_a, RoleC, SessionMpstSix, 6, 1);
create_send_mpst_session!(send_mpst_c_to_b, RoleB, next_b, RoleC, SessionMpstSix, 6, 2);
create_send_mpst_session!(send_mpst_c_to_d, RoleD, next_d, RoleC, SessionMpstSix, 6, 3);
create_send_mpst_session!(send_mpst_c_to_e, RoleE, next_e, RoleC, SessionMpstSix, 6, 4);
create_send_mpst_session!(send_mpst_c_to_f, RoleF, next_f, RoleC, SessionMpstSix, 6, 5);
// D
create_send_mpst_session!(send_mpst_d_to_a, RoleA, next_a, RoleD, SessionMpstSix, 6, 1);
create_send_mpst_session!(send_mpst_d_to_b, RoleB, next_b, RoleD, SessionMpstSix, 6, 2);
create_send_mpst_session!(send_mpst_d_to_c, RoleC, next_c, RoleD, SessionMpstSix, 6, 3);
create_send_mpst_session!(send_mpst_d_to_e, RoleE, next_e, RoleD, SessionMpstSix, 6, 4);
create_send_mpst_session!(send_mpst_d_to_f, RoleF, next_f, RoleD, SessionMpstSix, 6, 5);
// E
create_send_mpst_session!(send_mpst_e_to_a, RoleA, next_a, RoleE, SessionMpstSix, 6, 1);
create_send_mpst_session!(send_mpst_e_to_b, RoleB, next_b, RoleE, SessionMpstSix, 6, 2);
create_send_mpst_session!(send_mpst_e_to_c, RoleC, next_c, RoleE, SessionMpstSix, 6, 3);
create_send_mpst_session!(send_mpst_e_to_d, RoleD, next_d, RoleE, SessionMpstSix, 6, 4);
create_send_mpst_session!(send_mpst_e_to_f, RoleF, next_f, RoleE, SessionMpstSix, 6, 5);
// F
create_send_mpst_session!(send_mpst_f_to_a, RoleA, next_a, RoleF, SessionMpstSix, 6, 1);
create_send_mpst_session!(send_mpst_f_to_b, RoleB, next_b, RoleF, SessionMpstSix, 6, 2);
create_send_mpst_session!(send_mpst_f_to_c, RoleC, next_c, RoleF, SessionMpstSix, 6, 3);
create_send_mpst_session!(send_mpst_f_to_d, RoleD, next_d, RoleF, SessionMpstSix, 6, 4);
create_send_mpst_session!(send_mpst_f_to_e, RoleE, next_e, RoleF, SessionMpstSix, 6, 5);

// Create new recv functions and related types
// A
create_recv_mpst_session!(recv_mpst_a_to_b, RoleB, next_b, RoleA, SessionMpstSix, 6, 1);
create_recv_mpst_session!(recv_mpst_a_to_c, RoleC, next_c, RoleA, SessionMpstSix, 6, 2);
create_recv_mpst_session!(recv_mpst_a_to_d, RoleD, next_d, RoleA, SessionMpstSix, 6, 3);
create_recv_mpst_session!(recv_mpst_a_to_e, RoleE, next_e, RoleA, SessionMpstSix, 6, 4);
create_recv_mpst_session!(recv_mpst_a_to_f, RoleF, next_f, RoleA, SessionMpstSix, 6, 5);
// B
create_recv_mpst_session!(recv_mpst_b_to_a, RoleA, next_a, RoleB, SessionMpstSix, 6, 1);
create_recv_mpst_session!(recv_mpst_b_to_c, RoleC, next_c, RoleB, SessionMpstSix, 6, 2);
create_recv_mpst_session!(recv_mpst_b_to_d, RoleD, next_d, RoleB, SessionMpstSix, 6, 3);
create_recv_mpst_session!(recv_mpst_b_to_e, RoleE, next_e, RoleB, SessionMpstSix, 6, 4);
create_recv_mpst_session!(recv_mpst_b_to_f, RoleF, next_f, RoleB, SessionMpstSix, 6, 5);
// C
create_recv_mpst_session!(recv_mpst_c_to_a, RoleA, next_a, RoleC, SessionMpstSix, 6, 1);
create_recv_mpst_session!(recv_mpst_c_to_b, RoleB, next_b, RoleC, SessionMpstSix, 6, 2);
create_recv_mpst_session!(recv_mpst_c_to_d, RoleD, next_d, RoleC, SessionMpstSix, 6, 3);
create_recv_mpst_session!(recv_mpst_c_to_e, RoleE, next_e, RoleC, SessionMpstSix, 6, 4);
create_recv_mpst_session!(recv_mpst_c_to_f, RoleF, next_f, RoleC, SessionMpstSix, 6, 5);
// D
create_recv_mpst_session!(recv_mpst_d_to_a, RoleA, next_a, RoleD, SessionMpstSix, 6, 1);
create_recv_mpst_session!(recv_mpst_d_to_b, RoleB, next_b, RoleD, SessionMpstSix, 6, 2);
create_recv_mpst_session!(recv_mpst_d_to_c, RoleC, next_c, RoleD, SessionMpstSix, 6, 3);
create_recv_mpst_session!(recv_mpst_d_to_e, RoleE, next_e, RoleD, SessionMpstSix, 6, 4);
create_recv_mpst_session!(recv_mpst_d_to_f, RoleF, next_f, RoleD, SessionMpstSix, 6, 5);
// E
create_recv_mpst_session!(recv_mpst_e_to_a, RoleA, next_a, RoleE, SessionMpstSix, 6, 1);
create_recv_mpst_session!(recv_mpst_e_to_b, RoleB, next_b, RoleE, SessionMpstSix, 6, 2);
create_recv_mpst_session!(recv_mpst_e_to_c, RoleC, next_c, RoleE, SessionMpstSix, 6, 3);
create_recv_mpst_session!(recv_mpst_e_to_d, RoleD, next_d, RoleE, SessionMpstSix, 6, 4);
create_recv_mpst_session!(recv_mpst_e_to_f, RoleF, next_f, RoleE, SessionMpstSix, 6, 5);
// F
create_recv_mpst_session!(recv_mpst_f_to_a, RoleA, next_a, RoleF, SessionMpstSix, 6, 1);
create_recv_mpst_session!(recv_mpst_f_to_b, RoleB, next_b, RoleF, SessionMpstSix, 6, 2);
create_recv_mpst_session!(recv_mpst_f_to_c, RoleC, next_c, RoleF, SessionMpstSix, 6, 3);
create_recv_mpst_session!(recv_mpst_f_to_d, RoleD, next_d, RoleF, SessionMpstSix, 6, 4);
create_recv_mpst_session!(recv_mpst_f_to_e, RoleE, next_e, RoleF, SessionMpstSix, 6, 5);

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
// Binary
// A
enum BranchingFforA {
    More(
        SessionMpstSix<
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursAtoF>>,
            RoleF<RoleF<RoleB<RoleB<RoleC<RoleC<RoleD<RoleD<RoleE<RoleE<RoleF<RoleEnd>>>>>>>>>>>,
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
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursBtoF>>,
            RoleF<RoleF<RoleA<RoleA<RoleC<RoleC<RoleD<RoleD<RoleE<RoleE<RoleF<RoleEnd>>>>>>>>>>>,
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
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursCtoF>>,
            RoleF<RoleF<RoleA<RoleA<RoleB<RoleB<RoleD<RoleD<RoleE<RoleE<RoleF<RoleEnd>>>>>>>>>>>,
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
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursDtoF>>,
            RoleF<RoleF<RoleA<RoleA<RoleB<RoleB<RoleC<RoleC<RoleE<RoleE<RoleF<RoleEnd>>>>>>>>>>>,
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
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), RecursEtoF>>,
            RoleF<RoleF<RoleA<RoleA<RoleB<RoleB<RoleC<RoleC<RoleD<RoleD<RoleF<RoleEnd>>>>>>>>>>>,
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
        simple_five_endpoint_a,
        simple_five_endpoint_b,
        simple_five_endpoint_c,
        simple_five_endpoint_d,
        simple_five_endpoint_e,
        simple_five_endpoint_f,
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
        let (thread, s): (JoinHandle<()>, RecursB) = fork_with_thread_id(binary_a_to_b);

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

static SIZE: i64 = 15;

fn main() {
    all_binaries().unwrap();
    all_mpst().unwrap();
}
