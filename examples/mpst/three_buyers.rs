#![allow(clippy::type_complexity)]

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{random, thread_rng, Rng};

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for four participants
generate!("recursive", MeshedChannels, A, C, S);

// Types
// A
type Choose0fromCtoA = Send<Branching0fromCtoA, End>;
type Choose0fromCtoS = Send<Branching0fromCtoS, End>;

// A
enum Branching0fromCtoA {
    Accept(MeshedChannels<Recv<i32, End>, End, RoleC<RoleEnd>, NameA>),
    Quit(MeshedChannels<End, End, RoleEnd, NameA>),
}

// S
enum Branching0fromCtoS {
    Accept(MeshedChannels<End, Recv<i32, Send<i32, End>>, RoleC<RoleC<RoleEnd>>, NameS>),
    Quit(MeshedChannels<End, End, RoleEnd, NameS>),
}

// Creating the MP sessions
// A
type EndpointA = MeshedChannels<
    Send<i32, Recv<Branching0fromCtoA, End>>,
    Send<i32, Recv<i32, End>>,
    RoleS<RoleS<RoleC<RoleC<RoleEnd>>>>,
    NameA,
>;

// C
type EndpointC = MeshedChannels<
    Recv<i32, Choose0fromCtoA>,
    Recv<i32, Choose0fromCtoS>,
    RoleS<RoleA<RoleBroadcast>>,
    NameC,
>;
type EndpointCAccept =
    MeshedChannels<Send<i32, End>, Send<i32, Recv<i32, End>>, RoleA<RoleS<RoleS<RoleEnd>>>, NameC>;

// S
type EndpointS = MeshedChannels<
    Recv<i32, Send<i32, End>>,
    Send<i32, Recv<Branching0fromCtoS, End>>,
    RoleA<RoleA<RoleC<RoleC<RoleEnd>>>>,
    NameS,
>;

// Functions
fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    let s = s.send(random());
    let (_empty2, s) = s.recv();
    let s = s.send(random());
    offer_mpst!(s, {
        Branching0fromCtoA::Accept(s) => {
            let (_ok, s) = s.recv();
            s.close()
        },
        Branching0fromCtoA::Quit(s) => {
            s.close()
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let (_empty3, s) = s.recv();
    let (_empty4, s) = s.recv();

    let choice: i32 = thread_rng().gen_range(1..=3);

    if choice != 1 {
        let s: EndpointCAccept =
            choose_mpst_c_to_all!(s, Branching0fromCtoA::Accept, Branching0fromCtoS::Accept);

        let s = s.send(random());
        let s = s.send(random());
        let (_empty5, s) = s.recv();

        s.close()
    } else {
        let s = choose_mpst_c_to_all!(s, Branching0fromCtoA::Quit, Branching0fromCtoS::Quit);
        s.close()
    }
}

fn endpoint_s(s: EndpointS) -> Result<(), Box<dyn Error>> {
    let (_empty1, s) = s.recv();
    let s = s.send(random());
    let s = s.send(random());
    offer_mpst!(s, {
        Branching0fromCtoS::Accept(s) => {
            let (_ok, s) = s.recv();
            let s = s.send(random());
            s.close()
        },
        Branching0fromCtoS::Quit(s) => {
            s.close()
        },
    })
}

/////////////////////////////////////////

fn main() {
    let (thread_a, thread_c, thread_s) = fork_mpst(endpoint_a, endpoint_c, endpoint_s);

    thread_a.join().unwrap();
    thread_c.join().unwrap();
    thread_s.join().unwrap();
}
