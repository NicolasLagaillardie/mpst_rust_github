#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::generate;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::error::Error;

// Create new roles
generate!(
    "rec_and_cancel",
    MeshedChannels,
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
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T
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
type R2L<R> = RoleL<RoleL<R>>;
type R2M<R> = RoleM<RoleM<R>>;
type R2N<R> = RoleN<RoleN<R>>;
type R2O<R> = RoleO<RoleO<R>>;
type R2P<R> = RoleP<RoleP<R>>;
type R2Q<R> = RoleQ<RoleQ<R>>;
type R2R<R> = RoleR<RoleR<R>>;
type R2S<R> = RoleS<RoleS<R>>;
type R2T<R> = RoleT<RoleT<R>>;

// A
enum Branching0fromTtoA {
    More(
        MeshedChannels<
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursAtoT>>,
            R2T<
                R2B<
                    R2C<
                        R2D<
                            R2E<
                                R2F<
                                    R2G<
                                        R2H<
                                            R2I<
                                                R2J<
                                                    R2K<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameA,
        >,
    ),
    Done(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RoleEnd,
            NameA,
        >,
    ),
}
type RecursAtoT = Recv<Branching0fromTtoA, End>;

// B
enum Branching0fromTtoB {
    More(
        MeshedChannels<
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursBtoT>>,
            R2T<
                R2A<
                    R2C<
                        R2D<
                            R2E<
                                R2F<
                                    R2G<
                                        R2H<
                                            R2I<
                                                R2J<
                                                    R2K<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameB,
        >,
    ),
    Done(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RoleEnd,
            NameB,
        >,
    ),
}
type RecursBtoT = Recv<Branching0fromTtoB, End>;

// C
enum Branching0fromTtoC {
    More(
        MeshedChannels<
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursCtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2D<
                            R2E<
                                R2F<
                                    R2G<
                                        R2H<
                                            R2I<
                                                R2J<
                                                    R2K<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameC,
        >,
    ),
    Done(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RoleEnd,
            NameC,
        >,
    ),
}
type RecursCtoT = Recv<Branching0fromTtoC, End>;

// D
enum Branching0fromTtoD {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursDtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2E<
                                R2F<
                                    R2G<
                                        R2H<
                                            R2I<
                                                R2J<
                                                    R2K<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameD,
        >,
    ),
    Done(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RoleEnd,
            NameD,
        >,
    ),
}
type RecursDtoT = Recv<Branching0fromTtoD, End>;

// E
enum Branching0fromTtoE {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursEtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2F<
                                    R2G<
                                        R2H<
                                            R2I<
                                                R2J<
                                                    R2K<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameE,
        >,
    ),
    Done(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RoleEnd,
            NameE,
        >,
    ),
}
type RecursEtoT = Recv<Branching0fromTtoE, End>;

// F
enum Branching0fromTtoF {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursFtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2G<
                                        R2H<
                                            R2I<
                                                R2J<
                                                    R2K<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameF,
        >,
    ),
    Done(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RoleEnd,
            NameF,
        >,
    ),
}
type RecursFtoT = Recv<Branching0fromTtoF, End>;

// G
enum Branching0fromTtoG {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursGtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2H<
                                            R2I<
                                                R2J<
                                                    R2K<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameG,
        >,
    ),
    Done(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RoleEnd,
            NameG,
        >,
    ),
}
type RecursGtoT = Recv<Branching0fromTtoG, End>;

// H
enum Branching0fromTtoH {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursHtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2I<
                                                R2J<
                                                    R2K<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameH,
        >,
    ),
    Done(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RoleEnd,
            NameH,
        >,
    ),
}
type RecursHtoT = Recv<Branching0fromTtoH, End>;

// I
enum Branching0fromTtoI {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursItoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2J<
                                                    R2K<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameI,
        >,
    ),
    Done(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RoleEnd,
            NameI,
        >,
    ),
}
type RecursItoT = Recv<Branching0fromTtoI, End>;

// J
enum Branching0fromTtoJ {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursJtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2I<
                                                    R2K<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameJ,
        >,
    ),
    Done(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RoleEnd,
            NameJ,
        >,
    ),
}
type RecursJtoT = Recv<Branching0fromTtoJ, End>;

// K
enum Branching0fromTtoK {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursKtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2I<
                                                    R2J<
                                                        R2L<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameK,
        >,
    ),
    Done(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RoleEnd,
            NameK,
        >,
    ),
}
type RecursKtoT = Recv<Branching0fromTtoK, End>;

// L
enum Branching0fromTtoL {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursLtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2I<
                                                    R2J<
                                                        R2K<
                                                            R2M<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameL,
        >,
    ),
    Done(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RoleEnd,
            NameL,
        >,
    ),
}
type RecursLtoT = Recv<Branching0fromTtoL, End>;

// M
enum Branching0fromTtoM {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursMtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2I<
                                                    R2J<
                                                        R2K<
                                                            R2L<
                                                                R2N<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameM,
        >,
    ),
    Done(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RoleEnd,
            NameM,
        >,
    ),
}
type RecursMtoT = Recv<Branching0fromTtoM, End>;

// N
enum Branching0fromTtoN {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursNtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2I<
                                                    R2J<
                                                        R2K<
                                                            R2L<
                                                                R2M<
                                                                    R2O<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameN,
        >,
    ),
    Done(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RoleEnd,
            NameN,
        >,
    ),
}
type RecursNtoT = Recv<Branching0fromTtoN, End>;

// O
enum Branching0fromTtoO {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursOtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2I<
                                                    R2J<
                                                        R2K<
                                                            R2L<
                                                                R2M<
                                                                    R2N<
                                                                        R2P<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameO,
        >,
    ),
    Done(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RoleEnd,
            NameO,
        >,
    ),
}
type RecursOtoT = Recv<Branching0fromTtoO, End>;

// P
enum Branching0fromTtoP {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursPtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2I<
                                                    R2J<
                                                        R2K<
                                                            R2L<
                                                                R2M<
                                                                    R2N<
                                                                        R2O<
                                                                            R2Q<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameP,
        >,
    ),
    Done(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RoleEnd,
            NameP,
        >,
    ),
}
type RecursPtoT = Recv<Branching0fromTtoP, End>;

// Q
enum Branching0fromTtoQ {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            Recv<(), Send<(), RecursQtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2I<
                                                    R2J<
                                                        R2K<
                                                            R2L<
                                                                R2M<
                                                                    R2N<
                                                                        R2O<
                                                                            R2P<
                                                                                R2R<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameQ,
        >,
    ),
    Done(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RoleEnd,
            NameQ,
        >,
    ),
}
type RecursQtoT = Recv<Branching0fromTtoQ, End>;

// R
enum Branching0fromTtoR {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            Recv<(), Send<(), RecursRtoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2I<
                                                    R2J<
                                                        R2K<
                                                            R2L<
                                                                R2M<
                                                                    R2N<
                                                                        R2O<
                                                                            R2P<
                                                                                R2Q<
                                                                                    R2S<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameR,
        >,
    ),
    Done(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RoleEnd,
            NameR,
        >,
    ),
}
type RecursRtoT = Recv<Branching0fromTtoR, End>;

// S
enum Branching0fromTtoS {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            Recv<(), Send<(), RecursStoT>>,
            R2T<
                R2A<
                    R2B<
                        R2C<
                            R2D<
                                R2E<
                                    R2F<
                                        R2G<
                                            R2H<
                                                R2I<
                                                    R2J<
                                                        R2K<
                                                            R2L<
                                                                R2M<
                                                                    R2N<
                                                                        R2O<
                                                                            R2P<
                                                                                R2Q<
                                                                                    R2R<
                                                                                        RoleT<
                                                                                            RoleEnd,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            NameS,
        >,
    ),
    Done(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RoleEnd,
            NameS,
        >,
    ),
}
type RecursStoT = Recv<Branching0fromTtoS, End>;

// T
type Choose0fromTtoA = Send<Branching0fromTtoA, End>;
type Choose0fromTtoB = Send<Branching0fromTtoB, End>;
type Choose0fromTtoC = Send<Branching0fromTtoC, End>;
type Choose0fromTtoD = Send<Branching0fromTtoD, End>;
type Choose0fromTtoE = Send<Branching0fromTtoE, End>;
type Choose0fromTtoF = Send<Branching0fromTtoF, End>;
type Choose0fromTtoG = Send<Branching0fromTtoG, End>;
type Choose0fromTtoH = Send<Branching0fromTtoH, End>;
type Choose0fromTtoI = Send<Branching0fromTtoI, End>;
type Choose0fromTtoJ = Send<Branching0fromTtoJ, End>;
type Choose0fromTtoK = Send<Branching0fromTtoK, End>;
type Choose0fromTtoL = Send<Branching0fromTtoL, End>;
type Choose0fromTtoM = Send<Branching0fromTtoM, End>;
type Choose0fromTtoN = Send<Branching0fromTtoN, End>;
type Choose0fromTtoO = Send<Branching0fromTtoO, End>;
type Choose0fromTtoP = Send<Branching0fromTtoP, End>;
type Choose0fromTtoQ = Send<Branching0fromTtoQ, End>;
type Choose0fromTtoR = Send<Branching0fromTtoR, End>;
type Choose0fromTtoS = Send<Branching0fromTtoS, End>;
type EndpointMoreT = MeshedChannels<
    Send<(), Recv<(), Choose0fromTtoA>>,
    Send<(), Recv<(), Choose0fromTtoB>>,
    Send<(), Recv<(), Choose0fromTtoC>>,
    Send<(), Recv<(), Choose0fromTtoD>>,
    Send<(), Recv<(), Choose0fromTtoE>>,
    Send<(), Recv<(), Choose0fromTtoF>>,
    Send<(), Recv<(), Choose0fromTtoG>>,
    Send<(), Recv<(), Choose0fromTtoH>>,
    Send<(), Recv<(), Choose0fromTtoI>>,
    Send<(), Recv<(), Choose0fromTtoJ>>,
    Send<(), Recv<(), Choose0fromTtoK>>,
    Send<(), Recv<(), Choose0fromTtoL>>,
    Send<(), Recv<(), Choose0fromTtoM>>,
    Send<(), Recv<(), Choose0fromTtoN>>,
    Send<(), Recv<(), Choose0fromTtoO>>,
    Send<(), Recv<(), Choose0fromTtoP>>,
    Send<(), Recv<(), Choose0fromTtoQ>>,
    Send<(), Recv<(), Choose0fromTtoR>>,
    Send<(), Recv<(), Choose0fromTtoS>>,
    R2A<
        R2B<
            R2C<
                R2D<
                    R2E<
                        R2F<
                            R2G<
                                R2H<
                                    R2I<
                                        R2J<
                                            R2K<
                                                R2L<
                                                    R2M<
                                                        R2N<R2O<R2P<R2Q<R2R<R2S<RoleBroadcast>>>>>>,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >,
    NameT,
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursAtoT,
    RoleT<RoleEnd>,
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursBtoT,
    RoleT<RoleEnd>,
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursCtoT,
    RoleT<RoleEnd>,
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursDtoT,
    RoleT<RoleEnd>,
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursEtoT,
    RoleT<RoleEnd>,
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursFtoT,
    RoleT<RoleEnd>,
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursGtoT,
    RoleT<RoleEnd>,
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursHtoT,
    RoleT<RoleEnd>,
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursItoT,
    RoleT<RoleEnd>,
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
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursJtoT,
    RoleT<RoleEnd>,
    NameJ,
>;
type EndpointK = MeshedChannels<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursKtoT,
    RoleT<RoleEnd>,
    NameK,
>;
type EndpointL = MeshedChannels<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursLtoT,
    RoleT<RoleEnd>,
    NameL,
>;
type EndpointM = MeshedChannels<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursMtoT,
    RoleT<RoleEnd>,
    NameM,
>;
type EndpointN = MeshedChannels<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursNtoT,
    RoleT<RoleEnd>,
    NameN,
>;
type EndpointO = MeshedChannels<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursOtoT,
    RoleT<RoleEnd>,
    NameO,
>;
type EndpointP = MeshedChannels<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursPtoT,
    RoleT<RoleEnd>,
    NameP,
>;
type EndpointQ = MeshedChannels<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursQtoT,
    RoleT<RoleEnd>,
    NameQ,
>;
type EndpointR = MeshedChannels<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursRtoT,
    RoleT<RoleEnd>,
    NameR,
>;
type EndpointS = MeshedChannels<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    RecursStoT,
    RoleT<RoleEnd>,
    NameS,
>;
type EndpointT = MeshedChannels<
    Choose0fromTtoA,
    Choose0fromTtoB,
    Choose0fromTtoC,
    Choose0fromTtoD,
    Choose0fromTtoE,
    Choose0fromTtoF,
    Choose0fromTtoG,
    Choose0fromTtoH,
    Choose0fromTtoI,
    Choose0fromTtoJ,
    Choose0fromTtoK,
    Choose0fromTtoL,
    Choose0fromTtoM,
    Choose0fromTtoN,
    Choose0fromTtoO,
    Choose0fromTtoP,
    Choose0fromTtoQ,
    Choose0fromTtoR,
    Choose0fromTtoS,
    RoleBroadcast,
    NameT,
>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromTtoA::Done(s) => {
            s.close()
        },
        Branching0fromTtoA::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromTtoB::Done(s) => {
            s.close()
        },
        Branching0fromTtoB::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromTtoC::Done(s) => {
            s.close()
        },
        Branching0fromTtoC::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromTtoD::Done(s) => {
            s.close()
        },
        Branching0fromTtoD::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromTtoE::Done(s) => {
            s.close()
        },
        Branching0fromTtoE::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_e(s)
        },
    })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromTtoF::Done(s) => {
            s.close()
        },
        Branching0fromTtoF::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_f(s)
        },
    })
}

fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromTtoG::Done(s) => {
            s.close()
        },
        Branching0fromTtoG::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_g(s)
        },
    })
}

fn endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromTtoH::Done(s) => {
            s.close()
        },
        Branching0fromTtoH::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_h(s)
        },
    })
}

fn endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromTtoI::Done(s) => {
            s.close()
        },
        Branching0fromTtoI::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_i(s)
        },
    })
}

fn endpoint_j(s: EndpointJ) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromTtoJ::Done(s) => {
            s.close()
        },
        Branching0fromTtoJ::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_j(s)
        },
    })
}

fn endpoint_k(s: EndpointK) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromTtoK::Done(s) => {
            s.close()
        },
        Branching0fromTtoK::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_k(s)
        },
    })
}

fn endpoint_l(s: EndpointL) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromTtoL::Done(s) => {
            s.close()
        },
        Branching0fromTtoL::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_l(s)
        },
    })
}

fn endpoint_m(s: EndpointM) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromTtoM::Done(s) => {
            s.close()
        },
        Branching0fromTtoM::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_m(s)
        },
    })
}

fn endpoint_n(s: EndpointN) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromTtoN::Done(s) => {
            s.close()
        },
        Branching0fromTtoN::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_n(s)
        },
    })
}

fn endpoint_o(s: EndpointO) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromTtoO::Done(s) => {
            s.close()
        },
        Branching0fromTtoO::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_o(s)
        },
    })
}

fn endpoint_p(s: EndpointP) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromTtoP::Done(s) => {
            s.close()
        },
        Branching0fromTtoP::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_p(s)
        },
    })
}

fn endpoint_q(s: EndpointQ) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromTtoQ::Done(s) => {
            s.close()
        },
        Branching0fromTtoQ::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_q(s)
        },
    })
}

fn endpoint_r(s: EndpointR) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromTtoR::Done(s) => {
            s.close()
        },
        Branching0fromTtoR::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_r(s)
        },
    })
}

fn endpoint_s(s: EndpointS) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromTtoS::Done(s) => {
            s.close()
        },
        Branching0fromTtoS::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            endpoint_s(s)
        },
    })
}

fn endpoint_t(s: EndpointT) -> Result<(), Box<dyn Error>> {
    recurs_t(s, LOOPS)
}

fn recurs_t(s: EndpointT, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_t_to_all!(
                s,
                Branching0fromTtoA::Done,
                Branching0fromTtoB::Done,
                Branching0fromTtoC::Done,
                Branching0fromTtoD::Done,
                Branching0fromTtoE::Done,
                Branching0fromTtoF::Done,
                Branching0fromTtoG::Done,
                Branching0fromTtoH::Done,
                Branching0fromTtoI::Done,
                Branching0fromTtoJ::Done,
                Branching0fromTtoK::Done,
                Branching0fromTtoL::Done,
                Branching0fromTtoM::Done,
                Branching0fromTtoN::Done,
                Branching0fromTtoO::Done,
                Branching0fromTtoP::Done,
                Branching0fromTtoQ::Done,
                Branching0fromTtoR::Done,
                Branching0fromTtoS::Done
            );

            s.close()
        }
        i => {
            let s: EndpointMoreT = choose_mpst_t_to_all!(
                s,
                Branching0fromTtoA::More,
                Branching0fromTtoB::More,
                Branching0fromTtoC::More,
                Branching0fromTtoD::More,
                Branching0fromTtoE::More,
                Branching0fromTtoF::More,
                Branching0fromTtoG::More,
                Branching0fromTtoH::More,
                Branching0fromTtoI::More,
                Branching0fromTtoJ::More,
                Branching0fromTtoK::More,
                Branching0fromTtoL::More,
                Branching0fromTtoM::More,
                Branching0fromTtoN::More,
                Branching0fromTtoO::More,
                Branching0fromTtoP::More,
                Branching0fromTtoQ::More,
                Branching0fromTtoR::More,
                Branching0fromTtoS::More
            );

            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;

            recurs_t(s, i - 1)
        }
    }
}

fn aux() {
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
        thread_l,
        thread_m,
        thread_n,
        thread_o,
        thread_p,
        thread_q,
        thread_r,
        thread_s,
        thread_t,
    ) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
        black_box(endpoint_d),
        black_box(endpoint_e),
        black_box(endpoint_f),
        black_box(endpoint_g),
        black_box(endpoint_h),
        black_box(endpoint_i),
        black_box(endpoint_j),
        black_box(endpoint_k),
        black_box(endpoint_l),
        black_box(endpoint_m),
        black_box(endpoint_n),
        black_box(endpoint_o),
        black_box(endpoint_p),
        black_box(endpoint_q),
        black_box(endpoint_r),
        black_box(endpoint_s),
        black_box(endpoint_t),
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
    thread_l.join().unwrap();
    thread_m.join().unwrap();
    thread_n.join().unwrap();
    thread_o.join().unwrap();
    thread_p.join().unwrap();
    thread_q.join().unwrap();
    thread_r.join().unwrap();
    thread_s.join().unwrap();
    thread_t.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn mesh(c: &mut Criterion) {
    c.bench_function(&format!("mesh twenty baking AMPST {LOOPS}"), |b| {
        b.iter(aux)
    });
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(100000);
    targets = mesh,
}

criterion_main! {
    bench
}
