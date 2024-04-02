use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::choose;
use mpstthree::offer;

use rand::{thread_rng, Rng};

// use std::boxed::Box;
use std::error::Error;
use std::marker;

enum SumOp<N: marker::Send + 'static> {
    More(Recv<N, Send<N, NiceSumServer<N>>>),
    MoreToo(Recv<N, Send<N, NiceSumServer<N>>>),
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
            let (y, s) = recv(s)?;
            let s = send(0, s);
            nice_sum_server_accum(s, x.wrapping_add(y))
        },
        SumOp::MoreToo(s) => {
            let (y, s) = recv(s)?;
            let s = send(0, s);
            nice_sum_server_accum(s, x.wrapping_add(y))
        },
        SumOp::Done(s) => {
            let s = send(x, s);
            close(s)
        },
    })
}

fn nice_sum_client_accum(s: NiceSumClient<i32>, mut xs: Vec<i32>) -> Result<i32, Box<dyn Error>> {
    match xs.pop() {
        Option::Some(x) if x % 2 == 0 => {
            let s = choose!(SumOp::More, s);
            let s = send(x, s);
            let (_, s) = recv(s)?;
            nice_sum_client_accum(s, xs)
        }
        Option::Some(x) => {
            let s = choose!(SumOp::MoreToo, s);
            let s = send(x, s);
            let (_, s) = recv(s)?;
            nice_sum_client_accum(s, xs)
        }
        Option::None => {
            let s = choose!(SumOp::Done, s);
            let (sum, s) = recv(s)?;
            close(s)?;
            Ok(sum)
        }
    }
}

pub fn main() {
    // Pick some random numbers.
    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();
    let sum1: i32 = xs.iter().fold(0, |sum, &x| sum.wrapping_add(x));

    let (other_thread, s) = fork_with_thread_id(nice_sum_server);

    assert!(|| -> Result<(), Box<dyn Error>> {
        let sum2 = nice_sum_client_accum(s, xs)?;
        assert_eq!(sum1, sum2);
        Ok(())
    }()
    .is_ok());

    assert!(other_thread.join().is_ok());
}
