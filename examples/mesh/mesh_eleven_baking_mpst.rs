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
generate!(
    "recursive",
    MeshedChannelsEleven,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K
);

// Types
// Send/Recv
type RS = Recv<(), Send<(), End>>;
type SR = Send<(), Recv<(), End>>;

// Roles
type R2A<R> = RoleA<RoleA<R>>;
type R2B<R> = RoleB<RoleB<R>>;
type R2C<R> = RoleC<RoleC<R>>;
type R2D<R> = RoleD<RoleD<R>>;
type R2E<R> = RoleE<RoleE<R>>;
type R2F<R> = RoleF<RoleF<R>>;
type R2G<R> = RoleG<RoleG<R>>;
type R2H<R> = RoleH<RoleH<R>>;
type R2I<R> = RoleI<RoleI<R>>;
type R2J<R> = RoleJ<RoleJ<R>>;
type R2K<R> = RoleK<RoleK<R>>;

// A
enum Branching0fromKtoA {
    More(
        MeshedChannelsEleven<
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursAtoK>>,
            R2K<R2B<R2C<R2D<R2E<R2F<R2G<R2H<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoK = Recv<Branching0fromKtoA, End>;

// B
enum Branching0fromKtoB {
    More(
        MeshedChannelsEleven<
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursBtoK>>,
            R2K<R2A<R2C<R2D<R2E<R2F<R2G<R2H<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoK = Recv<Branching0fromKtoB, End>;

// C
enum Branching0fromKtoC {
    More(
        MeshedChannelsEleven<
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursCtoK>>,
            R2K<R2A<R2B<R2D<R2E<R2F<R2G<R2H<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoK = Recv<Branching0fromKtoC, End>;

// D
enum Branching0fromKtoD {
    More(
        MeshedChannelsEleven<
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursDtoK>>,
            R2K<R2A<R2B<R2C<R2E<R2F<R2G<R2H<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoK = Recv<Branching0fromKtoD, End>;

// E
enum Branching0fromKtoE {
    More(
        MeshedChannelsEleven<
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursEtoK>>,
            R2K<R2A<R2B<R2C<R2D<R2F<R2G<R2H<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameE,
        >,
    ),
    Done(MeshedChannelsEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoK = Recv<Branching0fromKtoE, End>;

// F
enum Branching0fromKtoF {
    More(
        MeshedChannelsEleven<
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursFtoK>>,
            R2K<R2A<R2B<R2C<R2D<R2E<R2G<R2H<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameF,
        >,
    ),
    Done(MeshedChannelsEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoK = Recv<Branching0fromKtoF, End>;

// G
enum Branching0fromKtoG {
    More(
        MeshedChannelsEleven<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursGtoK>>,
            R2K<R2A<R2B<R2C<R2D<R2E<R2F<R2H<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameG,
        >,
    ),
    Done(MeshedChannelsEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoK = Recv<Branching0fromKtoG, End>;

// H
enum Branching0fromKtoH {
    More(
        MeshedChannelsEleven<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            Recv<(), Send<(), RecursHtoK>>,
            R2K<R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2I<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameH,
        >,
    ),
    Done(MeshedChannelsEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameH>),
}
type RecursHtoK = Recv<Branching0fromKtoH, End>;

// I
enum Branching0fromKtoI {
    More(
        MeshedChannelsEleven<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            Recv<(), Send<(), RecursItoK>>,
            R2K<R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2H<R2J<RoleK<RoleEnd>>>>>>>>>>>,
            NameI,
        >,
    ),
    Done(MeshedChannelsEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameI>),
}
type RecursItoK = Recv<Branching0fromKtoI, End>;

// J
enum Branching0fromKtoJ {
    More(
        MeshedChannelsEleven<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            Recv<(), Send<(), RecursJtoK>>,
            R2K<R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2H<R2I<RoleK<RoleEnd>>>>>>>>>>>,
            NameJ,
        >,
    ),
    Done(MeshedChannelsEleven<End, End, End, End, End, End, End, End, End, End, RoleEnd, NameJ>),
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
type EndpointMoreK = MeshedChannelsEleven<
    Send<(), Recv<(), Choose0fromKtoA>>,
    Send<(), Recv<(), Choose0fromKtoB>>,
    Send<(), Recv<(), Choose0fromKtoC>>,
    Send<(), Recv<(), Choose0fromKtoD>>,
    Send<(), Recv<(), Choose0fromKtoE>>,
    Send<(), Recv<(), Choose0fromKtoF>>,
    Send<(), Recv<(), Choose0fromKtoG>>,
    Send<(), Recv<(), Choose0fromKtoH>>,
    Send<(), Recv<(), Choose0fromKtoI>>,
    Send<(), Recv<(), Choose0fromKtoJ>>,
    R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2H<R2I<R2J<RoleBroadcast>>>>>>>>>>,
    NameK,
>;

// Creating the MP sessions
type EndpointA = MeshedChannelsEleven<
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
type EndpointB = MeshedChannelsEleven<
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
type EndpointC = MeshedChannelsEleven<
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
type EndpointD = MeshedChannelsEleven<
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
type EndpointE = MeshedChannelsEleven<
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
type EndpointF = MeshedChannelsEleven<
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
type EndpointG = MeshedChannelsEleven<
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
type EndpointH = MeshedChannelsEleven<
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
type EndpointI = MeshedChannelsEleven<
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
type EndpointJ = MeshedChannelsEleven<
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
type EndpointK = MeshedChannelsEleven<
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
        Branching0fromKtoA::More(s) => {
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromKtoB::Done(s) => {
            s.close()
        },
        Branching0fromKtoB::More(s) => {
            let (_, s) = s.recv();
            let s = s.send(());
            let s = s.send(());
            let (_, s) = s.recv();
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromKtoC::Done(s) => {
            s.close()
        },
        Branching0fromKtoC::More(s) => {
            let (_, s) = s.recv();
            let s = s.send(());
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromKtoD::Done(s) => {
            s.close()
        },
        Branching0fromKtoD::More(s) => {
            let (_, s) = s.recv();
            let s = s.send(());
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromKtoE::Done(s) => {
            s.close()
        },
        Branching0fromKtoE::More(s) => {
            let (_, s) = s.recv();
            let s = s.send(());
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            endpoint_e(s)
        },
    })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromKtoF::Done(s) => {
            s.close()
        },
        Branching0fromKtoF::More(s) => {
            let (_, s) = s.recv();
            let s = s.send(());
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            endpoint_f(s)
        },
    })
}

fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromKtoG::Done(s) => {
            s.close()
        },
        Branching0fromKtoG::More(s) => {
            let (_, s) = s.recv();
            let s = s.send(());
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            endpoint_g(s)
        },
    })
}

fn endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromKtoH::Done(s) => {
            s.close()
        },
        Branching0fromKtoH::More(s) => {
            let (_, s) = s.recv();
            let s = s.send(());
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            endpoint_h(s)
        },
    })
}

fn endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromKtoI::Done(s) => {
            s.close()
        },
        Branching0fromKtoI::More(s) => {
            let (_, s) = s.recv();
            let s = s.send(());
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let (_, s) = s.recv();
            let s = s.send(());
            endpoint_i(s)
        },
    })
}

fn endpoint_j(s: EndpointJ) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromKtoJ::Done(s) => {
            s.close()
        },
        Branching0fromKtoJ::More(s) => {
            let (_, s) = s.recv();
            let s = s.send(());
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            endpoint_j(s)
        },
    })
}

fn endpoint_k(s: EndpointK) -> Result<(), Box<dyn Error>> {
    recurs_k(s, LOOPS)
}

fn recurs_k(s: EndpointK, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_k_to_all!(
                s,
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
        i => {
            let s: EndpointMoreK = choose_mpst_k_to_all!(
                s,
                Branching0fromKtoA::More,
                Branching0fromKtoB::More,
                Branching0fromKtoC::More,
                Branching0fromKtoD::More,
                Branching0fromKtoE::More,
                Branching0fromKtoF::More,
                Branching0fromKtoG::More,
                Branching0fromKtoH::More,
                Branching0fromKtoI::More,
                Branching0fromKtoJ::More
            );

            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();

            recurs_k(s, i - 1)
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
