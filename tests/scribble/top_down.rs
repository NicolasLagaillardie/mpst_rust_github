use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use std::marker;

use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;

/////////////////////////////////////////

use rand::{thread_rng, Rng};

use std::boxed::Box;
use std::error::Error;

use mpstthree::functionmpst::close::close_mpst;
use mpstthree::functionmpst::fork::fork_mpst;

// Get recv functions
use mpstthree::functionmpst::recv::recv_mpst_a_from_b;
use mpstthree::functionmpst::recv::recv_mpst_a_from_c;

// Get send functions
use mpstthree::functionmpst::send::send_mpst_b_to_a;
use mpstthree::functionmpst::send::send_mpst_c_to_a;
use mpstthree::functionmpst::send::send_mpst_c_to_b;

use mpstthree::choose_mpst_c_to_all;
use mpstthree::offer_mpst_a_to_c;
use mpstthree::offer_mpst_b_to_c;

///////////////////////////////////////// START

type ADDAtoB<N> = Recv<N, End>;

type OrderingA6Full = RoleB<RoleEnd>;
type EndpointA7<N> = MeshedChannels<ADDAtoB<N>, End, OrderingA6Full, RoleA<RoleEnd>>;
type BYEAtoB = Recv<(), End>;

type OrderingA8Full = RoleB<RoleEnd>;
type EndpointA9 = MeshedChannels<BYEAtoB, End, OrderingA8Full, RoleA<RoleEnd>>;

enum Branches0AtoC<N: marker::Send> {
    Add(EndpointA7<N>),
    Bye(EndpointA9),
}
type Choose0forAtoC<N> = Send<Branches0AtoC<N>, End>;

type TestAtoC<N> = Recv<N, Recv<Branches0AtoC<N>, End>>;

type OrderingA10 = RoleC<RoleEnd>;
type OrderingA11Full = RoleC<OrderingA10>;
type EndpointA12<N> = MeshedChannels<End, TestAtoC<N>, OrderingA11Full, RoleA<RoleEnd>>;

type ADDBtoA<N> = Send<N, End>;
type ADDBtoC<N> = Recv<N, End>;

type OrderingB8Full = RoleC<RoleA<RoleEnd>>;
type EndpointB9<N> = MeshedChannels<ADDBtoA<N>, ADDBtoC<N>, OrderingB8Full, RoleB<RoleEnd>>;
type BYEBtoA = Send<(), End>;
type BYEBtoC = Recv<(), End>;

type OrderingB10Full = RoleC<RoleA<RoleEnd>>;
type EndpointB11 = MeshedChannels<BYEBtoA, BYEBtoC, OrderingB10Full, RoleB<RoleEnd>>;

enum Branches0BtoC<N: marker::Send> {
    Add(EndpointB9<N>),
    Bye(EndpointB11),
}
type Choose0forBtoC<N> = Send<Branches0BtoC<N>, End>;

type OrderingB12 = RoleC<RoleEnd>;
type OrderingB13Full = OrderingB12;
type EndpointB14<N> =
    MeshedChannels<End, Recv<Branches0BtoC<N>, End>, OrderingB13Full, RoleB<RoleEnd>>;

type TestCtoA<N> = Send<N, Choose0forAtoC<N>>;

type OrderingC2Full = RoleA<RoleBroadcast>;
type EndpointC3<N> = MeshedChannels<TestCtoA<N>, Choose0forBtoC<N>, OrderingC2Full, RoleC<RoleEnd>>;

///////////////////////////////////////// END

// Functions related to endpoints
fn server(s: EndpointB14<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_b_to_c!(s, {
        Branches0BtoC::Bye(s) => {
            let (x, s) = recv_mpst_b_from_c(s)?;

            let s = send_mpst_b_to_a(x, s);
            close_mpst(s)
        },
        Branches0BtoC::Add(s) => {
            let (id, s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_a(id + 1, s);
            close_mpst(s)
        },
    })
}

fn authenticator(s: EndpointA12<i32>) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_a_from_c(s)?;

    offer_mpst_a_to_c!(s, {
        Branches0AtoC::Bye(s) => {
            let (_x, s) = recv_mpst_a_from_b(s)?;

            close_mpst(s)
        },
        Branches0AtoC::Add(s) => {
            let (_, s) = recv_mpst_a_from_b(s)?;
            close_mpst(s)
        },
    })
}

fn client(s: EndpointC3<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let x: u32 = rng.gen_range(0..2);

    let s = send_mpst_c_to_a(0, s);

    if x == 1 {
        let s = choose_mpst_c_to_all!(s, Branches0AtoC::Add, Branches0BtoC::Add);
        let s = send_mpst_c_to_b(1, s);
        close_mpst(s)
    } else {
        let s = choose_mpst_c_to_all!(s, Branches0AtoC::Bye, Branches0BtoC::Bye);
        let s = send_mpst_c_to_b((), s);
        close_mpst(s)
    }
}

/////////////////////////////////////////

pub fn top_down_approach() {
    for _i in 0..200 {
        assert!(|| -> Result<(), Box<dyn Error>> {
            {
                let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client);

                assert!(thread_a.join().is_ok());
                assert!(thread_b.join().is_ok());
                assert!(thread_c.join().is_ok());
            }
            Ok(())
        }()
        .is_ok());
    }
}
