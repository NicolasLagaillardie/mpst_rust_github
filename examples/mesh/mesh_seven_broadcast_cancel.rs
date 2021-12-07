use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    broadcast_cancel, bundle_struct_fork_close_multi,
    create_fn_choose_mpst_cancel_multi_to_all_bundle, create_multiple_normal_role_short,
    create_recv_mpst_session_bundle, create_send_check_cancel_bundle, offer_cancel_mpst,
};

use std::error::Error;

// Create the new MeshedChannels for seven participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsEight, 8);

// Create new roles
// normal
create_multiple_normal_role_short!(Central, A, B, C, D, E, F, G);

// Create new send functions
// A
create_send_check_cancel_bundle!(
    send_mpst_a_to_b, RoleB, 2 |
    send_mpst_a_to_c, RoleC, 3 |
    send_mpst_a_to_d, RoleD, 4 |
    send_mpst_a_to_e, RoleE, 5 |
    send_mpst_a_to_f, RoleF, 6 |
    send_mpst_a_to_g, RoleG, 7 | =>
    RoleA, MeshedChannelsEight, 8
);
// B
create_send_check_cancel_bundle!(
    send_mpst_b_to_a, RoleA, 2 |
    send_mpst_b_to_c, RoleC, 3 |
    send_mpst_b_to_d, RoleD, 4 |
    send_mpst_b_to_e, RoleE, 5 |
    send_mpst_b_to_f, RoleF, 6 |
    send_mpst_b_to_g, RoleG, 7 | =>
    RoleB, MeshedChannelsEight, 8
);
// C
create_send_check_cancel_bundle!(
    send_mpst_c_to_a, RoleA, 2 |
    send_mpst_c_to_b, RoleB, 3 |
    send_mpst_c_to_d, RoleD, 4 |
    send_mpst_c_to_e, RoleE, 5 |
    send_mpst_c_to_f, RoleF, 6 |
    send_mpst_c_to_g, RoleG, 7 | =>
    RoleC, MeshedChannelsEight, 8
);
// D
create_send_check_cancel_bundle!(
    send_mpst_d_to_a, RoleA, 2 |
    send_mpst_d_to_b, RoleB, 3 |
    send_mpst_d_to_c, RoleC, 4 |
    send_mpst_d_to_e, RoleE, 5 |
    send_mpst_d_to_f, RoleF, 6 |
    send_mpst_d_to_g, RoleG, 7 | =>
    RoleD, MeshedChannelsEight, 8
);
// E
create_send_check_cancel_bundle!(
    send_mpst_e_to_a, RoleA, 2 |
    send_mpst_e_to_b, RoleB, 3 |
    send_mpst_e_to_c, RoleC, 4 |
    send_mpst_e_to_d, RoleD, 5 |
    send_mpst_e_to_f, RoleF, 6 |
    send_mpst_e_to_g, RoleG, 7 | =>
    RoleE, MeshedChannelsEight, 8
);
// F
create_send_check_cancel_bundle!(
    send_mpst_f_to_a, RoleA, 2 |
    send_mpst_f_to_b, RoleB, 3 |
    send_mpst_f_to_c, RoleC, 4 |
    send_mpst_f_to_d, RoleD, 5 |
    send_mpst_f_to_e, RoleE, 6 |
    send_mpst_f_to_g, RoleG, 7 | =>
    RoleF, MeshedChannelsEight, 8
);
// G
create_send_check_cancel_bundle!(
    send_mpst_g_to_a, RoleA, 2 |
    send_mpst_g_to_b, RoleB, 3 |
    send_mpst_g_to_c, RoleC, 4 |
    send_mpst_g_to_d, RoleD, 5 |
    send_mpst_g_to_e, RoleE, 6 |
    send_mpst_g_to_f, RoleF, 7 | =>
    RoleG, MeshedChannelsEight, 8
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 2 |
    recv_mpst_a_from_c, RoleC, 3 |
    recv_mpst_a_from_d, RoleD, 4 |
    recv_mpst_a_from_e, RoleE, 5 |
    recv_mpst_a_from_f, RoleF, 6 |
    recv_mpst_a_from_g, RoleG, 7 | =>
    RoleA, MeshedChannelsEight, 8
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 2 |
    recv_mpst_b_from_c, RoleC, 3 |
    recv_mpst_b_from_d, RoleD, 4 |
    recv_mpst_b_from_e, RoleE, 5 |
    recv_mpst_b_from_f, RoleF, 6 |
    recv_mpst_b_from_g, RoleG, 7 | =>
    RoleB, MeshedChannelsEight, 8
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a, RoleA, 2 |
    recv_mpst_c_from_b, RoleB, 3 |
    recv_mpst_c_from_d, RoleD, 4 |
    recv_mpst_c_from_e, RoleE, 5 |
    recv_mpst_c_from_f, RoleF, 6 |
    recv_mpst_c_from_g, RoleG, 7 | =>
    RoleC, MeshedChannelsEight, 8
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_a, RoleA, 2 |
    recv_mpst_d_from_b, RoleB, 3 |
    recv_mpst_d_from_c, RoleC, 4 |
    recv_mpst_d_from_e, RoleE, 5 |
    recv_mpst_d_from_f, RoleF, 6 |
    recv_mpst_d_from_g, RoleG, 7 | =>
    RoleD, MeshedChannelsEight, 8
);
// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_a, RoleA, 2 |
    recv_mpst_e_from_b, RoleB, 3 |
    recv_mpst_e_from_c, RoleC, 4 |
    recv_mpst_e_from_d, RoleD, 5 |
    recv_mpst_e_from_f, RoleF, 6 |
    recv_mpst_e_from_g, RoleG, 7 | =>
    RoleE, MeshedChannelsEight, 8
);
// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_from_a, RoleA, 2 |
    recv_mpst_f_from_b, RoleB, 3 |
    recv_mpst_f_from_c, RoleC, 4 |
    recv_mpst_f_from_d, RoleD, 5 |
    recv_mpst_f_from_e, RoleE, 6 |
    recv_mpst_f_from_g, RoleG, 7 | =>
    RoleF, MeshedChannelsEight, 8
);
// G
create_recv_mpst_session_bundle!(
    recv_mpst_g_from_a, RoleA, 2 |
    recv_mpst_g_from_b, RoleB, 3 |
    recv_mpst_g_from_c, RoleC, 4 |
    recv_mpst_g_from_d, RoleD, 5 |
    recv_mpst_g_from_e, RoleE, 6 |
    recv_mpst_g_from_f, RoleF, 7 | =>
    RoleG, MeshedChannelsEight, 8
);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;
type NameE = RoleE<RoleEnd>;
type NameF = RoleF<RoleEnd>;
type NameG = RoleG<RoleEnd>;

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
// A
enum Branching0fromGtoA {
    More(
        MeshedChannelsEight<
            End,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursAtoG>>,
            R2G<R2B<R2C<R2D<R2E<R2F<RoleG<RoleEnd>>>>>>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoG = Recv<(End, Branching0fromGtoA), End>;
// B
enum Branching0fromGtoB {
    More(
        MeshedChannelsEight<
            End,
            SR,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursBtoG>>,
            R2G<R2A<R2C<R2D<R2E<R2F<RoleG<RoleEnd>>>>>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoG = Recv<(End, Branching0fromGtoB), End>;
// C
enum Branching0fromGtoC {
    More(
        MeshedChannelsEight<
            End,
            SR,
            SR,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursCtoG>>,
            R2G<R2A<R2B<R2D<R2E<R2F<RoleG<RoleEnd>>>>>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoG = Recv<(End, Branching0fromGtoC), End>;
// D
enum Branching0fromGtoD {
    More(
        MeshedChannelsEight<
            End,
            SR,
            SR,
            SR,
            RS,
            RS,
            Recv<(), Send<(), RecursDtoG>>,
            R2G<R2A<R2B<R2C<R2E<R2F<RoleG<RoleEnd>>>>>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoG = Recv<(End, Branching0fromGtoD), End>;
// E
enum Branching0fromGtoE {
    More(
        MeshedChannelsEight<
            End,
            SR,
            SR,
            SR,
            SR,
            RS,
            Recv<(), Send<(), RecursEtoG>>,
            R2G<R2A<R2B<R2C<R2D<R2F<RoleG<RoleEnd>>>>>>>,
            NameE,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoG = Recv<(End, Branching0fromGtoE), End>;
// F
enum Branching0fromGtoF {
    More(
        MeshedChannelsEight<
            End,
            SR,
            SR,
            SR,
            SR,
            SR,
            Recv<(), Send<(), RecursFtoG>>,
            R2G<R2A<R2B<R2C<R2D<R2E<RoleG<RoleEnd>>>>>>>,
            NameF,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoG = Recv<(End, Branching0fromGtoF), End>;
// G
type Choose0fromGtoA = <RecursAtoG as Session>::Dual;
type Choose0fromGtoB = <RecursBtoG as Session>::Dual;
type Choose0fromGtoC = <RecursCtoG as Session>::Dual;
type Choose0fromGtoD = <RecursDtoG as Session>::Dual;
type Choose0fromGtoE = <RecursEtoG as Session>::Dual;
type Choose0fromGtoF = <RecursFtoG as Session>::Dual;
type EndpointDoneG = MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameG>;
type EndpointMoreG = MeshedChannelsEight<
    End,
    Send<(), Recv<(), Choose0fromGtoA>>,
    Send<(), Recv<(), Choose0fromGtoB>>,
    Send<(), Recv<(), Choose0fromGtoC>>,
    Send<(), Recv<(), Choose0fromGtoD>>,
    Send<(), Recv<(), Choose0fromGtoE>>,
    Send<(), Recv<(), Choose0fromGtoF>>,
    R2A<R2B<R2C<R2D<R2E<R2F<RoleBroadcast>>>>>>,
    NameG,
>;

// Creating the MP sessions
type EndpointCentral =
    MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, RoleCentral<RoleEnd>>;
type EndpointA = MeshedChannelsEight<End, End, End, End, End, End, RecursAtoG, RoleG<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsEight<End, End, End, End, End, End, RecursBtoG, RoleG<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsEight<End, End, End, End, End, End, RecursCtoG, RoleG<RoleEnd>, NameC>;
type EndpointD = MeshedChannelsEight<End, End, End, End, End, End, RecursDtoG, RoleG<RoleEnd>, NameD>;
type EndpointE = MeshedChannelsEight<End, End, End, End, End, End, RecursEtoG, RoleG<RoleEnd>, NameE>;
type EndpointF = MeshedChannelsEight<End, End, End, End, End, End, RecursFtoG, RoleG<RoleEnd>, NameF>;
type EndpointG = MeshedChannelsEight<
    End,
    Choose0fromGtoA,
    Choose0fromGtoB,
    Choose0fromGtoC,
    Choose0fromGtoD,
    Choose0fromGtoE,
    Choose0fromGtoF,
    RoleBroadcast,
    NameG,
>;

create_fn_choose_mpst_cancel_multi_to_all_bundle!(
    done_from_g_to_all, more_from_g_to_all, =>
    Done, More, =>
    EndpointDoneG, EndpointMoreG, =>
    Branching0fromGtoA,
    Branching0fromGtoB,
    Branching0fromGtoC,
    Branching0fromGtoD,
    Branching0fromGtoE,
    Branching0fromGtoF, =>
    RoleA,
    RoleB,
    RoleC,
    RoleD,
    RoleE,
    RoleF, =>
    RoleCentral, RoleG, MeshedChannelsEight, 8
);

fn endpoint_central(s: EndpointCentral) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, 8)
}

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_a_from_g, {
        Branching0fromGtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromGtoA::More(s) => {
            let (_, s) = recv_mpst_a_from_g(s)?;
            let s = send_mpst_a_to_g((), s)?;
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
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_b_from_g, {
        Branching0fromGtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromGtoB::More(s) => {
            let (_, s) = recv_mpst_b_from_g(s)?;
            let s = send_mpst_b_to_g((), s)?;
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
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_c_from_g, {
        Branching0fromGtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromGtoC::More(s) => {
            let (_, s) = recv_mpst_c_from_g(s)?;
            let s = send_mpst_c_to_g((), s)?;
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
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_d_from_g, {
        Branching0fromGtoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromGtoD::More(s) => {
            let (_, s) = recv_mpst_d_from_g(s)?;
            let s = send_mpst_d_to_g((), s)?;
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
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_e_from_g, {
        Branching0fromGtoE::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromGtoE::More(s) => {
            let (_, s) = recv_mpst_e_from_g(s)?;
            let s = send_mpst_e_to_g((), s)?;
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
            endpoint_e(s)
        },
    })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_f_from_g, {
        Branching0fromGtoF::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromGtoF::More(s) => {
            let (_, s) = recv_mpst_f_from_g(s)?;
            let s = send_mpst_f_to_g((), s)?;
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
            endpoint_f(s)
        },
    })
}

fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    recurs_g(s, 100)
}

fn recurs_g(s: EndpointG, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_g_to_all(s)?;

            close_mpst_multi(s)
        }
        i => {
            let s = more_from_g_to_all(s)?;

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

            recurs_g(s, i - 1)
        }
    }
}

fn main() {
    let (thread_central, thread_a, thread_b, thread_c, thread_d, thread_e, thread_f, thread_g) =
        fork_mpst(
            endpoint_central,
            endpoint_a,
            endpoint_b,
            endpoint_c,
            endpoint_d,
            endpoint_e,
            endpoint_f,
            endpoint_g,
        );

    assert!(thread_central.join().is_ok());
    assert!(thread_a.join().is_ok());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_ok());
    assert!(thread_d.join().is_ok());
    assert!(thread_e.join().is_ok());
    assert!(thread_f.join().is_ok());
    assert!(thread_g.join().is_ok());
}
