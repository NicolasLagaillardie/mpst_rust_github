#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::error::Error;

static LOOPS: i64 = 100;

// Create new roles
generate!("rec_and_cancel", MeshedChannels, A, B, C, D, E, F, G, H, I, J, K);

// Types
// A
enum Branching0fromKtoA {
    Forward(
        MeshedChannels<
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursAtoK,
            RoleB<RoleK<RoleEnd>>,
            NameA,
        >,
    ),
    Backward(
        MeshedChannels<
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursAtoK,
            RoleB<RoleK<RoleEnd>>,
            NameA,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoK = Recv<Branching0fromKtoA, End>;

// B
enum Branching0fromKtoB {
    Forward(
        MeshedChannels<
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursBtoK,
            RoleA<RoleC<RoleK<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannels<
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursBtoK,
            RoleC<RoleA<RoleK<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoK = Recv<Branching0fromKtoB, End>;

// C
enum Branching0fromKtoC {
    Forward(
        MeshedChannels<
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursCtoK,
            RoleB<RoleD<RoleK<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursCtoK,
            RoleD<RoleB<RoleK<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoK = Recv<Branching0fromKtoC, End>;

// D
enum Branching0fromKtoD {
    Forward(
        MeshedChannels<
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            RecursDtoK,
            RoleC<RoleE<RoleK<RoleEnd>>>,
            NameD,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            RecursDtoK,
            RoleE<RoleC<RoleK<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoK = Recv<Branching0fromKtoD, End>;

// E
enum Branching0fromKtoE {
    Forward(
        MeshedChannels<
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            RecursEtoK,
            RoleD<RoleF<RoleK<RoleEnd>>>,
            NameE,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            RecursEtoK,
            RoleF<RoleD<RoleK<RoleEnd>>>,
            NameE,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoK = Recv<Branching0fromKtoE, End>;

// F
enum Branching0fromKtoF {
    Forward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            RecursFtoK,
            RoleE<RoleG<RoleK<RoleEnd>>>,
            NameF,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            RecursFtoK,
            RoleG<RoleE<RoleK<RoleEnd>>>,
            NameF,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoK = Recv<Branching0fromKtoF, End>;

// G
enum Branching0fromKtoG {
    Forward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            RecursGtoK,
            RoleF<RoleH<RoleK<RoleEnd>>>,
            NameG,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            RecursGtoK,
            RoleH<RoleF<RoleK<RoleEnd>>>,
            NameG,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoK = Recv<Branching0fromKtoG, End>;

// H
enum Branching0fromKtoH {
    Forward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            RecursHtoK,
            RoleG<RoleI<RoleK<RoleEnd>>>,
            NameH,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            RecursHtoK,
            RoleI<RoleG<RoleK<RoleEnd>>>,
            NameH,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameH>),
}
type RecursHtoK = Recv<Branching0fromKtoH, End>;

// I
enum Branching0fromKtoI {
    Forward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            RecursItoK,
            RoleH<RoleJ<RoleK<RoleEnd>>>,
            NameI,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            RecursItoK,
            RoleJ<RoleH<RoleK<RoleEnd>>>,
            NameI,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameI>),
}
type RecursItoK = Recv<Branching0fromKtoI, End>;

// J
enum Branching0fromKtoJ {
    Forward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), RecursJtoK>,
            RoleI<RoleK<RoleK<RoleEnd>>>,
            NameJ,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), RecursJtoK>,
            RoleK<RoleI<RoleK<RoleEnd>>>,
            NameJ,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameJ>),
}
type RecursJtoK = Recv<Branching0fromKtoJ, End>;

// K
type Choose0fromKtoA = Send<Branching0fromKtoA, End>;
type Choose0fromKtoB = Send<Branching0fromKtoB, End>;
type Choose0fromKtoC = Send<Branching0fromKtoC, End>;
type Choose0fromKtoD = Send<Branching0fromKtoD, End>;
type Choose0fromKtoE = Send<Branching0fromKtoE, End>;
type Choose0fromKtoF = Send<Branching0fromKtoF, End>;
type Choose0fromKtoG = Send<Branching0fromKtoG, End>;
type Choose0fromKtoH = Send<Branching0fromKtoH, End>;
type Choose0fromKtoI = Send<Branching0fromKtoI, End>;
type Choose0fromKtoJ = Send<Branching0fromKtoJ, End>;
type EndpointForwardK = MeshedChannels<
    Choose0fromKtoA,
    Choose0fromKtoB,
    Choose0fromKtoC,
    Choose0fromKtoD,
    Choose0fromKtoE,
    Choose0fromKtoF,
    Choose0fromKtoG,
    Choose0fromKtoH,
    Choose0fromKtoI,
    Recv<(), Choose0fromKtoJ>,
    RoleJ<RoleBroadcast>,
    NameK,
>;
type EndpointBackwardK = MeshedChannels<
    Choose0fromKtoA,
    Choose0fromKtoB,
    Choose0fromKtoC,
    Choose0fromKtoD,
    Choose0fromKtoE,
    Choose0fromKtoF,
    Choose0fromKtoG,
    Choose0fromKtoH,
    Choose0fromKtoI,
    Send<(), Choose0fromKtoJ>,
    RoleJ<RoleBroadcast>,
    NameK,
>;

// Creating the MP sessions
type EndpointA = MeshedChannels<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursAtoK,
    RoleK<RoleEnd>,
    NameA,
>;
type EndpointB = MeshedChannels<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursBtoK,
    RoleK<RoleEnd>,
    NameB,
>;
type EndpointC = MeshedChannels<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursCtoK,
    RoleK<RoleEnd>,
    NameC,
>;
type EndpointD = MeshedChannels<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursDtoK,
    RoleK<RoleEnd>,
    NameD,
>;
type EndpointE = MeshedChannels<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursEtoK,
    RoleK<RoleEnd>,
    NameE,
>;
type EndpointF = MeshedChannels<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursFtoK,
    RoleK<RoleEnd>,
    NameF,
>;
type EndpointG = MeshedChannels<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursGtoK,
    RoleK<RoleEnd>,
    NameG,
>;
type EndpointH = MeshedChannels<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursHtoK,
    RoleK<RoleEnd>,
    NameH,
>;
type EndpointI = MeshedChannels<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursItoK,
    RoleK<RoleEnd>,
    NameI,
>;
type EndpointJ = MeshedChannels<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursJtoK,
    RoleK<RoleEnd>,
    NameJ,
>;
type EndpointK = MeshedChannels<
    Choose0fromKtoA,
    Choose0fromKtoB,
    Choose0fromKtoC,
    Choose0fromKtoD,
    Choose0fromKtoE,
    Choose0fromKtoF,
    Choose0fromKtoG,
    Choose0fromKtoH,
    Choose0fromKtoI,
    Choose0fromKtoJ,
    RoleBroadcast,
    NameK,
>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromKtoA::Done(s) => {
            s.close()
        },
        Branching0fromKtoA::Forward(s) => {
            let s = s.send(())?;
            endpoint_a(s)
        },
        Branching0fromKtoA::Backward(s) => {
            let (_, s) = s.recv()?;
            endpoint_a(s)
        },
    })
}

#[inline]
fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromKtoB::Done(s) => {
            s.close()
        },
        Branching0fromKtoB::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_b(s)
        },
        Branching0fromKtoB::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_b(s)
        },
    })
}

#[inline]
fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromKtoC::Done(s) => {
            s.close()
        },
        Branching0fromKtoC::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_c(s)
        },
        Branching0fromKtoC::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_c(s)
        },
    })
}

#[inline]
fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromKtoD::Done(s) => {
            s.close()
        },
        Branching0fromKtoD::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_d(s)
        },
        Branching0fromKtoD::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_d(s)
        },
    })
}

#[inline]
fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromKtoE::Done(s) => {
            s.close()
        },
        Branching0fromKtoE::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_e(s)
        },
        Branching0fromKtoE::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_e(s)
        },
    })
}

#[inline]
fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromKtoF::Done(s) => {
            s.close()
        },
        Branching0fromKtoF::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_f(s)
        },
        Branching0fromKtoF::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_f(s)
        },
    })
}

#[inline]
fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromKtoG::Done(s) => {
            s.close()
        },
        Branching0fromKtoG::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_g(s)
        },
        Branching0fromKtoG::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_g(s)
        },
    })
}

#[inline]
fn endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromKtoH::Done(s) => {
            s.close()
        },
        Branching0fromKtoH::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_h(s)
        },
        Branching0fromKtoH::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_h(s)
        },
    })
}

#[inline]
fn endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromKtoI::Done(s) => {
            s.close()
        },
        Branching0fromKtoI::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_i(s)
        },
        Branching0fromKtoI::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_i(s)
        },
    })
}

#[inline]
fn endpoint_j(s: EndpointJ) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromKtoJ::Done(s) => {
            s.close()
        },
        Branching0fromKtoJ::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_j(s)
        },
        Branching0fromKtoJ::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_j(s)
        },
    })
}

#[inline]
fn endpoint_k(s: EndpointK) -> Result<(), Box<dyn Error>> {
    let mut temp_s = s;

    for i in 1..LOOPS {
        temp_s = recurs_k(temp_s, i)?;
    }

    let s = choose_mpst_k_to_all!(
        temp_s,
        Branching0fromKtoA::Done,
        Branching0fromKtoB::Done,
        Branching0fromKtoC::Done,
        Branching0fromKtoD::Done,
        Branching0fromKtoE::Done,
        Branching0fromKtoF::Done,
        Branching0fromKtoG::Done,
        Branching0fromKtoH::Done,
        Branching0fromKtoI::Done,
        Branching0fromKtoJ::Done
    );

    s.close()
}

fn recurs_k(s: EndpointK, index: i64) -> Result<EndpointK, Box<dyn Error>> {
    match index {
        i if i % 2 == 0 => {
            let s: EndpointForwardK = choose_mpst_k_to_all!(
                s,
                Branching0fromKtoA::Forward,
                Branching0fromKtoB::Forward,
                Branching0fromKtoC::Forward,
                Branching0fromKtoD::Forward,
                Branching0fromKtoE::Forward,
                Branching0fromKtoF::Forward,
                Branching0fromKtoG::Forward,
                Branching0fromKtoH::Forward,
                Branching0fromKtoI::Forward,
                Branching0fromKtoJ::Forward
            );

            let (_, s) = s.recv()?;

            Ok(s)
        }
        _ => {
            let s: EndpointBackwardK = choose_mpst_k_to_all!(
                s,
                Branching0fromKtoA::Backward,
                Branching0fromKtoB::Backward,
                Branching0fromKtoC::Backward,
                Branching0fromKtoD::Backward,
                Branching0fromKtoE::Backward,
                Branching0fromKtoF::Backward,
                Branching0fromKtoG::Backward,
                Branching0fromKtoH::Backward,
                Branching0fromKtoI::Backward,
                Branching0fromKtoJ::Backward
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
        thread_k,
    ) = fork_mpst(
        endpoint_a, endpoint_b, endpoint_c, endpoint_d, endpoint_e, endpoint_f, endpoint_g,
        endpoint_h, endpoint_i, endpoint_j, endpoint_k,
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
    thread_k.join().unwrap();
}
