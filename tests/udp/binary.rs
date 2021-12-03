use mpstthree::binary::close::close;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::transport::udp::{fork::fork_udp, recv::recv_udp, send::send_udp};
use mpstthree::{choose_udp, offer_udp};

use std::error::Error;
use std::net::UdpSocket;
use std::panic::set_hook;
use std::thread::{spawn, JoinHandle};

type Data = ((), [u8; 128]);

static LOOPS: i64 = 5;

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
fn binary_a_to_b(s: RecursA, socket: UdpSocket) -> Result<(), Box<dyn Error>> {
    offer_udp!(s, {
        BinaryA::Done(s) => {
            close(s)
        },
        BinaryA::More(s) => {
            let (_payload, s, data, _r, socket) = recv_udp(s, socket, false)?;
            let (s, _data, socket) = send_udp((), &data, s, socket, false)?;
            binary_a_to_b(s, socket)
        },
    })
}

fn binary_b_to_a(
    s: Send<Data, Recv<Data, RecursB>>,
    socket: UdpSocket,
) -> Result<RecursB, Box<dyn Error>> {
    let (s, _data, socket) = send_udp((), &[0_u8; 128], s, socket, true)?;
    let (_payload, s, _data, _r, _socket) = recv_udp(s, socket, true)?;
    Ok(s)
}

fn udp_client_aux(mut sessions: Vec<RecursB>, socket: UdpSocket) -> Result<(), Box<dyn Error>> {
    for _ in 0..LOOPS {
        let mut temp = Vec::new();

        for s in sessions {
            let copy_stream = socket.try_clone()?;
            let elt = binary_b_to_a(choose_udp!(BinaryA::More, s, [0_u8; 128]), copy_stream)?;
            temp.push(elt);
        }

        sessions = temp;
    }

    let mut temp = Vec::<End>::new();

    for s in sessions {
        temp.push(choose_udp!(BinaryA::Done, s, [0_u8; 128]));
    }

    for s in temp {
        close(s)?;
    }

    Ok(())
}

fn udp_client() -> Result<(), Box<dyn Error>> {
    let mut sessions = Vec::new();

    let (thread, s, socket): (JoinHandle<()>, RecursB, UdpSocket) =
        fork_udp(binary_a_to_b, "127.0.0.1:8081", "127.0.0.1:8080")?;

    sessions.push(s);

    let aux = spawn(move || {
        set_hook(Box::new(|_info| {
            // do nothing
        }));
        match udp_client_aux(sessions, socket) {
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

/////////////////////////

fn udp_server() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;

    for _ in 0..LOOPS {
        let mut buf = [0; 128];
        let _ = socket.recv_from(&mut buf)?;
        let _ = socket.send_to(&[0, 1, 2], "127.0.0.1:8081")?;
    }

    // close the socket server
    drop(socket);
    Ok(())
}

/////////////////////////

pub fn main() {
    let client = spawn(move || {
        set_hook(Box::new(|_info| {
            // do nothing
        }));
        match udp_client() {
            Ok(()) => (),
            Err(e) => panic!("{:?}", e),
        }
    });

    let server = spawn(move || {
        set_hook(Box::new(|_info| {
            // do nothing
        }));
        match udp_server() {
            Ok(()) => (),
            Err(e) => panic!("{:?}", e),
        }
    });

    assert!(client.join().is_ok());
    assert!(server.join().is_ok());
}
