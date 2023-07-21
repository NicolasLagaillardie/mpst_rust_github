#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    broadcast_cancel, bundle_struct_fork_close_multi,
    create_fn_choose_mpst_cancel_multi_to_all_bundle, create_multiple_normal_name_short,
    create_multiple_normal_role_short, create_recv_mpst_session_bundle,
    create_send_check_cancel_bundle, offer_cancel_mpst,
};

use std::error::Error;

// use std::time::Duration;

// Create the new MeshedChannels for eight participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannels, 9);

// Create new roles
// normal
create_multiple_normal_role_short!(Central, A, B, C, D, E, F, G, H);

// Create new names
create_multiple_normal_name_short!(Central, A, B, C, D, E, F, G, H);

// Create new send functions
// A
create_send_check_cancel_bundle!(
    send_mpst_a_to_b, RoleB, 2 |
    send_mpst_a_to_c, RoleC, 3 |
    send_mpst_a_to_d, RoleD, 4 |
    send_mpst_a_to_e, RoleE, 5 |
    send_mpst_a_to_f, RoleF, 6 |
    send_mpst_a_to_g, RoleG, 7 |
    send_mpst_a_to_h, RoleH, 8 | =>
    NameA, MeshedChannels, 9
);

// B
create_send_check_cancel_bundle!(
    send_mpst_b_to_a, RoleA, 2 |
    send_mpst_b_to_c, RoleC, 3 |
    send_mpst_b_to_d, RoleD, 4 |
    send_mpst_b_to_e, RoleE, 5 |
    send_mpst_b_to_f, RoleF, 6 |
    send_mpst_b_to_g, RoleG, 7 |
    send_mpst_b_to_h, RoleH, 8 | =>
    NameB, MeshedChannels, 9
);

// C
create_send_check_cancel_bundle!(
    send_mpst_c_to_a, RoleA, 2 |
    send_mpst_c_to_b, RoleB, 3 |
    send_mpst_c_to_d, RoleD, 4 |
    send_mpst_c_to_e, RoleE, 5 |
    send_mpst_c_to_f, RoleF, 6 |
    send_mpst_c_to_g, RoleG, 7 |
    send_mpst_c_to_h, RoleH, 8 | =>
    NameC, MeshedChannels, 9
);

// D
create_send_check_cancel_bundle!(
    send_mpst_d_to_a, RoleA, 2 |
    send_mpst_d_to_b, RoleB, 3 |
    send_mpst_d_to_c, RoleC, 4 |
    send_mpst_d_to_e, RoleE, 5 |
    send_mpst_d_to_f, RoleF, 6 |
    send_mpst_d_to_g, RoleG, 7 |
    send_mpst_d_to_h, RoleH, 8 | =>
    NameD, MeshedChannels, 9
);

// E
create_send_check_cancel_bundle!(
    send_mpst_e_to_a, RoleA, 2 |
    send_mpst_e_to_b, RoleB, 3 |
    send_mpst_e_to_c, RoleC, 4 |
    send_mpst_e_to_d, RoleD, 5 |
    send_mpst_e_to_f, RoleF, 6 |
    send_mpst_e_to_g, RoleG, 7 |
    send_mpst_e_to_h, RoleH, 8 | =>
    NameE, MeshedChannels, 9
);

// F
create_send_check_cancel_bundle!(
    send_mpst_f_to_a, RoleA, 2 |
    send_mpst_f_to_b, RoleB, 3 |
    send_mpst_f_to_c, RoleC, 4 |
    send_mpst_f_to_d, RoleD, 5 |
    send_mpst_f_to_e, RoleE, 6 |
    send_mpst_f_to_g, RoleG, 7 |
    send_mpst_f_to_h, RoleH, 8 | =>
    NameF, MeshedChannels, 9
);

// G
create_send_check_cancel_bundle!(
    send_mpst_g_to_a, RoleA, 2 |
    send_mpst_g_to_b, RoleB, 3 |
    send_mpst_g_to_c, RoleC, 4 |
    send_mpst_g_to_d, RoleD, 5 |
    send_mpst_g_to_e, RoleE, 6 |
    send_mpst_g_to_f, RoleF, 7 |
    send_mpst_g_to_h, RoleH, 8 | =>
    NameG, MeshedChannels, 9
);

// H
create_send_check_cancel_bundle!(
    send_mpst_h_to_a, RoleA, 2 |
    send_mpst_h_to_b, RoleB, 3 |
    send_mpst_h_to_c, RoleC, 4 |
    send_mpst_h_to_d, RoleD, 5 |
    send_mpst_h_to_e, RoleE, 6 |
    send_mpst_h_to_f, RoleF, 7 |
    send_mpst_h_to_g, RoleG, 8 | =>
    NameH, MeshedChannels, 9
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 2 |
    recv_mpst_a_from_c, RoleC, 3 |
    recv_mpst_a_from_d, RoleD, 4 |
    recv_mpst_a_from_e, RoleE, 5 |
    recv_mpst_a_from_f, RoleF, 6 |
    recv_mpst_a_from_g, RoleG, 7 |
    recv_mpst_a_from_h, RoleH, 8 | =>
    NameA, MeshedChannels, 9
);

// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 2 |
    recv_mpst_b_from_c, RoleC, 3 |
    recv_mpst_b_from_d, RoleD, 4 |
    recv_mpst_b_from_e, RoleE, 5 |
    recv_mpst_b_from_f, RoleF, 6 |
    recv_mpst_b_from_g, RoleG, 7 |
    recv_mpst_b_from_h, RoleH, 8 | =>
    NameB, MeshedChannels, 9
);

// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a, RoleA, 2 |
    recv_mpst_c_from_b, RoleB, 3 |
    recv_mpst_c_from_d, RoleD, 4 |
    recv_mpst_c_from_e, RoleE, 5 |
    recv_mpst_c_from_f, RoleF, 6 |
    recv_mpst_c_from_g, RoleG, 7 |
    recv_mpst_c_from_h, RoleH, 8 | =>
    NameC, MeshedChannels, 9
);

// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_a, RoleA, 2 |
    recv_mpst_d_from_b, RoleB, 3 |
    recv_mpst_d_from_c, RoleC, 4 |
    recv_mpst_d_from_e, RoleE, 5 |
    recv_mpst_d_from_f, RoleF, 6 |
    recv_mpst_d_from_g, RoleG, 7 |
    recv_mpst_d_from_h, RoleH, 8 | =>
    NameD, MeshedChannels, 9
);

// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_a, RoleA, 2 |
    recv_mpst_e_from_b, RoleB, 3 |
    recv_mpst_e_from_c, RoleC, 4 |
    recv_mpst_e_from_d, RoleD, 5 |
    recv_mpst_e_from_f, RoleF, 6 |
    recv_mpst_e_from_g, RoleG, 7 |
    recv_mpst_e_from_h, RoleH, 8 | =>
    NameE, MeshedChannels, 9
);

// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_from_a, RoleA, 2 |
    recv_mpst_f_from_b, RoleB, 3 |
    recv_mpst_f_from_c, RoleC, 4 |
    recv_mpst_f_from_d, RoleD, 5 |
    recv_mpst_f_from_e, RoleE, 6 |
    recv_mpst_f_from_g, RoleG, 7 |
    recv_mpst_f_from_h, RoleH, 8 | =>
    NameF, MeshedChannels, 9
);

// G
create_recv_mpst_session_bundle!(
    recv_mpst_g_from_a, RoleA, 2 |
    recv_mpst_g_from_b, RoleB, 3 |
    recv_mpst_g_from_c, RoleC, 4 |
    recv_mpst_g_from_d, RoleD, 5 |
    recv_mpst_g_from_e, RoleE, 6 |
    recv_mpst_g_from_f, RoleF, 7 |
    recv_mpst_g_from_h, RoleH, 8 | =>
    NameG, MeshedChannels, 9
);

// H
create_recv_mpst_session_bundle!(
    recv_mpst_h_from_a, RoleA, 2 |
    recv_mpst_h_from_b, RoleB, 3 |
    recv_mpst_h_from_c, RoleC, 4 |
    recv_mpst_h_from_d, RoleD, 5 |
    recv_mpst_h_from_e, RoleE, 6 |
    recv_mpst_h_from_f, RoleF, 7 |
    recv_mpst_h_from_g, RoleG, 8 | =>
    NameH, MeshedChannels, 9
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

// A
enum Branching0fromHtoA {
    More(
        MeshedChannels<
            End,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursAtoH>>,
            R2H<R2B<R2C<R2D<R2E<R2F<R2G<RoleH<RoleEnd>>>>>>>>,
            NameA,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoH = Recv<(End, Branching0fromHtoA), End>;

// B
enum Branching0fromHtoB {
    More(
        MeshedChannels<
            End,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursBtoH>>,
            R2H<R2A<R2C<R2D<R2E<R2F<R2G<RoleH<RoleEnd>>>>>>>>,
            NameB,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoH = Recv<(End, Branching0fromHtoB), End>;

// C
enum Branching0fromHtoC {
    More(
        MeshedChannels<
            End,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursCtoH>>,
            R2H<R2A<R2B<R2D<R2E<R2F<R2G<RoleH<RoleEnd>>>>>>>>,
            NameC,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoH = Recv<(End, Branching0fromHtoC), End>;

// D
enum Branching0fromHtoD {
    More(
        MeshedChannels<
            End,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursDtoH>>,
            R2H<R2A<R2B<R2C<R2E<R2F<R2G<RoleH<RoleEnd>>>>>>>>,
            NameD,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoH = Recv<(End, Branching0fromHtoD), End>;

// E
enum Branching0fromHtoE {
    More(
        MeshedChannels<
            End,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            Recv<(), Send<(), RecursEtoH>>,
            R2H<R2A<R2B<R2C<R2D<R2F<R2G<RoleH<RoleEnd>>>>>>>>,
            NameE,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoH = Recv<(End, Branching0fromHtoE), End>;

// F
enum Branching0fromHtoF {
    More(
        MeshedChannels<
            End,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            Recv<(), Send<(), RecursFtoH>>,
            R2H<R2A<R2B<R2C<R2D<R2E<R2G<RoleH<RoleEnd>>>>>>>>,
            NameF,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoH = Recv<(End, Branching0fromHtoF), End>;

// G
enum Branching0fromHtoG {
    More(
        MeshedChannels<
            End,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            Recv<(), Send<(), RecursGtoH>>,
            R2H<R2A<R2B<R2C<R2D<R2E<R2F<RoleH<RoleEnd>>>>>>>>,
            NameG,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoH = Recv<(End, Branching0fromHtoG), End>;

// H
type Choose0fromHtoA = <RecursAtoH as Session>::Dual;
type Choose0fromHtoB = <RecursBtoH as Session>::Dual;
type Choose0fromHtoC = <RecursCtoH as Session>::Dual;
type Choose0fromHtoD = <RecursDtoH as Session>::Dual;
type Choose0fromHtoE = <RecursEtoH as Session>::Dual;
type Choose0fromHtoF = <RecursFtoH as Session>::Dual;
type Choose0fromHtoG = <RecursGtoH as Session>::Dual;
type EndpointDoneH = MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameH>;
type EndpointMoreH = MeshedChannels<
    End,
    Send<(), Recv<(), Choose0fromHtoA>>,
    Send<(), Recv<(), Choose0fromHtoB>>,
    Send<(), Recv<(), Choose0fromHtoC>>,
    Send<(), Recv<(), Choose0fromHtoD>>,
    Send<(), Recv<(), Choose0fromHtoE>>,
    Send<(), Recv<(), Choose0fromHtoF>>,
    Send<(), Recv<(), Choose0fromHtoG>>,
    R2A<R2B<R2C<R2D<R2E<R2F<R2G<RoleBroadcast>>>>>>>,
    NameH,
>;

// Creating the MP sessions
type EndpointCentral = MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameCentral>;
type EndpointA =
    MeshedChannels<End, End, End, End, End, End, End, RecursAtoH, RoleH<RoleEnd>, NameA>;
type EndpointB =
    MeshedChannels<End, End, End, End, End, End, End, RecursBtoH, RoleH<RoleEnd>, NameB>;
type EndpointC =
    MeshedChannels<End, End, End, End, End, End, End, RecursCtoH, RoleH<RoleEnd>, NameC>;
type EndpointD =
    MeshedChannels<End, End, End, End, End, End, End, RecursDtoH, RoleH<RoleEnd>, NameD>;
type EndpointE =
    MeshedChannels<End, End, End, End, End, End, End, RecursEtoH, RoleH<RoleEnd>, NameE>;
type EndpointF =
    MeshedChannels<End, End, End, End, End, End, End, RecursFtoH, RoleH<RoleEnd>, NameF>;
type EndpointG =
    MeshedChannels<End, End, End, End, End, End, End, RecursGtoH, RoleH<RoleEnd>, NameG>;
type EndpointH = MeshedChannels<
    End,
    Choose0fromHtoA,
    Choose0fromHtoB,
    Choose0fromHtoC,
    Choose0fromHtoD,
    Choose0fromHtoE,
    Choose0fromHtoF,
    Choose0fromHtoG,
    RoleBroadcast,
    NameH,
>;

create_fn_choose_mpst_cancel_multi_to_all_bundle!(
    done_from_h_to_all, more_from_h_to_all, =>
    Done, More, =>
    EndpointDoneH, EndpointMoreH, =>
    Branching0fromHtoA,
    Branching0fromHtoB,
    Branching0fromHtoC,
    Branching0fromHtoD,
    Branching0fromHtoE,
    Branching0fromHtoF,
    Branching0fromHtoG, =>
    NameA,
    NameB,
    NameC,
    NameD,
    NameE,
    NameF,
    NameG, =>
    NameCentral, NameH, MeshedChannels, 9
);

fn endpoint_central(s: EndpointCentral) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, 9)
}

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_a_from_h, {
        Branching0fromHtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromHtoA::More(s) => {
            let (_, s) = recv_mpst_a_from_h(s)?;
            let s = send_mpst_a_to_h((), s)?;
            let (_, s) = recv_mpst_a_from_b(s)?;
            let s = send_mpst_a_to_b((), s)?;
            let (_, s) = recv_mpst_a_from_c(s)?;
            let s = send_mpst_a_to_c((), s)?;
            let (_, s) = recv_mpst_a_from_d(s)?;
            let s = send_mpst_a_to_d((), s)?;
            let (_, s) = recv_mpst_a_from_e(s)?;
            let s = send_mpst_a_to_e((), s)?;
            let (_, s) = recv_mpst_a_from_f(s)?;
            let s = send_mpst_a_to_f((), s)?;
            let (_, s) = recv_mpst_a_from_g(s)?;
            let s = send_mpst_a_to_g((), s)?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_b_from_h, {
        Branching0fromHtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromHtoB::More(s) => {
            let (_, s) = recv_mpst_b_from_h(s)?;
            let s = send_mpst_b_to_h((), s)?;
            let s = send_mpst_b_to_a((), s)?;
            let (_, s) = recv_mpst_b_from_a(s)?;
            let (_, s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_c((), s)?;
            let (_, s) = recv_mpst_b_from_d(s)?;
            let s = send_mpst_b_to_d((), s)?;
            let (_, s) = recv_mpst_b_from_e(s)?;
            let s = send_mpst_b_to_e((), s)?;
            let (_, s) = recv_mpst_b_from_f(s)?;
            let s = send_mpst_b_to_f((), s)?;
            let (_, s) = recv_mpst_b_from_g(s)?;
            let s = send_mpst_b_to_g((), s)?;
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_c_from_h, {
        Branching0fromHtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromHtoC::More(s) => {
            let (_, s) = recv_mpst_c_from_h(s)?;
            let s = send_mpst_c_to_h((), s)?;
            let s = send_mpst_c_to_a((), s)?;
            let (_, s) = recv_mpst_c_from_a(s)?;
            let s = send_mpst_c_to_b((), s)?;
            let (_, s) = recv_mpst_c_from_b(s)?;
            let (_, s) = recv_mpst_c_from_d(s)?;
            let s = send_mpst_c_to_d((), s)?;
            let (_, s) = recv_mpst_c_from_e(s)?;
            let s = send_mpst_c_to_e((), s)?;
            let (_, s) = recv_mpst_c_from_f(s)?;
            let s = send_mpst_c_to_f((), s)?;
            let (_, s) = recv_mpst_c_from_g(s)?;
            let s = send_mpst_c_to_g((), s)?;
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_d_from_h, {
        Branching0fromHtoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromHtoD::More(s) => {
            let (_, s) = recv_mpst_d_from_h(s)?;
            let s = send_mpst_d_to_h((), s)?;
            let s = send_mpst_d_to_a((), s)?;
            let (_, s) = recv_mpst_d_from_a(s)?;
            let s = send_mpst_d_to_b((), s)?;
            let (_, s) = recv_mpst_d_from_b(s)?;
            let s = send_mpst_d_to_c((), s)?;
            let (_, s) = recv_mpst_d_from_c(s)?;
            let (_, s) = recv_mpst_d_from_e(s)?;
            let s = send_mpst_d_to_e((), s)?;
            let (_, s) = recv_mpst_d_from_f(s)?;
            let s = send_mpst_d_to_f((), s)?;
            let (_, s) = recv_mpst_d_from_g(s)?;
            let s = send_mpst_d_to_g((), s)?;
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_e_from_h, {
        Branching0fromHtoE::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromHtoE::More(s) => {
            let (_, s) = recv_mpst_e_from_h(s)?;
            let s = send_mpst_e_to_h((), s)?;
            let s = send_mpst_e_to_a((), s)?;
            let (_, s) = recv_mpst_e_from_a(s)?;
            let s = send_mpst_e_to_b((), s)?;
            let (_, s) = recv_mpst_e_from_b(s)?;
            let s = send_mpst_e_to_c((), s)?;
            let (_, s) = recv_mpst_e_from_c(s)?;
            let s = send_mpst_e_to_d((), s)?;
            let (_, s) = recv_mpst_e_from_d(s)?;
            let (_, s) = recv_mpst_e_from_f(s)?;
            let s = send_mpst_e_to_f((), s)?;
            let (_, s) = recv_mpst_e_from_g(s)?;
            let s = send_mpst_e_to_g((), s)?;
            endpoint_e(s)
        },
    })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_f_from_h, {
        Branching0fromHtoF::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromHtoF::More(s) => {
            let (_, s) = recv_mpst_f_from_h(s)?;
            let s = send_mpst_f_to_h((), s)?;
            let s = send_mpst_f_to_a((), s)?;
            let (_, s) = recv_mpst_f_from_a(s)?;
            let s = send_mpst_f_to_b((), s)?;
            let (_, s) = recv_mpst_f_from_b(s)?;
            let s = send_mpst_f_to_c((), s)?;
            let (_, s) = recv_mpst_f_from_c(s)?;
            let s = send_mpst_f_to_d((), s)?;
            let (_, s) = recv_mpst_f_from_d(s)?;
            let s = send_mpst_f_to_e((), s)?;
            let (_, s) = recv_mpst_f_from_e(s)?;
            let (_, s) = recv_mpst_f_from_g(s)?;
            let s = send_mpst_f_to_g((), s)?;
            endpoint_f(s)
        },
    })
}

fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_g_from_h, {
        Branching0fromHtoG::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromHtoG::More(s) => {
            let (_, s) = recv_mpst_g_from_h(s)?;
            let s = send_mpst_g_to_h((), s)?;
            let s = send_mpst_g_to_a((), s)?;
            let (_, s) = recv_mpst_g_from_a(s)?;
            let s = send_mpst_g_to_b((), s)?;
            let (_, s) = recv_mpst_g_from_b(s)?;
            let s = send_mpst_g_to_c((), s)?;
            let (_, s) = recv_mpst_g_from_c(s)?;
            let s = send_mpst_g_to_d((), s)?;
            let (_, s) = recv_mpst_g_from_d(s)?;
            let s = send_mpst_g_to_e((), s)?;
            let (_, s) = recv_mpst_g_from_e(s)?;
            let s = send_mpst_g_to_f((), s)?;
            let (_, s) = recv_mpst_g_from_f(s)?;
            endpoint_g(s)
        },
    })
}

fn endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    recurs_h(s, LOOPS)
}

fn recurs_h(s: EndpointH, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_h_to_all(s)?;

            close_mpst_multi(s)
        }
        i => {
            let s = more_from_h_to_all(s)?;

            let s = send_mpst_h_to_a((), s)?;
            let (_, s) = recv_mpst_h_from_a(s)?;
            let s = send_mpst_h_to_b((), s)?;
            let (_, s) = recv_mpst_h_from_b(s)?;
            let s = send_mpst_h_to_c((), s)?;
            let (_, s) = recv_mpst_h_from_c(s)?;
            let s = send_mpst_h_to_d((), s)?;
            let (_, s) = recv_mpst_h_from_d(s)?;
            let s = send_mpst_h_to_e((), s)?;
            let (_, s) = recv_mpst_h_from_e(s)?;
            let s = send_mpst_h_to_f((), s)?;
            let (_, s) = recv_mpst_h_from_f(s)?;
            let s = send_mpst_h_to_g((), s)?;
            let (_, s) = recv_mpst_h_from_g(s)?;

            recurs_h(s, i - 1)
        }
    }
}

fn aux() {
    let (
        thread_central,
        thread_a,
        thread_b,
        thread_c,
        thread_d,
        thread_e,
        thread_f,
        thread_g,
        thread_h,
    ) = fork_mpst(
        black_box(endpoint_central),
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
        black_box(endpoint_d),
        black_box(endpoint_e),
        black_box(endpoint_f),
        black_box(endpoint_g),
        black_box(endpoint_h),
    );

    thread_central.join().unwrap();
    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
    thread_f.join().unwrap();
    thread_g.join().unwrap();
    thread_h.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn mesh_protocol_mpst(c: &mut Criterion) {
    c.bench_function(
        &format!("mesh eight cancel broadcast protocol {LOOPS}"),
        |b| b.iter(aux),
    );
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(20000);
    targets = mesh_protocol_mpst,
}

criterion_main! {
    bench
}
