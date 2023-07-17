use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use std::error::Error;

generate!("rec_and_cancel", MeshedChannels, A, B, C);

type AddAtoB = Recv<i32, End>;

type OrderingA21 = RoleC<RoleEnd>;
type OrderingA22Full = RoleB<OrderingA21>;
type EndpointA23 = MeshedChannels<AddAtoB, Recv<Branches0AtoC, End>, OrderingA22Full, NameA>;

type ByeAtoB = Recv<(), End>;

type OrderingA24Full = RoleB<RoleEnd>;
type EndpointA25 = MeshedChannels<ByeAtoB, End, OrderingA24Full, NameA>;

enum Branches0AtoC {
    Add(EndpointA23),
    Bye(EndpointA25),
}
type Choose0forAtoC = Send<Branches0AtoC, End>;

type TestAtoC = Recv<i32, Recv<Branches0AtoC, End>>;

type OrderingA46 = RoleC<RoleEnd>;
type OrderingA47Full = RoleC<OrderingA46>;
type EndpointA48 = MeshedChannels<End, TestAtoC, OrderingA47Full, NameA>;

type AddBtoA = Send<i32, End>;
type AddBtoC = Recv<i32, Recv<Branches0BtoC, End>>;

type OrderingB23 = RoleC<RoleEnd>;
type OrderingB24Full = RoleC<RoleA<OrderingB23>>;
type EndpointB25 = MeshedChannels<AddBtoA, AddBtoC, OrderingB24Full, NameB>;

type ByeBtoA = Send<(), End>;
type ByeBtoC = Recv<(), End>;

type OrderingB26Full = RoleC<RoleA<RoleEnd>>;
type EndpointB27 = MeshedChannels<ByeBtoA, ByeBtoC, OrderingB26Full, NameB>;

enum Branches0BtoC {
    Add(EndpointB25),
    Bye(EndpointB27),
}
type Choose0forBtoC = Send<Branches0BtoC, End>;

type OrderingB48 = RoleC<RoleEnd>;
type OrderingB49Full = OrderingB48;
type EndpointB50 = MeshedChannels<End, Recv<Branches0BtoC, End>, OrderingB49Full, NameB>;

type TestCtoA = Send<i32, Choose0forAtoC>;

type ByeCtoB = Send<(), End>;

type OrderingC8Full = RoleB<RoleEnd>;
type EndpointC9 = MeshedChannels<End, ByeCtoB, OrderingC8Full, NameC>;

type AddCtoB = Send<i32, Choose0forBtoC>;

type OrderingC6Full = RoleB<RoleBroadcast>;
type EndpointC7 = MeshedChannels<Choose0forAtoC, AddCtoB, OrderingC6Full, NameC>;

type EndpointC10 = MeshedChannels<Choose0forAtoC, Choose0forBtoC, RoleBroadcast, NameC>;

type OrderingC12Full = RoleA<RoleBroadcast>;
type EndpointC13 = MeshedChannels<TestCtoA, Choose0forBtoC, OrderingC12Full, NameC>;

/////////////////////////

fn endpoint_a(s: EndpointA48) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv()?;
    offer_mpst!(s, {
        Branches0AtoC::Add(s) => {
            recurs_a(s)
        },
        Branches0AtoC::Bye(s) => {
            let (_,s) = s.recv()?;
            s.close()
        },
    })
}

fn recurs_a(s: EndpointA23) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv()?;
    offer_mpst!(s, {
        Branches0AtoC::Add(s) => {
            recurs_a(s)
        },
        Branches0AtoC::Bye(s) => {
            let (_,s) = s.recv()?;
            s.close()
        },
    })
}

/////////////////////////

fn endpoint_b(s: EndpointB50) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branches0BtoC::Add(s) => {
            let (_,s) = s.recv()?;
            let s = s.send(0)?;
            endpoint_b(s)
        },
        Branches0BtoC::Bye(s) => {
            let (_,s) = s.recv()?;
            let s = s.send(())?;
            s.close()
        },
    })
}

/////////////////////////

fn endpoint_c(s: EndpointC13) -> Result<(), Box<dyn Error>> {
    let s = s.send(0)?;
    recurs_c(s, 5)
}

fn recurs_c(s: EndpointC10, loops: i32) -> Result<(), Box<dyn Error>> {
    if loops <= 0 {
        let s: EndpointC7 = choose_mpst_c_to_all!(s, Branches0AtoC::Add, Branches0BtoC::Add);
        let s = s.send(0)?;

        recurs_c(s, loops - 1)
    } else {
        let s: EndpointC9 = choose_mpst_c_to_all!(s, Branches0AtoC::Bye, Branches0BtoC::Bye);
        let s = s.send(())?;

        s.close()
    }
}

/////////////////////////

fn main() {
    checking();

    let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
}

/////////////////////////

// Check for bottom-up approach
fn checking() {
    let (graphs, kmc) = mpstthree::checker_concat!(
        "Adder_checking",
        EndpointA48,
        EndpointC13,
        EndpointB50
        =>
        [
            EndpointC7,
            Branches0AtoC, Add,
            Branches0BtoC, Add,
        ],
        [
            EndpointC9,
            Branches0AtoC, Bye,
            Branches0BtoC, Bye,
        ]
    )
    .unwrap();

    println!("graph A: {:?}", petgraph::dot::Dot::new(&graphs["RoleA"]));
    println!("\n/////////////////////////\n");
    println!("graph B: {:?}", petgraph::dot::Dot::new(&graphs["RoleB"]));
    println!("\n/////////////////////////\n");
    println!("graph C: {:?}", petgraph::dot::Dot::new(&graphs["RoleC"]));
    println!("\n/////////////////////////\n");
    println!("min kMC: {kmc:?}");
}
