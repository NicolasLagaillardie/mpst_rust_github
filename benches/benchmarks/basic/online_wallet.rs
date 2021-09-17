#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose_mpst_multi_to_all, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, offer_mpst,
};

use rand::{distributions::Alphanumeric, random, thread_rng, Rng};

use std::error::Error;
// use std::time::Duration;

// See the folder scribble_protocols for the related Scribble protocol

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsThree, 3);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleC, RoleCDual |
    RoleS, RoleSDual |
);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_c, RoleC, 1 |
    send_mpst_a_to_s, RoleS, 2 | =>
    RoleA, MeshedChannelsThree, 3
);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a, RoleA, 1 |
    send_mpst_c_to_s, RoleS, 2 | =>
    RoleC, MeshedChannelsThree, 3
);
// S
create_send_mpst_session_bundle!(
    send_mpst_s_to_c, RoleC, 2 | =>
    RoleS, MeshedChannelsThree, 3
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_c, RoleC, 1 | =>
    RoleA, MeshedChannelsThree, 3
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a, RoleA, 1 |
    recv_mpst_c_from_s, RoleS, 2 | =>
    RoleC, MeshedChannelsThree, 3
);
// S
create_recv_mpst_session_bundle!(
    recv_mpst_s_from_a, RoleA, 1 |
    recv_mpst_s_from_c, RoleC, 2 | =>
    RoleS, MeshedChannelsThree, 3
);

// Names
type NameA = RoleA<RoleEnd>;
type NameS = RoleS<RoleEnd>;
type NameC = RoleC<RoleEnd>;

// Types
// A
type Choose0fromAtoS = <Recurs0StoA as Session>::Dual;
type Choose0fromAtoC = <Recurs0CtoA as Session>::Dual;

enum Branching1fromCtoA {
    Pay(MeshedChannelsThree<Recurs1AtoC, End, RoleC<RoleEnd>, NameA>),
    Quit(MeshedChannelsThree<End, End, RoleEnd, NameA>),
}

type Recurs1AtoC = Recv<Branching1fromCtoA, End>;

// S
enum Branching0fromAtoS {
    Login(
        MeshedChannelsThree<
            Recv<(), End>,
            Send<(i64, i64), Recurs1StoC>,
            RoleA<RoleC<RoleC<RoleEnd>>>,
            NameS,
        >,
    ),
    Fail(MeshedChannelsThree<Recv<String, End>, End, RoleA<RoleEnd>, NameS>),
}

type Recurs0StoA = Recv<Branching0fromAtoS, End>;

enum Branching1fromCtoS {
    Pay(
        MeshedChannelsThree<
            End,
            Recv<(String, i64), Send<(i64, i64), Recurs1StoC>>,
            RoleC<RoleC<RoleC<RoleEnd>>>,
            NameS,
        >,
    ),
    Quit(MeshedChannelsThree<End, Recv<(), End>, RoleC<RoleEnd>, NameS>),
}

type Recurs1StoC = Recv<Branching1fromCtoS, End>;
// C
enum Branching0fromAtoC {
    Login(
        MeshedChannelsThree<
            Recv<(), Choose1fromCtoA>,
            Recv<(i64, i64), Choose1fromCtoS>,
            RoleA<RoleS<RoleBroadcast>>,
            NameC,
        >,
    ),
    Fail(MeshedChannelsThree<Recv<String, End>, End, RoleA<RoleEnd>, NameC>),
}
type Recurs0CtoA = Recv<Branching0fromAtoC, End>;

type Choose1fromCtoA = <Recurs1AtoC as Session>::Dual;
type Choose1fromCtoS = <Recurs1StoC as Session>::Dual;

// Creating the MP sessions
// Step 1
type EndpointA1 = MeshedChannelsThree<Recurs1AtoC, End, RoleC<RoleEnd>, NameA>;
type EndpointC1 = MeshedChannelsThree<
    Choose1fromCtoA,
    Recv<(i64, i64), Choose1fromCtoS>,
    RoleS<RoleBroadcast>,
    NameC,
>;
type EndpointS1 =
    MeshedChannelsThree<End, Send<(i64, i64), Recurs1StoC>, RoleC<RoleC<RoleEnd>>, NameS>;
// Step 0
type EndpointA0 = MeshedChannelsThree<
    Recv<(String, String), Choose0fromAtoC>,
    Choose0fromAtoS,
    RoleC<RoleBroadcast>,
    NameA,
>;
type EndpointC0 =
    MeshedChannelsThree<Send<(String, String), Recurs0CtoA>, End, RoleA<RoleA<RoleEnd>>, NameC>;
type EndpointS0 = MeshedChannelsThree<Recurs0StoA, End, RoleA<RoleEnd>, NameS>;

// Functions
fn endpoint_a(s: EndpointA0) -> Result<(), Box<dyn Error>> {
    let ((_id, _pw), s) = recv_mpst_a_from_c(s)?;

    if 0 == 1 {
        // actual condition id != pw
        let s = choose_mpst_multi_to_all!(
            s,
            Branching0fromAtoC::Fail,
            Branching0fromAtoS::Fail, =>
            RoleC,
            RoleS, =>
            RoleA,
            MeshedChannelsThree,
            1
        );

        let s = send_mpst_a_to_c("Fail".to_string(), s);
        let s = send_mpst_a_to_s("Fail".to_string(), s);

        close_mpst_multi(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching0fromAtoC::Login,
            Branching0fromAtoS::Login, =>
            RoleC,
            RoleS, =>
            RoleA,
            MeshedChannelsThree,
            1
        );

        let s = send_mpst_a_to_c((), s);
        let s = send_mpst_a_to_s((), s);

        recurs_a(s)
    }
}

fn recurs_a(s: EndpointA1) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_c, {
        Branching1fromCtoA::Quit(s) => {
            close_mpst_multi(s)
        },
        Branching1fromCtoA::Pay(s) => {
            recurs_a(s)
        },
    })
}

fn endpoint_s(s: EndpointS0) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_s_from_a, {
        Branching0fromAtoS::Fail(s) => {
            let (_, s) = recv_mpst_s_from_a(s)?;
            close_mpst_multi(s)
        },
        Branching0fromAtoS::Login(s) => {
            let (_, s) = recv_mpst_s_from_a(s)?;
            recurs_s(s)
        },
    })
}

fn recurs_s(s: EndpointS1) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_s_to_c(random(), s);

    offer_mpst!(s, recv_mpst_s_from_c, {
        Branching1fromCtoS::Quit(s) => {
            let (_, s) = recv_mpst_s_from_c(s)?;
            close_mpst_multi(s)
        },
        Branching1fromCtoS::Pay(s) => {
            let (_, s) = recv_mpst_s_from_c(s)?;
            recurs_s(s)
        },
    })
}

fn endpoint_c(s: EndpointC0) -> Result<(), Box<dyn Error>> {
    let id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(1)
        .map(char::from)
        .collect();

    let pw: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(1)
        .map(char::from)
        .collect();

    let s = send_mpst_c_to_a((id, pw), s);

    offer_mpst!(s, recv_mpst_c_from_a, {
        Branching0fromAtoC::Fail(s) => {
            let (_, s) = recv_mpst_c_from_a(s)?;
            close_mpst_multi(s)
        },
        Branching0fromAtoC::Login(s) => {
            let (_, s) = recv_mpst_c_from_a(s)?;
            recurs_c(s, 100)
        },
    })
}

fn recurs_c(s: EndpointC1, loops: i32) -> Result<(), Box<dyn Error>> {
    let ((balance, overdraft), s) = recv_mpst_c_from_s(s)?;

    match loops {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                Branching1fromCtoA::Quit,
                Branching1fromCtoS::Quit, =>
                RoleA,
                RoleS, =>
                RoleC,
                MeshedChannelsThree,
                2
            );

            let s = send_mpst_c_to_s((), s);

            close_mpst_multi(s)
        }
        _ => {
            let s = choose_mpst_multi_to_all!(
                s,
                Branching1fromCtoA::Pay,
                Branching1fromCtoS::Pay, =>
                RoleA,
                RoleS, =>
                RoleC,
                MeshedChannelsThree,
                2
            );

            let sum = balance + overdraft;

            let payee: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(3)
                .map(char::from)
                .collect();

            let s = send_mpst_c_to_s((payee, thread_rng().gen_range(1..=sum)), s);

            recurs_c(s, loops - 1)
        }
    }
}

fn all_mpst() -> Result<(), Box<dyn std::any::Any + std::marker::Send>> {
    let (thread_a, thread_s, thread_c) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_c),
        black_box(endpoint_s),
    );

    thread_a.join()?;
    thread_c.join()?;
    thread_s.join()?;

    Ok(())
}

/////////////////////////

fn online_wallet_main(c: &mut Criterion) {
    c.bench_function(&format!("Online wallet"), |b| b.iter(|| all_mpst()));
}

// fn long_warmup() -> Criterion {
//     Criterion::default().measurement_time(Duration::new(30, 0))
// }

criterion_group! {
    name = online_wallet;
    // config = long_warmup();
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = online_wallet_main
}

criterion_main!(online_wallet);
