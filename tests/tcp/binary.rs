use mpstthree::binary::{close_tcp, fork_tcp, recv_tcp, send_tcp, End, Recv, Send, Session};
use mpstthree::{choose_tcp, offer_tcp};

use std::error::Error;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;

type Data = ((), [u8; 128]);

/////////////////////////
// A
#[derive(Debug)]
enum BinaryA {
    More(Recv<Data, Send<Data, RecursA>>),
    Done(End),
}
type RecursA = Recv<([u8; 128], BinaryA), End>;
fn binary_a_to_b(s: RecursA, stream: &TcpStream) -> Result<(), Box<dyn Error>> {
    offer_tcp!(s, {
        BinaryA::Done(s) => {

            println!("Closing A");

            close_tcp(s, stream)?;

            println!("A closed");

            Ok(())
        },
        BinaryA::More(s) => {

            println!("A Receiving");

            let (_payload, s, _data, _r) = recv_tcp(s, stream, false)?;
            println!("A Sending");
            let s = send_tcp((), &[0_u8; 128], s, stream, false)?;
            println!("A Received");

            binary_a_to_b(s, stream)
        },
    })
}

// B
type RecursB = <RecursA as Session>::Dual;
fn binary_b_to_a(
    s: Send<Data, Recv<Data, RecursB>>,
    stream: &TcpStream,
) -> Result<RecursB, Box<dyn Error>> {
    println!("B Sending tcp");

    let s = send_tcp((), &[0_u8; 128], s, stream, true)?;

    println!("B Receiving tcp");

    let (_payload, s, _data, _r) = recv_tcp(s, stream, true)?;

    println!("B Received tcp");

    Ok(s)
}

fn tcp_client() -> Result<(), Box<dyn Error>> {
    println!("About to start");
    let mut sessions = Vec::new();

    let (thread, s, stream): (JoinHandle<()>, RecursB, TcpStream) =
        fork_tcp(binary_a_to_b, "localhost:3333")?;

    sessions.push(s);

    for _ in 0..SIZE {
        println!("About to send/recv");

        let mut temp = Vec::new();

        for s in sessions {
            temp.push(binary_b_to_a(choose_tcp!(BinaryA::More, s, [0_u8; 128]), &stream).unwrap());
        }

        sessions = temp;
    }

    let mut temp = Vec::<mpstthree::binary::End>::new();

    for s in sessions {
        temp.push(choose_tcp!(BinaryA::Done, s, [0_u8; 128]));
    }

    for s in temp {
        close_tcp(s, &stream).unwrap();
    }

    println!("Closed");

    match thread.join() {
        Ok(_) => {
            sleep(Duration::from_millis(1000));
            println!("Client ok");
            Ok(())
        }
        Err(e) => {
            println!("Error client: {:?}", e);
            panic!("Error client: {:?}", e)
        }
    }
}

static SIZE: i64 = 5;

/////////////////////////

fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut data = [0_u8; 65535]; // using 50 byte buffer
    let mut index = 0;
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size])?;
            match index {
                i if i >= SIZE => {
                    stream.shutdown(Shutdown::Both)?;
                    false
                }
                _ => {
                    sleep(Duration::from_millis(1000));
                    index += 1;
                    true
                }
            }
        }
        Err(_) => {
            stream.shutdown(Shutdown::Both)?;
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            panic!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            )
        }
    } {}

    println!("Handle client ok");

    Ok(())
}

fn tcp_server() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new
    // thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream)?;
            }
            Err(e) => {
                println!("Error server: {}", e);
                panic!("Error server: {}", e);
                /* connection failed */
            }
        }
        break;
    }

    // close the socket server
    drop(listener);

    println!("Server ok");

    Ok(())
}

/////////////////////////

pub fn main() {
    let server = spawn(move || {
        std::panic::set_hook(Box::new(|_info| {
            // do nothing
        }));
        match tcp_server() {
            Ok(()) => (),
            Err(e) => panic!("{:?}", e),
        }
    });
    let client = spawn(move || {
        std::panic::set_hook(Box::new(|_info| {
            // do nothing
        }));
        match tcp_client() {
            Ok(()) => (),
            Err(e) => panic!("{:?}", e),
        }
    });

    assert!(client.join().is_ok());
    assert!(server.join().is_ok());
}
