use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::transport::tcp::{
    cancel::cancel_tcp, close::close_tcp, fork::fork_tcp, recv::recv_tcp, send::send_tcp,
};
use mpstthree::{choose_tcp, offer_tcp};

use std::error::Error;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
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
// A
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

// B
fn binary_b_to_a(
    s: Send<Data, Recv<Data, RecursB>>,
    stream: TcpStream,
    index: i64,
) -> Result<RecursB, Box<dyn Error>> {
    let (s, stream) = send_tcp((), &[0_u8; 128], s, stream, true)?;
    let (_payload, s, _data, _r, stream) = recv_tcp(s, stream, true)?;
    if index >= 2 {
        cancel_tcp(s, stream);
        panic!("Cancelling B");
    }

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
        fork_tcp(binary_a_to_b, "localhost:3333")?;

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

fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut data = [0_u8; 65535]; // using 50 byte buffer
    let mut index = 0;
    while match stream.read(&mut data) {
        Ok(0) => {
            stream.shutdown(Shutdown::Both).unwrap_or(());
            panic!("Buffer size: 0")
        }
        Ok(size) => {
            // echo everything!
            match stream.write(&data[0..size]) {
                Ok(0) => {
                    stream.shutdown(Shutdown::Both).unwrap_or(());
                    panic!("An error occurred during write")
                }
                _ => match index {
                    i if i + 1 >= LOOPS => {
                        stream.shutdown(Shutdown::Both).unwrap_or(());
                        index += 1;
                        false
                    }
                    _ => {
                        index += 1;
                        true
                    }
                },
            }
        }
        Err(_) => {
            stream.shutdown(Shutdown::Both).unwrap_or(());
            panic!("An error occurred during read")
        }
    } {}

    Ok(())
}

/////////////////////////

fn tcp_server(listener: TcpListener) -> Result<(), Box<dyn Error>> {
    'outer: for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream)?;
                break 'outer;
            }
            Err(e) => {
                panic!("Error server: {:?}", e);
                /* connection failed */
            }
        }
    }

    // close the socket server
    drop(listener);
    Ok(())
}

/////////////////////////

pub fn main() {
    let listener = TcpListener::bind("localhost:3333").unwrap();
    let server = spawn(move || {
        set_hook(Box::new(|_info| {
            // do nothing
        }));
        match tcp_server(listener) {
            Ok(()) => (),
            Err(e) => panic!("{:?}", e),
        }
    });
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
    assert!(server.join().is_err());
}
