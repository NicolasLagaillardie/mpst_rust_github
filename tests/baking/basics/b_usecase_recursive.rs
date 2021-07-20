use rand::{thread_rng, Rng};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::broadcast::RoleBroadcast;

use std::boxed::Box;
use std::error::Error;
use std::marker;

// Get roles
use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

use mpstthree::choose_mpst_a_to_all;
use mpstthree::offer_mpst_b_to_a;
use mpstthree::offer_mpst_c_to_a;

/// Test our usecase
/// Simple types
/// Client = B → Y → A
/// Authenticator = C → Z → B
/// Server = A → X → C

type BtoAClose = End;
type BtoCClose = End;
type BtoCVideo<N> = Send<N, Recv<N, End>>;
type BtoAVideo<N> = Recv<N, Send<N, RecursBtoA<N>>>;

type InitB<N> = Recv<N, Send<N, RecursBtoA<N>>>;

type CtoBClose = <BtoCClose as Session>::Dual;
type CtoAClose = End;
type CtoBVideo<N> = <BtoCVideo<N> as Session>::Dual;

type RecursBtoA<N> = Recv<Branches0BtoA<N>, End>;
type RecursCtoA<N> = Recv<Branches0CtoA<N>, End>;

enum Branches0BtoA<N: marker::Send> {
    End(MeshedChannels<BtoAClose, BtoCClose, StackBEnd, RoleB<RoleEnd>>),
    Video(MeshedChannels<BtoAVideo<N>, BtoCVideo<N>, StackBVideo, RoleB<RoleEnd>>),
}
enum Branches0CtoA<N: marker::Send> {
    End(MeshedChannels<CtoAClose, CtoBClose, StackCEnd, RoleC<RoleEnd>>),
    Video(MeshedChannels<RecursCtoA<N>, CtoBVideo<N>, StackCVideo, RoleC<RoleEnd>>),
}
type Choose0fromAtoB<N> = Send<Branches0BtoA<N>, End>;
type Choose0fromAtoC<N> = Send<Branches0CtoA<N>, End>;

type InitA<N> = Send<N, Recv<N, Choose0fromAtoB<N>>>;

/// Stacks
type StackBEnd = RoleEnd;
type StackBVideo = RoleA<RoleC<RoleC<RoleA<RoleA<RoleEnd>>>>>;
type StackBRecurs = RoleA<RoleEnd>;
type StackBInit = RoleA<RoleA<RoleA<RoleEnd>>>;

type StackCEnd = RoleEnd;
type StackCVideo = RoleB<RoleB<RoleA<RoleEnd>>>;
type StackCRecurs = RoleA<RoleEnd>;

type StackARecurs = RoleBroadcast;
type StackAFull = RoleB<RoleB<StackARecurs>>;

/// Creating the MP sessions

/// For B
type EndpointARecurs<N> =
    MeshedChannels<Choose0fromAtoB<N>, Choose0fromAtoC<N>, StackARecurs, RoleA<RoleEnd>>;
type EndpointAFull<N> = MeshedChannels<InitA<N>, Choose0fromAtoC<N>, StackAFull, RoleA<RoleEnd>>;

/// For C
type EndpointBRecurs<N> = MeshedChannels<RecursBtoA<N>, End, StackBRecurs, RoleB<RoleEnd>>;
type EndpointBFull<N> = MeshedChannels<InitB<N>, End, StackBInit, RoleB<RoleEnd>>;

/// For A
type EndpointCRecurs<N> = MeshedChannels<RecursCtoA<N>, End, StackCRecurs, RoleC<RoleEnd>>;

/// Functions related to endpoints
fn server(s: EndpointCRecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_c_to_a!(s, {
        Branches0CtoA::End(s) => {
            s.close()
        },
        Branches0CtoA::Video(s) => {
            let (request, s) = s.recv()?;
            let s = s.send(request + 1);
            server(s)
        },
    })
}

fn authenticator(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = s.recv()?;
    let s = s.send(id + 1);

    authenticator_recurs(s)
}

fn authenticator_recurs(s: EndpointBRecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_b_to_a!(s, {
        Branches0BtoA::End(s) => {
            s.close()
        },
        Branches0BtoA::Video(s) => {
            let (request, s) = s.recv()?;
            let (video, s) = s.send(request + 1).recv()?;
            let s = s.send(video + 1);
            authenticator_recurs(s)
        },
    })
}

fn client(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let xs: Vec<i32> = (1..100).map(|_| thread_rng().gen()).collect();

    let (_, s) = s.send(0).recv()?;

    client_recurs(s, xs, 1)
}

fn client_recurs(
    s: EndpointARecurs<i32>,
    mut xs: Vec<i32>,
    index: i32,
) -> Result<(), Box<dyn Error>> {
    match xs.pop() {
        Option::Some(_) => {
            let s: EndpointAFull<i32> =
                choose_mpst_a_to_all!(s, Branches0BtoA::Video, Branches0CtoA::Video);

            let (_, s) = s.send(1).recv()?;

            client_recurs(s, xs, index + 1)
        }
        Option::None => {
            let s = choose_mpst_a_to_all!(s, Branches0BtoA::End, Branches0CtoA::End);

            assert_eq!(index, 100);

            s.close()
        }
    }
}

/////////////////////////////////////////

pub fn run_b_usecase_recursive() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(client, authenticator, server);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}
