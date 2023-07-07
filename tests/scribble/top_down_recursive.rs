use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use std::marker;

use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;

use mpstthree::name::a::NameA;
use mpstthree::name::b::NameB;
use mpstthree::name::c::NameC;

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

type OrderingA11 = RoleC<RoleEnd>;
type OrderingA12Full = RoleB<OrderingA11>;
type EndpointA13<N> =
    MeshedChannels<ADDAtoB<N>, Recv<Branches0AtoC<N>, End>, OrderingA12Full, NameA>;
type BYEAtoB = Recv<(), End>;

type OrderingA14Full = RoleB<RoleEnd>;
type EndpointA15 = MeshedChannels<BYEAtoB, End, OrderingA14Full, NameA>;

enum Branches0AtoC<N: marker::Send> {
    Add(EndpointA13<N>),
    Bye(EndpointA15),
}
type Choose0forAtoC<N> = Send<Branches0AtoC<N>, End>;

type TestAtoC<N> = Recv<N, Recv<Branches0AtoC<N>, End>>;

type OrderingA16 = RoleC<RoleEnd>;
type OrderingA17Full = RoleC<OrderingA16>;
type EndpointA18<N> = MeshedChannels<End, TestAtoC<N>, OrderingA17Full, NameA>;

type ADDBtoA<N> = Send<N, End>;
type ADDBtoC<N> = Recv<N, Recv<Branches0BtoC<N>, End>>;

type OrderingB13 = RoleC<RoleEnd>;
type OrderingB14Full = RoleC<RoleA<OrderingB13>>;
type EndpointB15<N> = MeshedChannels<ADDBtoA<N>, ADDBtoC<N>, OrderingB14Full, NameB>;
type BYEBtoA = Send<(), End>;
type BYEBtoC = Recv<(), End>;

type OrderingB16Full = RoleC<RoleA<RoleEnd>>;
type EndpointB17 = MeshedChannels<BYEBtoA, BYEBtoC, OrderingB16Full, NameB>;

enum Branches0BtoC<N: marker::Send> {
    Add(EndpointB15<N>),
    Bye(EndpointB17),
}
type Choose0forBtoC<N> = Send<Branches0BtoC<N>, End>;

type OrderingB18 = RoleC<RoleEnd>;
type OrderingB19Full = OrderingB18;
type EndpointB20<N> = MeshedChannels<End, Recv<Branches0BtoC<N>, End>, OrderingB19Full, NameB>;

type TestCtoA<N> = Send<N, Choose0forAtoC<N>>;

type OrderingC2Full = RoleA<RoleBroadcast>;
type EndpointC3<N> = MeshedChannels<TestCtoA<N>, Choose0forBtoC<N>, OrderingC2Full, NameC>;

///////////////////////////////////////// END

///////////////////////////////////////// For verification
///////////////////////////////////////// with functions

type EndpointA19<N> = MeshedChannels<End, Recv<Branches0AtoC<N>, End>, OrderingA16, NameA>;

type EndpointC2<N> = MeshedChannels<Choose0forAtoC<N>, Choose0forBtoC<N>, RoleBroadcast, NameC>;

///////////////////////////////////////// END

// Functions related to endpoints
fn server(s: EndpointB20<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_b_to_c!(s, {
        Branches0BtoC::Bye(s) => {
            let (_, s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_a((), s);
            close_mpst(s)
        },
        Branches0BtoC::Add(s) => {
            let (id, s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_a(id + 1, s);
            server(s)
        },
    })
}

fn authenticator(s: EndpointA18<i32>) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_a_from_c(s)?;

    authenticator_recurs(s)
}

fn authenticator_recurs(s: EndpointA19<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_a_to_c!(s, {
        Branches0AtoC::Bye(s) => {
            let (_, s) = recv_mpst_a_from_b(s)?;
            close_mpst(s)
        },
        Branches0AtoC::Add(s) => {
            let (_, s) = recv_mpst_a_from_b(s)?;
            authenticator_recurs(s)
        },
    })
}

fn client(s: EndpointC3<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_c_to_a(0, s);

    client_recurs(s)
}

fn client_recurs(s: EndpointC2<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let x: u32 = rng.gen_range(0..2);

    if x == 1 {
        let s = choose_mpst_c_to_all!(s, Branches0AtoC::Add, Branches0BtoC::Add);
        let s = send_mpst_c_to_b(1, s);
        client_recurs(s)
    } else {
        let s = choose_mpst_c_to_all!(s, Branches0AtoC::Bye, Branches0BtoC::Bye);
        let s = send_mpst_c_to_b((), s);
        close_mpst(s)
    }
}

/////////////////////////////////////////

pub fn top_down_approach() {
    for _i in 0..200 {
        assert!({
            {
                let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client);

                assert!(thread_a.join().is_ok());
                assert!(thread_b.join().is_ok());
                assert!(thread_c.join().is_ok());
            }
            Ok::<(), Box<dyn Error>>(())
        }
        .is_ok());
    }
}
