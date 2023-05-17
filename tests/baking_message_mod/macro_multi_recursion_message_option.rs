// Test for parametrisation on the number of roles
use rand::{thread_rng, Rng};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::message::Message;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{baker, choose_mpst_create_multi_to_all};

use std::error::Error;
use Option::{None, Some};

// Create new roles
baker!("basic", MeshedChannels, A, B, D);

// Test our usecase
// Simple types
// Client = D
// Authenticator = A
// Server = B

type AtoDClose = End;
type AtoBClose = End;
type AtoBVideo = Send<Message<String, Option<i32>>, Recv<Message<String, Option<i32>>, End>>;
type AtoDVideo = Recv<Message<String, Option<i32>>, Send<Message<String, Option<i32>>, RecursAtoD>>;

type InitA = Recv<Message<String, i32>, Send<Message<String, i32>, RecursAtoD>>;

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
type Choose0fromCtoA = Send<Branches0AtoD, End>;
type Choose0fromCtoB = Send<Branches0BtoD, End>;

type InitD = Send<Message<String, i32>, Recv<Message<String, i32>, Choose0fromCtoA>>;

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
type EndpointDRecurs = MeshedChannels<Choose0fromCtoA, Choose0fromCtoB, StackDRecurs, NameD>;
type EndpointDFull = MeshedChannels<InitD, Choose0fromCtoB, StackDFull, NameD>;

// For A
type EndpointARecurs = MeshedChannels<End, RecursAtoD, StackARecurs, NameA>;
type EndpointAFull = MeshedChannels<End, InitA, StackAInit, NameA>;

// For B
type EndpointBRecurs = MeshedChannels<End, RecursBtoD, StackBRecurs, NameB>;

choose_mpst_create_multi_to_all!(choose_mpst_client_to_all, NameD, MeshedChannels, 3);

// Functions related to endpoints
fn server(s: EndpointBRecurs) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branches0BtoD::End(s) => {
            s.close()
        },
        Branches0BtoD::Video(s) => {
            let (_, s) = s.recv();
            let s = s.send(Message {
                label: "video".to_string(),
                payload: None,
            });
            server(s)
        },
    })
}

fn authenticator(s: EndpointAFull) -> Result<(), Box<dyn Error>> {
    let (id, s) = s.recv();
    let s = s.send(id);

    authenticator_recurs(s)
}

fn authenticator_recurs(s: EndpointARecurs) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branches0AtoD::End(s) => {
            s.close()
        },
        Branches0AtoD::Video(s) => {
            let (_, s) = s.recv();
            let (_, s) = s.send(Message {
                label: "request".to_string(),
                payload: None,
            }).recv();
            let s = s.send(Message {
                label: "video".to_string(),
                payload: Some(0),
            });
            authenticator_recurs(s)
        },
    })
}

fn client(s: EndpointDFull) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();

    let (_, s) = s
        .send(Message {
            label: String::from("Start"),
            payload: 0,
        })
        .recv();

    client_recurs(s, xs, 1)
}

fn client_recurs(s: EndpointDRecurs, mut xs: Vec<i32>, index: i32) -> Result<(), Box<dyn Error>> {
    match xs.pop() {
        Some(_) => {
            let s: EndpointDVideo =
                choose_mpst_client_to_all!(s, Branches0AtoD::Video, Branches0BtoD::Video);

            let (_, s) = s
                .send(Message {
                    label: format!("Loop number {index}"),
                    payload: Some(index),
                })
                .recv();

            client_recurs(s, xs, index + 1)
        }
        None => {
            let s = choose_mpst_client_to_all!(s, Branches0AtoD::End, Branches0BtoD::End);

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
