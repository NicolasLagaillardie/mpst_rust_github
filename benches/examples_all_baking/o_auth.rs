#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{thread_rng, Rng};

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

generate!("rec_and_cancel", MeshedChannels, A, C, S);

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
type Choose0fromAtoC = <Choice0fromCtoA as Session>::Dual;
type Choose0fromAtoS = <Choice0fromStoA as Session>::Dual;

// C
enum Branching0fromAtoC {
    Success(
        MeshedChannels<
            Recv<Success, End>,
            Send<Success, Recv<Token, End>>,
            RoleA<RoleS<RoleS<RoleEnd>>>,
            NameC,
        >,
    ),
    Fail(
        MeshedChannels<
            Recv<Fail, End>,
            Send<Fail, Recv<Received, End>>,
            RoleA<RoleS<RoleS<RoleEnd>>>,
            NameC,
        >,
    ),
}
type Choice0fromCtoA = Recv<Branching0fromAtoC, End>;

// S
enum Branching0fromAtoS {
    Success(
        MeshedChannels<
            Send<Token, Recv<Token, End>>,
            Recv<Success, Send<Token, End>>,
            RoleC<RoleA<RoleA<RoleC<RoleEnd>>>>,
            NameS,
        >,
    ),
    Fail(MeshedChannels<End, Recv<Fail, Send<Received, End>>, RoleC<RoleC<RoleEnd>>, NameS>),
}
type Choice0fromStoA = Recv<Branching0fromAtoS, End>;

// Creating the MP sessions
// A
type EndpointASuccess = MeshedChannels<
    Send<Success, End>,
    Recv<Token, Send<Token, End>>,
    RoleC<RoleS<RoleS<RoleEnd>>>,
    NameA,
>;
type EndpointAFail = MeshedChannels<Send<Fail, End>, End, RoleC<RoleEnd>, NameA>;
type EndpointA = MeshedChannels<
    Recv<Login, Send<Auth, Recv<Password, Choose0fromAtoC>>>,
    Choose0fromAtoS,
    RoleC<RoleC<RoleC<RoleBroadcast>>>,
    NameA,
>;

// C
type EndpointC = MeshedChannels<
    Send<Login, Recv<Auth, Send<Password, Choice0fromCtoA>>>,
    Send<Start, Recv<Redirect, End>>,
    RoleS<RoleS<RoleA<RoleA<RoleA<RoleA<RoleEnd>>>>>>,
    NameC,
>;

// S
type EndpointS = MeshedChannels<
    Choice0fromStoA,
    Recv<Start, Send<Redirect, End>>,
    RoleC<RoleC<RoleA<RoleEnd>>>,
    NameS,
>;

// Functions
fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
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

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let s = s.send(Start {})?;
    let (_, s) = s.recv()?;
    let s = s.send(Login {})?;
    let (_, s) = s.recv()?;
    let s = s.send(Password {})?;

    offer_mpst!(s, {
        Branching0fromAtoC::Success(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(Success {  })?;
            let (_, s) = s.recv()?;
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

fn endpoint_s(s: EndpointS) -> Result<(), Box<dyn Error>> {
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

fn aux() {
    let (thread_a, thread_c, thread_s) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_c),
        black_box(endpoint_s),
    );

    thread_a.join().unwrap();
    thread_c.join().unwrap();
    thread_s.join().unwrap();
}

/////////////////////////

pub fn o_auth(c: &mut Criterion) {
    c.bench_function("oAuth", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(20000);
    targets = o_auth,
}

criterion_main! {
    bench
}
