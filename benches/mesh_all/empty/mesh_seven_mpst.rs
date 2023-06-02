use crossbeam_channel::bounded;

use criterion::{black_box, Criterion};

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose, create_fn_choose_mpst_multi_to_all_bundle,
    create_multiple_normal_name_short, create_multiple_normal_role_short,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, offer, offer_mpst,
};

use std::error::Error;
use std::thread::{spawn, JoinHandle};

// use std::time::Duration;

// Create the new MeshedChannels for six participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsSeven, 7);

// Create new roles
// normal
create_multiple_normal_role_short!(A, B, C, D, E, F, G);

// Create new names
create_multiple_normal_name_short!(A, B, C, D, E, F, G);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, 1 |
    send_mpst_a_to_c, RoleC, 2 |
    send_mpst_a_to_d, RoleD, 3 |
    send_mpst_a_to_e, RoleE, 4 |
    send_mpst_a_to_f, RoleF, 5 |
    send_mpst_a_to_g, RoleG, 6 | =>
    NameA, MeshedChannelsSeven, 7
);

// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a, RoleA, 1 |
    send_mpst_b_to_c, RoleC, 2 |
    send_mpst_b_to_d, RoleD, 3 |
    send_mpst_b_to_e, RoleE, 4 |
    send_mpst_b_to_f, RoleF, 5 |
    send_mpst_b_to_g, RoleG, 6 | =>
    NameB, MeshedChannelsSeven, 7
);

// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a, RoleA, 1 |
    send_mpst_c_to_b, RoleB, 2 |
    send_mpst_c_to_d, RoleD, 3 |
    send_mpst_c_to_e, RoleE, 4 |
    send_mpst_c_to_f, RoleF, 5 |
    send_mpst_c_to_g, RoleG, 6 | =>
    NameC, MeshedChannelsSeven, 7
);

// D
create_send_mpst_session_bundle!(
    send_mpst_d_to_a, RoleA, 1 |
    send_mpst_d_to_b, RoleB, 2 |
    send_mpst_d_to_c, RoleC, 3 |
    send_mpst_d_to_e, RoleE, 4 |
    send_mpst_d_to_f, RoleF, 5 |
    send_mpst_d_to_g, RoleG, 6 | =>
    NameD, MeshedChannelsSeven, 7
);

// E
create_send_mpst_session_bundle!(
    send_mpst_e_to_a, RoleA, 1 |
    send_mpst_e_to_b, RoleB, 2 |
    send_mpst_e_to_c, RoleC, 3 |
    send_mpst_e_to_d, RoleD, 4 |
    send_mpst_e_to_f, RoleF, 5 |
    send_mpst_e_to_g, RoleG, 6 | =>
    NameE, MeshedChannelsSeven, 7
);

// F
create_send_mpst_session_bundle!(
    send_mpst_f_to_a, RoleA, 1 |
    send_mpst_f_to_b, RoleB, 2 |
    send_mpst_f_to_c, RoleC, 3 |
    send_mpst_f_to_d, RoleD, 4 |
    send_mpst_f_to_e, RoleE, 5 |
    send_mpst_f_to_g, RoleG, 6 | =>
    NameF, MeshedChannelsSeven, 7
);

// G
create_send_mpst_session_bundle!(
    send_mpst_g_to_a, RoleA, 1 |
    send_mpst_g_to_b, RoleB, 2 |
    send_mpst_g_to_c, RoleC, 3 |
    send_mpst_g_to_d, RoleD, 4 |
    send_mpst_g_to_e, RoleE, 5 |
    send_mpst_g_to_f, RoleF, 6 | =>
    NameG, MeshedChannelsSeven, 7
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 1 |
    recv_mpst_a_from_c, RoleC, 2 |
    recv_mpst_a_from_d, RoleD, 3 |
    recv_mpst_a_from_e, RoleE, 4 |
    recv_mpst_a_from_f, RoleF, 5 |
    recv_mpst_a_from_g, RoleG, 6 | =>
    NameA, MeshedChannelsSeven, 7
);

// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 1 |
    recv_mpst_b_from_c, RoleC, 2 |
    recv_mpst_b_from_d, RoleD, 3 |
    recv_mpst_b_from_e, RoleE, 4 |
    recv_mpst_b_from_f, RoleF, 5 |
    recv_mpst_b_from_g, RoleG, 6 | =>
    NameB, MeshedChannelsSeven, 7
);

// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a, RoleA, 1 |
    recv_mpst_c_from_b, RoleB, 2 |
    recv_mpst_c_from_d, RoleD, 3 |
    recv_mpst_c_from_e, RoleE, 4 |
    recv_mpst_c_from_f, RoleF, 5 |
    recv_mpst_c_from_g, RoleG, 6 | =>
    NameC, MeshedChannelsSeven, 7
);

// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_a, RoleA, 1 |
    recv_mpst_d_from_b, RoleB, 2 |
    recv_mpst_d_from_c, RoleC, 3 |
    recv_mpst_d_from_e, RoleE, 4 |
    recv_mpst_d_from_f, RoleF, 5 |
    recv_mpst_d_from_g, RoleG, 6 | =>
    NameD, MeshedChannelsSeven, 7
);

// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_a, RoleA, 1 |
    recv_mpst_e_from_b, RoleB, 2 |
    recv_mpst_e_from_c, RoleC, 3 |
    recv_mpst_e_from_d, RoleD, 4 |
    recv_mpst_e_from_f, RoleF, 5 |
    recv_mpst_e_from_g, RoleG, 6 | =>
    NameE, MeshedChannelsSeven, 7
);

// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_from_a, RoleA, 1 |
    recv_mpst_f_from_b, RoleB, 2 |
    recv_mpst_f_from_c, RoleC, 3 |
    recv_mpst_f_from_d, RoleD, 4 |
    recv_mpst_f_from_e, RoleE, 5 |
    recv_mpst_f_from_g, RoleG, 6 | =>
    NameF, MeshedChannelsSeven, 7
);

// G
create_recv_mpst_session_bundle!(
    recv_mpst_g_from_a, RoleA, 1 |
    recv_mpst_g_from_b, RoleB, 2 |
    recv_mpst_g_from_c, RoleC, 3 |
    recv_mpst_g_from_d, RoleD, 4 |
    recv_mpst_g_from_e, RoleE, 5 |
    recv_mpst_g_from_f, RoleF, 6 | =>
    NameG, MeshedChannelsSeven, 7
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

// A
enum Branching0fromGtoA {
    More(
        MeshedChannelsSeven<
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
    Done(MeshedChannelsSeven<End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoG = Recv<Branching0fromGtoA, End>;

// B
enum Branching0fromGtoB {
    More(
        MeshedChannelsSeven<
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
    Done(MeshedChannelsSeven<End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoG = Recv<Branching0fromGtoB, End>;

// C
enum Branching0fromGtoC {
    More(
        MeshedChannelsSeven<
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
    Done(MeshedChannelsSeven<End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoG = Recv<Branching0fromGtoC, End>;

// D
enum Branching0fromGtoD {
    More(
        MeshedChannelsSeven<
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
    Done(MeshedChannelsSeven<End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoG = Recv<Branching0fromGtoD, End>;

// E
enum Branching0fromGtoE {
    More(
        MeshedChannelsSeven<
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
    Done(MeshedChannelsSeven<End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoG = Recv<Branching0fromGtoE, End>;

// F
enum Branching0fromGtoF {
    More(
        MeshedChannelsSeven<
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
    Done(MeshedChannelsSeven<End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoG = Recv<Branching0fromGtoF, End>;

// F
type Choose0fromGtoA = Send<Branching0fromGtoA, End>;
type Choose0fromGtoB = Send<Branching0fromGtoB, End>;
type Choose0fromGtoC = Send<Branching0fromGtoC, End>;
type Choose0fromGtoD = Send<Branching0fromGtoD, End>;
type Choose0fromGtoE = Send<Branching0fromGtoE, End>;
type Choose0fromGtoF = Send<Branching0fromGtoF, End>;
type EndpointDoneG = MeshedChannelsSeven<End, End, End, End, End, End, RoleEnd, NameG>;
type EndpointMoreG = MeshedChannelsSeven<
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
type EndpointA = MeshedChannelsSeven<End, End, End, End, End, RecursAtoG, RoleG<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsSeven<End, End, End, End, End, RecursBtoG, RoleG<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsSeven<End, End, End, End, End, RecursCtoG, RoleG<RoleEnd>, NameC>;
type EndpointD = MeshedChannelsSeven<End, End, End, End, End, RecursDtoG, RoleG<RoleEnd>, NameD>;
type EndpointE = MeshedChannelsSeven<End, End, End, End, End, RecursEtoG, RoleG<RoleEnd>, NameE>;
type EndpointF = MeshedChannelsSeven<End, End, End, End, End, RecursFtoG, RoleG<RoleEnd>, NameF>;
type EndpointG = MeshedChannelsSeven<
    Choose0fromGtoA,
    Choose0fromGtoB,
    Choose0fromGtoC,
    Choose0fromGtoD,
    Choose0fromGtoE,
    Choose0fromGtoF,
    RoleBroadcast,
    NameG,
>;

create_fn_choose_mpst_multi_to_all_bundle!(
    done_from_g_to_all, more_from_g_to_all, =>
    Done, More, =>
    EndpointDoneG, EndpointMoreG, =>
    Branching0fromGtoA,
    Branching0fromGtoB,
    Branching0fromGtoC,
    Branching0fromGtoD,
    Branching0fromGtoE,
    Branching0fromGtoF, =>
    NameA,
    NameB,
    NameC,
    NameD,
    NameE,
    NameF, =>
    NameG, MeshedChannelsSeven, 7
);

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_g, {
        Branching0fromGtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromGtoA::More(s) => {
            let (_, s) = recv_mpst_a_from_g(s)?;
            let s = send_mpst_a_to_g((), s);
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
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_g, {
        Branching0fromGtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromGtoB::More(s) => {
            let (_, s) = recv_mpst_b_from_g(s)?;
            let s = send_mpst_b_to_g((), s);
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
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_g, {
        Branching0fromGtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromGtoC::More(s) => {
            let (_, s) = recv_mpst_c_from_g(s)?;
            let s = send_mpst_c_to_g((), s);
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
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_from_g, {
        Branching0fromGtoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromGtoD::More(s) => {
            let (_, s) = recv_mpst_d_from_g(s)?;
            let s = send_mpst_d_to_g((), s);
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
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_e_from_g, {
        Branching0fromGtoE::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromGtoE::More(s) => {
            let (_, s) = recv_mpst_e_from_g(s)?;
            let s = send_mpst_e_to_g((), s);
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
            endpoint_e(s)
        },
    })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_f_from_g, {
        Branching0fromGtoF::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromGtoF::More(s) => {
            let (_, s) = recv_mpst_f_from_g(s)?;
            let s = send_mpst_f_to_g((), s);
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
            endpoint_f(s)
        },
    })
}

fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    recurs_g(s, LOOPS)
}

fn recurs_g(s: EndpointG, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_g_to_all(s);

            close_mpst_multi(s)
        }
        i => {
            let s = more_from_g_to_all(s);

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

            recurs_g(s, i - 1)
        }
    }
}

fn all_mpst() {
    let (thread_a, thread_b, thread_c, thread_d, thread_e, thread_f, thread_g) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
        black_box(endpoint_d),
        black_box(endpoint_e),
        black_box(endpoint_f),
        black_box(endpoint_g),
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
    thread_f.join().unwrap();
    thread_g.join().unwrap();
}


/////////////////////////

static LOOPS: i64 = 0;

pub fn mesh_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("mesh seven empty protocol MPST {LOOPS}"), |b| {
        b.iter(all_mpst)
    });
}
