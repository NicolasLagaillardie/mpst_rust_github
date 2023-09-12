use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::error::Error;

generate!("rec_and_cancel", MeshedChannels, A, B, C,);

type AddAtoB = Recv<i32, End>;

type OrderingA10 = RoleC<RoleEnd>;
type OrderingA11Full = RoleB<OrderingA10>;
type EndpointA12 = MeshedChannels<AddAtoB, Recv<Branches0AtoC, End>, OrderingA11Full, NameA>;

type ByeAtoB = Recv<(), End>;

type OrderingA13Full = RoleB<RoleEnd>;
type EndpointA14 = MeshedChannels<ByeAtoB, End, OrderingA13Full, NameA>;

enum Branches0AtoC {
    Add(EndpointA12),
    Bye(EndpointA14),
}
type Choose0forAtoC = Send<Branches0AtoC, End>;

type TestAtoC = Recv<i32, Recv<Branches0AtoC, End>>;

type OrderingA35 = RoleC<RoleEnd>;
type OrderingA36Full = RoleC<OrderingA35>;
type EndpointA37 = MeshedChannels<End, TestAtoC, OrderingA36Full, NameA>;

type AddBtoA = Send<i32, End>;
type AddBtoC = Recv<i32, Recv<Branches0BtoC, End>>;

type OrderingB10 = RoleC<RoleEnd>;
type OrderingB11Full = RoleC<RoleA<OrderingB10>>;
type EndpointB12 = MeshedChannels<AddBtoA, AddBtoC, OrderingB11Full, NameB>;

type ByeBtoA = Send<(), End>;
type ByeBtoC = Recv<(), End>;

type OrderingB13Full = RoleC<RoleA<RoleEnd>>;
type EndpointB14 = MeshedChannels<ByeBtoA, ByeBtoC, OrderingB13Full, NameB>;

enum Branches0BtoC {
    Add(EndpointB12),
    Bye(EndpointB14),
}
type Choose0forBtoC = Send<Branches0BtoC, End>;

type OrderingB35 = RoleC<RoleEnd>;
type OrderingB36Full = OrderingB35;
type EndpointB37 = MeshedChannels<End, Recv<Branches0BtoC, End>, OrderingB36Full, NameB>;

type TestCtoA = Send<i32, Choose0forAtoC>;

type AddCtoB = Send<i32, End>;

type OrderingC4Full = RoleB<RoleBroadcast>;
type EndpointC5 = MeshedChannels<Choose0forAtoC, AddCtoB, OrderingC4Full, NameC>;

type ByeCtoB = Send<(), End>;

type OrderingC6Full = RoleB<RoleBroadcast>;
type EndpointC7 = MeshedChannels<Choose0forAtoC, ByeCtoB, OrderingC6Full, NameC>;

type EndpointC8 = MeshedChannels<Choose0forAtoC, Choose0forBtoC, RoleBroadcast, NameC>;

type OrderingC10Full = RoleA<RoleEnd>;
type EndpointC11 = MeshedChannels<TestCtoA, Choose0forBtoC, OrderingC10Full, NameC>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv()?;
    recurs_a(s, 5)
}

fn recurs_a(s: EndpointALoop, loops: i32) -> Result<(), Box<dyn Error>> {
    if loops > 0 {
        let s: EndpointAMore = choose_mpst_a_to_all!(s, Branching0fromAtoB::More);

        let (_, s) = s.recv()?;
        recurs_a(s, loops - 1)
    } else {
        let s: EndpointADone = choose_mpst_a_to_all!(s, Branching0fromAtoB::Done);

        let (_, s) = s.recv()?;
        s.close()
    }
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    let s = s.send(Request {})?;
    recurs_b(s)
}

fn recurs_b(s: EndpointBLoop) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromAtoB::More(s) => {
            let s = s.send(Response {})?;
            recurs_b(s)
        },
        Branching0fromAtoB::Done(s) => {
            let s = s.send(Stop {})?;
            s.close()
        },
    })
}

fn main() {
    let (thread_a, thread_b) = fork_mpst(endpoint_a, endpoint_b);

    thread_a.join().unwrap();
    thread_b.join().unwrap();
}
