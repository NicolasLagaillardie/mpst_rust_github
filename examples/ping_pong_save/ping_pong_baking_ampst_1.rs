use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::error::Error;

// Create new roles
generate!("rec_and_cancel", MeshedChannels, A, B);

// Types
// A
type Choose0fromAtoB = <RecursBtoA as Session>::Dual;

// B
enum Branching0fromAtoB {
    More(MeshedChannels<Recv<(), Send<(), RecursBtoA>>, RoleA<RoleA<RoleA<RoleEnd>>>, NameB>),
    Done(MeshedChannels<End, RoleEnd, NameB>),
}
type RecursBtoA = Recv<Branching0fromAtoB, End>;

// Creating the MP sessions
type EndpointA = MeshedChannels<Choose0fromAtoB, RoleBroadcast, NameA>;
type EndpointB = MeshedChannels<RecursBtoA, RoleA<RoleEnd>, NameB>;

// Functions
fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    recurs_a(s, LOOPS)
}

fn recurs_a(s: EndpointA, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_a_to_all!(s, Branching0fromAtoB::Done);

            s.close()
        }
        i => {
            let s = choose_mpst_a_to_all!(s, Branching0fromAtoB::More);

            let s = s.send(())?;
            let ((), s) = s.recv()?;

            recurs_a(s, i - 1)
        }
    }
}

fn recurs_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromAtoB::Done(s) => {
            s.close()
        },
        Branching0fromAtoB::More(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            recurs_b(s)
        },
    })
}

fn main() {
    let (thread_a, thread_b) = fork_mpst(endpoint_a, recurs_b);

    thread_a.join().unwrap();
    thread_b.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 1;
