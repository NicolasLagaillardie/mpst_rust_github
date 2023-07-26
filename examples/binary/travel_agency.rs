#![allow(clippy::type_complexity)]

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::{choose, offer};

use rand::{thread_rng, Rng};

use std::error::Error;
use std::thread::spawn;

// S
enum Binary0A {
    Accept(Rec1A),
    Query(Recv<(), Recv<(), Recv<(), Rec0A>>>),
}
enum Binary1A {
    Yes(Recv<(), Recv<(), Recv<(), Recv<(), Recv<(), End>>>>>),
    No(Recv<(), Recv<(), Recv<(), End>>>),
}
type Rec0A = Recv<Binary0A, End>;
type Rec1A = Recv<Binary1A, End>;
type FullA = Rec0A;

fn binary_a(s: FullA) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Binary0A::Accept(s) => {
            offer!(s, {
                Binary1A::Yes(s) => {
                    let (_yes_a, s) = recv(s)?;
                    let (_yes_s, s) = recv(s)?;
                    let (_payment, s) = recv(s)?;
                    let (_ack, s) = recv(s)?;
                    let (_bye, s) = recv(s)?;
                    close(s)
                },
                Binary1A::No(s) => {
                    let (_no_a, s) = recv(s)?;
                    let (_no_s, s) = recv(s)?;
                    let (_bye_a, s) = recv(s)?;
                    close(s)
                },
            })
        },
        Binary0A::Query(s) => {
            let (_query_a, s) = recv(s)?;
            let (_quote_c, s) = recv(s)?;
            let (_dummy_s, s) = recv(s)?;
            binary_a(s)
        },
    })
}

// C
type FullB = <FullA as Session>::Dual;

fn binary_yes_b(s: FullB) -> Result<(), Box<dyn Error>> {
    let s: Send<Binary1A, End> = choose!(Binary0A::Accept, s);
    let s = choose!(Binary1A::Yes, s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    close(s)
}

fn binary_no_b(s: FullB) -> Result<(), Box<dyn Error>> {
    let s: Send<Binary1A, End> = choose!(Binary0A::Accept, s);
    let s = choose!(Binary1A::No, s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    close(s)
}

fn binary_query_b(s: FullB) -> Result<FullB, Box<dyn Error>> {
    let s = choose!(Binary0A::Query, s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    Ok(s)
}

fn main() {
    let mut threads = Vec::new();
    let mut sessions = Vec::new();

    let (thread, session) = fork_with_thread_id(binary_a);

    threads.push(thread);
    sessions.push(session);

    let main = spawn(move || {
        for _ in 0..LOOPS {
            sessions = sessions
                .into_iter()
                .map(|s| binary_query_b(s).unwrap())
                .collect::<Vec<_>>();
        }

        let choice = thread_rng().gen_range(1..=2);

        if choice != 1 {
            sessions.into_iter().for_each(|s| binary_yes_b(s).unwrap());
        } else {
            sessions.into_iter().for_each(|s| binary_no_b(s).unwrap());
        }

        threads.into_iter().for_each(|elt| elt.join().unwrap());
    });

    main.join().unwrap();
}

static LOOPS: i32 = 100;
