use mpstthree::binary::{close_tcp, fork_tcp, recv_tcp, send_tcp, End, Recv, Send, Session};
use mpstthree::{choose_tcp, offer_tcp};

use std::error::Error;
use std::net::TcpStream;
use std::thread::{spawn, JoinHandle};

type Data = ((), [u8; 128]);
/////////////////////////
// A
#[derive(Debug)]
enum BinaryA {
    More(Recv<Data, Send<Data, RecursA>>),
    Done(End),
}
type RecursA = Recv<(BinaryA, [u8; 128]), End>;
fn binary_a_to_b(s: RecursA, stream: &TcpStream) -> Result<(), Box<dyn Error>> {
    offer_tcp!(s, {
        BinaryA::Done(s) => {

            println!("Closing A");

            close_tcp(s, stream)?;

            println!("A closed");

            Ok(())
        },
        BinaryA::More(s) => {

            println!("A Receiving tcp");

            let (_payload, s, _data, _r) = recv_tcp(s, stream, false)?;
            println!("A Sending tcp");
            let s = send_tcp((), &[0_u8; 128], s, stream, false)?;
            println!("A Received tcp");

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

fn all_binaries() -> Result<(), Box<dyn Error>> {
    let mut threads = Vec::new();
    let mut sessions = Vec::new();
    let mut streams = Vec::new();

    println!("About to start");

    for _ in 0..1 {
        let (thread, s, stream): (JoinHandle<()>, RecursB, TcpStream) =
            fork_tcp(binary_a_to_b, "localhost:3333")?;

        threads.push(thread);
        sessions.push(s);
        streams.push(stream);
    }

    let main = spawn(move || {
        for _ in 0..SIZE {
            let mut temp = Vec::new();
            let mut index = 0;

            println!("About to send/recv");

            for s in sessions {
                temp.push(
                    binary_b_to_a(choose_tcp!(BinaryA::More, s, [0_u8; 128]), &streams[index])
                        .unwrap(),
                );

                index += 1;
            }

            sessions = temp;
        }

        let mut temp = Vec::<mpstthree::binary::End>::new();
        let mut index = 0;

        for s in sessions {
            println!("About to choose close : {}", &index);

            temp.push(choose_tcp!(BinaryA::Done, s, [0_u8; 128]));

            index += 1;
        }

        index = 0;

        for s in temp {
            println!("About to close : {}", &index);

            close_tcp(s, &streams[index]).unwrap();
            index += 1;
        }

        println!("Closed");

        threads.into_iter().for_each(|elt| elt.join().unwrap());
    });

    main.join().unwrap();

    Ok(())
}

/////////////////////////

static SIZE: i64 = 5;

fn main() {
    all_binaries().unwrap();
}
