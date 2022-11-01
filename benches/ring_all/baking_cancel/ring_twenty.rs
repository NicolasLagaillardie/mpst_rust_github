use crossbeam_channel::bounded;

use criterion::{black_box, Criterion};

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{baker, choose, offer};

use std::error::Error;
use std::thread::{spawn, JoinHandle};
// use std::time::Duration;

// Create new roles
baker!(
    "rec_and_cancel",
    MeshedChannelsTwenty,
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
// A
enum Branching0fromTtoA {
    Forward(
        MeshedChannelsTwenty<
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
            RoleB<RoleT<RoleEnd>>,
            NameA,
        >,
    ),
    Backward(
        MeshedChannelsTwenty<
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
            RoleB<RoleT<RoleEnd>>,
            NameA,
        >,
    ),
    Done(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
    Forward(
        MeshedChannelsTwenty<
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
            RoleA<RoleC<RoleT<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannelsTwenty<
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
            RoleC<RoleA<RoleT<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
    Forward(
        MeshedChannelsTwenty<
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
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
            RoleB<RoleD<RoleT<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannelsTwenty<
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
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
            RoleD<RoleB<RoleT<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
    Forward(
        MeshedChannelsTwenty<
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
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
            RoleC<RoleE<RoleT<RoleEnd>>>,
            NameD,
        >,
    ),
    Backward(
        MeshedChannelsTwenty<
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
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
            RoleE<RoleC<RoleT<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
    Forward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
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
            RoleD<RoleF<RoleT<RoleEnd>>>,
            NameE,
        >,
    ),
    Backward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
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
            RoleF<RoleD<RoleT<RoleEnd>>>,
            NameE,
        >,
    ),
    Done(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
    Forward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
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
            RoleE<RoleG<RoleT<RoleEnd>>>,
            NameF,
        >,
    ),
    Backward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
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
            RoleG<RoleE<RoleT<RoleEnd>>>,
            NameF,
        >,
    ),
    Done(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
    Forward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
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
            RoleF<RoleH<RoleT<RoleEnd>>>,
            NameG,
        >,
    ),
    Backward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
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
            RoleH<RoleF<RoleT<RoleEnd>>>,
            NameG,
        >,
    ),
    Done(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
    Forward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
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
            RoleG<RoleI<RoleT<RoleEnd>>>,
            NameH,
        >,
    ),
    Backward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
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
            RoleI<RoleG<RoleT<RoleEnd>>>,
            NameH,
        >,
    ),
    Done(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
    Forward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
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
            RoleH<RoleJ<RoleT<RoleEnd>>>,
            NameI,
        >,
    ),
    Backward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
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
            RoleJ<RoleH<RoleT<RoleEnd>>>,
            NameI,
        >,
    ),
    Done(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
    Forward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursJtoT,
            RoleI<RoleK<RoleT<RoleEnd>>>,
            NameJ,
        >,
    ),
    Backward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursJtoT,
            RoleK<RoleI<RoleT<RoleEnd>>>,
            NameJ,
        >,
    ),
    Done(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
    Forward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursKtoT,
            RoleJ<RoleL<RoleT<RoleEnd>>>,
            NameK,
        >,
    ),
    Backward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursKtoT,
            RoleL<RoleJ<RoleT<RoleEnd>>>,
            NameK,
        >,
    ),
    Done(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
    Forward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursLtoT,
            RoleK<RoleM<RoleT<RoleEnd>>>,
            NameL,
        >,
    ),
    Backward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursLtoT,
            RoleM<RoleK<RoleT<RoleEnd>>>,
            NameL,
        >,
    ),
    Done(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
    Forward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            RecursMtoT,
            RoleL<RoleN<RoleT<RoleEnd>>>,
            NameM,
        >,
    ),
    Backward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            RecursMtoT,
            RoleN<RoleL<RoleT<RoleEnd>>>,
            NameM,
        >,
    ),
    Done(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
    Forward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            RecursNtoT,
            RoleM<RoleO<RoleT<RoleEnd>>>,
            NameN,
        >,
    ),
    Backward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            RecursNtoT,
            RoleO<RoleM<RoleT<RoleEnd>>>,
            NameN,
        >,
    ),
    Done(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
    Forward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            RecursOtoT,
            RoleN<RoleP<RoleT<RoleEnd>>>,
            NameO,
        >,
    ),
    Backward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            RecursOtoT,
            RoleP<RoleN<RoleT<RoleEnd>>>,
            NameO,
        >,
    ),
    Done(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
    Forward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            RecursPtoT,
            RoleO<RoleQ<RoleT<RoleEnd>>>,
            NameP,
        >,
    ),
    Backward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            RecursPtoT,
            RoleQ<RoleO<RoleT<RoleEnd>>>,
            NameP,
        >,
    ),
    Done(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
    Forward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            RecursQtoT,
            RoleP<RoleR<RoleT<RoleEnd>>>,
            NameQ,
        >,
    ),
    Backward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            RecursQtoT,
            RoleR<RoleP<RoleT<RoleEnd>>>,
            NameQ,
        >,
    ),
    Done(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
    Forward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            RecursRtoT,
            RoleQ<RoleS<RoleT<RoleEnd>>>,
            NameR,
        >,
    ),
    Backward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            RecursRtoT,
            RoleS<RoleQ<RoleT<RoleEnd>>>,
            NameR,
        >,
    ),
    Done(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
    Forward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), RecursStoT>,
            RoleR<RoleT<RoleT<RoleEnd>>>,
            NameS,
        >,
    ),
    Backward(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), RecursStoT>,
            RoleT<RoleR<RoleT<RoleEnd>>>,
            NameS,
        >,
    ),
    Done(
        MeshedChannelsTwenty<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
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
type EndpointForwardT = MeshedChannelsTwenty<
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
    Recv<(), Choose0fromTtoS>,
    RoleS<RoleBroadcast>,
    NameT,
>;
type EndpointBackwardT = MeshedChannelsTwenty<
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
    Send<(), Choose0fromTtoS>,
    RoleS<RoleBroadcast>,
    NameT,
>;

// Creating the MP sessions
type EndpointA = MeshedChannelsTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointB = MeshedChannelsTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointC = MeshedChannelsTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointD = MeshedChannelsTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointE = MeshedChannelsTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointF = MeshedChannelsTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointG = MeshedChannelsTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointH = MeshedChannelsTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointI = MeshedChannelsTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointJ = MeshedChannelsTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointK = MeshedChannelsTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointL = MeshedChannelsTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointM = MeshedChannelsTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointN = MeshedChannelsTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointO = MeshedChannelsTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointP = MeshedChannelsTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointQ = MeshedChannelsTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointR = MeshedChannelsTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointS = MeshedChannelsTwenty<
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
    End,
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
type EndpointT = MeshedChannelsTwenty<
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
        Branching0fromTtoA::Forward(s) => {
            let s = s.send(())?;
            endpoint_a(s)
        },
        Branching0fromTtoA::Backward(s) => {
            let (_, s) = s.recv()?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromTtoB::Done(s) => {
            s.close()
        },
        Branching0fromTtoB::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_b(s)
        },
        Branching0fromTtoB::Backward(s) => {
            let ((), s) = s.recv()?;
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
        Branching0fromTtoC::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_c(s)
        },
        Branching0fromTtoC::Backward(s) => {
            let ((), s) = s.recv()?;
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
        Branching0fromTtoD::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_d(s)
        },
        Branching0fromTtoD::Backward(s) => {
            let ((), s) = s.recv()?;
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
        Branching0fromTtoE::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_e(s)
        },
        Branching0fromTtoE::Backward(s) => {
            let ((), s) = s.recv()?;
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
        Branching0fromTtoF::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_f(s)
        },
        Branching0fromTtoF::Backward(s) => {
            let ((), s) = s.recv()?;
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
        Branching0fromTtoG::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_g(s)
        },
        Branching0fromTtoG::Backward(s) => {
            let ((), s) = s.recv()?;
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
        Branching0fromTtoH::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_h(s)
        },
        Branching0fromTtoH::Backward(s) => {
            let ((), s) = s.recv()?;
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
        Branching0fromTtoI::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_i(s)
        },
        Branching0fromTtoI::Backward(s) => {
            let ((), s) = s.recv()?;
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
        Branching0fromTtoJ::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_j(s)
        },
        Branching0fromTtoJ::Backward(s) => {
            let ((), s) = s.recv()?;
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
        Branching0fromTtoK::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_k(s)
        },
        Branching0fromTtoK::Backward(s) => {
            let ((), s) = s.recv()?;
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
        Branching0fromTtoL::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_l(s)
        },
        Branching0fromTtoL::Backward(s) => {
            let ((), s) = s.recv()?;
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
        Branching0fromTtoM::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_m(s)
        },
        Branching0fromTtoM::Backward(s) => {
            let ((), s) = s.recv()?;
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
        Branching0fromTtoN::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_n(s)
        },
        Branching0fromTtoN::Backward(s) => {
            let ((), s) = s.recv()?;
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
        Branching0fromTtoO::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_o(s)
        },
        Branching0fromTtoO::Backward(s) => {
            let ((), s) = s.recv()?;
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
        Branching0fromTtoP::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_p(s)
        },
        Branching0fromTtoP::Backward(s) => {
            let ((), s) = s.recv()?;
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
        Branching0fromTtoQ::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_q(s)
        },
        Branching0fromTtoQ::Backward(s) => {
            let ((), s) = s.recv()?;
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
        Branching0fromTtoR::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_r(s)
        },
        Branching0fromTtoR::Backward(s) => {
            let ((), s) = s.recv()?;
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
        Branching0fromTtoS::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_s(s)
        },
        Branching0fromTtoS::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
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
        i if i % 2 == 0 => {
            let s: EndpointForwardT = choose_mpst_t_to_all!(
                s,
                Branching0fromTtoA::Forward,
                Branching0fromTtoB::Forward,
                Branching0fromTtoC::Forward,
                Branching0fromTtoD::Forward,
                Branching0fromTtoE::Forward,
                Branching0fromTtoF::Forward,
                Branching0fromTtoG::Forward,
                Branching0fromTtoH::Forward,
                Branching0fromTtoI::Forward,
                Branching0fromTtoJ::Forward,
                Branching0fromTtoK::Forward,
                Branching0fromTtoL::Forward,
                Branching0fromTtoM::Forward,
                Branching0fromTtoN::Forward,
                Branching0fromTtoO::Forward,
                Branching0fromTtoP::Forward,
                Branching0fromTtoQ::Forward,
                Branching0fromTtoR::Forward,
                Branching0fromTtoS::Forward
            );

            let (_, s) = s.recv()?;

            recurs_t(s, i - 1)
        }
        i => {
            let s: EndpointBackwardT = choose_mpst_t_to_all!(
                s,
                Branching0fromTtoA::Backward,
                Branching0fromTtoB::Backward,
                Branching0fromTtoC::Backward,
                Branching0fromTtoD::Backward,
                Branching0fromTtoE::Backward,
                Branching0fromTtoF::Backward,
                Branching0fromTtoG::Backward,
                Branching0fromTtoH::Backward,
                Branching0fromTtoI::Backward,
                Branching0fromTtoJ::Backward,
                Branching0fromTtoK::Backward,
                Branching0fromTtoL::Backward,
                Branching0fromTtoM::Backward,
                Branching0fromTtoN::Backward,
                Branching0fromTtoO::Backward,
                Branching0fromTtoP::Backward,
                Branching0fromTtoQ::Backward,
                Branching0fromTtoR::Backward,
                Branching0fromTtoS::Backward
            );

            let s = s.send(())?;

            recurs_t(s, i - 1)
        }
    }
}

fn all_mpst() {
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
// A
enum BinaryA {
    Forward(Recv<(), Send<(), RecursA>>),
    Done(End),
}
type RecursA = Recv<BinaryA, End>;
fn binary_a_to_b(s: RecursA) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        BinaryA::Done(s) => {
            close(s)
        },
        BinaryA::Forward(s) => {
            let (_, s) = recv(s)?;
            let s = send((), s);
            binary_a_to_b(s)
        },
    })
}

// B
type RecursB = <RecursA as Session>::Dual;
fn binary_b_to_a(s: Send<(), Recv<(), RecursB>>) -> Result<RecursB, Box<dyn Error>> {
    let s = send((), s);
    let (_, s) = recv(s)?;
    Ok(s)
}

fn all_binaries() {
    let mut threads = Vec::new();
    let mut sessions = Vec::new();

    for _ in 0..20 {
        let (thread, s): (JoinHandle<()>, RecursB) = fork_with_thread_id(black_box(binary_a_to_b));

        threads.push(thread);
        sessions.push(s);
    }

    let main = spawn(move || {
        for _ in 0..LOOPS {
            sessions = sessions
                .into_iter()
                .map(|s| binary_b_to_a(choose!(BinaryA::Forward, s)).unwrap())
                .collect::<Vec<_>>();
        }

        sessions
            .into_iter()
            .for_each(|s| close(choose!(BinaryA::Done, s)).unwrap());

        threads.into_iter().for_each(|elt| elt.join().unwrap());
    });

    main.join().unwrap();
}

/////////////////////////

type ReceivingSendingReceiving = crossbeam_channel::Receiver<SendingReceiving>;
type SendingReceivingSending = crossbeam_channel::Sender<ReceivingSending>;

type SendingReceiving = crossbeam_channel::Sender<Receiving>;
type ReceivingSending = crossbeam_channel::Receiver<Sending>;

type Receiving = crossbeam_channel::Receiver<()>;
type Sending = crossbeam_channel::Sender<()>;

fn all_crossbeam() {
    let mut threads = Vec::new();

    for _ in 0..20 {
        let main = spawn(move || {
            for _ in 0..LOOPS {
                let (sender_0, receiver_0) = bounded::<ReceivingSendingReceiving>(1);
                let (sender_4, receiver_4) = bounded::<SendingReceivingSending>(1);

                let (sender_1, receiver_1) = bounded::<SendingReceiving>(1);
                let (sender_5, receiver_5) = bounded::<ReceivingSending>(1);

                let (sender_2, receiver_2) = bounded::<Receiving>(1);
                let (sender_6, receiver_6) = bounded::<Sending>(1);

                let (sender_3, receiver_3) = bounded::<()>(1);
                let (sender_7, receiver_7) = bounded::<()>(1);

                sender_0.send(receiver_1).unwrap();
                sender_4.send(sender_5).unwrap();

                let receiver_1_bis = receiver_0.recv().unwrap();
                let sender_5_bis = receiver_4.recv().unwrap();

                sender_1.send(sender_2).unwrap();
                sender_5_bis.send(receiver_6).unwrap();

                let sender_2_bis = receiver_1_bis.recv().unwrap();
                let receiver_6_bis = receiver_5.recv().unwrap();

                sender_2_bis.send(receiver_3).unwrap();
                sender_6.send(sender_7).unwrap();

                let receiver_2_bis = receiver_2.recv().unwrap();
                let sender_7_bis = receiver_6_bis.recv().unwrap();

                sender_3.send(()).unwrap();
                sender_7_bis.send(()).unwrap();

                receiver_2_bis.recv().unwrap();
                receiver_7.recv().unwrap();
            }

            // "Close" connection
            let (sender_close_1, receiver_close_1) = bounded::<()>(1);
            let (sender_close_2, receiver_close_2) = bounded::<()>(1);

            sender_close_1.send(()).unwrap_or(());
            sender_close_2.send(()).unwrap_or(());

            receiver_close_1.recv().unwrap_or(());
            receiver_close_2.recv().unwrap_or(());
        });

        threads.push(main);
    }

    threads.into_iter().for_each(|elt| elt.join().unwrap());
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("ring twenty baking protocol MPST {LOOPS}"), |b| {
        b.iter(all_mpst)
    });
}

pub fn ring_protocol_binary(c: &mut Criterion) {
    c.bench_function(
        &format!("ring twenty baking protocol binary {LOOPS}"),
        |b| b.iter(all_binaries),
    );
}

pub fn ring_protocol_crossbeam(c: &mut Criterion) {
    c.bench_function(
        &format!("ring twenty baking protocol crossbeam {LOOPS}"),
        |b| b.iter(all_crossbeam),
    );
}
