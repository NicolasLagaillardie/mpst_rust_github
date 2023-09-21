use mpstthree::binary::cancel::cancel;
use mpstthree::binary::choose::*;
use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::offer::*;
use mpstthree::binary::recv::recv;
use mpstthree::binary::select::select_mut;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::end::*;
use mpstthree::binary::struct_trait::recv::*;
use mpstthree::binary::struct_trait::send::*;
use mpstthree::binary::struct_trait::session::*;
use mpstthree::choose;
use mpstthree::offer;

use rand::{thread_rng, Rng};
use std::boxed::Box;
use std::error::Error;
use std::marker;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;

pub fn head_str() {
    assert_eq!(End::head_str(), "End".to_string());
    assert_eq!(Send::<i32, End>::head_str(), "Send".to_string());
    assert_eq!(Recv::<i32, End>::head_str(), "Recv".to_string());
}

pub fn tail_str() {
    assert!(End::tail_str().is_empty());
    assert_eq!(Send::<i32, End>::tail_str(), "End<>".to_string());
    assert_eq!(Recv::<i32, End>::tail_str(), "End<>".to_string());
}

pub fn self_head_str() {
    let (end_here, end_there) = End::new();
    assert_eq!(end_here.self_head_str(), "End".to_string());
    assert_eq!(end_there.self_head_str(), "End".to_string());

    let (send, recv) = Send::<i32, End>::new();
    assert_eq!(send.self_head_str(), "Send".to_string());
    assert_eq!(recv.self_head_str(), "Recv".to_string());
}

pub fn self_tail_str() {
    let (end_here, end_there) = End::new();
    assert!(end_here.self_tail_str().is_empty());
    assert!(end_there.self_tail_str().is_empty());

    let (send, recv) = Send::<i32, End>::new();
    assert_eq!(send.self_tail_str(), "End<>".to_string());
    assert_eq!(recv.self_tail_str(), "End<>".to_string());
}

pub fn new_types() {
    let (session_end_1, session_end_2) = End::new();

    assert!({
        match session_end_1.sender.send(Signal::Stop) {
            Ok(()) => Ok::<(), Box<dyn Error>>(()),
            _ => unreachable!(),
        }
    }
    .is_ok());

    assert!({
        match session_end_2.sender.send(Signal::Stop) {
            Ok(()) => Ok::<(), Box<dyn Error>>(()),
            _ => unreachable!(),
        }
    }
    .is_ok());

    assert!({
        match session_end_1.receiver.recv() {
            Ok(Signal::Stop) => Ok::<(), Box<dyn Error>>(()),
            _ => unreachable!(),
        }
    }
    .is_ok());

    assert!({
        match session_end_2.receiver.recv() {
            Ok(Signal::Stop) => Ok::<(), Box<dyn Error>>(()),
            _ => unreachable!(),
        }
    }
    .is_ok());
}

pub fn new_types_cancel() {
    let (session_end_1, session_end_2) = End::new();

    assert!({
        match session_end_1.sender.send(Signal::Cancel) {
            Ok(()) => Ok::<(), Box<dyn Error>>(()),
            _ => unreachable!(),
        }
    }
    .is_ok());

    assert!({
        match session_end_2.sender.send(Signal::Cancel) {
            Ok(()) => Ok::<(), Box<dyn Error>>(()),
            _ => unreachable!(),
        }
    }
    .is_ok());

    assert!({
        match session_end_1.receiver.recv() {
            Ok(Signal::Cancel) => Ok::<(), Box<dyn Error>>(()),
            _ => unreachable!(),
        }
    }
    .is_ok());

    assert!({
        match session_end_2.receiver.recv() {
            Ok(Signal::Cancel) => Ok::<(), Box<dyn Error>>(()),
            _ => unreachable!(),
        }
    }
    .is_ok());
}

// Test sending a ping across threads.

pub fn ping_works() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        let s = fork(move |s: Send<(), End>| {
            let s = send((), s);
            close(s)
        });
        let ((), s) = recv(s)?;
        close(s)
    }()
    .is_ok());
}

// Test writing a program which duplicates a session.
//
// ```compile_fail
// assert!({
//     let r1 = fork(move |s1: Se
//         let s2 = se
//         close(s2)?;

//         let s3 =
//
//     });

//     let ((),
//
// }
// .is_ok());
/// ```

// Test a simple calculator server, implemented using binary
// choice.

type NegServer<N> = Recv<N, Send<N, End>>;
type NegClient<N> = <NegServer<N> as Session>::Dual;

type AddServer<N> = Recv<N, Recv<N, Send<N, End>>>;
type AddClient<N> = <AddServer<N> as Session>::Dual;

type SimpleCalcServer<N> = Offer<NegServer<N>, AddServer<N>>;
type SimpleCalcClient<N> = <SimpleCalcServer<N> as Session>::Dual;

fn simple_calc_server(s: SimpleCalcServer<i32>) -> Result<(), Box<dyn Error>> {
    offer_either(
        s,
        |s: NegServer<i32>| {
            let (x, s) = recv(s)?;
            let s = send(-x, s);
            close(s)
        },
        |s: AddServer<i32>| {
            let (x, s) = recv(s)?;
            let (y, s) = recv(s)?;
            let s = send(x.wrapping_add(y), s);
            close(s)
        },
    )
}

pub fn simple_calc_works() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        let mut rng = thread_rng();

        // Test the negation function.
        {
            let s: SimpleCalcClient<i32> = fork(simple_calc_server);
            let x: i32 = rng.gen();
            let s = choose_left::<_, AddClient<i32>>(s);
            let s = send(x, s);
            let (y, s) = recv(s)?;
            close(s)?;
            assert_eq!(-x, y);
        }

        // Test the addition function.
        {
            let s: SimpleCalcClient<i32> = fork(simple_calc_server);
            let x: i32 = rng.gen();
            let y: i32 = rng.gen();
            let s = choose_right::<NegClient<i32>, _>(s);
            let s = send(x, s);
            let s = send(y, s);
            let (z, s) = recv(s)?;
            close(s)?;
            assert_eq!(x.wrapping_add(y), z);
        }

        Ok(())
    }()
    .is_ok());
}

// Test a nice calculator server, implemented using variant
// types.

enum CalcOp<N: marker::Send> {
    Neg(NegServer<N>),
    Add(AddServer<N>),
}
type NiceCalcServer<N> = Recv<CalcOp<N>, End>;
type NiceCalcClient<N> = <NiceCalcServer<N> as Session>::Dual;

fn nice_calc_server(s: NiceCalcServer<i32>) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        CalcOp::Neg(s) => {
            let (x, s) = recv(s)?;
            let s = send(-x, s);
            close(s)
        },
        CalcOp::Add(s) => {
            let (x, s) = recv(s)?;
            let (y, s) = recv(s)?;
            let s = send(x.wrapping_add(y), s);
            close(s)
        },
    })
}

pub fn nice_calc_works() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Pick some random numbers.
        let mut rng = thread_rng();

        // Test the negation function.
        {
            let s: NiceCalcClient<i32> = fork(nice_calc_server);
            let x: i32 = rng.gen();
            let s = choose!(CalcOp::Neg, s);
            let s = send(x, s);
            let (y, s) = recv(s)?;
            close(s)?;
            assert_eq!(-x, y);
        }

        // Test the addition function.
        {
            let s: NiceCalcClient<i32> = fork(nice_calc_server);
            let x: i32 = rng.gen();
            let y: i32 = rng.gen();
            let s = choose!(CalcOp::Add, s);
            let s = send(x, s);
            let s = send(y, s);
            let (z, s) = recv(s)?;
            close(s)?;
            assert_eq!(x.wrapping_add(y), z);
        }

        Ok(())
    }()
    .is_ok());
}

// Test cancellation.

pub fn cancel_recv_works() {
    let (other_thread, s) = fork_with_thread_id(nice_calc_server);

    assert!({
        cancel(s);
        Ok::<(), Box<dyn Error>>(())
    }
    .is_ok());

    assert!(other_thread.join().is_err());
}

pub fn cancel_send_works() {
    let (other_thread, s) = fork_with_thread_id(move |s: Recv<(), End>| {
        cancel(s);
        Ok(())
    });

    assert!({
        let s = send((), s);
        close(s)
    }
    .is_err());

    assert!(other_thread.join().is_ok());
}

// Test cancellation of delegation.

pub fn delegation_works() {
    let (other_thread1, s) = fork_with_thread_id(nice_calc_server);
    let (other_thread2, u) = fork_with_thread_id(move |u: Recv<NiceCalcClient<i32>, End>| {
        cancel(u);
        Ok(())
    });

    assert!({
        let u = send(s, u);
        close(u)
    }
    .is_err());

    assert!(other_thread1.join().is_err());
    assert!(other_thread2.join().is_ok());
}

// Test cancellation of closures.

pub fn closure_works() {
    let (other_thread, s) = fork_with_thread_id(nice_calc_server);

    assert!({
        // Create a closure which uses the session.
        let _f = move |x: i32| -> Result<i32, Box<dyn Error>> {
            let s = choose!(CalcOp::Neg, s);
            let s = send(x, s);
            let (y, s) = recv(s)?;
            close(s)?;
            Ok(y)
        };

        // Let the closure go out of scope.
        Err::<i32, Box<mpsc::RecvError>>(Box::new(mpsc::RecvError))
        // f(5)
    }
    .is_err());

    assert!(other_thread.join().is_err());
}

// Test recursive sessions.

enum SumOp<N: marker::Send> {
    More(Recv<N, NiceSumServer<N>>),
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
        Option::Some(x) => {
            let s = choose!(SumOp::More, s);
            let s = send(x, s);
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

pub fn recursion_works() {
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

pub fn cancel_recursion() {
    // Pick some random numbers.
    let mut rng = thread_rng();
    let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();
    let _sum1: i32 = xs.iter().fold(0, |sum, &x| sum.wrapping_add(x));

    let (other_thread, s) = fork_with_thread_id(nice_sum_server);

    assert!({
        cancel(s);
        Ok::<(), Box<dyn Error>>(())
    }
    .is_ok());

    assert!(other_thread.join().is_err());
}

// Test selection.

pub fn selection_works() {
    let mut other_threads = Vec::new();
    let mut rs = Vec::new();

    for i in 0..10 {
        let (other_thread, s) = fork_with_thread_id(move |s: Send<u64, End>| {
            sleep(Duration::from_millis(i * 1000));
            let s = send(9 - i, s);
            close(s)
        });
        other_threads.push(other_thread);
        rs.push(s);
    }

    assert!(
        || -> Result<(), Box<dyn Error>> {
            let mut current_index = 9;
            loop {
                if rs.is_empty() {
                    break Ok(());
                } else {
                    let (i, r) = select_mut(&mut rs)?;
                    close(r)?;
                    assert_eq!(current_index, i, "Messages were received out of order.");
                    current_index = current_index.overflowing_sub(1).0;
                    // decrement
                }
            }
        }()
        .is_ok(),
        "Main thread crashed."
    );

    for other_thread in other_threads {
        let msg = format!("Thread {other_thread:?} crashed.");
        assert!(other_thread.join().is_ok(), "{}", msg);
    }
}
