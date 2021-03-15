#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose_mpst_multi_to_all, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, offer_mpst,
};

use rand::{thread_rng, Rng};
use std::error::Error;
use std::marker;
use std::time::Duration;

// global protocol Proto(role A, role C, role S)
// {
//     choice at S
//     {
//         login(Int) from S to C;
//         password(Int) from C to A;
//         choice at A
//         {
//              Auth(Int) from A to S;
//              Auth(Int) from S to C;
//         }
//         or
//         {
//              again(Int) from A to S;
//              again(Int) from S to C;
//         }
//     }
//     or
//     {
//         cancel(Int) from S to C;
//         quit(Int) from C to A;
//     }
// }

// Create the new SessionMpst for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstThree, 3);

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
    send_mpst_a_to_c, RoleC, next_c, 1 |
    send_mpst_a_to_s, RoleS, next_s, 2 | =>
    RoleA, SessionMpstThree, 3
);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a, RoleA, next_a, 1 |
    send_mpst_c_to_s, RoleS, next_s, 2 | =>
    RoleC, SessionMpstThree, 3
);
// S
create_send_mpst_session_bundle!(
    send_mpst_s_to_a, RoleA, next_a, 1 |
    send_mpst_s_to_c, RoleC, next_c, 2 | =>
    RoleS, SessionMpstThree, 3
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_c, RoleC, next_c, 1 |
    recv_mpst_a_from_s, RoleS, next_s, 2 | =>
    RoleA, SessionMpstThree, 3
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a, RoleA, next_a, 1 |
    recv_mpst_c_from_s, RoleS, next_s, 2 | =>
    RoleC, SessionMpstThree, 3
);
// S
create_recv_mpst_session_bundle!(
    recv_mpst_s_from_a, RoleA, next_a, 1 |
    recv_mpst_s_from_c, RoleC, next_c, 2 | =>
    RoleS, SessionMpstThree, 3
);

// Names
type NameA = RoleA<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameS = RoleS<RoleEnd>;

// Types
// S
type Choose0fromStoA<N> = Send<Branching0fromStoA<N>, End>;
type Choose0fromStoC<N> = Send<Branching0fromStoC<N>, End>;
// A
type Choose1fromAtoC<N> = Send<Branching1fromAtoC<N>, End>;
type Choose1fromAtoS<N> = Send<Branching1fromAtoS<N>, End>;

// A
enum Branching0fromStoA<N: marker::Send> {
    Login(
        SessionMpstThree<
            Recv<N, Choose1fromAtoC<N>>,
            Choose1fromAtoS<N>,
            RoleC<RoleC<RoleS<RoleEnd>>>,
            NameA,
        >,
    ),
    Done(SessionMpstThree<Recv<N, End>, End, RoleC<RoleEnd>, NameA>),
}
// C
enum Branching0fromStoC<N: marker::Send> {
    Login(
        SessionMpstThree<
            Send<N, Choice1fromCtoA<N>>,
            Recv<N, End>,
            RoleS<RoleA<RoleA<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(SessionMpstThree<Send<N, End>, Recv<N, End>, RoleS<RoleA<RoleEnd>>, NameC>),
}
enum Branching1fromAtoC<N: marker::Send> {
    Auth(SessionMpstThree<End, Recv<N, End>, RoleS<RoleEnd>, NameC>),
    Again(
        SessionMpstThree<
            Send<N, Choice1fromCtoA<N>>,
            Recv<N, End>,
            RoleS<RoleA<RoleA<RoleEnd>>>,
            NameC,
        >,
    ),
}
type Choice1fromCtoA<N> = Recv<Branching1fromAtoC<N>, End>;
// S
enum Branching1fromAtoS<N: marker::Send> {
    Auth(SessionMpstThree<Recv<N, End>, Send<N, End>, RoleA<RoleC<RoleEnd>>, NameS>),
    Again(
        SessionMpstThree<
            Recv<N, Choice1fromStoA<N>>,
            Send<N, End>,
            RoleA<RoleC<RoleA<RoleEnd>>>,
            NameS,
        >,
    ),
}
type Choice1fromStoA<N> = Recv<Branching1fromAtoS<N>, End>;

// Creating the MP sessions
// A
type ChoiceA<N> = SessionMpstThree<
    Recv<N, Choose1fromAtoC<N>>,
    Choose1fromAtoS<N>,
    RoleC<RoleC<RoleS<RoleEnd>>>,
    NameA,
>;
type EndpointA<N> = SessionMpstThree<End, Recv<Branching0fromStoA<N>, End>, RoleS<RoleEnd>, NameA>;
// C
type ChoiceC<N> = SessionMpstThree<Send<N, Choice1fromCtoA<N>>, End, RoleA<RoleA<RoleEnd>>, NameC>;
type EndpointC<N> = SessionMpstThree<End, Recv<Branching0fromStoC<N>, End>, RoleS<RoleEnd>, NameC>;
// S
type ChoiceS<N> = SessionMpstThree<Choice1fromStoA<N>, End, RoleA<RoleEnd>, NameS>;
type EndpointS<N> =
    SessionMpstThree<Choose0fromStoA<N>, Choose0fromStoC<N>, RoleA<RoleC<RoleEnd>>, NameS>;

// Functions
fn simple_five_endpoint_a(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_s, {
        Branching0fromStoA::Done(s) => {
            let (_, s) = recv_mpst_a_from_c(s)?;

            close_mpst_multi(s)
        },
        Branching0fromStoA::Login(s) => {
            choice_a(s)
        },
    })
}

fn choice_a(s: ChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    let (pwd, s) = recv_mpst_a_from_c(s)?;

    let expected = thread_rng().gen_range(1..=3);

    if pwd == expected {
        let s = choose_mpst_multi_to_all!(
            s,
            send_mpst_a_to_c,
            send_mpst_a_to_s, =>
            Branching1fromAtoC::<i32>::Auth,
            Branching1fromAtoS::<i32>::Auth, =>
            RoleC,
            RoleS, =>
            RoleA,
            SessionMpstThree,
            3,
            1
        );

        let s = send_mpst_a_to_s(0, s);

        close_mpst_multi(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            send_mpst_a_to_c,
            send_mpst_a_to_s, =>
            Branching1fromAtoC::<i32>::Again,
            Branching1fromAtoS::<i32>::Again, =>
            RoleC,
            RoleS, =>
            RoleA,
            SessionMpstThree,
            3,
            1
        );

        let s = send_mpst_a_to_s(1, s);

        choice_a(s)
    }
}

fn simple_five_endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_s, {
        Branching0fromStoC::<i32>::Done(s) => {
            let (quit, s) = recv_mpst_c_from_s(s)?;
            let s = send_mpst_c_to_a(quit, s);
            close_mpst_multi(s)
        },
        Branching0fromStoC::<i32>::Login(s) => {
            let (_, s) = recv_mpst_c_from_s(s)?;

            choice_c(s)
        },
    })
}

fn choice_c(s: ChoiceC<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_c_to_a(thread_rng().gen_range(1..=3), s);

    offer_mpst!(s, recv_mpst_c_from_a, {
        Branching1fromAtoC::<i32>::Auth(s) => {
            let (_, s) = recv_mpst_c_from_s(s)?;

            close_mpst_multi(s)
        },
        Branching1fromAtoC::<i32>::Again(s) => {
            let (_, s) = recv_mpst_c_from_s(s)?;

            choice_c(s)
        },
    })
}

fn simple_five_endpoint_s(s: EndpointS<i32>) -> Result<(), Box<dyn Error>> {
    let choice = thread_rng().gen_range(1..=3);

    if choice != 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            send_mpst_s_to_a,
            send_mpst_s_to_c, =>
            Branching0fromStoA::<i32>::Done,
            Branching0fromStoC::<i32>::Done, =>
            RoleA,
            RoleC, =>
            RoleS,
            SessionMpstThree,
            3,
            3
        );

        let s = send_mpst_s_to_c(0, s);

        close_mpst_multi(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            send_mpst_s_to_a,
            send_mpst_s_to_c,  =>
            Branching0fromStoA::<i32>::Login,
            Branching0fromStoC::<i32>::Login, =>
            RoleA,
            RoleC, =>
            RoleS,
            SessionMpstThree,
            3,
            3
        );

        let s = send_mpst_s_to_c(1, s);

        choice_s(s)
    }
}

fn choice_s(s: ChoiceS<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_s_from_a, {
        Branching1fromAtoS::<i32>::Auth(s) => {
            let (success, s) = recv_mpst_s_from_a(s)?;

            let s = send_mpst_s_to_c(success, s);

            close_mpst_multi(s)
        },
        Branching1fromAtoS::<i32>::Again(s) => {
            let (fail, s) = recv_mpst_s_from_a(s)?;

            let s = send_mpst_s_to_c(fail, s);

            choice_s(s)
        },
    })
}

fn all_mpst() -> Result<(), Box<dyn std::any::Any + std::marker::Send>> {
    let (thread_a, thread_c, thread_s) = fork_mpst(
        black_box(simple_five_endpoint_a),
        black_box(simple_five_endpoint_c),
        black_box(simple_five_endpoint_s),
    );

    thread_a.join()?;
    thread_c.join()?;
    thread_s.join()?;

    Ok(())
}

/////////////////////////

fn o_auth_mpst(c: &mut Criterion) {
    c.bench_function(&format!("oAuth MPST"), |b| b.iter(|| all_mpst()));
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(30, 0))
}

criterion_group! {
    name = o_auth;
    config = long_warmup();
    targets = o_auth_mpst
}

criterion_main!(o_auth);
