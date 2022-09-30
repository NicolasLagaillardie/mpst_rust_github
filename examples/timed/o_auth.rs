#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use mpstthree::baker_timed;
use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{thread_rng, Rng};

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// See the folder scribble_protocols for the related Scribble protocol

baker_timed!(MeshedChannels, A, C, S);

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
type Choose0fromAtoC = SendTimed<Branching0fromAtoC, End, 'a', 0, true, 1, true, false>;
type Choose0fromAtoS = SendTimed<Branching0fromAtoS, End, 'a', 0, true, 1, true, false>;

// C
enum Branching0fromAtoC {
    Success(
        MeshedChannels<
            RecvTimed<Success, End, 'a', 0, true, 1, true, false>,
            SendTimed<Success, RecvTimed<Token, End, 'a', 0, true, 1, true, false>>,
            RoleA<RoleS<RoleS<RoleEnd>>>,
            NameC,
        >,
    ),
    Fail(
        MeshedChannels<
            RecvTimed<Fail, End, 'a', 0, true, 1, true, false>,
            SendTimed<Fail, RecvTimed<Received, End, 'a', 0, true, 1, true, false>>,
            RoleA<RoleS<RoleS<RoleEnd>>>,
            NameC,
        >,
    ),
}
type Offer0fromCtoA = RecvTimed<Branching0fromAtoC, End, 'a', 0, true, 1, true, false>;

// S
enum Branching0fromAtoS {
    Success(
        MeshedChannels<
            SendTimed<Token, RecvTimed<Token, End, 'a', 0, true, 1, true, false>>,
            RecvTimed<Success, SendTimed<Token, End, 'a', 0, true, 1, true, false>>,
            RoleC<RoleA<RoleA<RoleC<RoleEnd>>>>,
            NameS,
        >,
    ),
    Fail(MeshedChannels<End, RecvTimed<Fail, SendTimed<Received, End, 'a', 0, true, 1, true, false>>, RoleC<RoleC<RoleEnd>>, NameS>),
}
type Offer0fromStoA = RecvTimed<Branching0fromAtoS, End, 'a', 0, true, 1, true, false>;

// Creating the MP sessions
// A
type EndpointASuccess = MeshedChannels<
    SendTimed<Success, End, 'a', 0, true, 1, true, false>,
    RecvTimed<Token, SendTimed<Token, End, 'a', 0, true, 1, true, false>>,
    RoleC<RoleS<RoleS<RoleEnd>>>,
    NameA,
>;
type EndpointAFail = MeshedChannels<SendTimed<Fail, End, 'a', 0, true, 1, true, false>, End, RoleC<RoleEnd>, NameA>;
type EndpointA = MeshedChannels<
    RecvTimed<Login, SendTimed<Auth, RecvTimed<Password, Choose0fromAtoC, 'a', 0, true, 1, true, false>>>,
    Choose0fromAtoS,
    RoleC<RoleC<RoleC<RoleBroadcast>>>,
    NameA,
>;

// C
type EndpointC = MeshedChannels<
    SendTimed<Login, RecvTimed<Auth, SendTimed<Password, Offer0fromCtoA, 'a', 0, true, 1, true, false>>>,
    SendTimed<Start, RecvTimed<Redirect, End, 'a', 0, true, 1, true, false>>,
    RoleS<RoleS<RoleA<RoleA<RoleA<RoleA<RoleEnd>>>>>>,
    NameC,
>;

// S
type EndpointS = MeshedChannels<
    Offer0fromStoA,
    RecvTimed<Start, SendTimed<Redirect, End, 'a', 0, true, 1, true, false>>,
    RoleC<RoleC<RoleA<RoleEnd>>>,
    NameS,
>;

// Functions
fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (_, s) = s.recv()?;
    let s = s.send(Auth {})?;
    let (_, s) = s.recv()?;

    let expected: i32 = thread_rng().gen_range(1..=3);

    if 1 == expected {
        let s: EndpointASuccess =
            choose_mpst_a_to_all!(s, Branching0fromAtoC::Success, Branching0fromAtoS::Success);

        let s = s.send(Success {})?;
        let (_, s) = s.recv()?;
        let s = s.send(Token {})?;
        s.close()
    } else {
        let s: EndpointAFail =
            choose_mpst_a_to_all!(s, Branching0fromAtoC::Fail, Branching0fromAtoS::Fail);

        let s = s.send(Fail {})?;
        s.close()
    }
}

fn endpoint_c(s: EndpointC, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let s = s.send(Start {})?;
    let (_, s) = s.recv()?;
    let s = s.send(Login {})?;
    let (_, s) = s.recv()?;
    let s = s.send(Password {})?;

    offer_mpst!(s, {
        Branching0fromAtoC::Success(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(Success {  })?;
            let (_,s) = s.recv()?;
            s.close()
        },
        Branching0fromAtoC::Fail(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(Fail {  })?;
            let (_, s) = s.recv()?;
            s.close()
        },
    })
}

fn endpoint_s(s: EndpointS, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (_, s) = s.recv()?;
    let s = s.send(Redirect {})?;

    offer_mpst!(s, {
        Branching0fromAtoS::Success(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(Token {  })?;
            let (_, s) = s.recv()?;
            let s = s.send(Token {  })?;
            s.close()
        },
        Branching0fromAtoS::Fail(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(Received {  })?;
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
