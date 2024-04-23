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
enum BinaryA {
    Accept(Recv<(), Recv<(), Recv<(), End>>>),
    Quit(Recv<(), Recv<(), End>>),
}
type FullA = Recv<(), Recv<(), Recv<(), Recv<(), Recv<BinaryA, End>>>>>;

fn binary_a(s: FullA) -> Result<(), Box<dyn Error>> {
    let (_empty_1, s) = recv(s)?;
    let (_empty_2, s) = recv(s)?;
    let (_empty_3, s) = recv(s)?;
    let (_empty_4, s) = recv(s)?;

    offer!(s, {
        BinaryA::Accept(s) => {
            let (_ok_a, s) = recv(s)?;
            let (_ok_s, s) = recv(s)?;
            let (_empty_5, s) = recv(s)?;
            close(s)
        },
        BinaryA::Quit(s) => {
            let (_quit_a, s) = recv(s)?;
            let (_quit_s, s) = recv(s)?;
            close(s)
        },
    })
}

// C
type FullB = <FullA as Session>::Dual;

fn binary_ok_b(s: FullB) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    let s = choose!(BinaryA::Accept, s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    close(s)
}

fn binary_quit_b(s: FullB) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    let s = choose!(BinaryA::Quit, s);
    let s = send((), s);
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
            sessions.into_iter().for_each(|s| binary_ok_b(s).unwrap());
        } else {
            sessions.into_iter().for_each(|s| binary_quit_b(s).unwrap());
        }

        threads.into_iter().for_each(|elt| elt.join().unwrap());
    });

    main.join().unwrap();
}
