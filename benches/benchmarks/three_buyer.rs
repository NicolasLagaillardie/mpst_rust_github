#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    fork_mpst_multi, choose_mpst_multi_to_all, close_mpst, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, create_sessionmpst,
    offer_mpst,
};

use rand::{random, thread_rng, Rng};
use std::error::Error;
use std::marker;
use std::time::Duration;

// global protocol TwoBuyer(role A, role C, role S)
// {
//     empty1(int) from A to S;
//     empty2(int) from S to A;
//     empty3(int) from S to C;
//     empty4(int) from A to C;

//     choice at C
//     {
//         ok(int) from C to A;
//         ok(int) from C to S;
//         empty5(int) from S to C;
//     }
//     or
//     {
//         quit() from C to A;
//         quit() from C to S;
//     }
// }

// Create new SessionMpst for three participants
create_sessionmpst!(SessionMpstThree, 3);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, next_a, RoleADual, next_a_dual |
    RoleC, next_c, RoleCDual, next_c_dual |
    RoleS, next_s, RoleSDual, next_s_dual |
);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_c,
    RoleC,
    next_c,
    1 |
    send_mpst_a_to_s,
    RoleS,
    next_s,
    2 | =>
    RoleA,
    SessionMpstThree,
    3
);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a,
    RoleA,
    next_a,
    1 |
    send_mpst_c_to_s,
    RoleS,
    next_s,
    2 | =>
    RoleC,
    SessionMpstThree,
    3
);
// S
create_send_mpst_session_bundle!(
    send_mpst_s_to_a,
    RoleA,
    next_a,
    1 |
    send_mpst_s_to_c,
    RoleC,
    next_c,
    2 | =>
    RoleS,
    SessionMpstThree,
    3
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_to_c,
    RoleC,
    next_c,
    1 |
    recv_mpst_a_to_s,
    RoleS,
    next_s,
    2 | =>
    RoleA,
    SessionMpstThree,
    3
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_to_a,
    RoleA,
    next_a,
    1 |
    recv_mpst_c_to_s,
    RoleS,
    next_s,
    2 | =>
    RoleC,
    SessionMpstThree,
    3
);
// S
create_recv_mpst_session_bundle!(
    recv_mpst_s_to_a,
    RoleA,
    next_a,
    1 |
    recv_mpst_s_to_c,
    RoleC,
    next_c,
    2 | =>
    RoleS,
    SessionMpstThree,
    3
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstThree, 3);

// Create fork function
fork_mpst_multi!(fork_mpst,  SessionMpstThree, 3);

// Names
type NameA = RoleA<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameS = RoleS<RoleEnd>;

// Types
// A
type Choose0fromCtoA<N> = Send<Branching0fromCtoA<N>, End>;
type Choose0fromCtoS<N> = Send<Branching0fromCtoS<N>, End>;

// A
enum Branching0fromCtoA<N: marker::Send>
{
    Accept(SessionMpstThree<Recv<N, End>, End, RoleC<RoleEnd>, NameA>),
    Quit(SessionMpstThree<End, End, RoleEnd, NameA>),
}
// S
enum Branching0fromCtoS<N: marker::Send>
{
    Accept(SessionMpstThree<End, Recv<N, Send<N, End>>, RoleC<RoleC<RoleEnd>>, NameS>),
    Quit(SessionMpstThree<End, End, RoleEnd, NameS>),
}

// Creating the MP sessions
// A
type EndpointA<N> = SessionMpstThree<
    Send<N, Recv<Branching0fromCtoA<N>, End>>,
    Send<N, Recv<N, End>>,
    RoleS<RoleS<RoleC<RoleC<RoleEnd>>>>,
    NameA,
>;
// C
type EndpointC<N> = SessionMpstThree<
    Recv<N, Choose0fromCtoA<N>>,
    Recv<N, Choose0fromCtoS<N>>,
    RoleS<RoleA<RoleA<RoleS<RoleEnd>>>>,
    NameC,
>;
// S
type EndpointS<N> = SessionMpstThree<
    Recv<N, Send<N, End>>,
    Send<N, Recv<Branching0fromCtoS<N>, End>>,
    RoleA<RoleA<RoleC<RoleC<RoleEnd>>>>,
    NameS,
>;

// Functions
fn simple_five_endpoint_a(s: EndpointA<i32>) -> Result<(), Box<dyn Error>>
{
    let s = send_mpst_a_to_s(random(), s);
    let (_empty2, s) = recv_mpst_a_to_s(s)?;
    let s = send_mpst_a_to_c(random(), s);
    offer_mpst!(s, recv_mpst_a_to_c, {
        Branching0fromCtoA::Accept(s) => {
            let (_ok, s) = recv_mpst_a_to_c(s)?;
            close_mpst_multi(s)
        },
        Branching0fromCtoA::Quit(s) => {
            close_mpst_multi(s)
        },
    })
}

fn simple_five_endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>>
{
    let (_empty3, s) = recv_mpst_c_to_s(s)?;
    let (_empty4, s) = recv_mpst_c_to_a(s)?;

    let choice = thread_rng().gen_range(1..=3);

    if choice != 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            send_mpst_c_to_a,
            send_mpst_c_to_s, =>
            Branching0fromCtoA::<i32>::Accept,
            Branching0fromCtoS::<i32>::Accept, =>
            RoleA,
            RoleS, =>
            RoleC,
            SessionMpstThree,
            3,
            2
        );

        let s = send_mpst_c_to_a(random(), s);
        let s = send_mpst_c_to_s(random(), s);
        let (_empty5, s) = recv_mpst_c_to_s(s)?;

        close_mpst_multi(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            send_mpst_c_to_a,
            send_mpst_c_to_s, =>
            Branching0fromCtoA::<i32>::Quit,
            Branching0fromCtoS::<i32>::Quit, =>
            RoleA,
            RoleS, =>
            RoleC,
            SessionMpstThree,
            3,
            2
        );
        close_mpst_multi(s)
    }
}

fn simple_five_endpoint_s(s: EndpointS<i32>) -> Result<(), Box<dyn Error>>
{
    let (_empty1, s) = recv_mpst_s_to_a(s)?;
    let s = send_mpst_s_to_a(random(), s);
    let s = send_mpst_s_to_c(random(), s);
    offer_mpst!(s, recv_mpst_s_to_c, {
        Branching0fromCtoS::Accept(s) => {
            let (_ok, s) = recv_mpst_s_to_c(s)?;
            let s = send_mpst_s_to_c(random(), s);
            close_mpst_multi(s)
        },
        Branching0fromCtoS::Quit(s) => {
            close_mpst_multi(s)
        },
    })
}

fn all_mpst() -> Result<(), Box<dyn Error>>
{
    let (thread_a, thread_c, thread_s) = fork_mpst(
        black_box(simple_five_endpoint_a),
        black_box(simple_five_endpoint_c),
        black_box(simple_five_endpoint_s),
    );

    thread_a.join().unwrap();
    thread_c.join().unwrap();
    thread_s.join().unwrap();

    Ok(())
}

/////////////////////////

fn three_buyer_mpst(c: &mut Criterion)
{
    c.bench_function(&format!("Three buyer MPST"), |b| b.iter(|| all_mpst()));
}

fn long_warmup() -> Criterion
{
    Criterion::default().measurement_time(Duration::new(30, 0))
}

criterion_group! {
    name = three_buyer;
    config = long_warmup();
    targets = three_buyer_mpst
}

criterion_main!(three_buyer);
