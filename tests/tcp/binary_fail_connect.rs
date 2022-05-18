use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::transport::tcp::{close::close_tcp, fork::fork_tcp, recv::recv_tcp, send::send_tcp};
use mpstthree::{choose_tcp, offer_tcp};

use std::error::Error;
use std::net::TcpStream;
use std::panic::set_hook;
use std::thread::{spawn, JoinHandle};

type Data = ((), [u8; 128]);

/////////////////////////
// Types
// A
#[derive(Debug)]
enum BinaryA {
    More(Recv<Data, Send<Data, RecursA>>),
    Done(End),
}

type RecursA = Recv<([u8; 128], BinaryA), End>;

// B
type RecursB = <RecursA as Session>::Dual;

// Functions
fn binary_a_to_b(s: RecursA, stream: TcpStream) -> Result<(), Box<dyn Error>> {
    offer_tcp!(s, {
        BinaryA::Done(s) => {
            close_tcp(s, stream, false)
        },
        BinaryA::More(s) => {
            let (_payload, s, data, _r, stream) = recv_tcp(s, stream, false)?;
            let (s, stream) = send_tcp((), &data, s, stream, false)?;
            binary_a_to_b(s, stream)
        },
    })
}

fn binary_b_to_a(
    s: Send<Data, Recv<Data, RecursB>>,
    stream: TcpStream,
    _index: i64,
) -> Result<RecursB, Box<dyn Error>> {
    let (s, stream) = send_tcp((), &[0_u8; 128], s, stream, true)?;
    let (_payload, s, _data, _r, _stream) = recv_tcp(s, stream, true)?;
    Ok(s)
}

fn tcp_client_aux(mut sessions: Vec<RecursB>, stream: TcpStream) -> Result<(), Box<dyn Error>> {
    for i in 0..LOOPS {
        let mut temp = Vec::new();

        for s in sessions {
            let copy_stream = stream.try_clone()?;
            let elt = binary_b_to_a(choose_tcp!(BinaryA::More, s, [0_u8; 128]), copy_stream, i)?;
            temp.push(elt);
        }

        sessions = temp;
    }

    let mut temp = Vec::<End>::new();

    for s in sessions {
        temp.push(choose_tcp!(BinaryA::Done, s, [0_u8; 128]));
    }

    for s in temp {
        let copy_stream = stream.try_clone()?;
        close_tcp(s, copy_stream, true)?;
    }

    Ok(())
}

fn tcp_client() -> Result<(), Box<dyn Error>> {
    let mut sessions = Vec::new();

    let (thread, s, stream): (JoinHandle<()>, RecursB, TcpStream) =
        fork_tcp(binary_a_to_b, "does_not_exist:3333")?;

    sessions.push(s);

    let aux = spawn(move || {
        set_hook(Box::new(|_info| {
            // do nothing
        }));
        match tcp_client_aux(sessions, stream) {
            Ok(()) => (),
            Err(e) => panic!("{:?}", e),
        }
    });

    match thread.join() {
        Ok(_) => match aux.join() {
            Ok(_) => Ok(()),
            Err(e) => {
                panic!("Error aux: {:?}", e)
            }
        },
        Err(e) => {
            panic!("Error client: {:?}", e)
        }
    }
}

static LOOPS: i64 = 5;

/////////////////////////

pub fn main() {
    let client = spawn(move || {
        set_hook(Box::new(|_info| {
            // do nothing
        }));
        match tcp_client() {
            Ok(()) => (),
            Err(e) => panic!("{:?}", e),
        }
    });

    assert!(client.join().is_err());
}
