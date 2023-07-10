#![allow(clippy::type_complexity)]

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{thread_rng, Rng};

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for four participants
generate!("rec_and_cancel", MeshedChannelsThree, A, C, S);

// Types
// C0
type Choose0fromCtoA = Send<Branching0fromCtoA, End>;
type Choose0fromCtoS = Send<Branching0fromCtoS, End>;

// C1
type Choose1fromCtoA = <Choice1fromCtoA as Session>::Dual;
type Choose1fromCtoS = <Choice1fromCtoS as Session>::Dual;

// A
enum Branching0fromCtoA {
    Select(MeshedChannelsThree<Choice1fromCtoA, End, RoleC<RoleEnd>, NameA>),
    Loop(
        MeshedChannelsThree<
            Recv<i32, Send<i32, Choice0fromCtoA>>,
            Send<i32, End>,
            RoleC<RoleC<RoleS<RoleC<RoleEnd>>>>,
            NameA,
        >,
    ),
}
type Choice0fromCtoA = Recv<Branching0fromCtoA, End>;
enum Branching1fromCtoA {
    Yes(MeshedChannelsThree<Recv<i32, End>, Send<i32, End>, RoleC<RoleS<RoleEnd>>, NameA>),
    No(MeshedChannelsThree<Recv<i32, End>, Send<i32, End>, RoleC<RoleS<RoleEnd>>, NameA>),
}
type Choice1fromCtoA = Recv<Branching1fromCtoA, End>;

// S
enum Branching0fromCtoS {
    Select(MeshedChannelsThree<End, Choice1fromCtoS, RoleC<RoleEnd>, NameS>),
    Loop(MeshedChannelsThree<Recv<i32, End>, Choice0fromCtoS, RoleA<RoleC<RoleEnd>>, NameS>),
}
type Choice0fromCtoS = Recv<Branching0fromCtoS, End>;
enum Branching1fromCtoS {
    Yes(
        MeshedChannelsThree<
            Recv<i32, End>,
            Recv<i32, Send<i32, End>>,
            RoleA<RoleC<RoleC<RoleEnd>>>,
            NameS,
        >,
    ),
    No(MeshedChannelsThree<Recv<i32, End>, End, RoleA<RoleEnd>, NameS>),
}
type Choice1fromCtoS = Recv<Branching1fromCtoS, End>;

// Creating the MP sessions
// A
type ChoiceA = MeshedChannelsThree<Choice1fromCtoA, End, RoleC<RoleEnd>, NameA>;
type EndpointA = MeshedChannelsThree<Choice0fromCtoA, End, RoleC<RoleEnd>, NameA>;

// C
type ChoiceC = MeshedChannelsThree<Choose1fromCtoA, Choose1fromCtoS, RoleBroadcast, NameC>;
type ChoiceCYes = MeshedChannelsThree<
    Send<i32, End>,
    Send<i32, Recv<i32, End>>,
    RoleA<RoleS<RoleS<RoleEnd>>>,
    NameC,
>;
type ChoiceCNo = MeshedChannelsThree<Send<i32, End>, End, RoleA<RoleEnd>, NameC>;
type EndpointC = MeshedChannelsThree<Choose0fromCtoA, Choose0fromCtoS, RoleBroadcast, NameC>;
type EndpointCLoop = MeshedChannelsThree<
    Send<i32, Recv<i32, Choose0fromCtoA>>,
    Choose0fromCtoS,
    RoleA<RoleA<RoleBroadcast>>,
    NameC,
>;

// S
type ChoiceS = MeshedChannelsThree<End, Choice1fromCtoS, RoleC<RoleEnd>, NameS>;
type EndpointS = MeshedChannelsThree<End, Choice0fromCtoS, RoleC<RoleEnd>, NameS>;

// Functions
// A
fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromCtoA::Select(s) => {
            choice_a(s)
        },
        Branching0fromCtoA::Loop(s) => {
            let (query, s) = s.recv()?;
            let s = s.send(query)?;
            let s = s.send(1)?;
            endpoint_a(s)
        },
    })
}

fn choice_a(s: ChoiceA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching1fromCtoA::Yes(s) => {
            let (yes, s) = s.recv()?;
            let s = s.send(yes)?;
            s.close()
        },
        Branching1fromCtoA::No(s) => {
            let (no, s) = s.recv()?;
            let s = s.send(no)?;
            s.close()
        },
    })
}

fn endpoint_c_init(s: EndpointC) -> Result<(), Box<dyn Error>> {
    endpoint_c(s, 100)
}

fn endpoint_c(s: EndpointC, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s =
                choose_mpst_c_to_all!(s, Branching0fromCtoA::Select, Branching0fromCtoS::Select);
            choice_c(s)
        }
        _ => {
            let s: EndpointCLoop =
                choose_mpst_c_to_all!(s, Branching0fromCtoA::Loop, Branching0fromCtoS::Loop);

            let s = s.send(1)?;
            let (_quote, s) = s.recv()?;
            endpoint_c(s, loops - 1)
        }
    }
}

fn choice_c(s: ChoiceC) -> Result<(), Box<dyn Error>> {
    let choice: i32 = thread_rng().gen_range(1..3);

    if choice != 1 {
        let s: ChoiceCYes =
            choose_mpst_c_to_all!(s, Branching1fromCtoA::Yes, Branching1fromCtoS::Yes);

        let s = s.send(1)?;
        let s = s.send(1)?;
        let (_ack, s) = s.recv()?;
        s.close()
    } else {
        let s: ChoiceCNo = choose_mpst_c_to_all!(s, Branching1fromCtoA::No, Branching1fromCtoS::No);

        let s = s.send(0)?;
        s.close()
    }
}

fn endpoint_s(s: EndpointS) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromCtoS::Select(s) => {
            choice_s(s)
        },
        Branching0fromCtoS::Loop(s) => {
            let (_dummy, s) = s.recv()?;
            endpoint_s(s)
        },
    })
}

fn choice_s(s: ChoiceS) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching1fromCtoS::Yes(s) => {
            let (_yes, s) = s.recv()?;
            let (payment, s) = s.recv()?;
            let s = s.send(payment)?;
            s.close()
        },
        Branching1fromCtoS::No(s) => {
            let (_no, s) = s.recv()?;
            s.close()
        },
    })
}

fn main() {
    let (thread_a, thread_c, thread_s) = fork_mpst(endpoint_a, endpoint_c_init, endpoint_s);

    assert!(thread_a.join().is_ok());
    assert!(thread_c.join().is_ok());
    assert!(thread_s.join().is_ok());
}
