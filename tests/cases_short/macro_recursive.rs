// Test for Macro, exact same as usecase-recursive
use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use std::error::Error;
use std::marker;

use rand::{thread_rng, Rng};

use mpstthree::{bundle_impl, choose_mpst_multi_to_all, offer_mpst};

// Create new roles
bundle_impl!(SessionMpst => A, B, C => fork_mpst);

// Types
type AtoBVideo<N> = Send<N, Recv<N, End>>;
type AtoCVideo<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type InitA<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type BtoAVideo<N> = <AtoBVideo<N> as Session>::Dual;

type RecursAtoC<N> = Recv<Branches0AtoC<N>, End>;
type RecursBtoC<N> = Recv<Branches0BtoC<N>, End>;

enum Branches0AtoC<N: marker::Send> {
    End(SessionMpst<End, End, RoleEnd, RoleA<RoleEnd>>),
    Video(SessionMpst<AtoBVideo<N>, AtoCVideo<N>, StackAVideo, RoleA<RoleEnd>>),
}
enum Branches0BtoC<N: marker::Send> {
    End(SessionMpst<End, End, RoleEnd, RoleB<RoleEnd>>),
    Video(SessionMpst<BtoAVideo<N>, RecursBtoC<N>, StackBVideo, RoleB<RoleEnd>>),
}
type Choose0fromCtoA<N> = Send<Branches0AtoC<N>, End>;
type Choose0fromCtoB<N> = Send<Branches0BtoC<N>, End>;

type InitC<N> = Send<N, Recv<N, Choose0fromCtoA<N>>>;

/// Stacks
type StackAVideo = RoleC<RoleB<RoleB<RoleC<RoleC<RoleEnd>>>>>;
type StackAInit = RoleC<RoleC<RoleC<RoleEnd>>>;

type StackBVideo = RoleA<RoleA<RoleC<RoleEnd>>>;

type StackCRecurs = RoleBroadcast;
type StackCFull = RoleA<RoleA<StackCRecurs>>;

/// Creating the MP sessions
/// For C
type EndpointCVideo<N> = SessionMpst<
    <AtoCVideo<N> as Session>::Dual,
    <RecursBtoC<N> as Session>::Dual,
    RoleA<RoleA<RoleBroadcast>>,
    RoleC<RoleEnd>,
>;
type EndpointCRecurs<N> =
    SessionMpst<Choose0fromCtoA<N>, Choose0fromCtoB<N>, StackCRecurs, RoleC<RoleEnd>>;
type EndpointCFull<N> = SessionMpst<InitC<N>, Choose0fromCtoB<N>, StackCFull, RoleC<RoleEnd>>;

/// For A
type EndpointARecurs<N> = SessionMpst<End, RecursAtoC<N>, RoleC<RoleEnd>, RoleA<RoleEnd>>;
type EndpointAFull<N> = SessionMpst<End, InitA<N>, StackAInit, RoleA<RoleEnd>>;

/// For B
type EndpointBRecurs<N> = SessionMpst<End, RecursBtoC<N>, RoleC<RoleEnd>, RoleB<RoleEnd>>;

/// Functions related to endpoints
fn server(s: EndpointBRecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branches0BtoC::End(s) => {
            s.close()
        },
        Branches0BtoC::Video(s) => {
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
        Branches0AtoC::End(s) => {
            s.close()
        },
        Branches0AtoC::Video(s) => {
            let (request, s) = s.recv()?;
            let (video, s) = s.send(request + 1).recv()?;
            let s = s.send(video + 1);
            authenticator_recurs(s)
        },
    })
}

fn client(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();

    let (_, s) = s.send(0).recv()?;

    client_recurs(s, xs, 1)
}

fn client_recurs(
    s: EndpointCRecurs<i32>,
    mut xs: Vec<i32>,
    index: i32,
) -> Result<(), Box<dyn Error>> {
    match xs.pop() {
        Option::Some(_) => {
            let s: EndpointCVideo<i32> = choose_mpst_multi_to_all!(
                s,
                Branches0AtoC::Video,
                Branches0BtoC::Video, =>
                RoleA,
                RoleB, =>
                RoleC, SessionMpst,
                3
            );

            let (_, s) = s.send(1).recv()?;

            client_recurs(s, xs, index + 1)
        }
        Option::None => {
            let s = choose_mpst_multi_to_all!(
                s,
                Branches0AtoC::End,
                Branches0BtoC::End, =>
                RoleA,
                RoleB, =>
                RoleC, SessionMpst,
                3
            );

            assert_eq!(index, 100);

            s.close()
        }
    }
}

/////////////////////////////////////////

pub fn run_macro_recursive() {
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
