use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{baker, create_fn_choose_mpst_multi_to_all_bundle};

use std::error::Error;

// Create new roles
baker!("basic", MeshedChannelsThree, A, B, C);

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
    RoleA<RoleA<RoleA<RoleB<RoleBroadcast>>>>,
    NameC,
>;

// Creating the MP sessions
type EndpointA = MeshedChannelsThree<End, RecursAtoC, RoleC<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsThree<End, RecursBtoC, RoleC<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsThree<Choose0fromCtoA, Choose0fromCtoB, RoleBroadcast, NameC>;

create_fn_choose_mpst_multi_to_all_bundle!(
    done_from_c_to_all, more_from_c_to_all, =>
    Done, More, =>
    EndpointDoneC, EndpointMoreC, =>
    Branching0fromCtoA, Branching0fromCtoB, =>
    NameA, NameB, =>
    NameC, MeshedChannelsThree, 3
);

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(
        s,
        {
            Branching0fromCtoA::Done(s) => {
                s.close()
            },
            Branching0fromCtoA::More(s) => {
                let (_, s) = s.recv();
                let (_, s) = s.send(()).recv();
                let s = s.send(());
                endpoint_a(s)
            },
        }
    )
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(
        s,
        {
            Branching0fromCtoB::Done(s) => {
                s.close()
            },
            Branching0fromCtoB::More(s) => {
                let (_, s) = s.recv();
                let (_, s) = s.send(()).send(()).recv();
                endpoint_b(s)
            },
        }
    )
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    recurs_c(s, LOOPS)
}

fn recurs_c(s: EndpointC, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => done_from_c_to_all(s).close(),
        i => {
            let (_, s) = more_from_c_to_all(s).send(()).recv();
            let (_, s) = s.send(()).recv();

            recurs_c(s, i - 1)
        }
    }
}

/////////////////////////////////////////

pub fn main() {
    let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);

    assert!(thread_a.join().is_err());
    assert!(thread_b.join().is_err());
    assert!(thread_c.join().is_err());
}

/////////////////////////

static LOOPS: i64 = 15;
