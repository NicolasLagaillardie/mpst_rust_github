#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, create_fn_choose_mpst_multi_to_all_bundle,
    create_multiple_normal_role_short, create_recv_mpst_session_bundle,
    create_send_mpst_session_bundle, offer_mpst,create_multiple_normal_name_short
};

use std::error::Error;

// Create the new MeshedChannels for nine participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsNine, 9);

// Create new roles
// normal
create_multiple_normal_role_short!(A, B, C, D, E, F, G, H, I);

// Create new names
create_multiple_normal_name_short!(A, B, C, D, E, F, G, H, I);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, 1 |
    send_mpst_a_to_c, RoleC, 2 |
    send_mpst_a_to_d, RoleD, 3 |
    send_mpst_a_to_e, RoleE, 4 |
    send_mpst_a_to_f, RoleF, 5 |
    send_mpst_a_to_g, RoleG, 6 |
    send_mpst_a_to_h, RoleH, 7 |
    send_mpst_a_to_i, RoleI, 8 | =>
    NameA, MeshedChannelsNine, 9
);
// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a, RoleA, 1 |
    send_mpst_b_to_c, RoleC, 2 |
    send_mpst_b_to_d, RoleD, 3 |
    send_mpst_b_to_e, RoleE, 4 |
    send_mpst_b_to_f, RoleF, 5 |
    send_mpst_b_to_g, RoleG, 6 |
    send_mpst_b_to_h, RoleH, 7 |
    send_mpst_b_to_i, RoleI, 8 | =>
    NameB, MeshedChannelsNine, 9
);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a, RoleA, 1 |
    send_mpst_c_to_b, RoleB, 2 |
    send_mpst_c_to_d, RoleD, 3 |
    send_mpst_c_to_e, RoleE, 4 |
    send_mpst_c_to_f, RoleF, 5 |
    send_mpst_c_to_g, RoleG, 6 |
    send_mpst_c_to_h, RoleH, 7 |
    send_mpst_c_to_i, RoleI, 8 | =>
    NameC, MeshedChannelsNine, 9
);
// D
create_send_mpst_session_bundle!(
    send_mpst_d_to_a, RoleA, 1 |
    send_mpst_d_to_b, RoleB, 2 |
    send_mpst_d_to_c, RoleC, 3 |
    send_mpst_d_to_e, RoleE, 4 |
    send_mpst_d_to_f, RoleF, 5 |
    send_mpst_d_to_g, RoleG, 6 |
    send_mpst_d_to_h, RoleH, 7 |
    send_mpst_d_to_i, RoleI, 8 | =>
    NameD, MeshedChannelsNine, 9
);
// E
create_send_mpst_session_bundle!(
    send_mpst_e_to_a, RoleA, 1 |
    send_mpst_e_to_b, RoleB, 2 |
    send_mpst_e_to_c, RoleC, 3 |
    send_mpst_e_to_d, RoleD, 4 |
    send_mpst_e_to_f, RoleF, 5 |
    send_mpst_e_to_g, RoleG, 6 |
    send_mpst_e_to_h, RoleH, 7 |
    send_mpst_e_to_i, RoleI, 8 | =>
    NameE, MeshedChannelsNine, 9
);
// F
create_send_mpst_session_bundle!(
    send_mpst_f_to_a, RoleA, 1 |
    send_mpst_f_to_b, RoleB, 2 |
    send_mpst_f_to_c, RoleC, 3 |
    send_mpst_f_to_d, RoleD, 4 |
    send_mpst_f_to_e, RoleE, 5 |
    send_mpst_f_to_g, RoleG, 6 |
    send_mpst_f_to_h, RoleH, 7 |
    send_mpst_f_to_i, RoleI, 8 | =>
    NameF, MeshedChannelsNine, 9
);
// G
create_send_mpst_session_bundle!(
    send_mpst_g_to_a, RoleA, 1 |
    send_mpst_g_to_b, RoleB, 2 |
    send_mpst_g_to_c, RoleC, 3 |
    send_mpst_g_to_d, RoleD, 4 |
    send_mpst_g_to_e, RoleE, 5 |
    send_mpst_g_to_f, RoleF, 6 |
    send_mpst_g_to_h, RoleH, 7 |
    send_mpst_g_to_i, RoleI, 8 | =>
    NameG, MeshedChannelsNine, 9
);
// H
create_send_mpst_session_bundle!(
    send_mpst_h_to_a, RoleA, 1 |
    send_mpst_h_to_b, RoleB, 2 |
    send_mpst_h_to_c, RoleC, 3 |
    send_mpst_h_to_d, RoleD, 4 |
    send_mpst_h_to_e, RoleE, 5 |
    send_mpst_h_to_f, RoleF, 6 |
    send_mpst_h_to_g, RoleG, 7 |
    send_mpst_h_to_i, RoleI, 8 | =>
    NameH, MeshedChannelsNine, 9
);
// I
create_send_mpst_session_bundle!(
    send_mpst_i_to_a, RoleA, 1 |
    send_mpst_i_to_b, RoleB, 2 |
    send_mpst_i_to_c, RoleC, 3 |
    send_mpst_i_to_d, RoleD, 4 |
    send_mpst_i_to_e, RoleE, 5 |
    send_mpst_i_to_f, RoleF, 6 |
    send_mpst_i_to_g, RoleG, 7 |
    send_mpst_i_to_h, RoleH, 8 | =>
    NameI, MeshedChannelsNine, 9
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 1 |
    recv_mpst_a_from_c, RoleC, 2 |
    recv_mpst_a_from_d, RoleD, 3 |
    recv_mpst_a_from_e, RoleE, 4 |
    recv_mpst_a_from_f, RoleF, 5 |
    recv_mpst_a_from_g, RoleG, 6 |
    recv_mpst_a_from_h, RoleH, 7 |
    recv_mpst_a_from_i, RoleI, 8 | =>
    NameA, MeshedChannelsNine, 9
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 1 |
    recv_mpst_b_from_c, RoleC, 2 |
    recv_mpst_b_from_d, RoleD, 3 |
    recv_mpst_b_from_e, RoleE, 4 |
    recv_mpst_b_from_f, RoleF, 5 |
    recv_mpst_b_from_g, RoleG, 6 |
    recv_mpst_b_from_h, RoleH, 7 |
    recv_mpst_b_from_i, RoleI, 8 | =>
    NameB, MeshedChannelsNine, 9
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a, RoleA, 1 |
    recv_mpst_c_from_b, RoleB, 2 |
    recv_mpst_c_from_d, RoleD, 3 |
    recv_mpst_c_from_e, RoleE, 4 |
    recv_mpst_c_from_f, RoleF, 5 |
    recv_mpst_c_from_g, RoleG, 6 |
    recv_mpst_c_from_h, RoleH, 7 |
    recv_mpst_c_from_i, RoleI, 8 | =>
    NameC, MeshedChannelsNine, 9
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_a, RoleA, 1 |
    recv_mpst_d_from_b, RoleB, 2 |
    recv_mpst_d_from_c, RoleC, 3 |
    recv_mpst_d_from_e, RoleE, 4 |
    recv_mpst_d_from_f, RoleF, 5 |
    recv_mpst_d_from_g, RoleG, 6 |
    recv_mpst_d_from_h, RoleH, 7 |
    recv_mpst_d_from_i, RoleI, 8 | =>
    NameD, MeshedChannelsNine, 9
);
// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_a, RoleA, 1 |
    recv_mpst_e_from_b, RoleB, 2 |
    recv_mpst_e_from_c, RoleC, 3 |
    recv_mpst_e_from_d, RoleD, 4 |
    recv_mpst_e_from_f, RoleF, 5 |
    recv_mpst_e_from_g, RoleG, 6 |
    recv_mpst_e_from_h, RoleH, 7 |
    recv_mpst_e_from_i, RoleI, 8 | =>
    NameE, MeshedChannelsNine, 9
);
// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_from_a, RoleA, 1 |
    recv_mpst_f_from_b, RoleB, 2 |
    recv_mpst_f_from_c, RoleC, 3 |
    recv_mpst_f_from_d, RoleD, 4 |
    recv_mpst_f_from_e, RoleE, 5 |
    recv_mpst_f_from_g, RoleG, 6 |
    recv_mpst_f_from_h, RoleH, 7 |
    recv_mpst_f_from_i, RoleI, 8 | =>
    NameF, MeshedChannelsNine, 9
);
// G
create_recv_mpst_session_bundle!(
    recv_mpst_g_from_a, RoleA, 1 |
    recv_mpst_g_from_b, RoleB, 2 |
    recv_mpst_g_from_c, RoleC, 3 |
    recv_mpst_g_from_d, RoleD, 4 |
    recv_mpst_g_from_e, RoleE, 5 |
    recv_mpst_g_from_f, RoleF, 6 |
    recv_mpst_g_from_h, RoleH, 7 |
    recv_mpst_g_from_i, RoleI, 8 | =>
    NameG, MeshedChannelsNine, 9
);
// H
create_recv_mpst_session_bundle!(
    recv_mpst_h_from_a, RoleA, 1 |
    recv_mpst_h_from_b, RoleB, 2 |
    recv_mpst_h_from_c, RoleC, 3 |
    recv_mpst_h_from_d, RoleD, 4 |
    recv_mpst_h_from_e, RoleE, 5 |
    recv_mpst_h_from_f, RoleF, 6 |
    recv_mpst_h_from_g, RoleG, 7 |
    recv_mpst_h_from_i, RoleI, 8 | =>
    NameH, MeshedChannelsNine, 9
);
// I
create_recv_mpst_session_bundle!(
    recv_mpst_i_from_a, RoleA, 1 |
    recv_mpst_i_from_b, RoleB, 2 |
    recv_mpst_i_from_c, RoleC, 3 |
    recv_mpst_i_from_d, RoleD, 4 |
    recv_mpst_i_from_e, RoleE, 5 |
    recv_mpst_i_from_f, RoleF, 6 |
    recv_mpst_i_from_g, RoleG, 7 |
    recv_mpst_i_from_h, RoleH, 8 | =>
    NameI, MeshedChannelsNine, 9
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
// A
enum Branching0fromItoA {
    More(
        MeshedChannelsNine<
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
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoI = Recv<Branching0fromItoA, End>;
// B
enum Branching0fromItoB {
    More(
        MeshedChannelsNine<
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
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoI = Recv<Branching0fromItoB, End>;
// C
enum Branching0fromItoC {
    More(
        MeshedChannelsNine<
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
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoI = Recv<Branching0fromItoC, End>;
// D
enum Branching0fromItoD {
    More(
        MeshedChannelsNine<
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
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoI = Recv<Branching0fromItoD, End>;
// E
enum Branching0fromItoE {
    More(
        MeshedChannelsNine<
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
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoI = Recv<Branching0fromItoE, End>;
// F
enum Branching0fromItoF {
    More(
        MeshedChannelsNine<
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
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoI = Recv<Branching0fromItoF, End>;
// G
enum Branching0fromItoG {
    More(
        MeshedChannelsNine<
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
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoI = Recv<Branching0fromItoG, End>;
// H
enum Branching0fromItoH {
    More(
        MeshedChannelsNine<
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
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameH>),
}
type RecursHtoI = Recv<Branching0fromItoH, End>;
// I
type Choose0fromItoA = Send<Branching0fromItoA, End>;
type Choose0fromItoB = Send<Branching0fromItoB, End>;
type Choose0fromItoC = Send<Branching0fromItoC, End>;
type Choose0fromItoD = Send<Branching0fromItoD, End>;
type Choose0fromItoE = Send<Branching0fromItoE, End>;
type Choose0fromItoF = Send<Branching0fromItoF, End>;
type Choose0fromItoG = Send<Branching0fromItoG, End>;
type Choose0fromItoH = Send<Branching0fromItoH, End>;
type EndpointDoneI = MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameI>;
type EndpointMoreI = MeshedChannelsNine<
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
type EndpointA =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursAtoI, RoleI<RoleEnd>, NameA>;
type EndpointB =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursBtoI, RoleI<RoleEnd>, NameB>;
type EndpointC =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursCtoI, RoleI<RoleEnd>, NameC>;
type EndpointD =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursDtoI, RoleI<RoleEnd>, NameD>;
type EndpointE =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursEtoI, RoleI<RoleEnd>, NameE>;
type EndpointF =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursFtoI, RoleI<RoleEnd>, NameF>;
type EndpointG =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursGtoI, RoleI<RoleEnd>, NameG>;
type EndpointH =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursHtoI, RoleI<RoleEnd>, NameH>;
type EndpointI = MeshedChannelsNine<
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

create_fn_choose_mpst_multi_to_all_bundle!(
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
    NameA,
    NameB,
    NameC,
    NameD,
    NameE,
    NameF,
    NameG,
    NameH, =>
    NameI, MeshedChannelsNine, 9
);

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_i, {
        Branching0fromItoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoA::More(s) => {
            let (_, s) = recv_mpst_a_from_i(s)?;
            let s = send_mpst_a_to_i((), s);
            let (_, s) = recv_mpst_a_from_b(s)?;
            let s = send_mpst_a_to_b((), s);
            let (_, s) = recv_mpst_a_from_c(s)?;
            let s = send_mpst_a_to_c((), s);
            let (_, s) = recv_mpst_a_from_d(s)?;
            let s = send_mpst_a_to_d((), s);
            let (_, s) = recv_mpst_a_from_e(s)?;
            let s = send_mpst_a_to_e((), s);
            let (_, s) = recv_mpst_a_from_f(s)?;
            let s = send_mpst_a_to_f((), s);
            let (_, s) = recv_mpst_a_from_g(s)?;
            let s = send_mpst_a_to_g((), s);
            let (_, s) = recv_mpst_a_from_h(s)?;
            let s = send_mpst_a_to_h((), s);
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_i, {
        Branching0fromItoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoB::More(s) => {
            let (_, s) = recv_mpst_b_from_i(s)?;
            let s = send_mpst_b_to_i((), s);
            let s = send_mpst_b_to_a((), s);
            let (_, s) = recv_mpst_b_from_a(s)?;
            let (_, s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_c((), s);
            let (_, s) = recv_mpst_b_from_d(s)?;
            let s = send_mpst_b_to_d((), s);
            let (_, s) = recv_mpst_b_from_e(s)?;
            let s = send_mpst_b_to_e((), s);
            let (_, s) = recv_mpst_b_from_f(s)?;
            let s = send_mpst_b_to_f((), s);
            let (_, s) = recv_mpst_b_from_g(s)?;
            let s = send_mpst_b_to_g((), s);
            let (_, s) = recv_mpst_b_from_h(s)?;
            let s = send_mpst_b_to_h((), s);
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_i, {
        Branching0fromItoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoC::More(s) => {
            let (_, s) = recv_mpst_c_from_i(s)?;
            let s = send_mpst_c_to_i((), s);
            let s = send_mpst_c_to_a((), s);
            let (_, s) = recv_mpst_c_from_a(s)?;
            let s = send_mpst_c_to_b((), s);
            let (_, s) = recv_mpst_c_from_b(s)?;
            let (_, s) = recv_mpst_c_from_d(s)?;
            let s = send_mpst_c_to_d((), s);
            let (_, s) = recv_mpst_c_from_e(s)?;
            let s = send_mpst_c_to_e((), s);
            let (_, s) = recv_mpst_c_from_f(s)?;
            let s = send_mpst_c_to_f((), s);
            let (_, s) = recv_mpst_c_from_g(s)?;
            let s = send_mpst_c_to_g((), s);
            let (_, s) = recv_mpst_c_from_h(s)?;
            let s = send_mpst_c_to_h((), s);
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_from_i, {
        Branching0fromItoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoD::More(s) => {
            let (_, s) = recv_mpst_d_from_i(s)?;
            let s = send_mpst_d_to_i((), s);
            let s = send_mpst_d_to_a((), s);
            let (_, s) = recv_mpst_d_from_a(s)?;
            let s = send_mpst_d_to_b((), s);
            let (_, s) = recv_mpst_d_from_b(s)?;
            let s = send_mpst_d_to_c((), s);
            let (_, s) = recv_mpst_d_from_c(s)?;
            let (_, s) = recv_mpst_d_from_e(s)?;
            let s = send_mpst_d_to_e((), s);
            let (_, s) = recv_mpst_d_from_f(s)?;
            let s = send_mpst_d_to_f((), s);
            let (_, s) = recv_mpst_d_from_g(s)?;
            let s = send_mpst_d_to_g((), s);
            let (_, s) = recv_mpst_d_from_h(s)?;
            let s = send_mpst_d_to_h((), s);
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_e_from_i, {
        Branching0fromItoE::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoE::More(s) => {
            let (_, s) = recv_mpst_e_from_i(s)?;
            let s = send_mpst_e_to_i((), s);
            let s = send_mpst_e_to_a((), s);
            let (_, s) = recv_mpst_e_from_a(s)?;
            let s = send_mpst_e_to_b((), s);
            let (_, s) = recv_mpst_e_from_b(s)?;
            let s = send_mpst_e_to_c((), s);
            let (_, s) = recv_mpst_e_from_c(s)?;
            let s = send_mpst_e_to_d((), s);
            let (_, s) = recv_mpst_e_from_d(s)?;
            let (_, s) = recv_mpst_e_from_f(s)?;
            let s = send_mpst_e_to_f((), s);
            let (_, s) = recv_mpst_e_from_g(s)?;
            let s = send_mpst_e_to_g((), s);
            let (_, s) = recv_mpst_e_from_h(s)?;
            let s = send_mpst_e_to_h((), s);
            endpoint_e(s)
        },
    })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_f_from_i, {
        Branching0fromItoF::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoF::More(s) => {
            let (_, s) = recv_mpst_f_from_i(s)?;
            let s = send_mpst_f_to_i((), s);
            let s = send_mpst_f_to_a((), s);
            let (_, s) = recv_mpst_f_from_a(s)?;
            let s = send_mpst_f_to_b((), s);
            let (_, s) = recv_mpst_f_from_b(s)?;
            let s = send_mpst_f_to_c((), s);
            let (_, s) = recv_mpst_f_from_c(s)?;
            let s = send_mpst_f_to_d((), s);
            let (_, s) = recv_mpst_f_from_d(s)?;
            let s = send_mpst_f_to_e((), s);
            let (_, s) = recv_mpst_f_from_e(s)?;
            let (_, s) = recv_mpst_f_from_g(s)?;
            let s = send_mpst_f_to_g((), s);
            let (_, s) = recv_mpst_f_from_h(s)?;
            let s = send_mpst_f_to_h((), s);
            endpoint_f(s)
        },
    })
}

fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_g_from_i, {
        Branching0fromItoG::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoG::More(s) => {
            let (_, s) = recv_mpst_g_from_i(s)?;
            let s = send_mpst_g_to_i((), s);
            let s = send_mpst_g_to_a((), s);
            let (_, s) = recv_mpst_g_from_a(s)?;
            let s = send_mpst_g_to_b((), s);
            let (_, s) = recv_mpst_g_from_b(s)?;
            let s = send_mpst_g_to_c((), s);
            let (_, s) = recv_mpst_g_from_c(s)?;
            let s = send_mpst_g_to_d((), s);
            let (_, s) = recv_mpst_g_from_d(s)?;
            let s = send_mpst_g_to_e((), s);
            let (_, s) = recv_mpst_g_from_e(s)?;
            let s = send_mpst_g_to_f((), s);
            let (_, s) = recv_mpst_g_from_f(s)?;
            let (_, s) = recv_mpst_g_from_h(s)?;
            let s = send_mpst_g_to_h((), s);
            endpoint_g(s)
        },
    })
}

fn endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_h_from_i, {
        Branching0fromItoH::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromItoH::More(s) => {
            let (_, s) = recv_mpst_h_from_i(s)?;
            let s = send_mpst_h_to_i((), s);
            let s = send_mpst_h_to_a((), s);
            let (_, s) = recv_mpst_h_from_a(s)?;
            let s = send_mpst_h_to_b((), s);
            let (_, s) = recv_mpst_h_from_b(s)?;
            let s = send_mpst_h_to_c((), s);
            let (_, s) = recv_mpst_h_from_c(s)?;
            let s = send_mpst_h_to_d((), s);
            let (_, s) = recv_mpst_h_from_d(s)?;
            let s = send_mpst_h_to_e((), s);
            let (_, s) = recv_mpst_h_from_e(s)?;
            let s = send_mpst_h_to_f((), s);
            let (_, s) = recv_mpst_h_from_f(s)?;
            let s = send_mpst_h_to_g((), s);
            let (_, s) = recv_mpst_h_from_g(s)?;
            endpoint_h(s)
        },
    })
}

fn endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>> {
    recurs_i(s, 100)
}

fn recurs_i(s: EndpointI, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_i_to_all(s);

            close_mpst_multi(s)
        }
        i => {
            let s = more_from_i_to_all(s);

            let s = send_mpst_i_to_a((), s);
            let (_, s) = recv_mpst_i_from_a(s)?;
            let s = send_mpst_i_to_b((), s);
            let (_, s) = recv_mpst_i_from_b(s)?;
            let s = send_mpst_i_to_c((), s);
            let (_, s) = recv_mpst_i_from_c(s)?;
            let s = send_mpst_i_to_d((), s);
            let (_, s) = recv_mpst_i_from_d(s)?;
            let s = send_mpst_i_to_e((), s);
            let (_, s) = recv_mpst_i_from_e(s)?;
            let s = send_mpst_i_to_f((), s);
            let (_, s) = recv_mpst_i_from_f(s)?;
            let s = send_mpst_i_to_g((), s);
            let (_, s) = recv_mpst_i_from_g(s)?;
            let s = send_mpst_i_to_h((), s);
            let (_, s) = recv_mpst_i_from_h(s)?;

            recurs_i(s, i - 1)
        }
    }
}

fn main() {
    let (thread_a, thread_b, thread_c, thread_d, thread_e, thread_f, thread_g, thread_h, thread_i) =
        fork_mpst(
            endpoint_a, endpoint_b, endpoint_c, endpoint_d, endpoint_e, endpoint_f, endpoint_g,
            endpoint_h, endpoint_i,
        );

    assert!(thread_a.join().is_ok());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_ok());
    assert!(thread_d.join().is_ok());
    assert!(thread_e.join().is_ok());
    assert!(thread_f.join().is_ok());
    assert!(thread_g.join().is_ok());
    assert!(thread_h.join().is_ok());
    assert!(thread_i.join().is_ok());
}
