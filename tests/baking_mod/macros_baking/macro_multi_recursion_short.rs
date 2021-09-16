// Test for parametrisation on the number of roles
use rand::{thread_rng, Rng};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{bundle_impl_with_enum, offer_mpst};
use std::error::Error;
use std::marker;

// Create new roles
bundle_impl_with_enum!(MeshedChannels => A, B, D => fork_mpst);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameD = RoleD<RoleEnd>;

/// Test our usecase
/// Simple types
/// Client = D
/// Authenticator = A
/// Server = B

type AtoDClose = End;
type AtoBClose = End;
type AtoBVideo<N> = Send<N, Recv<N, End>>;
type AtoDVideo<N> = Recv<N, Send<N, RecursAtoD<N>>>;

type InitA<N> = Recv<N, Send<N, RecursAtoD<N>>>;

type BtoAClose = <AtoBClose as Session>::Dual;
type BtoDClose = End;
type BtoAVideo<N> = <AtoBVideo<N> as Session>::Dual;

type RecursAtoD<N> = <Choose0fromCtoA<N> as Session>::Dual;
type RecursBtoD<N> = <Choose0fromCtoB<N> as Session>::Dual;

enum Branches0AtoD<N: marker::Send> {
    End(MeshedChannels<AtoBClose, AtoDClose, StackAEnd, NameA>),
    Video(MeshedChannels<AtoBVideo<N>, AtoDVideo<N>, StackAVideo, NameA>),
}
enum Branches0BtoD<N: marker::Send> {
    End(MeshedChannels<BtoAClose, BtoDClose, StackBEnd, NameB>),
    Video(MeshedChannels<BtoAVideo<N>, RecursBtoD<N>, StackBVideo, NameB>),
}
type Choose0fromCtoA<N> = Send<Branches0AtoD<N>, End>;
type Choose0fromCtoB<N> = Send<Branches0BtoD<N>, End>;

type InitD<N> = Send<N, Recv<N, Choose0fromCtoA<N>>>;

// Stacks
type StackAEnd = RoleEnd;
type StackAVideo = RoleD<RoleB<RoleB<RoleD<RoleD<RoleEnd>>>>>;
type StackARecurs = RoleD<RoleEnd>;
type StackAInit = RoleD<RoleD<RoleD<RoleEnd>>>;

type StackBEnd = RoleEnd;
type StackBVideo = RoleA<RoleA<RoleD<RoleEnd>>>;
type StackBRecurs = RoleD<RoleEnd>;

type StackDRecurs = RoleBroadcast;
type StackDFull = RoleA<RoleA<StackDRecurs>>;

// Creating the MP sessions
// For D
type EndpointDVideo<N> = MeshedChannels<
    <AtoDVideo<N> as Session>::Dual,
    <RecursBtoD<N> as Session>::Dual,
    RoleA<RoleA<RoleBroadcast>>,
    NameD,
>;
type EndpointDRecurs<N> =
    MeshedChannels<Choose0fromCtoA<N>, Choose0fromCtoB<N>, StackDRecurs, NameD>;
type EndpointDFull<N> = MeshedChannels<InitD<N>, Choose0fromCtoB<N>, StackDFull, NameD>;

// For A
type EndpointARecurs<N> = MeshedChannels<End, RecursAtoD<N>, StackARecurs, NameA>;
type EndpointAFull<N> = MeshedChannels<End, InitA<N>, StackAInit, NameA>;

// For B
type EndpointBRecurs<N> = MeshedChannels<End, RecursBtoD<N>, StackBRecurs, NameB>;

// Functions related to endpoints
fn server(s: EndpointBRecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branches0BtoD::End(s) => {
            s.close()
        },
        Branches0BtoD::Video(s) => {
            let (request, s) = s.recv()?;
            let s = s.send(request + 1);
            server(s)
        },
    })
}

fn authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = s.recv()?;
    let s = s.send(id + 1);

    authenticator_recurs(s)
}

fn authenticator_recurs(s: EndpointARecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branches0AtoD::End(s) => {
            s.close()
        },
        Branches0AtoD::Video(s) => {
            let (request, s) = s.recv()?;
            let (video, s) = s.send(request + 1).recv()?;
            let s = s.send(video + 1);
            authenticator_recurs(s)
        },
    })
}

fn client(s: EndpointDFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();

    let (_, s) = s.send(0).recv()?;

    client_recurs(s, xs, 1)
}

fn client_recurs(
    s: EndpointDRecurs<i32>,
    mut xs: Vec<i32>,
    index: i32,
) -> Result<(), Box<dyn Error>> {
    match xs.pop() {
        Option::Some(_) => {
            let s: EndpointDVideo<i32> =
                choose_mpst_d_to_all!(s, Branches0AtoD::Video, Branches0BtoD::Video);

            let (_, s) = s.send(1).recv()?;

            client_recurs(s, xs, index + 1)
        }
        Option::None => {
            let s = choose_mpst_d_to_all!(s, Branches0AtoD::End, Branches0BtoD::End);

            assert_eq!(index, 100);

            s.close()
        }
    }
}

////////////////////////////////////////

pub fn new_run_usecase_recursive() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}
