use criterion::{black_box, Criterion};

use rand::{thread_rng, Rng};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::functionmpst::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::broadcast::RoleBroadcast;

use std::boxed::Box;
use std::error::Error;
use std::marker;
// use std::time::Duration;

use mpstthree::functionmpst::close::close_mpst;

// Get roles
use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

// Get recv functions
use mpstthree::functionmpst::recv::recv_mpst_a_from_b;
use mpstthree::functionmpst::recv::recv_mpst_a_from_c;
use mpstthree::functionmpst::recv::recv_mpst_b_from_a;
use mpstthree::functionmpst::recv::recv_mpst_c_from_a;

// Get send functions
use mpstthree::functionmpst::send::send_mpst_a_to_b;
use mpstthree::functionmpst::send::send_mpst_a_to_c;
use mpstthree::functionmpst::send::send_mpst_b_to_a;
use mpstthree::functionmpst::send::send_mpst_c_to_a;

use mpstthree::choose_mpst_c_to_all;
use mpstthree::offer_mpst_a_to_c;
use mpstthree::offer_mpst_b_to_c;

// See the folder scribble_protocols for the related Scribble protocol

// Test our usecase Video Stream
// Simple types
// Client = C
// Authenticator = A
// Server = B

type AtoCClose = End;
type AtoBClose = End;
type AtoBVideo<N> = Send<N, Recv<N, End>>;
type AtoCVideo<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type InitA<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type BtoAClose = <AtoBClose as Session>::Dual;
type BtoCClose = End;
type BtoAVideo<N> = <AtoBVideo<N> as Session>::Dual;

type RecursAtoC<N> = Recv<Branches0AtoC<N>, End>;
type RecursBtoC<N> = Recv<Branches0BtoC<N>, End>;

enum Branches0AtoC<N: marker::Send> {
    End(MeshedChannels<AtoBClose, AtoCClose, StackAEnd, RoleA<RoleEnd>>),
    Video(MeshedChannels<AtoBVideo<N>, AtoCVideo<N>, StackAVideo, RoleA<RoleEnd>>),
}
enum Branches0BtoC<N: marker::Send> {
    End(MeshedChannels<BtoAClose, BtoCClose, StackBEnd, RoleB<RoleEnd>>),
    Video(MeshedChannels<BtoAVideo<N>, RecursBtoC<N>, StackBVideo, RoleB<RoleEnd>>),
}
type Choose0fromCtoA<N> = Send<Branches0AtoC<N>, End>;
type Choose0fromCtoB<N> = Send<Branches0BtoC<N>, End>;

type InitC<N> = Send<N, Recv<N, Choose0fromCtoA<N>>>;

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
type EndpointCRecurs<N> =
    MeshedChannels<Choose0fromCtoA<N>, Choose0fromCtoB<N>, StackCRecurs, RoleC<RoleEnd>>;
type EndpointCFull<N> = MeshedChannels<InitC<N>, Choose0fromCtoB<N>, StackCFull, RoleC<RoleEnd>>;

// For A
type EndpointARecurs<N> = MeshedChannels<End, RecursAtoC<N>, StackARecurs, RoleA<RoleEnd>>;
type EndpointAFull<N> = MeshedChannels<End, InitA<N>, StackAInit, RoleA<RoleEnd>>;

// For B
type EndpointBRecurs<N> = MeshedChannels<End, RecursBtoC<N>, StackBRecurs, RoleB<RoleEnd>>;

// Functions related to endpoints
fn server(s: EndpointBRecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_b_to_c!(s, {
        Branches0BtoC::End(s) => {
            close_mpst(s)
        },
        Branches0BtoC::Video(s) => {
            let (request, s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_a(request + 1, s);
            server(s)
        },
    })
}

fn authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_a_from_c(s)?;
    let s = send_mpst_a_to_c(id + 1, s);

    authenticator_recurs(s)
}

fn authenticator_recurs(s: EndpointARecurs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_a_to_c!(s, {
        Branches0AtoC::End(s) => {
            close_mpst(s)
        },
        Branches0AtoC::Video(s) => {
            let (request, s) = recv_mpst_a_from_c(s)?;
            let s = send_mpst_a_to_b(request + 1, s);
            let (video, s) = recv_mpst_a_from_b(s)?;
            let s = send_mpst_a_to_c(video + 1, s);
            authenticator_recurs(s)
        },
    })
}

fn client(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();

    let s = send_mpst_c_to_a(0, s);
    let (_, s) = recv_mpst_c_from_a(s)?;

    client_recurs(s, xs, 1)
}

fn client_recurs(
    s: EndpointCRecurs<i32>,
    mut xs: Vec<i32>,
    index: i32,
) -> Result<(), Box<dyn Error>> {
    match xs.pop() {
        Option::Some(_) => {
            let s = choose_mpst_c_to_all!(s, Branches0AtoC::Video, Branches0BtoC::Video);

            let s = send_mpst_c_to_a(1, s);
            let (_, s) = recv_mpst_c_from_a(s)?;

            client_recurs(s, xs, index + 1)
        }
        Option::None => {
            let s = choose_mpst_c_to_all!(s, Branches0AtoC::End, Branches0BtoC::End);

            assert_eq!(index, 100);

            close_mpst(s)
        }
    }
}

/////////////////////////////////////////

fn all_mpst() {
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

pub fn video_stream_main(c: &mut Criterion) {
    c.bench_function("Video stream", |b| b.iter(all_mpst));
}
