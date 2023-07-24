#![allow(clippy::type_complexity)]

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, session::Session};

use std::error::Error;
use std::thread::spawn;

// S
type FullA = Recv<(), Recv<(), Recv<(), Recv<(), Recv<(), End>>>>>;

fn binary_a(s: FullA) -> Result<(), Box<dyn Error>> {
    let (_get_web_page_load_state, s) = recv(s)?;
    let (_outstanding_web_fonts, s) = recv(s)?;
    let (_get_current_state, s) = recv(s)?;
    let (_document_loading, s) = recv(s)?;
    let (_web_font_loaded, _s) = recv(s)?;

    Ok(())
}

// C
type FullB = <FullA as Session>::Dual;

fn binary_b(s: FullB) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
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
        sessions.into_iter().for_each(|s| binary_b(s).unwrap());

        threads.into_iter().for_each(|elt| elt.join().unwrap());
    });

    main.join().unwrap();
}
