use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::binary::*;
use mpstthree::choose;
use mpstthree::offer;

use rand::{thread_rng, Rng};

use std::boxed::Box;
use std::error::Error;
use std::marker;

enum SumOp<N: marker::Send + 'static> {
    More(Send<N, Recv<N, NiceSumServer<N>>>),
    MoreToo(Send<N, Recv<N, NiceSumServer<N>>>),
    Done(Send<N, End>),
}
type NiceSumServer<N> = Recv<SumOp<N>, End>;
type NiceSumClient<N> = <NiceSumServer<N> as Session>::Dual;

fn nice_sum_server(s: NiceSumServer<i32>) -> Result<(), Box<dyn Error>> {
    nice_sum_server_accum(s, 0)
}

fn nice_sum_server_accum(s: NiceSumServer<i32>, x: i32) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        SumOp::More(s) => {
            let s = send::send(0, s);
            let (y, s) = recv::recv(s)?;
            nice_sum_server_accum(s, x.wrapping_add(y))
        },
        SumOp::MoreToo(s) => {
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

fn nice_sum_client_accum(s: NiceSumClient<i32>, mut xs: Vec<i32>) -> Result<i32, Box<dyn Error>> {
    match xs.pop() {
        Option::Some(x) if x % 2 == 0 => {
            let s = choose!(SumOp::More, s);
            let (_, s) = recv::recv(s)?;
            let s = send::send(x, s);
            nice_sum_client_accum(s, xs)
        }
        Option::Some(x) => {
            let s = choose!(SumOp::MoreToo, s);
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

    assert!(|| -> Result<(), Box<dyn Error>> {
        let sum2 = nice_sum_client_accum(s, xs)?;
        assert_eq!(sum1, sum2);
        Ok(())
    }()
    .is_ok());

    assert!(other_thread.join().is_ok());
}
