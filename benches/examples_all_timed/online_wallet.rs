#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use criterion::{black_box, Criterion};

use mpstthree::baker_timed;
use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// See the folder scribble_protocols for the related Scribble protocol

// Create the new MeshedChannels for three participants and the close and fork functions
baker_timed!(MeshedChannels, A, C, S);

// Types
// A
type Choose0fromAtoS = SendTimed<Branching0fromAtoS, End, 'a', 0, true, 1, true, false>;
type Choose0fromAtoC = SendTimed<Branching0fromAtoC, End, 'a', 0, true, 1, true, false>;

enum Branching1fromCtoA {
    Pay(MeshedChannels<Recurs1AtoC, End, RoleC<RoleEnd>, NameA>),
    Quit(MeshedChannels<End, End, RoleEnd, NameA>),
}
type Recurs1AtoC = RecvTimed<Branching1fromCtoA, End, 'a', 0, true, 1, true, false>;

// S
enum Branching0fromAtoS {
    Login(
        MeshedChannels<
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            SDoubleRecurs1StoC,
            RoleACC,
            NameS,
        >,
    ),
    Fail(
        MeshedChannels<
            RecvTimed<String, End, 'a', 0, true, 1, true, false>,
            End,
            RoleA<RoleEnd>,
            NameS,
        >,
    ),
}
type RoleACC = RoleA<RoleC<RoleC<RoleEnd>>>;
type Recurs0StoA = RecvTimed<Branching0fromAtoS, End, 'a', 0, true, 1, true, false>;

enum Branching1fromCtoS {
    Pay(
        MeshedChannels<
            End,
            RecvTimed<(String, i32), SDoubleRecurs1StoC, 'a', 0, true, 1, true, false>,
            RoleCCC,
            NameS,
        >,
    ),
    Quit(
        MeshedChannels<
            End,
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            RoleC<RoleEnd>,
            NameS,
        >,
    ),
}
type RoleCCC = RoleC<RoleC<RoleC<RoleEnd>>>;
type Recurs1StoC = RecvTimed<Branching1fromCtoS, End, 'a', 0, true, 1, true, false>;

// C
enum Branching0fromAtoC {
    Login(MeshedChannels<RChoose1fromCtoA, RDoubleChoose1fromCtoS, RoleASBroad, NameC>),
    Fail(
        MeshedChannels<
            RecvTimed<String, End, 'a', 0, true, 1, true, false>,
            End,
            RoleA<RoleEnd>,
            NameC,
        >,
    ),
}
type RChoose1fromCtoA = RecvTimed<(), Choose1fromCtoA, 'a', 0, true, 1, true, false>;
type RDoubleChoose1fromCtoS = RecvTimed<(i32, i32), Choose1fromCtoS, 'a', 0, true, 1, true, false>;
type RoleASBroad = RoleA<RoleS<RoleBroadcast>>;
type Recurs0CtoA = RecvTimed<Branching0fromAtoC, End, 'a', 0, true, 1, true, false>;

type Choose1fromCtoA = SendTimed<Branching1fromCtoA, End, 'a', 0, true, 1, true, false>;
type Choose1fromCtoS = SendTimed<Branching1fromCtoS, End, 'a', 0, true, 1, true, false>;

// Creating the MP sessions
// Step 1_1
type EndpointC1Quit =
    MeshedChannels<End, SendTimed<(), End, 'a', 0, true, 1, true, false>, RoleS<RoleEnd>, NameC>;
type EndpointC1Pay = MeshedChannels<
    SendTimed<Branching1fromCtoA, End, 'a', 0, true, 1, true, false>,
    SendTimed<
        (String, i32),
        RecvTimed<
            (i32, i32),
            SendTimed<Branching1fromCtoS, End, 'a', 0, true, 1, true, false>,
            'a',
            0,
            true,
            1,
            true,
            false,
        >,
        'a',
        0,
        true,
        1,
        true,
        false,
    >,
    RoleS<RoleS<RoleBroadcast>>,
    NameC,
>;

// Step 1
type EndpointA1 = MeshedChannels<Recurs1AtoC, End, RoleC<RoleEnd>, NameA>;
type EndpointC1 =
    MeshedChannels<Choose1fromCtoA, RDoubleChoose1fromCtoS, RoleS<RoleBroadcast>, NameC>;
type EndpointS1 = MeshedChannels<End, SDoubleRecurs1StoC, RoleC<RoleC<RoleEnd>>, NameS>;
type SDoubleRecurs1StoC = SendTimed<(i32, i32), Recurs1StoC, 'a', 0, true, 1, true, false>;

// Step 0_1
type EndpointA0_1Fail = MeshedChannels<
    SendTimed<String, End, 'a', 0, true, 1, true, false>,
    SendTimed<String, End, 'a', 0, true, 1, true, false>,
    RoleC<RoleS<RoleEnd>>,
    NameA,
>;
type EndpointA0_1Login = MeshedChannels<
    SendTimed<
        (),
        RecvTimed<Branching1fromCtoA, End, 'a', 0, true, 1, true, false>,
        'a',
        0,
        true,
        1,
        true,
        false,
    >,
    SendTimed<(), End, 'a', 0, true, 1, true, false>,
    RoleC<RoleS<RoleC<RoleEnd>>>,
    NameA,
>;

// Step 0
type EndpointA0 = MeshedChannels<
    RecvTimed<(String, String), Choose0fromAtoC, 'a', 0, true, 1, true, false>,
    Choose0fromAtoS,
    RoleC<RoleBroadcast>,
    NameA,
>;
type EndpointC0 = MeshedChannels<
    SendTimed<(String, String), Recurs0CtoA, 'a', 0, true, 1, true, false>,
    End,
    RoleA<RoleA<RoleEnd>>,
    NameC,
>;
type EndpointS0 = MeshedChannels<Recurs0StoA, End, RoleA<RoleEnd>, NameS>;

// Functions
fn endpoint_a(
    s: EndpointA0,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let ((id, pw), s) = s.recv(all_clocks)?;

    if id != pw {
        let s: EndpointA0_1Fail = choose_mpst_a_to_all!(
            s,
            all_clocks,
            Branching0fromAtoC::Fail,
            Branching0fromAtoS::Fail,
        );

        let s = s.send("Fail".to_string(), all_clocks)?;
        let s = s.send("Fail".to_string(), all_clocks)?;

        s.close()
    } else {
        let s: EndpointA0_1Login = choose_mpst_a_to_all!(
            s,
            all_clocks,
            Branching0fromAtoC::Login,
            Branching0fromAtoS::Login,
        );

        let s = s.send((), all_clocks)?;
        let s = s.send((), all_clocks)?;

        recurs_a(s, all_clocks)
    }
}

fn recurs_a(s: EndpointA1, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branching1fromCtoA::Quit(s) => {
            s.close()
        },
        Branching1fromCtoA::Pay(s) => {
            recurs_a(s, all_clocks)
        },
    })
}

fn endpoint_s(
    s: EndpointS0,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    offer_mpst!(s, all_clocks, {
        Branching0fromAtoS::Fail(s) => {
            let (_, s) = s.recv(all_clocks)?;
            s.close()
        },
        Branching0fromAtoS::Login(s) => {
            let (_, s) = s.recv(all_clocks)?;
            recurs_s(s, all_clocks)
        },
    })
}

fn recurs_s(s: EndpointS1, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    let s = s.send((1, 1), all_clocks)?;

    offer_mpst!(s, all_clocks, {
        Branching1fromCtoS::Quit(s) => {
            let (_, s) = s.recv(all_clocks)?;
            s.close()
        },
        Branching1fromCtoS::Pay(s) => {
            let (_, s) = s.recv(all_clocks)?;
            recurs_s(s, all_clocks)
        },
    })
}

fn endpoint_c(
    s: EndpointC0,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let id = String::from("id");
    let pw = String::from("pw");

    let s = s.send((id, pw), all_clocks)?;

    offer_mpst!(s, all_clocks, {
        Branching0fromAtoC::Fail(s) => {
            let (_, s) = s.recv(all_clocks)?;
            s.close()
        },
        Branching0fromAtoC::Login(s) => {
            let (_, s) = s.recv(all_clocks)?;
            recurs_c(s, 100, all_clocks)
        },
    })
}

fn recurs_c(
    s: EndpointC1,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    let ((balance, overdraft), s) = s.recv(all_clocks)?;

    match loops {
        0 => {
            let s: EndpointC1Quit = choose_mpst_c_to_all!(
                s,
                all_clocks,
                Branching1fromCtoA::Quit,
                Branching1fromCtoS::Quit,
            );

            let s = s.send((), all_clocks)?;

            s.close()
        }
        _ => {
            let s: EndpointC1Pay = choose_mpst_c_to_all!(
                s,
                all_clocks,
                Branching1fromCtoA::Pay,
                Branching1fromCtoS::Pay,
            );

            let sum = balance + overdraft;

            let payee = String::from("payee");

            let s = s.send((payee, sum), all_clocks)?;

            recurs_c(s, loops - 1, all_clocks)
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

pub fn online_wallet_main(c: &mut Criterion) {
    c.bench_function("Timed Online wallet baking", |b| b.iter(all_mpst));
}
