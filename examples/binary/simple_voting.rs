#![allow(clippy::type_complexity, dead_code)]

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, session::Session};
use mpstthree::{choose, offer};

use rand::{thread_rng, Rng};

use std::error::Error;
use std::thread::spawn;

// S
enum Binary0A {
    Accept(Recv<(), Rec1A>),
    // Not used
    Reject(Recv<(), End>),
}
enum Binary1A {
    Yes(Recv<(), End>),
    No(Recv<(), End>),
}
type Rec0A = Recv<Binary0A, End>;
type Rec1A = Recv<Binary1A, End>;
type FullA = Recv<(), Rec0A>;

fn binary_a(s: FullA) -> Result<(), Box<dyn Error>> {
    let (_authenticate, s) = recv(s)?;

    offer!(s, {
        Binary0A::Accept(s) => {
            let (_ok, s) = recv(s)?;

            offer!(s, {
                Binary1A::Yes(s) => {
                    let (_yes, s) = recv(s)?;
                    close(s)
                },
                Binary1A::No(s) => {
                    let (_no, s) = recv(s)?;
                    close(s)
                },
            })
        },
        Binary0A::Reject(s) => {
            let (_reject, s) = recv(s)?;
            close(s)
        },
    })
}

// C
type FullB = <FullA as Session>::Dual;

fn binary_yes_b(s: FullB) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let s = choose!(Binary0A::Accept, s);
    let s: mpstthree::binary::struct_trait::send::Send<Binary1A, End> = send((), s);
    let s = choose!(Binary1A::Yes, s);
    let s = send((), s);
    close(s)
}

fn binary_no_b(s: FullB) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let s = choose!(Binary0A::Accept, s);
    let s: mpstthree::binary::struct_trait::send::Send<Binary1A, End> = send((), s);
    let s = choose!(Binary1A::No, s);
    let s = send((), s);
    close(s)
}

// Not used
fn binary_reject_b(s: FullB) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let s = choose!(Binary0A::Reject, s);
    let s = send((), s);
    close(s)
}

fn main() {
    let mut threads = Vec::new();
    let mut sessions = Vec::new();

    let (thread, session) = fork_with_thread_id(binary_a);

    threads.push(thread);
    sessions.push(session);

    let main = spawn(move || {
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
