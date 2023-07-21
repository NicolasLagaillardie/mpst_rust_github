#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rand::{thread_rng, Rng};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::boxed::Box;
use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for four participants
generate!("rec_and_cancel", MeshedChannels, A, B, C);

type AtoCClose = End;
type AtoBClose = End;
type AtoBVideo = Send<i32, Recv<i32, End>>;
type AtoCVideo = Recv<i32, Send<i32, RecursAtoC>>;

type InitA = Recv<i32, Send<i32, RecursAtoC>>;

type BtoAClose = <AtoBClose as Session>::Dual;
type BtoCClose = End;
type BtoAVideo = <AtoBVideo as Session>::Dual;

type RecursAtoC = Recv<Branches0AtoC, End>;
type RecursBtoC = Recv<Branches0BtoC, End>;

enum Branches0AtoC {
    End(MeshedChannels<AtoBClose, AtoCClose, StackAEnd, NameA>),
    Video(MeshedChannels<AtoBVideo, AtoCVideo, StackAVideo, NameA>),
}
enum Branches0BtoC {
    End(MeshedChannels<BtoAClose, BtoCClose, StackBEnd, NameB>),
    Video(MeshedChannels<BtoAVideo, RecursBtoC, StackBVideo, NameB>),
}
type Choose0fromCtoA = Send<Branches0AtoC, End>;
type Choose0fromCtoB = Send<Branches0BtoC, End>;

type InitC = Send<i32, Recv<i32, Choose0fromCtoA>>;

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
    Send<i32, Recv<i32, Choose0fromCtoA>>,
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
fn server(s: EndpointBRecurs) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branches0BtoC::End(s) => {
            s.close()
        },
        Branches0BtoC::Video(s) => {
            let (request, s) = s.recv()?;
            let s = s.send(request + 1)?;
            server(s)
        },
    })
}

fn authenticator(s: EndpointAFull) -> Result<(), Box<dyn Error>> {
    let (id, s) = s.recv()?;
    let s = s.send(id + 1)?;

    authenticator_recurs(s)
}

fn authenticator_recurs(s: EndpointARecurs) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branches0AtoC::End(s) => {
            s.close()
        },
        Branches0AtoC::Video(s) => {
            let (request, s) = s.recv()?;
            let s = s.send(request + 1)?;
            let (video, s) = s.recv()?;
            let s = s.send(video + 1)?;
            authenticator_recurs(s)
        },
    })
}

fn client(s: EndpointCFull) -> Result<(), Box<dyn Error>> {
    let s = s.send(0)?;
    let (_, s) = s.recv()?;

    client_recurs(s, LOOPS)
}

fn client_recurs(s: EndpointCRecurs, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose_mpst_c_to_all!(s, Branches0AtoC::End, Branches0BtoC::End);

            s.close()
        }
        _ => {
            let s: EndpointCVideo =
                choose_mpst_c_to_all!(s, Branches0AtoC::Video, Branches0BtoC::Video);

            let s = s.send(1)?;
            let (_, s) = s.recv()?;

            client_recurs(s, loops - 1)
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
    c.bench_function("Video stream", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(20000);
    targets = video_stream,
}

criterion_main! {
    bench
}
