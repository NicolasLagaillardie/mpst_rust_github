#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_timed;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{distributions::Alphanumeric, random, thread_rng, Rng};

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// See the folder scribble_protocols for the related Scribble protocol

// Create the new MeshedChannels for three participants and the close and fork functions
generate_timed!(MeshedChannels, A, C, S);

// Types
// A
type Choose0fromAtoS = SendTimed<Branching0fromAtoS, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromAtoC = SendTimed<Branching0fromAtoC, 'a', 0, true, 10, true, ' ', End>;

enum Branching1fromCtoA {
    Pay(MeshedChannels<Recurs1AtoC, End, RoleC<RoleEnd>, NameA>),
    Quit(MeshedChannels<End, End, RoleEnd, NameA>),
}
type Recurs1AtoC = RecvTimed<Branching1fromCtoA, 'a', 0, true, 10, true, ' ', End>;

// S
enum Branching0fromAtoS {
    Login(
        MeshedChannels<
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SDoubleRecurs1StoC,
            RoleACC,
            NameS,
        >,
    ),
    Fail(
        MeshedChannels<
            RecvTimed<String, 'a', 0, true, 10, true, ' ', End>,
            End,
            RoleA<RoleEnd>,
            NameS,
        >,
    ),
}
type RoleACC = RoleA<RoleC<RoleC<RoleEnd>>>;
type Recurs0StoA = RecvTimed<Branching0fromAtoS, 'a', 0, true, 10, true, ' ', End>;

enum Branching1fromCtoS {
    Pay(
        MeshedChannels<
            End,
            RecvTimed<(String, i32), 'a', 0, true, 10, true, ' ', SDoubleRecurs1StoC>,
            RoleCCC,
            NameS,
        >,
    ),
    Quit(
        MeshedChannels<End, RecvTimed<(), 'a', 0, true, 10, true, ' ', End>, RoleC<RoleEnd>, NameS>,
    ),
}
type RoleCCC = RoleC<RoleC<RoleC<RoleEnd>>>;
type Recurs1StoC = RecvTimed<Branching1fromCtoS, 'a', 0, true, 10, true, ' ', End>;

// C
enum Branching0fromAtoC {
    Login(MeshedChannels<RChoose1fromCtoA, RDoubleChoose1fromCtoS, RoleASBroad, NameC>),
    Fail(
        MeshedChannels<
            RecvTimed<String, 'a', 0, true, 10, true, ' ', End>,
            End,
            RoleA<RoleEnd>,
            NameC,
        >,
    ),
}
type RChoose1fromCtoA = RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose1fromCtoA>;
type RDoubleChoose1fromCtoS = RecvTimed<(i32, i32), 'a', 0, true, 10, true, ' ', Choose1fromCtoS>;
type RoleASBroad = RoleA<RoleS<RoleBroadcast>>;
type Recurs0CtoA = RecvTimed<Branching0fromAtoC, 'a', 0, true, 10, true, ' ', End>;

type Choose1fromCtoA = SendTimed<Branching1fromCtoA, 'a', 0, true, 10, true, ' ', End>;
type Choose1fromCtoS = SendTimed<Branching1fromCtoS, 'a', 0, true, 10, true, ' ', End>;

// Creating the MP sessions
// Step 1_1
type EndpointC11Quit =
    MeshedChannels<End, SendTimed<(), 'a', 0, true, 10, true, ' ', End>, RoleS<RoleEnd>, NameC>;
type EndpointC11Pay = MeshedChannels<
    SendTimed<Branching1fromCtoA, 'a', 0, true, 10, true, ' ', End>,
    SendTimed<
        (String, i32),
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<
            (i32, i32),
            'a',
            0,
            true,
            10,
            true,
            ' ',
            SendTimed<Branching1fromCtoS, 'a', 0, true, 10, true, ' ', End>,
        >,
    >,
    RoleS<RoleS<RoleBroadcast>>,
    NameC,
>;

// Step 1
type EndpointA1 = MeshedChannels<Recurs1AtoC, End, RoleC<RoleEnd>, NameA>;
type EndpointC1 =
    MeshedChannels<Choose1fromCtoA, RDoubleChoose1fromCtoS, RoleS<RoleBroadcast>, NameC>;
type EndpointS1 = MeshedChannels<End, SDoubleRecurs1StoC, RoleC<RoleC<RoleEnd>>, NameS>;
type SDoubleRecurs1StoC = SendTimed<(i32, i32), 'a', 0, true, 10, true, ' ', Recurs1StoC>;

// Step 0_1
type EndpointA0_1Fail = MeshedChannels<
    SendTimed<String, 'a', 0, true, 10, true, ' ', End>,
    SendTimed<String, 'a', 0, true, 10, true, ' ', End>,
    RoleC<RoleS<RoleEnd>>,
    NameA,
>;
type EndpointA0_1Login = MeshedChannels<
    SendTimed<
        (),
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<Branching1fromCtoA, 'a', 0, true, 10, true, ' ', End>,
    >,
    SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
    RoleC<RoleS<RoleC<RoleEnd>>>,
    NameA,
>;

// Step 0
type EndpointA0 = MeshedChannels<
    RecvTimed<(String, String), 'a', 0, true, 10, true, ' ', Choose0fromAtoC>,
    Choose0fromAtoS,
    RoleC<RoleBroadcast>,
    NameA,
>;
type EndpointC0 = MeshedChannels<
    SendTimed<(String, String), 'a', 0, true, 10, true, ' ', Recurs0CtoA>,
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
    let s = s.send(random(), all_clocks)?;

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

    let id: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(1)
        .map(char::from)
        .collect();

    let pw: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(1)
        .map(char::from)
        .collect();

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
            let s: EndpointC11Quit = choose_mpst_c_to_all!(
                s,
                all_clocks,
                Branching1fromCtoA::Quit,
                Branching1fromCtoS::Quit,
            );

            let s = s.send((), all_clocks)?;

            s.close()
        }
        _ => {
            let s: EndpointC11Pay = choose_mpst_c_to_all!(
                s,
                all_clocks,
                Branching1fromCtoA::Pay,
                Branching1fromCtoS::Pay,
            );

            let payee: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(3)
                .map(char::from)
                .collect();

            let s = s.send((payee, balance + overdraft), all_clocks)?;

            recurs_c(s, loops - 1, all_clocks)
        }
    }
}

fn main() {
    let (thread_a, thread_s, thread_c) = fork_mpst(endpoint_a, endpoint_c, endpoint_s);

    assert!(thread_a.join().is_ok());
    assert!(thread_c.join().is_ok());
    assert!(thread_s.join().is_ok());
}
