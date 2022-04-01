// Test for affine timed protocols
use rand::{thread_rng, Rng};

use mpstthree::binary::struct_trait::{end::End, session::Session};
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::bundle_impl_timed_with_enum_and_cancel;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use std::collections::HashMap;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

bundle_impl_timed_with_enum_and_cancel!(MeshedChannels, A, B, D);

// Test our usecase
// Simple types
// Client = D
// Authenticator = A
// Server = B

// A
type InitA = RecvTimed<
    i32,
    SendTimed<i32, <Choose0fromDtoAOutLoop as Session>::Dual, 'a', 2, true, 4, true, false>,
    'a',
    0,
    true,
    2,
    true,
    false,
>;

type AtoDClose = End;
type AtoBClose = End;
type AtoBVideo =
    SendTimed<i32, RecvTimed<i32, End, 'a', 4, true, 6, true, false>, 'a', 2, true, 4, true, false>;
type AtoDVideo = RecvTimed<
    i32,
    SendTimed<i32, RecursAtoD, 'a', 6, true, 8, true, false>,
    'a',
    0,
    true,
    2,
    true,
    false,
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
type Choose0fromDtoAInLoop = SendTimed<Branches0AtoD, End, 'a', 8, true, 10, true, true>;
type Choose0fromDtoBInLoop = SendTimed<Branches0BtoD, End, 'a', 8, true, 10, true, true>;

type Choose0fromDtoAOutLoop = SendTimed<Branches0AtoD, End, 'a', 4, true, 6, true, true>;
type Choose0fromDtoBOutLoop = SendTimed<Branches0BtoD, End, 'a', 4, true, 6, true, true>;

type InitD = SendTimed<
    i32,
    RecvTimed<i32, Choose0fromDtoAOutLoop, 'a', 2, true, 4, true, false>,
    'a',
    0,
    true,
    2,
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
    println!("pause of 5 for B");
    println!("Time constraint 2 for B: {:#?}", s.session2.constraint());
    if let Some(clock) = all_clocks.get(&'a') {
        println!("clock {:#?} for B", clock.elapsed().as_secs());
    };
    (move || -> Result<_, _> {
        println!("Going to receive offer for B");
        let (l, s) = match s.recv(all_clocks) {
            Ok((l, s)) => (l, s),
            Err(e) => {
                println!("error: {:#?}", e);
                panic!("Ah")
            }
        };
        println!("Received offer for B");
        s.cancel();
        println!("Canceled");
        match l {
            Branches0BtoD::End(s) => s.close(),
            Branches0BtoD::Video(s) => {
                if let Some(clock) = all_clocks.get(&'a') {
                    println!("clock {:#?} for B", clock.elapsed().as_secs());
                };
                sleep(Duration::from_secs(3));
                println!("pause of 3 for B");
                let (request, s) = s.recv(all_clocks)?;
                sleep(Duration::from_secs(2));
                println!("pause of 2 for B");
                let s = s.send(request + 1, all_clocks)?;
                sleep(Duration::from_secs(4));
                println!("pause of 4 for B");

                server_recurs(s, all_clocks)
            }
        }
    })()
}

fn server_recurs(
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
                    if let Some(clock) = all_clocks.get(&'a') {
                        println!("clock {:#?} for B", clock.elapsed().as_secs());
                    };
                    sleep(Duration::from_secs(3));
                    println!("pause of 3 for B");
                    let (request, s) = s.recv(all_clocks)?;
                    sleep(Duration::from_secs(2));
                    println!("pause of 2 for B");
                    let s = s.send(request + 1, all_clocks)?;
                    sleep(Duration::from_secs(4));
                    println!("pause of 4 for B");

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
    println!("pause of 1 for A");
    let (id, s) = s.recv(all_clocks)?;
    sleep(Duration::from_secs(2));
    println!("pause of 2 for A");
    let s = s.send(id + 1, all_clocks)?;
    sleep(Duration::from_secs(2));
    println!("pause of 2 for A");

    offer_mpst!(
        all_clocks,
        s,
        {
            Branches0AtoD::End(s) => {
                s.close()
            },
            Branches0AtoD::Video(s) => {
                println!("Received offer for A");
                if let Some(clock) = all_clocks.get(&'a') {
                    println!("clock {:#?} for A", clock.elapsed().as_secs());
                };
                println!("Time constraint 1 for A: {:#?}", s.session1.constraint());
                println!("Time constraint 2 for A: {:#?}", s.session2.constraint());
                sleep(Duration::from_secs(1));
                println!("pause of 1 for A");
                if let Some(clock) = all_clocks.get(&'a') {
                    println!("clock {:#?} for A", clock.elapsed().as_secs());
                };
                let (request, s) = s.recv(all_clocks)?;
                sleep(Duration::from_secs(2));
                println!("pause of 2 for A");
                let s = s.send(request + 1, all_clocks)?;
                sleep(Duration::from_secs(2));
                println!("pause of 2 for A");
                let (video, s) = s.recv(all_clocks)?;
                sleep(Duration::from_secs(2));
                println!("pause of 2 for A");
                let s = s.send(video + 1, all_clocks)?;
                sleep(Duration::from_secs(2));
                println!("pause of 2 for A");

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
        all_clocks,
        s,
        {
            Branches0AtoD::End(s) => {
                s.close()
            },
            Branches0AtoD::Video(s) => {
                println!("Received offer for A");
                if let Some(clock) = all_clocks.get(&'a') {
                    println!("clock {:#?} for A", clock.elapsed().as_secs());
                };
                sleep(Duration::from_secs(1));
                println!("pause of 1 for A");
                let (request, s) = s.recv(all_clocks)?;
                sleep(Duration::from_secs(2));
                println!("pause of 2 for A");
                let s = s.send(request + 1, all_clocks)?;
                sleep(Duration::from_secs(2));
                println!("pause of 2 for A");
                let (video, s) = s.recv(all_clocks)?;
                sleep(Duration::from_secs(2));
                println!("pause of 2 for A");
                let s = s.send(video + 1, all_clocks)?;
                sleep(Duration::from_secs(2));
                println!("pause of 2 for A");

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
    println!("pause of 1 for C");
    let s = s.send(0, all_clocks)?;
    sleep(Duration::from_secs(2));
    println!("pause of 2 for C");
    let (_, s) = s.recv(all_clocks)?;
    sleep(Duration::from_secs(2));
    println!("pause of 2 for C");

    match xs.pop() {
        Option::Some(_) => {
            println!("Right branch");
            println!("Time constraint 1 for C: {:#?}", s.session1.constraint());
            println!("Time constraint 2 for C: {:#?}", s.session2.constraint());
            if let Some(clock) = all_clocks.get(&'a') {
                println!("clock {:#?} for C", clock.elapsed().as_secs());
            };
            let s: EndpointDVideo = 
                choose_mpst_d_to_all!(s, all_clocks, Branches0AtoD::Video, Branches0BtoD::Video);

            if let Some(clock) = all_clocks.get(&'a') {
                println!("clock {:#?} for C", clock.elapsed().as_secs());
            };
            sleep(Duration::from_secs(1));
            println!("pause of 1 for C");
            let s = s.send(1, all_clocks)?;
            sleep(Duration::from_secs(6));
            println!("pause of 6 for C");
            let (_, s) = s.recv(all_clocks)?;
            sleep(Duration::from_secs(2));
            println!("pause of 2 for C");

            client_recurs(s, all_clocks, xs, 2)
        }
        Option::None => {
            println!("Wrong branch");
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

            if let Some(clock) = all_clocks.get(&'a') {
                println!("clock {:#?} for C", clock.elapsed().as_secs());
            };
            sleep(Duration::from_secs(1));
            println!("pause of 1 for C");
            let s = s.send(1, all_clocks)?;
            sleep(Duration::from_secs(6));
            println!("pause of 6 for C");
            let (_, s) = s.recv(all_clocks)?;
            sleep(Duration::from_secs(2));
            println!("pause of 2 for C");

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

#[test]
pub fn main_timed() {
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
