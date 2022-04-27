#![allow(clippy::type_complexity)]

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{baker, checker_concat};

use rand::{thread_rng, Rng};

use std::error::Error;

// Create new MeshedChannels for four participants
baker!("rec_and_cancel", MeshedChannels, A, B, C);

type AtoCClose = End;
type AtoBClose = End;
type AtoBVideo = Send<i32, Recv<i32, End>>;
type AtoCVideo = Recv<i32, Send<i32, RecursAtoC>>;

type InitA = Recv<i32, Send<i32, RecursAtoC>>;

type BtoAClose = End;
type BtoCClose = End;
type BtoAVideo = Recv<i32, Send<i32, End>>;

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
type EndpointCEnd = MeshedChannels<End, End, RoleEnd, NameC>;
type EndpointCVideo = MeshedChannels<
    Send<i32, Recv<i32, Choose0fromCtoA>>,
    Choose0fromCtoB,
    RoleA<RoleA<RoleBroadcast>>,
    NameC,
>;

type EndpointCRecurs = MeshedChannels<Choose0fromCtoA, Choose0fromCtoB, StackCRecurs, NameC>;
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
    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();

    let s = s.send(0)?;
    let (_, s) = s.recv()?;

    client_recurs(s, xs)
}

fn client_recurs(s: EndpointCRecurs, mut xs: Vec<i32>) -> Result<(), Box<dyn Error>> {
    match xs.pop() {
        Option::Some(_) => {
            let s: EndpointCVideo =
                choose_mpst_c_to_all!(s, Branches0AtoC::Video, Branches0BtoC::Video);

            let s = s.send(1)?;
            let (_, s) = s.recv()?;

            client_recurs(s, xs)
        }
        Option::None => {
            let s: EndpointCEnd = choose_mpst_c_to_all!(s, Branches0AtoC::End, Branches0BtoC::End);

            s.close()
        }
    }
}

/////////////////////////////////////////

// Check for bottom-up approach
fn checking() {
    let _ = checker_concat!(
        "video_stream",
        EndpointAFull,
        EndpointCFull,
        EndpointBRecurs
        =>
        [
            EndpointCVideo,
            Branches0AtoC, Video,
            Branches0BtoC, Video,
        ],
        [
            EndpointCEnd,
            Branches0AtoC, End,
            Branches0BtoC, End,
        ]
    )
    .unwrap();
}

/////////////////////////////////////////

fn main() {
    checking();

    let (thread_a, thread_s, thread_c) = fork_mpst(authenticator, server, client);

    assert!(thread_a.join().is_ok());
    assert!(thread_s.join().is_ok());
    assert!(thread_c.join().is_ok());
}
