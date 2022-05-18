use criterion::{black_box, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    broadcast_cancel, bundle_struct_fork_close_multi,
    create_fn_choose_mpst_cancel_multi_to_all_bundle, create_multiple_normal_role_short,
    create_recv_mpst_session_bundle, create_send_check_cancel_bundle, offer_cancel_mpst,
};

use std::error::Error;
// use std::time::Duration;

// Create the new MeshedChannels for nine participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsTen, 10);

// Create new roles
// normal
create_multiple_normal_role_short!(Central, A, B, C, D, E, F, G, H, I);

// Create new send functions
// A
create_send_check_cancel_bundle!(
    send_mpst_a_to_b, RoleB, 2 |
    send_mpst_a_to_c, RoleC, 3 |
    send_mpst_a_to_d, RoleD, 4 |
    send_mpst_a_to_e, RoleE, 5 |
    send_mpst_a_to_f, RoleF, 6 |
    send_mpst_a_to_g, RoleG, 7 |
    send_mpst_a_to_h, RoleH, 8 |
    send_mpst_a_to_i, RoleI, 9 | =>
    RoleA, MeshedChannelsTen, 10
);
// B
create_send_check_cancel_bundle!(
    send_mpst_b_to_a, RoleA, 2 |
    send_mpst_b_to_c, RoleC, 3 |
    send_mpst_b_to_d, RoleD, 4 |
    send_mpst_b_to_e, RoleE, 5 |
    send_mpst_b_to_f, RoleF, 6 |
    send_mpst_b_to_g, RoleG, 7 |
    send_mpst_b_to_h, RoleH, 8 |
    send_mpst_b_to_i, RoleI, 9 | =>
    RoleB, MeshedChannelsTen, 10
);
// C
create_send_check_cancel_bundle!(
    send_mpst_c_to_a, RoleA, 2 |
    send_mpst_c_to_b, RoleB, 3 |
    send_mpst_c_to_d, RoleD, 4 |
    send_mpst_c_to_e, RoleE, 5 |
    send_mpst_c_to_f, RoleF, 6 |
    send_mpst_c_to_g, RoleG, 7 |
    send_mpst_c_to_h, RoleH, 8 |
    send_mpst_c_to_i, RoleI, 9 | =>
    RoleC, MeshedChannelsTen, 10
);
// D
create_send_check_cancel_bundle!(
    send_mpst_d_to_a, RoleA, 2 |
    send_mpst_d_to_b, RoleB, 3 |
    send_mpst_d_to_c, RoleC, 4 |
    send_mpst_d_to_e, RoleE, 5 |
    send_mpst_d_to_f, RoleF, 6 |
    send_mpst_d_to_g, RoleG, 7 |
    send_mpst_d_to_h, RoleH, 8 |
    send_mpst_d_to_i, RoleI, 9 | =>
    RoleD, MeshedChannelsTen, 10
);
// E
create_send_check_cancel_bundle!(
    send_mpst_e_to_a, RoleA, 2 |
    send_mpst_e_to_b, RoleB, 3 |
    send_mpst_e_to_c, RoleC, 4 |
    send_mpst_e_to_d, RoleD, 5 |
    send_mpst_e_to_f, RoleF, 6 |
    send_mpst_e_to_g, RoleG, 7 |
    send_mpst_e_to_h, RoleH, 8 |
    send_mpst_e_to_i, RoleI, 9 | =>
    RoleE, MeshedChannelsTen, 10
);
// F
create_send_check_cancel_bundle!(
    send_mpst_f_to_a, RoleA, 2 |
    send_mpst_f_to_b, RoleB, 3 |
    send_mpst_f_to_c, RoleC, 4 |
    send_mpst_f_to_d, RoleD, 5 |
    send_mpst_f_to_e, RoleE, 6 |
    send_mpst_f_to_g, RoleG, 7 |
    send_mpst_f_to_h, RoleH, 8 |
    send_mpst_f_to_i, RoleI, 9 | =>
    RoleF, MeshedChannelsTen, 10
);
// G
create_send_check_cancel_bundle!(
    send_mpst_g_to_a, RoleA, 2 |
    send_mpst_g_to_b, RoleB, 3 |
    send_mpst_g_to_c, RoleC, 4 |
    send_mpst_g_to_d, RoleD, 5 |
    send_mpst_g_to_e, RoleE, 6 |
    send_mpst_g_to_f, RoleF, 7 |
    send_mpst_g_to_h, RoleH, 8 |
    send_mpst_g_to_i, RoleI, 9 | =>
    RoleG, MeshedChannelsTen, 10
);
// H
create_send_check_cancel_bundle!(
    send_mpst_h_to_a, RoleA, 2 |
    send_mpst_h_to_b, RoleB, 3 |
    send_mpst_h_to_c, RoleC, 4 |
    send_mpst_h_to_d, RoleD, 5 |
    send_mpst_h_to_e, RoleE, 6 |
    send_mpst_h_to_f, RoleF, 7 |
    send_mpst_h_to_g, RoleG, 8 |
    send_mpst_h_to_i, RoleI, 9 | =>
    RoleH, MeshedChannelsTen, 10
);
// I
create_send_check_cancel_bundle!(
    send_mpst_i_to_a, RoleA, 2 |
    send_mpst_i_to_b, RoleB, 3 |
    send_mpst_i_to_c, RoleC, 4 |
    send_mpst_i_to_d, RoleD, 5 |
    send_mpst_i_to_e, RoleE, 6 |
    send_mpst_i_to_f, RoleF, 7 |
    send_mpst_i_to_g, RoleG, 8 |
    send_mpst_i_to_h, RoleH, 9 | =>
    RoleI, MeshedChannelsTen, 10
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
    recv_mpst_a_from_h, RoleH, 8 |
    recv_mpst_a_from_i, RoleI, 9 | =>
    RoleA, MeshedChannelsTen, 10
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 2 |
    recv_mpst_b_from_c, RoleC, 3 |
    recv_mpst_b_from_d, RoleD, 4 |
    recv_mpst_b_from_e, RoleE, 5 |
    recv_mpst_b_from_f, RoleF, 6 |
    recv_mpst_b_from_g, RoleG, 7 |
    recv_mpst_b_from_h, RoleH, 8 |
    recv_mpst_b_from_i, RoleI, 9 | =>
    RoleB, MeshedChannelsTen, 10
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a, RoleA, 2 |
    recv_mpst_c_from_b, RoleB, 3 |
    recv_mpst_c_from_d, RoleD, 4 |
    recv_mpst_c_from_e, RoleE, 5 |
    recv_mpst_c_from_f, RoleF, 6 |
    recv_mpst_c_from_g, RoleG, 7 |
    recv_mpst_c_from_h, RoleH, 8 |
    recv_mpst_c_from_i, RoleI, 9 | =>
    RoleC, MeshedChannelsTen, 10
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_a, RoleA, 2 |
    recv_mpst_d_from_b, RoleB, 3 |
    recv_mpst_d_from_c, RoleC, 4 |
    recv_mpst_d_from_e, RoleE, 5 |
    recv_mpst_d_from_f, RoleF, 6 |
    recv_mpst_d_from_g, RoleG, 7 |
    recv_mpst_d_from_h, RoleH, 8 |
    recv_mpst_d_from_i, RoleI, 9 | =>
    RoleD, MeshedChannelsTen, 10
);
// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_a, RoleA, 2 |
    recv_mpst_e_from_b, RoleB, 3 |
    recv_mpst_e_from_c, RoleC, 4 |
    recv_mpst_e_from_d, RoleD, 5 |
    recv_mpst_e_from_f, RoleF, 6 |
    recv_mpst_e_from_g, RoleG, 7 |
    recv_mpst_e_from_h, RoleH, 8 |
    recv_mpst_e_from_i, RoleI, 9 | =>
    RoleE, MeshedChannelsTen, 10
);
// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_from_a, RoleA, 2 |
    recv_mpst_f_from_b, RoleB, 3 |
    recv_mpst_f_from_c, RoleC, 4 |
    recv_mpst_f_from_d, RoleD, 5 |
    recv_mpst_f_from_e, RoleE, 6 |
    recv_mpst_f_from_g, RoleG, 7 |
    recv_mpst_f_from_h, RoleH, 8 |
    recv_mpst_f_from_i, RoleI, 9 | =>
    RoleF, MeshedChannelsTen, 10
);
// G
create_recv_mpst_session_bundle!(
    recv_mpst_g_from_a, RoleA, 2 |
    recv_mpst_g_from_b, RoleB, 3 |
    recv_mpst_g_from_c, RoleC, 4 |
    recv_mpst_g_from_d, RoleD, 5 |
    recv_mpst_g_from_e, RoleE, 6 |
    recv_mpst_g_from_f, RoleF, 7 |
    recv_mpst_g_from_h, RoleH, 8 |
    recv_mpst_g_from_i, RoleI, 9 | =>
    RoleG, MeshedChannelsTen, 10
);
// H
create_recv_mpst_session_bundle!(
    recv_mpst_h_from_a, RoleA, 2 |
    recv_mpst_h_from_b, RoleB, 3 |
    recv_mpst_h_from_c, RoleC, 4 |
    recv_mpst_h_from_d, RoleD, 5 |
    recv_mpst_h_from_e, RoleE, 6 |
    recv_mpst_h_from_f, RoleF, 7 |
    recv_mpst_h_from_g, RoleG, 8 |
    recv_mpst_h_from_i, RoleI, 9 | =>
    RoleH, MeshedChannelsTen, 10
);
// I
create_recv_mpst_session_bundle!(
    recv_mpst_i_from_a, RoleA, 2 |
    recv_mpst_i_from_b, RoleB, 3 |
    recv_mpst_i_from_c, RoleC, 4 |
    recv_mpst_i_from_d, RoleD, 5 |
    recv_mpst_i_from_e, RoleE, 6 |
    recv_mpst_i_from_f, RoleF, 7 |
    recv_mpst_i_from_g, RoleG, 8 |
    recv_mpst_i_from_h, RoleH, 9 | =>
    RoleI, MeshedChannelsTen, 10
);

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
// A
enum Branching0fromItoA {
    More(
        MeshedChannelsTen<
            End,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursAtoI>>,
            R2I<R2B<R2C<R2D<R2E<R2F<R2G<R2H<RoleI<RoleEnd>>>>>>>>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoI = Recv<(End, Branching0fromItoA), End>;
// B
enum Branching0fromItoB {
    More(
        MeshedChannelsTen<
            End,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursBtoI>>,
            R2I<R2A<R2C<R2D<R2E<R2F<R2G<R2H<RoleI<RoleEnd>>>>>>>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoI = Recv<(End, Branching0fromItoB), End>;
// C
enum Branching0fromItoC {
    More(
        MeshedChannelsTen<
            End,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursCtoI>>,
            R2I<R2A<R2B<R2D<R2E<R2F<R2G<R2H<RoleI<RoleEnd>>>>>>>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoI = Recv<(End, Branching0fromItoC), End>;
// D
enum Branching0fromItoD {
    More(
        MeshedChannelsTen<
            End,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursDtoI>>,
            R2I<R2A<R2B<R2C<R2E<R2F<R2G<R2H<RoleI<RoleEnd>>>>>>>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoI = Recv<(End, Branching0fromItoD), End>;
// E
enum Branching0fromItoE {
    More(
        MeshedChannelsTen<
            End,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursEtoI>>,
            R2I<R2A<R2B<R2C<R2D<R2F<R2G<R2H<RoleI<RoleEnd>>>>>>>>>,
            NameE,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoI = Recv<(End, Branching0fromItoE), End>;
// F
enum Branching0fromItoF {
    More(
        MeshedChannelsTen<
            End,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            RS,
            Recv<(), Send<(), RecursFtoI>>,
            R2I<R2A<R2B<R2C<R2D<R2E<R2G<R2H<RoleI<RoleEnd>>>>>>>>>,
            NameF,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoI = Recv<(End, Branching0fromItoF), End>;
// G
enum Branching0fromItoG {
    More(
        MeshedChannelsTen<
            End,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            RS,
            Recv<(), Send<(), RecursGtoI>>,
            R2I<R2A<R2B<R2C<R2D<R2E<R2F<R2H<RoleI<RoleEnd>>>>>>>>>,
            NameG,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoI = Recv<(End, Branching0fromItoG), End>;
// H
enum Branching0fromItoH {
    More(
        MeshedChannelsTen<
            End,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            SR,
            Recv<(), Send<(), RecursHtoI>>,
            R2I<R2A<R2B<R2C<R2D<R2E<R2F<R2G<RoleI<RoleEnd>>>>>>>>>,
            NameH,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameH>),
}
type RecursHtoI = Recv<(End, Branching0fromItoH), End>;
// I
type Choose0fromItoA = <RecursAtoI as Session>::Dual;
type Choose0fromItoB = <RecursBtoI as Session>::Dual;
type Choose0fromItoC = <RecursCtoI as Session>::Dual;
type Choose0fromItoD = <RecursDtoI as Session>::Dual;
type Choose0fromItoE = <RecursEtoI as Session>::Dual;
type Choose0fromItoF = <RecursFtoI as Session>::Dual;
type Choose0fromItoG = <RecursGtoI as Session>::Dual;
type Choose0fromItoH = <RecursHtoI as Session>::Dual;
type EndpointDoneI = MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameI>;
type EndpointMoreI = MeshedChannelsTen<
    End,
    Send<(), Recv<(), Choose0fromItoA>>,
    Send<(), Recv<(), Choose0fromItoB>>,
    Send<(), Recv<(), Choose0fromItoC>>,
    Send<(), Recv<(), Choose0fromItoD>>,
    Send<(), Recv<(), Choose0fromItoE>>,
    Send<(), Recv<(), Choose0fromItoF>>,
    Send<(), Recv<(), Choose0fromItoG>>,
    Send<(), Recv<(), Choose0fromItoH>>,
    R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2H<RoleBroadcast>>>>>>>>,
    NameI,
>;

// Creating the MP sessions
type EndpointCentral =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, RoleCentral<RoleEnd>>;
type EndpointA =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursAtoI, RoleI<RoleEnd>, NameA>;
type EndpointB =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursBtoI, RoleI<RoleEnd>, NameB>;
type EndpointC =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursCtoI, RoleI<RoleEnd>, NameC>;
type EndpointD =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursDtoI, RoleI<RoleEnd>, NameD>;
type EndpointE =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursEtoI, RoleI<RoleEnd>, NameE>;
type EndpointF =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursFtoI, RoleI<RoleEnd>, NameF>;
type EndpointG =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursGtoI, RoleI<RoleEnd>, NameG>;
type EndpointH =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursHtoI, RoleI<RoleEnd>, NameH>;
type EndpointI = MeshedChannelsTen<
    End,
    Choose0fromItoA,
    Choose0fromItoB,
    Choose0fromItoC,
    Choose0fromItoD,
    Choose0fromItoE,
    Choose0fromItoF,
    Choose0fromItoG,
    Choose0fromItoH,
    RoleBroadcast,
    NameI,
>;

create_fn_choose_mpst_cancel_multi_to_all_bundle!(
    done_from_i_to_all, more_from_i_to_all, =>
    Done, More, =>
    EndpointDoneI, EndpointMoreI, =>
    Branching0fromItoA,
    Branching0fromItoB,
    Branching0fromItoC,
    Branching0fromItoD,
    Branching0fromItoE,
    Branching0fromItoF,
    Branching0fromItoG,
    Branching0fromItoH, =>
    RoleA,
    RoleB,
    RoleC,
    RoleD,
    RoleE,
    RoleF,
    RoleG,
    RoleH, =>
    RoleCentral, RoleI, MeshedChannelsTen, 10
);

fn endpoint_central(s: EndpointCentral) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, 10)
}

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_a_from_i, {
        Branching0fromItoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoA::More(s) => {
            let (_, s) = recv_mpst_a_from_i(s)?;
            let s = send_mpst_a_to_i((), s)?;
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
            let (_, s) = recv_mpst_a_from_h(s)?;
            let s = send_mpst_a_to_h((), s)?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_b_from_i, {
        Branching0fromItoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoB::More(s) => {
            let (_, s) = recv_mpst_b_from_i(s)?;
            let s = send_mpst_b_to_i((), s)?;
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
            let (_, s) = recv_mpst_b_from_h(s)?;
            let s = send_mpst_b_to_h((), s)?;
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_c_from_i, {
        Branching0fromItoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoC::More(s) => {
            let (_, s) = recv_mpst_c_from_i(s)?;
            let s = send_mpst_c_to_i((), s)?;
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
            let (_, s) = recv_mpst_c_from_h(s)?;
            let s = send_mpst_c_to_h((), s)?;
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_d_from_i, {
        Branching0fromItoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoD::More(s) => {
            let (_, s) = recv_mpst_d_from_i(s)?;
            let s = send_mpst_d_to_i((), s)?;
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
            let (_, s) = recv_mpst_d_from_h(s)?;
            let s = send_mpst_d_to_h((), s)?;
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_e_from_i, {
        Branching0fromItoE::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoE::More(s) => {
            let (_, s) = recv_mpst_e_from_i(s)?;
            let s = send_mpst_e_to_i((), s)?;
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
            let (_, s) = recv_mpst_e_from_h(s)?;
            let s = send_mpst_e_to_h((), s)?;
            endpoint_e(s)
        },
    })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_f_from_i, {
        Branching0fromItoF::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoF::More(s) => {
            let (_, s) = recv_mpst_f_from_i(s)?;
            let s = send_mpst_f_to_i((), s)?;
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
            let (_, s) = recv_mpst_f_from_h(s)?;
            let s = send_mpst_f_to_h((), s)?;
            endpoint_f(s)
        },
    })
}

fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_g_from_i, {
        Branching0fromItoG::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoG::More(s) => {
            let (_, s) = recv_mpst_g_from_i(s)?;
            let s = send_mpst_g_to_i((), s)?;
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
            let (_, s) = recv_mpst_g_from_h(s)?;
            let s = send_mpst_g_to_h((), s)?;
            endpoint_g(s)
        },
    })
}

fn endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_h_from_i, {
        Branching0fromItoH::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoH::More(s) => {
            let (_, s) = recv_mpst_h_from_i(s)?;
            let s = send_mpst_h_to_i((), s)?;
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
            endpoint_h(s)
        },
    })
}

fn endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>> {
    recurs_i(s, LOOPS)
}

fn recurs_i(s: EndpointI, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_i_to_all(s)?;

            close_mpst_multi(s)
        }
        i => {
            let s = more_from_i_to_all(s)?;

            let s = send_mpst_i_to_a((), s)?;
            let (_, s) = recv_mpst_i_from_a(s)?;
            let s = send_mpst_i_to_b((), s)?;
            let (_, s) = recv_mpst_i_from_b(s)?;
            let s = send_mpst_i_to_c((), s)?;
            let (_, s) = recv_mpst_i_from_c(s)?;
            let s = send_mpst_i_to_d((), s)?;
            let (_, s) = recv_mpst_i_from_d(s)?;
            let s = send_mpst_i_to_e((), s)?;
            let (_, s) = recv_mpst_i_from_e(s)?;
            let s = send_mpst_i_to_f((), s)?;
            let (_, s) = recv_mpst_i_from_f(s)?;
            let s = send_mpst_i_to_g((), s)?;
            let (_, s) = recv_mpst_i_from_g(s)?;
            let s = send_mpst_i_to_h((), s)?;
            let (_, s) = recv_mpst_i_from_h(s)?;

            recurs_i(s, i - 1)
        }
    }
}

fn all_mpst() {
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
        thread_i,
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
        black_box(endpoint_i),
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
    thread_i.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn mesh_protocol_mpst(c: &mut Criterion) {
    c.bench_function(
        &format!("mesh nine cancel broadcast protocol MPST {}", LOOPS),
        |b| b.iter(all_mpst),
    );
}
