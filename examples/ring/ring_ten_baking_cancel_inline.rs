#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::bundle_impl_with_enum_and_cancel;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::error::Error;

static LOOPS: i64 = 100;

// Create new roles
bundle_impl_with_enum_and_cancel!(MeshedChannelsTen, A, B, C, D, E, F, G, H, I, J);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;
type NameE = RoleE<RoleEnd>;
type NameF = RoleF<RoleEnd>;
type NameG = RoleG<RoleEnd>;
type NameH = RoleH<RoleEnd>;
type NameI = RoleI<RoleEnd>;
type NameJ = RoleJ<RoleEnd>;

// Types
// A
enum Branching0fromJtoA {
    Forward(
        MeshedChannelsTen<
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursAtoJ,
            RoleB<RoleJ<RoleEnd>>,
            NameA,
        >,
    ),
    Backward(
        MeshedChannelsTen<
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursAtoJ,
            RoleB<RoleJ<RoleEnd>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoJ = Recv<Branching0fromJtoA, End>;
// B
enum Branching0fromJtoB {
    Forward(
        MeshedChannelsTen<
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursBtoJ,
            RoleA<RoleC<RoleJ<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannelsTen<
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursBtoJ,
            RoleC<RoleA<RoleJ<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoJ = Recv<Branching0fromJtoB, End>;
// C
enum Branching0fromJtoC {
    Forward(
        MeshedChannelsTen<
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            RecursCtoJ,
            RoleB<RoleD<RoleJ<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannelsTen<
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            RecursCtoJ,
            RoleD<RoleB<RoleJ<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoJ = Recv<Branching0fromJtoC, End>;
// D
enum Branching0fromJtoD {
    Forward(
        MeshedChannelsTen<
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            RecursDtoJ,
            RoleC<RoleE<RoleJ<RoleEnd>>>,
            NameD,
        >,
    ),
    Backward(
        MeshedChannelsTen<
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            RecursDtoJ,
            RoleE<RoleC<RoleJ<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoJ = Recv<Branching0fromJtoD, End>;
// E
enum Branching0fromJtoE {
    Forward(
        MeshedChannelsTen<
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            RecursEtoJ,
            RoleD<RoleF<RoleJ<RoleEnd>>>,
            NameE,
        >,
    ),
    Backward(
        MeshedChannelsTen<
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            RecursEtoJ,
            RoleF<RoleD<RoleJ<RoleEnd>>>,
            NameE,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoJ = Recv<Branching0fromJtoE, End>;
// F
enum Branching0fromJtoF {
    Forward(
        MeshedChannelsTen<
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            RecursFtoJ,
            RoleE<RoleG<RoleJ<RoleEnd>>>,
            NameF,
        >,
    ),
    Backward(
        MeshedChannelsTen<
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            RecursFtoJ,
            RoleG<RoleE<RoleJ<RoleEnd>>>,
            NameF,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoJ = Recv<Branching0fromJtoF, End>;
// G
enum Branching0fromJtoG {
    Forward(
        MeshedChannelsTen<
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            RecursGtoJ,
            RoleF<RoleH<RoleJ<RoleEnd>>>,
            NameG,
        >,
    ),
    Backward(
        MeshedChannelsTen<
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            RecursGtoJ,
            RoleH<RoleF<RoleJ<RoleEnd>>>,
            NameG,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoJ = Recv<Branching0fromJtoG, End>;
// H
enum Branching0fromJtoH {
    Forward(
        MeshedChannelsTen<
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            RecursHtoJ,
            RoleG<RoleI<RoleJ<RoleEnd>>>,
            NameH,
        >,
    ),
    Backward(
        MeshedChannelsTen<
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            RecursHtoJ,
            RoleI<RoleG<RoleJ<RoleEnd>>>,
            NameH,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameH>),
}
type RecursHtoJ = Recv<Branching0fromJtoH, End>;
// I
enum Branching0fromJtoI {
    Forward(
        MeshedChannelsTen<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), RecursItoJ>,
            RoleH<RoleJ<RoleJ<RoleEnd>>>,
            NameI,
        >,
    ),
    Backward(
        MeshedChannelsTen<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), RecursItoJ>,
            RoleJ<RoleH<RoleJ<RoleEnd>>>,
            NameI,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameI>),
}
type RecursItoJ = Recv<Branching0fromJtoI, End>;
// J
type Choose0fromJtoA = Send<Branching0fromJtoA, End>;
type Choose0fromJtoB = Send<Branching0fromJtoB, End>;
type Choose0fromJtoC = Send<Branching0fromJtoC, End>;
type Choose0fromJtoD = Send<Branching0fromJtoD, End>;
type Choose0fromJtoE = Send<Branching0fromJtoE, End>;
type Choose0fromJtoF = Send<Branching0fromJtoF, End>;
type Choose0fromJtoG = Send<Branching0fromJtoG, End>;
type Choose0fromJtoH = Send<Branching0fromJtoH, End>;
type Choose0fromJtoI = Send<Branching0fromJtoI, End>;
type EndpointForwardJ = MeshedChannelsTen<
    Choose0fromJtoA,
    Choose0fromJtoB,
    Choose0fromJtoC,
    Choose0fromJtoD,
    Choose0fromJtoE,
    Choose0fromJtoF,
    Choose0fromJtoG,
    Choose0fromJtoH,
    Recv<(), Choose0fromJtoI>,
    RoleI<RoleBroadcast>,
    NameJ,
>;
type EndpointBackwardJ = MeshedChannelsTen<
    Choose0fromJtoA,
    Choose0fromJtoB,
    Choose0fromJtoC,
    Choose0fromJtoD,
    Choose0fromJtoE,
    Choose0fromJtoF,
    Choose0fromJtoG,
    Choose0fromJtoH,
    Send<(), Choose0fromJtoI>,
    RoleI<RoleBroadcast>,
    NameJ,
>;

// Creating the MP sessions
type EndpointA =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursAtoJ, RoleJ<RoleEnd>, NameA>;
type EndpointB =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursBtoJ, RoleJ<RoleEnd>, NameB>;
type EndpointC =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursCtoJ, RoleJ<RoleEnd>, NameC>;
type EndpointD =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursDtoJ, RoleJ<RoleEnd>, NameD>;
type EndpointE =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursEtoJ, RoleJ<RoleEnd>, NameE>;
type EndpointF =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursFtoJ, RoleJ<RoleEnd>, NameF>;
type EndpointG =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursGtoJ, RoleJ<RoleEnd>, NameG>;
type EndpointH =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursHtoJ, RoleJ<RoleEnd>, NameH>;
type EndpointI =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursItoJ, RoleJ<RoleEnd>, NameI>;
type EndpointJ = MeshedChannelsTen<
    Choose0fromJtoA,
    Choose0fromJtoB,
    Choose0fromJtoC,
    Choose0fromJtoD,
    Choose0fromJtoE,
    Choose0fromJtoF,
    Choose0fromJtoG,
    Choose0fromJtoH,
    Choose0fromJtoI,
    RoleBroadcast,
    NameJ,
>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromJtoA::Done(s) => {
            s.close()
        },
        Branching0fromJtoA::Forward(s) => {
            let s = s.send(())?;
            endpoint_a(s)
        },
        Branching0fromJtoA::Backward(s) => {
            let (_, s) = s.recv()?;
            endpoint_a(s)
        },
    })
}

#[inline]
fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromJtoB::Done(s) => {
            s.close()
        },
        Branching0fromJtoB::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_b(s)
        },
        Branching0fromJtoB::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_b(s)
        },
    })
}

#[inline]
fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromJtoC::Done(s) => {
            s.close()
        },
        Branching0fromJtoC::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_c(s)
        },
        Branching0fromJtoC::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_c(s)
        },
    })
}

#[inline]
fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromJtoD::Done(s) => {
            s.close()
        },
        Branching0fromJtoD::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_d(s)
        },
        Branching0fromJtoD::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_d(s)
        },
    })
}

#[inline]
fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromJtoE::Done(s) => {
            s.close()
        },
        Branching0fromJtoE::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_e(s)
        },
        Branching0fromJtoE::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_e(s)
        },
    })
}

#[inline]
fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromJtoF::Done(s) => {
            s.close()
        },
        Branching0fromJtoF::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_f(s)
        },
        Branching0fromJtoF::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_f(s)
        },
    })
}

#[inline]
fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromJtoG::Done(s) => {
            s.close()
        },
        Branching0fromJtoG::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_g(s)
        },
        Branching0fromJtoG::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_g(s)
        },
    })
}

#[inline]
fn endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromJtoH::Done(s) => {
            s.close()
        },
        Branching0fromJtoH::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_h(s)
        },
        Branching0fromJtoH::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_h(s)
        },
    })
}

#[inline]
fn endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromJtoI::Done(s) => {
            s.close()
        },
        Branching0fromJtoI::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_i(s)
        },
        Branching0fromJtoI::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_i(s)
        },
    })
}

#[inline]
fn endpoint_j(s: EndpointJ) -> Result<(), Box<dyn Error>> {
    let mut temp_s = s;

    for i in 1..LOOPS {
        temp_s = recurs_j(temp_s, i)?;
    }

    let s = choose_mpst_j_to_all!(
        temp_s,
        Branching0fromJtoA::Done,
        Branching0fromJtoB::Done,
        Branching0fromJtoC::Done,
        Branching0fromJtoD::Done,
        Branching0fromJtoE::Done,
        Branching0fromJtoF::Done,
        Branching0fromJtoG::Done,
        Branching0fromJtoH::Done,
        Branching0fromJtoI::Done
    );

    s.close()
}

fn recurs_j(s: EndpointJ, index: i64) -> Result<EndpointJ, Box<dyn Error>> {
    match index {
        i if i % 2 == 0 => {
            let s: EndpointForwardJ = choose_mpst_j_to_all!(
                s,
                Branching0fromJtoA::Forward,
                Branching0fromJtoB::Forward,
                Branching0fromJtoC::Forward,
                Branching0fromJtoD::Forward,
                Branching0fromJtoE::Forward,
                Branching0fromJtoF::Forward,
                Branching0fromJtoG::Forward,
                Branching0fromJtoH::Forward,
                Branching0fromJtoI::Forward
            );

            let (_, s) = s.recv()?;

            Ok(s)
        }
        _ => {
            let s: EndpointBackwardJ = choose_mpst_j_to_all!(
                s,
                Branching0fromJtoA::Backward,
                Branching0fromJtoB::Backward,
                Branching0fromJtoC::Backward,
                Branching0fromJtoD::Backward,
                Branching0fromJtoE::Backward,
                Branching0fromJtoF::Backward,
                Branching0fromJtoG::Backward,
                Branching0fromJtoH::Backward,
                Branching0fromJtoI::Backward
            );

            let s = s.send(())?;

            Ok(s)
        }
    }
}

fn main() {
    let (
        thread_a,
        thread_b,
        thread_c,
        thread_d,
        thread_e,
        thread_f,
        thread_g,
        thread_h,
        thread_i,
        thread_j,
    ) = fork_mpst(
        endpoint_a, endpoint_b, endpoint_c, endpoint_d, endpoint_e, endpoint_f, endpoint_g,
        endpoint_h, endpoint_i, endpoint_j,
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
    thread_f.join().unwrap();
    thread_g.join().unwrap();
    thread_h.join().unwrap();
    thread_i.join().unwrap();
    thread_j.join().unwrap();
}