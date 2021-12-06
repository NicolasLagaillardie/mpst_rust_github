use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::bundle_impl_with_enum_and_cancel;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use std::error::Error;

bundle_impl_with_enum_and_cancel!(MeshedChannels, A, B, C,);
type ADDAtoB = Recv<i32, End>;

type OrderingA6Full = RoleB<RoleEnd>;
type EndpointA7 = MeshedChannels<ADDAtoB, End, OrderingA6Full, RoleA<RoleEnd>>;
type BYEAtoB = Recv<(), End>;

type OrderingA8Full = RoleB<RoleEnd>;
type EndpointA9 = MeshedChannels<BYEAtoB, End, OrderingA8Full, RoleA<RoleEnd>>;

enum Branches0AtoC {
    ADD(EndpointA7),
    BYE(EndpointA9),
}
type Choose0forAtoC = Send<Branches0AtoC, End>;

type TestAtoC = Recv<i32, Recv<Branches0AtoC, End>>;

type OrderingA10 = RoleC<RoleEnd>;
type OrderingA11Full = RoleC<OrderingA10>;
type EndpointA12 = MeshedChannels<End, TestAtoC, OrderingA11Full, RoleA<RoleEnd>>;

type ADDBtoA = Send<i32, End>;
type ADDBtoC = Recv<i32, End>;

type OrderingB8Full = RoleC<RoleA<RoleEnd>>;
type EndpointB9 = MeshedChannels<ADDBtoA, ADDBtoC, OrderingB8Full, RoleB<RoleEnd>>;
type BYEBtoA = Send<(), End>;
type BYEBtoC = Recv<(), End>;

type OrderingB10Full = RoleC<RoleA<RoleEnd>>;
type EndpointB11 = MeshedChannels<BYEBtoA, BYEBtoC, OrderingB10Full, RoleB<RoleEnd>>;

enum Branches0BtoC {
    ADD(EndpointB9),
    BYE(EndpointB11),
}
type Choose0forBtoC = Send<Branches0BtoC, End>;

type OrderingB12 = RoleC<RoleEnd>;
type OrderingB13Full = OrderingB12;
type EndpointB14 = MeshedChannels<End, Recv<Branches0BtoC, End>, OrderingB13Full, RoleB<RoleEnd>>;

type TestCtoA = Send<i32, Choose0forAtoC>;

type BYECtoB = Send<(), End>;

type OrderingC4Full = RoleB<RoleEnd>;
type EndpointC5 = MeshedChannels<End, BYECtoB, OrderingC4Full, RoleC<RoleEnd>>;

type ADDCtoB = Send<i32, End>;

type OrderingC2Full = RoleB<RoleEnd>;
type EndpointC3 = MeshedChannels<End, ADDCtoB, OrderingC2Full, RoleC<RoleEnd>>;

type OrderingC6Full = RoleA<RoleBroadcast>;
type EndpointC7 = MeshedChannels<TestCtoA, Choose0forBtoC, OrderingC6Full, RoleC<RoleEnd>>;

fn endpoint_a(s: EndpointA12) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv()?;
    offer_mpst!(s, {
        Branches0AtoC::ADD(s) => {
            let (_,s) = s.recv()?;
            s.close()
        },
        Branches0AtoC::BYE(s) => {
            let (_,s) = s.recv()?;
            s.close()
        },
    })
}

fn endpoint_b(s: EndpointB14) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branches0BtoC::ADD(s) => {
            let (_,s) = s.recv()?;
            let s = s.send(0)?;
            s.close()
        },
        Branches0BtoC::BYE(s) => {
            let (_,s) = s.recv()?;
            let s = s.send(())?;
            s.close()
        },
    })
}

fn endpoint_c(s: EndpointC7) -> Result<(), Box<dyn Error>> {
    recurs_c(s, 5)
}

fn recurs_c(s: EndpointC7, loops: i32) -> Result<(), Box<dyn Error>> {
    let s = s.send(0)?;

    if loops <= 0 {
        let s: EndpointC5 = choose_mpst_c_to_all!(s, Branches0AtoC::BYE, Branches0BtoC::BYE);
        let _ = s.send(())?;

        Ok(())
    } else {
        let s: EndpointC3 = choose_mpst_c_to_all!(s, Branches0AtoC::ADD, Branches0BtoC::ADD);
        let s = s.send(0)?;

        s.close()
    }
}

/////////////////////////

fn main() {
    checking();

    let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);

    assert!(thread_a.join().is_ok());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_ok());
}

/////////////////////////

// Check for bottom-up approach
fn checking() {
    let (graphs, kmc) = mpstthree::checker_concat!(
        "adder_checking",
        EndpointA12,
        EndpointC7,
        EndpointB11
        =>
        [
            EndpointC3,
            Branches0AtoC, ADD,
            Branches0BtoC, ADD,
        ],
        [
            EndpointC5,
            Branches0AtoC, BYE,
            Branches0BtoC, BYE,
        ]
    )
    .unwrap();

    println!("graph A: {:?}", petgraph::dot::Dot::new(&graphs["RoleA"]));
    println!("graph B: {:?}", petgraph::dot::Dot::new(&graphs["RoleB"]));
    println!("graph C: {:?}", petgraph::dot::Dot::new(&graphs["RoleC"]));
    println!("min kMC: {:?}", kmc);
}