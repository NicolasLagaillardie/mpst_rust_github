use rand::random;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            // let msg = b"Hello!";
            let msg = &[random::<u8>(); 128];

            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0_u8; 128]; // using 128 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!: {:?}", &msg);
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {text:?}");
                    }
                }
                Err(e) => {
                    println!("Failed to receive data: {e:?}");
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {e:?}");
        }
    }
    println!("Terminated.");
}
