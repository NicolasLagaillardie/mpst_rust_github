#![allow(clippy::type_complexity)]

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::{choose, offer};

use std::error::Error;
use std::thread::{spawn, JoinHandle};

// S
enum BinaryS {
    Sum(Send<(), End>),
    Diff(Send<(), End>),
}
type OfferS = Recv<BinaryS, Recv<BinaryS, Recv<BinaryS, End>>>;

fn binary_a_to_b(s: OfferS) -> Result<(), Box<dyn Error>> {
    let (elt_1, s) = recv(s)?;
    let (elt_2, s) = recv(s)?;

    offer!(s, {
        BinaryS::Sum(s) => {
            let s = send(elt_1 + elt_2, s);
            close(s)
        },
        BinaryS::Diff(s) => {
            let s = send(elt_1 - elt_2, s);
            close(s)
        },
    })
}

// C
type RecursC = <OfferS as Session>::Dual;

fn binary_b_to_a(s: Send<(), Recv<(), RecursC>>) -> Result<RecursC, Box<dyn Error>> {
    let s = send((), s);
    let (_, s) = recv(s)?;
    Ok(s)
}

fn main() {
    let mut threads = Vec::new();
    let mut sessions = Vec::new();

    for _ in 0..28 {
        let (thread, s): (JoinHandle<()>, RecursB) = fork_with_thread_id(binary_a_to_b);

        threads.push(thread);
        sessions.push(s);
    }

    let main = spawn(move || {
        sessions = sessions
            .into_iter()
            .map(|s| binary_b_to_a(choose!(BinaryS::More, s)).unwrap())
            .collect::<Vec<_>>();

        sessions
            .into_iter()
            .for_each(|s| close(choose!(BinaryS::Done, s)).unwrap());

        threads.into_iter().for_each(|elt| elt.join().unwrap());
    });

    main.join().unwrap();
}

// Types
// A
type Choose0fromCtoA = Send<Branching0fromCtoA, End>;
type Choose0fromCtoS = Send<Branching0fromCtoS, End>;

// A
enum Branching0fromCtoA {
    Sum(MeshedChannels<End, End, RoleEnd, NameA>),
    Diff(MeshedChannels<End, End, RoleEnd, NameA>),
}

// S
enum Branching0fromCtoS {
    Sum(MeshedChannels<End, Send<i32, End>, RoleC<RoleEnd>, NameS>),
    Diff(MeshedChannels<End, Send<i32, End>, RoleC<RoleEnd>, NameS>),
}

// Creating the MP sessions
// A
type EndpointA = MeshedChannels<Recv<Branching0fromCtoA, End>, End, RoleC<RoleEnd>, NameA>;

// C
type EndpointC = MeshedChannels<
    Choose0fromCtoA,
    Send<i32, Send<i32, Choose0fromCtoS>>,
    RoleS<RoleS<RoleBroadcast>>,
    NameC,
>;
type EndpointCSum = MeshedChannels<End, Recv<i32, End>, RoleS<RoleEnd>, NameC>;
type EndpointCDiff = MeshedChannels<End, Recv<i32, End>, RoleS<RoleEnd>, NameC>;

// S
type EndpointS = MeshedChannels<
    End,
    Recv<i32, Recv<i32, Recv<Branching0fromCtoS, End>>>,
    RoleC<RoleC<RoleC<RoleEnd>>>,
    NameS,
>;

// Functions
fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromCtoA::Sum(s) => {
            s.close()
        },
        Branching0fromCtoA::Diff(s) => {
            s.close()
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let s = s.send(thread_rng().gen_range(1..=100))?;
    let s = s.send(thread_rng().gen_range(1..=100))?;

    let choice: i32 = thread_rng().gen_range(1..=2);

    if choice != 1 {
        let s: EndpointCSum =
            choose_mpst_c_to_all!(s, Branching0fromCtoA::Sum, Branching0fromCtoS::Sum);

        let (_sum, s) = s.recv()?;

        s.close()
    } else {
        let s: EndpointCDiff =
            choose_mpst_c_to_all!(s, Branching0fromCtoA::Diff, Branching0fromCtoS::Diff);

        let (_diff, s) = s.recv()?;

        s.close()
    }
}

fn endpoint_s(s: EndpointS) -> Result<(), Box<dyn Error>> {
    let (elt_1, s) = s.recv()?;
    let (elt_2, s) = s.recv()?;

    offer_mpst!(s, {
        Branching0fromCtoS::Sum(s) => {
            let s = s.send(elt_1 + elt_2)?;
            s.close()
        },
        Branching0fromCtoS::Diff(s) => {
            let s = s.send(elt_1 - elt_2)?;
            s.close()
        },
    })
}

fn main() {
    let (thread_c, thread_s) = fork_mpst(endpoint_c, endpoint_s);

    thread_c.join().unwrap();
    thread_s.join().unwrap();
}
