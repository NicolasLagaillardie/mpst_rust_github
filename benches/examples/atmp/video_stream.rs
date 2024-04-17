#![allow(clippy::type_complexity)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, session::Session};
use mpstthree::binary_atmp::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_atmp;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for four participants
generate_atmp!(MeshedChannels, A, B, C);

type AtoCClose = End;
type AtoBClose = End;
type AtoBVideo =
    SendTimed<i32, 'a', 0, true, 10, true, ' ', RecvTimed<i32, 'a', 0, true, 10, true, ' ', End>>;
type AtoCVideo = RecvTimed<
    i32,
    'a',
    0,
    true,
    10,
    true,
    ' ',
    SendTimed<i32, 'a', 0, true, 10, true, ' ', RecursAtoC>,
>;

type InitA = RecvTimed<
    i32,
    'a',
    0,
    true,
    10,
    true,
    ' ',
    SendTimed<i32, 'a', 0, true, 10, true, ' ', RecursAtoC>,
>;

type BtoAClose = <AtoBClose as Session>::Dual;
type BtoCClose = End;
type BtoAVideo = <AtoBVideo as Session>::Dual;

type RecursAtoC = RecvTimed<Branches0AtoC, 'a', 0, true, 10, true, ' ', End>;
type RecursBtoC = RecvTimed<Branches0BtoC, 'a', 0, true, 10, true, ' ', End>;

enum Branches0AtoC {
    End(MeshedChannels<AtoBClose, AtoCClose, StackAEnd, NameA>),
    Video(MeshedChannels<AtoBVideo, AtoCVideo, StackAVideo, NameA>),
}
enum Branches0BtoC {
    End(MeshedChannels<BtoAClose, BtoCClose, StackBEnd, NameB>),
    Video(MeshedChannels<BtoAVideo, RecursBtoC, StackBVideo, NameB>),
}
type Choose0fromCtoA = SendTimed<Branches0AtoC, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromCtoB = SendTimed<Branches0BtoC, 'a', 0, true, 10, true, ' ', End>;

type InitC = SendTimed<
    i32,
    'a',
    0,
    true,
    10,
    true,
    ' ',
    RecvTimed<i32, 'a', 0, true, 10, true, ' ', Choose0fromCtoA>,
>;

// Stacks
type StackAEnd = RoleEnd;
type StackAVideo = RoleC<RoleB<RoleB<RoleC<RoleC<RoleEnd>>>>>;
type StackARecurs = RoleC<RoleEnd>;
type StackAInit = RoleC<RoleC<RoleC<RoleEnd>>>;

type StackBEnd = RoleEnd;
type StackBVideo = RoleA<RoleA<RoleC<RoleEnd>>>;
type StackBRecurs = RoleC<RoleEnd>;

type StackCRecurs = RoleBroadcast;
type StackCFull = RoleA<RoleA<StackCRecurs>>;

// Creating the MP sessions

// For C
type EndpointCRecurs = MeshedChannels<Choose0fromCtoA, Choose0fromCtoB, StackCRecurs, NameC>;
type EndpointCVideo = MeshedChannels<
    SendTimed<
        i32,
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<i32, 'a', 0, true, 10, true, ' ', Choose0fromCtoA>,
    >,
    Choose0fromCtoB,
    RoleA<RoleA<StackCRecurs>>,
    NameC,
>;
type EndpointCFull = MeshedChannels<InitC, Choose0fromCtoB, StackCFull, NameC>;

// For A
type EndpointARecurs = MeshedChannels<End, RecursAtoC, StackARecurs, NameA>;
type EndpointAFull = MeshedChannels<End, InitA, StackAInit, NameA>;

// For B
type EndpointBRecurs = MeshedChannels<End, RecursBtoC, StackBRecurs, NameB>;

// Functions related to endpoints
fn server(
    s: EndpointBRecurs,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    offer_mpst!(s, all_clocks, {
        Branches0BtoC::End(s) => {
            s.close()
        },
        Branches0BtoC::Video(s) => {
            let (request, s) = s.recv(all_clocks)?;
            let s = s.send(request + 1, all_clocks)?;
            server(s, all_clocks)
        },
    })
}

fn authenticator(
    s: EndpointAFull,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (id, s) = s.recv(all_clocks)?;
    let s = s.send(id + 1, all_clocks)?;

    authenticator_recurs(s, all_clocks)
}

fn authenticator_recurs(
    s: EndpointARecurs,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branches0AtoC::End(s) => {
            s.close()
        },
        Branches0AtoC::Video(s) => {
            let (request, s) = s.recv(all_clocks)?;
            let s = s.send(request + 1, all_clocks)?;
            let (video, s) = s.recv(all_clocks)?;
            let s = s.send(video + 1, all_clocks)?;
            authenticator_recurs(s, all_clocks)
        },
    })
}

fn client(s: EndpointCFull, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let s = s.send(0, all_clocks)?;
    let (_, s) = s.recv(all_clocks)?;

    client_recurs(s, LOOPS, all_clocks)
}

fn client_recurs(
    s: EndpointCRecurs,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose_mpst_c_to_all!(s, all_clocks, Branches0AtoC::End, Branches0BtoC::End);

            s.close()
        }
        _ => {
            let s: EndpointCVideo =
                choose_mpst_c_to_all!(s, all_clocks, Branches0AtoC::Video, Branches0BtoC::Video);

            let s = s.send(1, all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;

            client_recurs(s, loops - 1, all_clocks)
        }
    }
}

/////////////////////////////////////////

fn aux() {
    let (thread_a, thread_s, thread_c) = fork_mpst(
        black_box(authenticator),
        black_box(server),
        black_box(client),
    );

    thread_a.join().unwrap();
    thread_s.join().unwrap();
    thread_c.join().unwrap();
}

/////////////////////////

static LOOPS: i32 = 100;

pub fn video_stream(c: &mut Criterion) {
    c.bench_function("Timed Video stream", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(100000);
    targets = video_stream,
}

criterion_main! {
    bench
}
