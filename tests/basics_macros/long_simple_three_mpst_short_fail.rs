use mpstthree::binary::struct_trait::{End, Recv, Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, create_fn_choose_mpst_multi_to_all_bundle,
    create_multiple_normal_role_short, create_recv_mpst_session_bundle, offer_mpst, recv_mpst,
    send_mpst,
};

use std::error::Error;

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsThree, 3);

// Create new roles
create_multiple_normal_role_short!(A, B, C);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;

// Types
// Send/Recv
type RS = Recv<(), Send<(), End>>;
type SR = Send<(), Recv<(), End>>;
// Roles
type R2A<R> = RoleA<RoleA<R>>;
type R2B<R> = RoleB<RoleB<R>>;
type R2C<R> = RoleC<RoleC<R>>;
// A
enum Branching0fromCtoA {
    More(MeshedChannelsThree<RS, Recv<(), Send<(), RecursAtoC>>, R2C<R2B<RoleC<RoleEnd>>>, NameA>),
    Done(MeshedChannelsThree<End, End, RoleEnd, NameA>),
}
type RecursAtoC = Recv<Branching0fromCtoA, End>;
// B
enum Branching0fromCtoB {
    More(MeshedChannelsThree<SR, Recv<(), Send<(), RecursBtoC>>, R2C<R2A<RoleC<RoleEnd>>>, NameB>),
    Done(MeshedChannelsThree<End, End, RoleEnd, NameB>),
}
type RecursBtoC = Recv<Branching0fromCtoB, End>;
// C
type Choose0fromCtoA = Send<Branching0fromCtoA, End>;
type Choose0fromCtoB = Send<Branching0fromCtoB, End>;
type EndpointDoneC = MeshedChannelsThree<End, End, RoleEnd, NameC>;
type EndpointMoreC = MeshedChannelsThree<
    Send<(), Recv<(), Choose0fromCtoA>>,
    Send<(), Recv<(), Choose0fromCtoB>>,
    R2A<R2B<RoleBroadcast>>,
    NameC,
>;

// Creating the MP sessions
type EndpointA = MeshedChannelsThree<End, RecursAtoC, RoleC<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsThree<End, RecursBtoC, RoleC<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsThree<Choose0fromCtoA, Choose0fromCtoB, RoleBroadcast, NameC>;

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_c, RoleC, 2 | =>
    RoleA, MeshedChannelsThree, 3
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_c, RoleC, 2 | =>
    RoleB, MeshedChannelsThree, 3
);

create_fn_choose_mpst_multi_to_all_bundle!(
    done_from_c_to_all, more_from_c_to_all, =>
    Done, More, =>
    EndpointDoneC, EndpointMoreC, =>
    Branching0fromCtoA, Branching0fromCtoB, =>
    RoleA, RoleB, =>
    RoleC, MeshedChannelsThree, 3
);

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_c, {
        Branching0fromCtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromCtoA::More(s) => {
            let (_, s) = recv_mpst!(s, RoleC, RoleB, MeshedChannelsThree, 3, 2)()?;
            let s = send_mpst!(s, (), RoleC, RoleB, MeshedChannelsThree, 3, 2);
            let (_, s) = recv_mpst!(s, RoleB, RoleA, MeshedChannelsThree, 3, 1)()?;
            let s = send_mpst!(s, (), RoleB, RoleA, MeshedChannelsThree, 3, 1);
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_c, {
        Branching0fromCtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromCtoB::More(s) => {
            let (_, s) = recv_mpst!(s, RoleC, RoleB, MeshedChannelsThree, 3, 2)()?;
            let s = send_mpst!(s, (), RoleC, RoleB, MeshedChannelsThree, 3, 2);
            let s = send_mpst!(s, (), RoleA, RoleB, MeshedChannelsThree, 3, 1);
            let (_, s) = recv_mpst!(s, RoleA, RoleB, MeshedChannelsThree, 3, 1)()?;
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    recurs_c(s, SIZE)
}

fn recurs_c(s: EndpointC, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_c_to_all(s);

            close_mpst_multi(s)
        }
        i => {
            let s = more_from_c_to_all(s);

            let s = send_mpst!(s, (), RoleA, RoleC, MeshedChannelsThree, 3, 1);
            let (_, s) = recv_mpst!(s, RoleA, RoleC, MeshedChannelsThree, 3, 1)()?;
            let s = send_mpst!(s, (), RoleB, RoleC, MeshedChannelsThree, 3, 2);
            let (_, s) = recv_mpst!(s, RoleB, RoleC, MeshedChannelsThree, 3, 2)()?;

            recurs_c(s, i - 1)
        }
    }
}

pub fn main() {
    let (thread_a, thread_b, thread_c) = fork_mpst(
        endpoint_a,
        endpoint_b,
        endpoint_c,
    );

    assert!(thread_a.join().is_err());
    assert!(thread_b.join().is_err());
    assert!(thread_c.join().is_err());
}

/////////////////////////

static SIZE: i64 = 15;
