#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{bundle_impl_with_enum_and_cancel, offer_mpst};

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for four participants
bundle_impl_with_enum_and_cancel!(MeshedChannelsThree, A, C, S);

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
type EndpointC1Pay = MeshedChannelsThree<
    Choose1fromCtoA,
    Send<(String, i64), Recv<(i64, i64), Choose1fromCtoS>>,
    RoleS<RoleS<RoleBroadcast>>,
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
type EndpointA0Fail =
    MeshedChannelsThree<Send<String, End>, Send<String, End>, RoleC<RoleS<RoleEnd>>, NameA>;
type EndpointA0Login =
    MeshedChannelsThree<Send<(), Recurs1AtoC>, Send<(), End>, RoleC<RoleS<RoleC<RoleEnd>>>, NameA>;
type EndpointC0 =
    MeshedChannelsThree<Send<(String, String), Recurs0CtoA>, End, RoleA<RoleA<RoleEnd>>, NameC>;
type EndpointS0 = MeshedChannelsThree<Recurs0StoA, End, RoleA<RoleEnd>, NameS>;

// Functions
fn endpoint_a(s: EndpointA0) -> Result<(), Box<dyn Error>> {
    let ((_id, _pw), s) = s.recv()?;

    if 0 == 1 {
        // actual condition id != pw
        let s: EndpointA0Fail =
            choose_mpst_a_to_all!(s, Branching0fromAtoC::Fail, Branching0fromAtoS::Fail);

        let s = s.send("Fail".to_string())?;
        let s = s.send("Fail".to_string())?;

        s.close()
    } else {
        let s: EndpointA0Login =
            choose_mpst_a_to_all!(s, Branching0fromAtoC::Login, Branching0fromAtoS::Login);

        let s = s.send(())?;
        let s = s.send(())?;

        recurs_a(s)
    }
}

fn recurs_a(s: EndpointA1) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching1fromCtoA::Quit(s) => {
            s.close()
        },
        Branching1fromCtoA::Pay(s) => {
            recurs_a(s)
        },
    })
}

fn endpoint_s(s: EndpointS0) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromAtoS::Fail(s) => {
            let (_, s) = s.recv()?;
            s.close()
        },
        Branching0fromAtoS::Login(s) => {
            let (_, s) = s.recv()?;
            recurs_s(s)
        },
    })
}

fn recurs_s(s: EndpointS1) -> Result<(), Box<dyn Error>> {
    let s = s.send((1, 1))?;

    offer_mpst!(s, {
        Branching1fromCtoS::Quit(s) => {
            let (_, s) = s.recv()?;
            s.close()
        },
        Branching1fromCtoS::Pay(s) => {
            let (_, s) = s.recv()?;
            recurs_s(s)
        },
    })
}

fn endpoint_c(s: EndpointC0) -> Result<(), Box<dyn Error>> {
    let id = String::from("id");
    let pw = String::from("pw");

    let s = s.send((id, pw))?;

    offer_mpst!(s, {
        Branching0fromAtoC::Fail(s) => {
            let (_, s) = s.recv()?;
            s.close()
        },
        Branching0fromAtoC::Login(s) => {
            let (_, s) = s.recv()?;
            recurs_c(s, 100)
        },
    })
}

fn recurs_c(s: EndpointC1, loops: i32) -> Result<(), Box<dyn Error>> {
    let ((balance, overdraft), s) = s.recv()?;

    match loops {
        0 => {
            let s = choose_mpst_c_to_all!(s, Branching1fromCtoA::Quit, Branching1fromCtoS::Quit);

            let s = s.send(())?;

            s.close()
        }
        _ => {
            let s: EndpointC1Pay =
                choose_mpst_c_to_all!(s, Branching1fromCtoA::Pay, Branching1fromCtoS::Pay);

            let sum = balance + overdraft;

            let payee = String::from("payee");

            let s = s.send((payee, sum))?;

            recurs_c(s, loops - 1)
        }
    }
}

fn all_mpst() {
    let (thread_a, thread_s, thread_c) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_c),
        black_box(endpoint_s),
    );

    thread_a.join().unwrap();
    thread_c.join().unwrap();
    thread_s.join().unwrap();
}

/////////////////////////

fn online_wallet_main(c: &mut Criterion) {
    c.bench_function(&format!("Online wallet baking"), |b| b.iter(|| all_mpst()));
}

criterion_group! {
    name = online_wallet;
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = online_wallet_main
}

criterion_main!(online_wallet);
