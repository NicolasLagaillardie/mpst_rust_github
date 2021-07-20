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

use mpstthree::choose_mpst_b_to_all;
use mpstthree::offer_mpst_a_to_b;
use mpstthree::offer_mpst_c_to_b;

/// Test our usecase
/// Simple types
/// Client = B
/// Authenticator = C
/// Server = A

type CtoBClose = End;
type CtoAClose = End;
type CtoAVideo<N> = Send<N, Recv<N, End>>;
type CtoBVideo<N> = Recv<N, Send<N, RecursCtoB<N>>>;

type InitC<N> = Recv<N, Send<N, RecursCtoB<N>>>;

type AtoCClose = <CtoAClose as Session>::Dual;
type AtoBClose = End;
type AtoCVideo<N> = <CtoAVideo<N> as Session>::Dual;

type RecursCtoB<N> = Recv<Branches0CtoB<N>, End>;
type RecursAtoB<N> = Recv<Branches0AtoB<N>, End>;

enum Branches0CtoB<N: marker::Send> {
    End(MeshedChannels<CtoAClose, CtoBClose, StackCEnd, RoleC<RoleEnd>>),
    Video(MeshedChannels<CtoAVideo<N>, CtoBVideo<N>, StackCVideo, RoleC<RoleEnd>>),
}
enum Branches0AtoB<N: marker::Send> {
    End(MeshedChannels<AtoBClose, AtoCClose, StackAEnd, RoleA<RoleEnd>>),
    Video(MeshedChannels<RecursAtoB<N>, AtoCVideo<N>, StackAVideo, RoleA<RoleEnd>>),
}
type Choose0fromBtoC<N> = Send<Branches0CtoB<N>, End>;
type Choose0fromBtoA<N> = Send<Branches0AtoB<N>, End>;

type InitB<N> = Send<N, Recv<N, Choose0fromBtoC<N>>>;

/// Stacks
type StackCEnd = RoleEnd;
type StackCVideo = RoleB<RoleA<RoleA<RoleB<RoleB<RoleEnd>>>>>;
type StackCRecurs = RoleB<RoleEnd>;
type StackCInit = RoleB<RoleB<RoleB<RoleEnd>>>;

type StackAEnd = RoleEnd;
type StackAVideo = RoleC<RoleC<RoleB<RoleEnd>>>;
type StackARecurs = RoleB<RoleEnd>;

type StackBRecurs = RoleBroadcast;
type StackBFull = RoleC<RoleC<StackBRecurs>>;

/// Creating the MP sessions

/// For B
type EndpointBRecurs<N> =
    MeshedChannels<Choose0fromBtoA<N>, Choose0fromBtoC<N>, StackBRecurs, RoleB<RoleEnd>>;
type EndpointBFull<N> = MeshedChannels<Choose0fromBtoA<N>, InitB<N>, StackBFull, RoleB<RoleEnd>>;

/// For C
type EndpointCRecurs<N> = MeshedChannels<End, RecursCtoB<N>, StackCRecurs, RoleC<RoleEnd>>;
type EndpointCFull<N> = MeshedChannels<End, InitC<N>, StackCInit, RoleC<RoleEnd>>;

/// For A
type EndpointARecurs<N> = MeshedChannels<RecursAtoB<N>, End, StackARecurs, RoleA<RoleEnd>>;

/// Functions related to endpoints
fn server(s: EndpointARecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_a_to_b!(s, {
        Branches0AtoB::End(s) => {
            s.close()
        },
        Branches0AtoB::Video(s) => {
            let (request, s) = s.recv()?;
            let s = s.send(request + 1);
            server(s)
        },
    })
}

fn authenticator(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = s.recv()?;
    let s = s.send(id + 1);

    authenticator_recurs(s)
}

fn authenticator_recurs(s: EndpointCRecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_c_to_b!(s, {
        Branches0CtoB::End(s) => {
            s.close()
        },
        Branches0CtoB::Video(s) => {
            let (request, s) = s.recv()?;
            let (video, s) = s.send(request + 1).recv()?;
            let s = s.send(video + 1);
            authenticator_recurs(s)
        },
    })
}

fn client(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    let xs: Vec<i32> = (1..100).map(|_| thread_rng().gen()).collect();

    let (_, s) = s.send(0).recv()?;

    client_recurs(s, xs, 1)
}

fn client_recurs(
    s: EndpointBRecurs<i32>,
    mut xs: Vec<i32>,
    index: i32,
) -> Result<(), Box<dyn Error>> {
    match xs.pop() {
        Option::Some(_) => {
            let s: EndpointBFull<i32> =
                choose_mpst_b_to_all!(s, Branches0AtoB::Video, Branches0CtoB::Video);

            let (_, s) = s.send(1).recv()?;

            client_recurs(s, xs, index + 1)
        }
        Option::None => {
            let s = choose_mpst_b_to_all!(s, Branches0AtoB::End, Branches0CtoB::End);

            assert_eq!(index, 100);

            s.close()
        }
    }
}

/////////////////////////////////////////

pub fn run_a_usecase_recursive() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(server, client, authenticator);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}
