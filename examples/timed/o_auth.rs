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

use rand::{thread_rng, Rng};

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// See the folder scribble_protocols for the related Scribble protocol

generate_timed!(MeshedChannels, A, C, S);

// Payloads
struct Start;
struct Redirect;
struct Login;
struct Auth;
struct Password;
struct Success;
struct Token;
struct Fail;
struct Received;

// Types

// A
type Choose0fromAtoC = SendTimed<Branching0fromAtoC, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromAtoS = SendTimed<Branching0fromAtoS, 'a', 0, true, 10, true, ' ', End>;

// C
enum Branching0fromAtoC {
    Success(
        MeshedChannels<
            RecvTimed<Success, 'a', 0, true, 10, true, ' ', End>,
            SendTimed<
                Success,
                'a',
                0,
                true,
                10,
                true,
                ' ',
                RecvTimed<Token, 'a', 0, true, 10, true, ' ', End>,
            >,
            RoleA<RoleS<RoleS<RoleEnd>>>,
            NameC,
        >,
    ),
    Fail(
        MeshedChannels<
            RecvTimed<Fail, 'a', 0, true, 10, true, ' ', End>,
            SendTimed<
                Fail,
                'a',
                0,
                true,
                10,
                true,
                ' ',
                RecvTimed<Received, 'a', 0, true, 10, true, ' ', End>,
            >,
            RoleA<RoleS<RoleS<RoleEnd>>>,
            NameC,
        >,
    ),
}
type Offer0fromCtoA = RecvTimed<Branching0fromAtoC, 'a', 0, true, 10, true, ' ', End>;

// S
enum Branching0fromAtoS {
    Success(
        MeshedChannels<
            SendTimed<
                Token,
                'a',
                0,
                true,
                10,
                true,
                ' ',
                RecvTimed<Token, 'a', 0, true, 10, true, ' ', End>,
            >,
            RecvTimed<
                Success,
                'a',
                0,
                true,
                10,
                true,
                ' ',
                SendTimed<Token, 'a', 0, true, 10, true, ' ', End>,
            >,
            RoleC<RoleA<RoleA<RoleC<RoleEnd>>>>,
            NameS,
        >,
    ),
    Fail(
        MeshedChannels<
            End,
            RecvTimed<
                Fail,
                'a',
                0,
                true,
                10,
                true,
                ' ',
                SendTimed<Received, 'a', 0, true, 10, true, ' ', End>,
            >,
            RoleC<RoleC<RoleEnd>>,
            NameS,
        >,
    ),
}
type Offer0fromStoA = RecvTimed<Branching0fromAtoS, 'a', 0, true, 10, true, ' ', End>;

// Creating the MP sessions
// A
type EndpointASuccess = MeshedChannels<
    SendTimed<Success, 'a', 0, true, 10, true, ' ', End>,
    RecvTimed<
        Token,
        'a',
        0,
        true,
        10,
        true,
        ' ',
        SendTimed<Token, 'a', 0, true, 10, true, ' ', End>,
    >,
    RoleC<RoleS<RoleS<RoleEnd>>>,
    NameA,
>;
type EndpointAFail =
    MeshedChannels<SendTimed<Fail, 'a', 0, true, 10, true, ' ', End>, End, RoleC<RoleEnd>, NameA>;
type EndpointA = MeshedChannels<
    RecvTimed<
        Login,
        'a',
        0,
        true,
        10,
        true,
        ' ',
        SendTimed<
            Auth,
            'a',
            0,
            true,
            10,
            true,
            ' ',
            RecvTimed<Password, 'a', 0, true, 10, true, ' ', Choose0fromAtoC>,
        >,
    >,
    Choose0fromAtoS,
    RoleC<RoleC<RoleC<RoleBroadcast>>>,
    NameA,
>;

// C
type EndpointC = MeshedChannels<
    SendTimed<
        Login,
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<
            Auth,
            'a',
            0,
            true,
            10,
            true,
            ' ',
            SendTimed<Password, 'a', 0, true, 10, true, ' ', Offer0fromCtoA>,
        >,
    >,
    SendTimed<
        Start,
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<Redirect, 'a', 0, true, 10, true, ' ', End>,
    >,
    RoleS<RoleS<RoleA<RoleA<RoleA<RoleA<RoleEnd>>>>>>,
    NameC,
>;

// S
type EndpointS = MeshedChannels<
    Offer0fromStoA,
    RecvTimed<
        Start,
        'a',
        0,
        true,
        10,
        true,
        ' ',
        SendTimed<Redirect, 'a', 0, true, 10, true, ' ', End>,
    >,
    RoleC<RoleC<RoleA<RoleEnd>>>,
    NameS,
>;

// Functions
fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (_, s) = s.recv(all_clocks)?;
    let s = s.send(Auth {}, all_clocks)?;
    let (_, s) = s.recv(all_clocks)?;

    let expected: i32 = thread_rng().gen_range(1..=3);

    if 1 == expected {
        let s: EndpointASuccess = choose_mpst_a_to_all!(
            s,
            all_clocks,
            Branching0fromAtoC::Success,
            Branching0fromAtoS::Success
        );

        let s = s.send(Success {}, all_clocks)?;
        let (_, s) = s.recv(all_clocks)?;
        let s = s.send(Token {}, all_clocks)?;
        s.close()
    } else {
        let s: EndpointAFail = choose_mpst_a_to_all!(
            s,
            all_clocks,
            Branching0fromAtoC::Fail,
            Branching0fromAtoS::Fail
        );

        let s = s.send(Fail {}, all_clocks)?;
        s.close()
    }
}

fn endpoint_c(s: EndpointC, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let s = s.send(Start {}, all_clocks)?;
    let (_, s) = s.recv(all_clocks)?;
    let s = s.send(Login {}, all_clocks)?;
    let (_, s) = s.recv(all_clocks)?;
    let s = s.send(Password {}, all_clocks)?;

    offer_mpst!(s, all_clocks, {
        Branching0fromAtoC::Success(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send(Success {  }, all_clocks)?;
            let (_,s) = s.recv(all_clocks)?;
            s.close()
        },
        Branching0fromAtoC::Fail(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send(Fail {  }, all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            s.close()
        },
    })
}

fn endpoint_s(s: EndpointS, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (_, s) = s.recv(all_clocks)?;
    let s = s.send(Redirect {}, all_clocks)?;

    offer_mpst!(s, all_clocks, {
        Branching0fromAtoS::Success(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send(Token {  }, all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send(Token {  }, all_clocks)?;
            s.close()
        },
        Branching0fromAtoS::Fail(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send(Received {  }, all_clocks)?;
            s.close()
        },
    })
}

fn main() {
    let (thread_a, thread_c, thread_s) = fork_mpst(endpoint_a, endpoint_c, endpoint_s);

    assert!(thread_a.join().is_ok());
    assert!(thread_c.join().is_ok());
    assert!(thread_s.join().is_ok());
}
