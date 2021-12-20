#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::bundle_impl_with_enum_and_cancel;

use std::error::Error;

static LOOPS: i64 = 100;

// Create new roles
bundle_impl_with_enum_and_cancel!(MeshedChannelsTwo, A, B);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;

// Types
// A
enum Branching0fromBtoA {
    More(MeshedChannelsTwo<Recv<(), Send<(), RecursAtoB>>, RoleB<RoleB<RoleB<RoleEnd>>>, NameA>),
    Done(MeshedChannelsTwo<End, RoleEnd, NameA>),
}
type RecursAtoB = Recv<Branching0fromBtoA, End>;
// C
type Choose0fromBtoA = Send<Branching0fromBtoA, End>;
type EndpointMoreB =
    MeshedChannelsTwo<Send<(), Recv<(), Choose0fromBtoA>>, RoleA<RoleA<RoleBroadcast>>, NameB>;

// Creating the MP sessions
type EndpointA = MeshedChannelsTwo<RecursAtoB, RoleB<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsTwo<Choose0fromBtoA, RoleBroadcast, NameB>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromBtoA::Done(s) => {
            s.close()
        },
        Branching0fromBtoA::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_a(s)
        },
    })
}

#[inline]
fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    let mut temp_s = s;

    for _ in 1..LOOPS {
        temp_s = recurs_b(temp_s)?;
    }

    let s = choose_mpst_b_to_all!(temp_s, Branching0fromBtoA::Done);

    s.close()
}

fn recurs_b(s: EndpointB) -> Result<EndpointB, Box<dyn Error>> {
    let s: EndpointMoreB = choose_mpst_b_to_all!(s, Branching0fromBtoA::More);

    let s = s.send(())?;
    let (_, s) = s.recv()?;
    Ok(s)
}

fn main() {
    let (thread_a, thread_b) = fork_mpst(endpoint_a, endpoint_b);

    thread_a.join().unwrap();
    thread_b.join().unwrap();
}
