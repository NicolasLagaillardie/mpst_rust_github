// Test for affine timed protocols
use rand::{thread_rng, Rng};

use mpstthree::binary::struct_trait::{end::End, session::Session};
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::bundle_impl_timed_with_enum_and_cancel;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

bundle_impl_timed_with_enum_and_cancel!(MeshedChannels, A, B, D);

// Test our usecase
// Simple types
// Client = D
// Authenticator = A
// Server = B

type AtoDClose = End;
type AtoBClose = End;
type AtoBVideo =
    SendTimed<i32, RecvTimed<i32, End, 'a', 0, true, 1, true, false>, 'a', 0, true, 1, true, false>;
type AtoDVideo = RecvTimed<
    i32,
    SendTimed<i32, RecursAtoD, 'a', 0, true, 1, true, false>,
    'a',
    0,
    true,
    1,
    true,
    false,
>;

type InitA = RecvTimed<
    i32,
    SendTimed<i32, RecursAtoD, 'a', 0, true, 1, true, false>,
    'a',
    0,
    true,
    1,
    true,
    false,
>;

type BtoAClose = <AtoBClose as Session>::Dual;
type BtoDClose = End;
type BtoAVideo = <AtoBVideo as Session>::Dual;

type RecursAtoD = <Choose0fromCtoA as Session>::Dual;
type RecursBtoD = <Choose0fromCtoB as Session>::Dual;

enum Branches0AtoD {
    End(MeshedChannels<AtoBClose, AtoDClose, StackAEnd, NameA>),
    Video(MeshedChannels<AtoBVideo, AtoDVideo, StackAVideo, NameA>),
}
enum Branches0BtoD {
    End(MeshedChannels<BtoAClose, BtoDClose, StackBEnd, NameB>),
    Video(MeshedChannels<BtoAVideo, RecursBtoD, StackBVideo, NameB>),
}
type Choose0fromCtoA = SendTimed<Branches0AtoD, End, 'a', 0, true, 1, true, false>;
type Choose0fromCtoB = SendTimed<Branches0BtoD, End, 'a', 0, true, 1, true, false>;

type InitD = SendTimed<
    i32,
    RecvTimed<i32, Choose0fromCtoA, 'a', 0, true, 1, true, false>,
    'a',
    0,
    true,
    1,
    true,
    false,
>;

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
type EndpointDVideo = MeshedChannels<
    <AtoDVideo as Session>::Dual,
    <RecursBtoD as Session>::Dual,
    RoleA<RoleA<RoleBroadcast>>,
    NameD,
>;
type EndpointDEnd = MeshedChannels<End, End, RoleEnd, NameD>;
type EndpointDRecurs = MeshedChannels<Choose0fromCtoA, Choose0fromCtoB, StackDRecurs, NameD>;
type EndpointDFull = MeshedChannels<InitD, Choose0fromCtoB, StackDFull, NameD>;

// For A
type EndpointARecurs = MeshedChannels<End, RecursAtoD, StackARecurs, NameA>;
type EndpointAFull = MeshedChannels<End, InitA, StackAInit, NameA>;

// For B
type EndpointBRecurs = MeshedChannels<End, RecursBtoD, StackBRecurs, NameB>;

// Functions related to endpoints
fn server(
    s: EndpointBRecurs,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(
            all_clocks,
            s,
            {
                Branches0BtoD::End(s) => {
                    s.close()
                },
                Branches0BtoD::Video(s) => {
                    let (request, s) = s.recv(all_clocks)?;
                    let s = s.send(request + 1, all_clocks)?;
                    server(s, all_clocks)
                },
            }
    )
}

fn authenticator(
    s: EndpointAFull,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    let (id, s) = s.recv(all_clocks)?;
    let s = s.send(id + 1, all_clocks)?;

    authenticator_recurs(s, all_clocks)
}

fn authenticator_recurs(
    s: EndpointARecurs,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(
        all_clocks,
        s,
        {
            Branches0AtoD::End(s) => {
                s.close()
            },
            Branches0AtoD::Video(s) => {
                let (request, s) = s.recv(all_clocks)?;
                let (video, s) = s.send(request + 1, all_clocks)?.recv(all_clocks)?;
                let s = s.send(video + 1, all_clocks)?;
                authenticator_recurs(s, all_clocks)
            },
        }
    )
}

fn client(s: EndpointDFull, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();

    let (_, s) = s.send(0, all_clocks)?.recv(all_clocks)?;

    client_recurs(s, all_clocks, xs, 1)
}

fn client_recurs(
    s: EndpointDRecurs,
    all_clocks: &mut HashMap<char, Instant>,
    mut xs: Vec<i32>,
    index: i32,
) -> Result<(), Box<dyn Error>> {
    match xs.pop() {
        Option::Some(_) => {
            let s: EndpointDVideo =
                choose_mpst_d_to_all!(s, all_clocks, Branches0AtoD::Video, Branches0BtoD::Video);

            let (_, s) = s.send(1, all_clocks)?.recv(all_clocks)?;

            client_recurs(s, all_clocks, xs, index + 1)
        }
        Option::None => {
            let s: EndpointDEnd =
                choose_mpst_d_to_all!(s, all_clocks, Branches0AtoD::End, Branches0BtoD::End);

            assert_eq!(index, 100);

            s.close()
        }
    }
}

////////////////////////////////////////

pub fn main() {
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
