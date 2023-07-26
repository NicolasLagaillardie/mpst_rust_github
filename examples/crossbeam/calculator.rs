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
enum BinaryS {
    Sum(Send<i32, End>),
    Diff(Send<i32, End>),
}
type OfferS = Recv<BinaryS, End>;
type FullS = Recv<i32, Recv<i32, OfferS>>;

fn binary_s(s: FullS) -> Result<(), Box<dyn Error>> {
    let (elt_1, s) = recv(s)?;
    let (elt_2, s) = recv(s)?;

    offer!(s, {
        BinaryS::Sum(s) => {
            let s = send(elt_1 + elt_2, s);
            close(s)
        },
        BinaryS::Diff(s) => {
            let s = send(elt_1 - elt_2, s);
            close(s)
        },
    })
}

// C
type ChoiceC = <OfferS as Session>::Dual;
type FullC = <FullS as Session>::Dual;

fn binary_c(s: FullC) -> Result<ChoiceC, Box<dyn Error>> {
    let s = send(thread_rng().gen_range(1..=100), s);
    let s = send(thread_rng().gen_range(1..=100), s);
    Ok(s)
}

fn main() {
    let (thread, session) = fork_with_thread_id(binary_s);

    let main = spawn(move || {
        let choice: i32 = thread_rng().gen_range(1..=2);

        let session = binary_c(session).unwrap();

        if choice != 1 {
            let session = choose!(BinaryS::Sum, session);
            let (_, session) = recv(session).unwrap();
            close(session).unwrap();
        } else {
            let session = choose!(BinaryS::Diff, session);
            let (_, session) = recv(session).unwrap();
            close(session).unwrap();
        }

        thread.join().unwrap()
    });

    main.join().unwrap();
}
