use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::binary::*;
use mpstthree::choose;
use mpstthree::offer;

use rand::{thread_rng, Rng};

use std::boxed::Box;
use std::error::Error;

enum SumOp {
    More(Send<i32, Recv<i32, Send<i32, Recv<i32, Recv<Self, End>>>>>),
    Done(Send<i32, End>),
}
type NiceSumServer = Recv<SumOp, End>;
type NiceSumClient = <NiceSumServer as Session>::Dual;

fn nice_sum_server(s: NiceSumServer) -> Result<(), Box<dyn Error>> {
    nice_sum_server_accum(s, 0)
}

fn nice_sum_server_accum(s: NiceSumServer, x: i32) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        SumOp::More(s) => {
            let s = send::send(0, s);
            let (y, s) = recv::recv(s)?;
            let s = send::send(0, s);
            let (y, s) = recv::recv(s)?;
            nice_sum_server_accum(s, x.wrapping_add(y))
        },
        SumOp::Done(s) => {
            let s = send::send(x, s);
            close::close(s)
        },
    })?;
    Ok(())
}

fn nice_sum_client_accum(s: NiceSumClient, mut xs: Vec<i32>) -> Result<i32, Box<dyn Error>> {
    match xs.pop() {
        Option::Some(x) => {
            let s = choose!(SumOp::More, s);
            let (_, s) = recv::recv(s)?;
            let s = send::send(x, s);
            let (_, s) = recv::recv(s)?;
            let s = send::send(x, s);
            nice_sum_client_accum(s, xs)
        }
        Option::None => {
            let s = choose!(SumOp::Done, s);
            let (sum, s) = recv::recv(s)?;
            close::close(s)?;
            Ok(sum)
        }
    }
}

fn main() {
    // Pick some random numbers.
    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();
    let sum1: i32 = xs.iter().fold(0, |sum, &x| sum.wrapping_add(x));

    let (other_thread, s) = fork::fork_with_thread_id(nice_sum_server);

    assert!({
        let sum2 = nice_sum_client_accum(s, xs)?;
        assert_eq!(sum1, sum2);
        Ok(())
    }
    .is_ok());

    assert!(other_thread.join().is_ok());
}
