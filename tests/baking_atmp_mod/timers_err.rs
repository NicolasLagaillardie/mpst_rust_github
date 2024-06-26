// Test for affine timed protocols
use rand::{thread_rng, Rng};

use mpstthree::binary::struct_trait::{end::End, session::Session};
use mpstthree::binary_atmp::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_atmp;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::collections::HashMap;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

generate_atmp!(MeshedChannels, A, B, D);

// Test our usecase
// Simple types
// Client = D
// Authenticator = A
// Server = B

// A
type InitA = RecvTimed<
    i32,
    'a',
    0,
    true,
    1,
    true,
    ' ',
    SendTimed<i32, 'a', 0, true, 1, true, ' ', <Choose0fromDtoAOutLoop as Session>::Dual>,
>;

type AtoDClose = End;
type AtoBClose = End;
type AtoBVideo =
    SendTimed<i32, 'a', 0, true, 1, true, ' ', RecvTimed<i32, 'a', 0, true, 1, true, ' ', End>>;
type AtoDVideo = RecvTimed<
    i32,
    'a',
    0,
    true,
    1,
    true,
    ' ',
    SendTimed<i32, 'a', 0, true, 1, true, ' ', RecursAtoD>,
>;

type RecursAtoD = <Choose0fromDtoAInLoop as Session>::Dual;

enum Branches0AtoD {
    End(MeshedChannels<AtoBClose, AtoDClose, StackAEnd, NameA>),
    Video(MeshedChannels<AtoBVideo, AtoDVideo, StackAVideo, NameA>),
}

// B
type InitB = <Choose0fromDtoBOutLoop as Session>::Dual;

type BtoAClose = <AtoBClose as Session>::Dual;
type BtoDClose = End;
type BtoAVideo = <AtoBVideo as Session>::Dual;

type RecursBtoD = <Choose0fromDtoBInLoop as Session>::Dual;

enum Branches0BtoD {
    End(MeshedChannels<BtoAClose, BtoDClose, StackBEnd, NameB>),
    Video(MeshedChannels<BtoAVideo, RecursBtoD, StackBVideo, NameB>),
}

// D
type Choose0fromDtoAInLoop = SendTimed<Branches0AtoD, 'a', 0, true, 1, true, 'a', End>;
type Choose0fromDtoBInLoop = SendTimed<Branches0BtoD, 'a', 0, true, 1, true, 'a', End>;

type Choose0fromDtoAOutLoop = SendTimed<Branches0AtoD, 'a', 0, true, 1, true, 'a', End>;
type Choose0fromDtoBOutLoop = SendTimed<Branches0BtoD, 'a', 0, true, 1, true, 'a', End>;

type InitD = SendTimed<
    i32,
    'a',
    0,
    true,
    1,
    true,
    ' ',
    RecvTimed<i32, 'a', 0, true, 1, true, ' ', Choose0fromDtoAOutLoop>,
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
type EndpointDRecurs =
    MeshedChannels<Choose0fromDtoAInLoop, Choose0fromDtoBInLoop, StackDRecurs, NameD>;
type EndpointDFull = MeshedChannels<InitD, Choose0fromDtoBOutLoop, StackDFull, NameD>;

// For A
type EndpointARecurs = MeshedChannels<End, RecursAtoD, StackARecurs, NameA>;
type EndpointAFull = MeshedChannels<End, InitA, StackAInit, NameA>;

// For B
type EndpointBRecurs = MeshedChannels<End, RecursBtoD, StackBRecurs, NameB>;
type EndpointBFull = MeshedChannels<End, InitB, StackBRecurs, NameB>;

// Functions related to endpoints
fn server(s: EndpointBFull, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    sleep(Duration::from_secs(5));

    offer_mpst!(
        s,
        all_clocks,
        {
            Branches0BtoD::End(s) => {
                s.close()
            },
            Branches0BtoD::Video(s) => {
                sleep(Duration::from_secs(3));
                let (request, s) = s.recv(all_clocks)?;
                sleep(Duration::from_secs(2));
                let s = s.send(request + 1, all_clocks)?;
                sleep(Duration::from_secs(4));

                server_recurs(s, all_clocks)
            },
        }
    )
}

fn server_recurs(
    s: EndpointBRecurs,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(
        s,
        all_clocks,
        {
            Branches0BtoD::End(s) => {
                s.close()
            },
            Branches0BtoD::Video(s) => {
                sleep(Duration::from_secs(3));
                let (request, s) = s.recv(all_clocks)?;
                sleep(Duration::from_secs(2));
                let s = s.send(request + 1, all_clocks)?;
                sleep(Duration::from_secs(4));

                server_recurs(s, all_clocks)
            },
        }
    )
}

fn authenticator(
    s: EndpointAFull,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    sleep(Duration::from_secs(1));

    let (id, s) = s.recv(all_clocks)?;
    sleep(Duration::from_secs(2));
    let s = s.send(id + 1, all_clocks)?;
    sleep(Duration::from_secs(2));

    offer_mpst!(
        s,
        all_clocks,
        {
            Branches0AtoD::End(s) => {
                s.close()
            },
            Branches0AtoD::Video(s) => {
                sleep(Duration::from_secs(1));
                let (request, s) = s.recv(all_clocks)?;
                sleep(Duration::from_secs(2));
                let s = s.send(request + 1, all_clocks)?;
                sleep(Duration::from_secs(2));
                let (video, s) = s.recv(all_clocks)?;
                sleep(Duration::from_secs(2));
                let s = s.send(video + 1, all_clocks)?;
                sleep(Duration::from_millis(1500));

                authenticator_recurs(s, all_clocks)
            },
        }
    )
}

fn authenticator_recurs(
    s: EndpointARecurs,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(
        s,
        all_clocks,
        {
            Branches0AtoD::End(s) => {
                s.close()
            },
            Branches0AtoD::Video(s) => {
                sleep(Duration::from_secs(1));
                let (request, s) = s.recv(all_clocks)?;
                sleep(Duration::from_secs(2));
                let s = s.send(request + 1, all_clocks)?;
                sleep(Duration::from_secs(2));
                let (video, s) = s.recv(all_clocks)?;
                sleep(Duration::from_secs(2));
                let s = s.send(video + 1, all_clocks)?;
                sleep(Duration::from_secs(2));

                authenticator_recurs(s, all_clocks)
            },
        }
    )
}

fn client(s: EndpointDFull, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let mut xs: Vec<i32> = (1..5).map(|_| rng.gen()).collect();
    all_clocks.insert('a', Instant::now());

    sleep(Duration::from_secs(1));

    let s = s.send(0, all_clocks)?;
    sleep(Duration::from_secs(2));
    let (_, s) = s.recv(all_clocks)?;
    sleep(Duration::from_secs(2));

    match xs.pop() {
        Option::Some(_) => {
            let s: EndpointDVideo =
                choose_mpst_d_to_all!(s, all_clocks, Branches0AtoD::Video, Branches0BtoD::Video);

            sleep(Duration::from_secs(1));
            let s = s.send(1, all_clocks)?;
            sleep(Duration::from_secs(6));
            let (_, s) = s.recv(all_clocks)?;
            sleep(Duration::from_secs(2));

            client_recurs(s, all_clocks, xs, 2)
        }
        Option::None => {
            let s: EndpointDEnd =
                choose_mpst_d_to_all!(s, all_clocks, Branches0AtoD::End, Branches0BtoD::End);

            s.close()
        }
    }
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

            sleep(Duration::from_secs(1));
            let s = s.send(1, all_clocks)?;
            sleep(Duration::from_secs(6));
            let (_, s) = s.recv(all_clocks)?;
            sleep(Duration::from_secs(2));

            client_recurs(s, all_clocks, xs, index + 1)
        }
        Option::None => {
            let s: EndpointDEnd =
                choose_mpst_d_to_all!(s, all_clocks, Branches0AtoD::End, Branches0BtoD::End);

            assert_eq!(index, 5);

            s.close()
        }
    }
}

////////////////////////////////////////

pub fn main() {
    let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client);

    assert!(thread_a.join().is_err());
    assert!(thread_b.join().is_err());
    assert!(thread_c.join().is_err());
}
