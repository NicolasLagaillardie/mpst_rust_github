// use mpstthree::binary::cancel::cancel;
// use mpstthree::binary::choose::*;
use mpstthree::binary::close::close;
// use mpstthree::binary::offer::*;
// use mpstthree::binary::recv::recv;
// use mpstthree::binary::select::select_mut;
// use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::end::*;
// use mpstthree::binary::struct_trait::recv::*;
// use mpstthree::binary::struct_trait::send::*;
use mpstthree::binary::struct_trait::session::*;
// use mpstthree::timed_choose;
// use mpstthree::timed_offer;

use mpstthree::binary_timed::choose::{choose_left, choose_right};
use mpstthree::binary_timed::fork::fork;
use mpstthree::binary_timed::offer::offer_either;
use mpstthree::binary_timed::offer::OfferTimed;
use mpstthree::binary_timed::recv::recv;
use mpstthree::binary_timed::send::send;
use mpstthree::binary_timed::struct_trait::recv::RecvTimed;
use mpstthree::binary_timed::struct_trait::send::SendTimed;

use rand::{thread_rng, Rng};
use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;
// use std::marker;
// use std::mem;
// use std::sync::mpsc;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn head_str() {
    assert_eq!(
        SendTimed::<i32, End, 'a', 0, true, 5, true, false>::head_str(),
        "Send".to_string()
    );
    assert_eq!(
        RecvTimed::<i32, End, 'a', 0, true, 5, true, false>::head_str(),
        "Recv".to_string()
    );
}

pub fn tail_str() {
    assert_eq!(
        SendTimed::<i32, End, 'a', 0, true, 5, true, false>::tail_str(),
        "End<>".to_string()
    );
    assert_eq!(
        RecvTimed::<i32, End, 'a', 0, true, 5, true, false>::tail_str(),
        "End<>".to_string()
    );
}

pub fn self_head_str() {
    let (send, recv) = SendTimed::<i32, End, 'a', 0, true, 5, true, false>::new();
    assert_eq!(send.self_head_str(), "Send".to_string());
    assert_eq!(recv.self_head_str(), "Recv".to_string());
}

pub fn self_tail_str() {
    let (send, recv) = SendTimed::<i32, End, 'a', 0, true, 5, true, false>::new();
    assert_eq!(send.self_tail_str(), "End<>".to_string());
    assert_eq!(recv.self_tail_str(), "End<>".to_string());
}

// Constraints

pub fn constraint_start_excluded() {
    let (send, recv) = SendTimed::<i32, End, 'a', 5, false, -5, true, false>::new();
    assert_eq!(send.constraint(), "5 < a".to_string());
    assert_eq!(recv.constraint(), "5 < a".to_string());
}

pub fn constraint_start_included() {
    let (send, recv) = SendTimed::<i32, End, 'a', 5, true, -5, true, false>::new();
    assert_eq!(send.constraint(), "5 <= a".to_string());
    assert_eq!(recv.constraint(), "5 <= a".to_string());
}

pub fn constraint_end_excluded() {
    let (send, recv) = SendTimed::<i32, End, 'a', -5, true, 5, false, false>::new();
    assert_eq!(send.constraint(), "a < 5".to_string());
    assert_eq!(recv.constraint(), "a < 5".to_string());
}

pub fn constraint_end_included() {
    let (send, recv) = SendTimed::<i32, End, 'a', -5, true, 5, true, false>::new();
    assert_eq!(send.constraint(), "a <= 5".to_string());
    assert_eq!(recv.constraint(), "a <= 5".to_string());
}

pub fn constraint_start_excluded_end_excluded() {
    let (send, recv) = SendTimed::<i32, End, 'a', 5, false, 10, false, false>::new();
    assert_eq!(send.constraint(), "5 < a < 10".to_string());
    assert_eq!(recv.constraint(), "5 < a < 10".to_string());
}

pub fn constraint_start_excluded_end_included() {
    let (send, recv) = SendTimed::<i32, End, 'a', 5, false, 10, true, false>::new();
    assert_eq!(send.constraint(), "5 < a <= 10".to_string());
    assert_eq!(recv.constraint(), "5 < a <= 10".to_string());
}

pub fn constraint_start_included_end_excluded() {
    let (send, recv) = SendTimed::<i32, End, 'a', 5, true, 10, false, false>::new();
    assert_eq!(send.constraint(), "5 <= a < 10".to_string());
    assert_eq!(recv.constraint(), "5 <= a < 10".to_string());
}

pub fn constraint_start_included_end_included() {
    let (send, recv) = SendTimed::<i32, End, 'a', 5, true, 10, true, false>::new();
    assert_eq!(send.constraint(), "5 <= a <= 10".to_string());
    assert_eq!(recv.constraint(), "5 <= a <= 10".to_string());
}

// Test a simple calculator server, implemented using timed binary
// choice.

type NegServer =
    RecvTimed<i32, SendTimed<i32, End, 'a', 4, true, 6, true, false>, 'a', 2, true, 4, true, false>;
type NegClient = <NegServer as Session>::Dual;

type AddServer = RecvTimed<
    i32,
    RecvTimed<i32, SendTimed<i32, End, 'a', 6, true, 8, true, false>, 'a', 4, true, 6, true, false>,
    'a',
    2,
    true,
    4,
    true,
    false,
>;
type AddClient = <AddServer as Session>::Dual;

type SimpleCalcServer = OfferTimed<NegServer, AddServer, 'a', 0, true, 2, true, false>;
type SimpleCalcClient = <SimpleCalcServer as Session>::Dual;

fn simple_calc_server(
    s: SimpleCalcServer,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_either(
        all_clocks,
        s,
        |all_clocks: &mut HashMap<char, Instant>, s: NegServer| {
            sleep(Duration::from_secs(3));
            let (x, s) = recv(all_clocks, s)?;
            sleep(Duration::from_secs(2));
            let s = send(-x, all_clocks, s)?;
            sleep(Duration::from_secs(2));
            close(s)
        },
        |all_clocks: &mut HashMap<char, Instant>, s: AddServer| {
            sleep(Duration::from_secs(3));
            let (x, s) = recv(all_clocks, s)?;
            sleep(Duration::from_secs(2));
            let (y, s) = recv(all_clocks, s)?;
            sleep(Duration::from_secs(2));
            let s = send(x.wrapping_add(y), all_clocks, s)?;
            sleep(Duration::from_secs(2));
            close(s)
        },
    )
}

pub fn simple_calc_works() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        let mut rng = thread_rng();

        // Test the negation function.
        {
            let s: SimpleCalcClient = fork(simple_calc_server);

            let x: i32 = rng.gen();

            let mut all_clocks = HashMap::<char, Instant>::new();
            all_clocks.insert('a', Instant::now());

            let s = choose_left::<_, AddClient, 'a', 0, true, 2, true, false>(&mut all_clocks, s)?;
            sleep(Duration::from_secs(3));
            let s = send(x, &mut all_clocks, s)?;
            sleep(Duration::from_secs(2));
            let (y, s) = recv(&mut all_clocks, s)?;
            sleep(Duration::from_secs(2));
            close(s)?;

            assert_eq!(-x, y);
        }

        // Test the addition function.
        {
            let s: SimpleCalcClient = fork(simple_calc_server);

            let x: i32 = rng.gen();
            let y: i32 = rng.gen();

            let mut all_clocks = HashMap::<char, Instant>::new();
            all_clocks.insert('a', Instant::now());

            let s = choose_right::<NegClient, _, 'a', 0, true, 2, true, false>(&mut all_clocks, s)?;

            sleep(Duration::from_secs(3));
            let s = send(x, &mut all_clocks, s)?;
            sleep(Duration::from_secs(2));
            let s = send(y, &mut all_clocks, s)?;
            sleep(Duration::from_secs(2));
            let (z, s) = recv(&mut all_clocks, s)?;
            sleep(Duration::from_secs(2));
            close(s)?;

            assert_eq!(x.wrapping_add(y), z);
        }

        Ok(())
    }()
    .is_ok());
}

// // Test a nice calculator server, implemented using variant
// // types.

// enum CalcOp<N: marker::Send> {
//     Neg(NegServer),
//     Add(AddServer),
// }
// type NiceCalcServer = Recv<CalcOp, End>;
// type NiceCalcClient = <NiceCalcServer as Session>::Dual;

// fn nice_calc_server(s: NiceCalcServer<i32>) -> Result<(), Box<dyn Error>> {
//     timed_offer!(s, {
//         CalcOp::Neg(s) => {
//             let (x, s) = recv(s)?;
//             let s = send(-x, s);
//             close(s)
//         },
//         CalcOp::Add(s) => {
//             let (x, s) = recv(s)?;
//             let (y, s) = recv(s)?;
//             let s = send(x.wrapping_add(y), s);
//             close(s)
//         },
//     })
// }

// pub fn nice_calc_works() {
//     assert!(|| -> Result<(), Box<dyn Error>> {
//         // Pick some random numbers.
//         let mut rng = thread_rng();

//         // Test the negation function.
//         {
//             let s: NiceCalcClient<i32> = fork(nice_calc_server);
//             let x: i32 = rng.gen();
//             let s = timed_choose!(CalcOp::Neg, s);
//             let s = send(x, s);
//             let (y, s) = recv(s)?;
//             close(s)?;
//             assert_eq!(-x, y);
//         }

//         // Test the addition function.
//         {
//             let s: NiceCalcClient<i32> = fork(nice_calc_server);
//             let x: i32 = rng.gen();
//             let y: i32 = rng.gen();
//             let s = timed_choose!(CalcOp::Add, s);
//             let s = send(x, s);
//             let s = send(y, s);
//             let (z, s) = recv(s)?;
//             close(s)?;
//             assert_eq!(x.wrapping_add(y), z);
//         }

//         Ok(())
//     }()
//     .is_ok());
// }

// // Test cancellation.

// pub fn cancel_recv_works() {
//     let (other_thread, s) = fork_with_thread_id(nice_calc_server);

//     assert!(|| -> Result<(), Box<dyn Error>> {
//         cancel(s);
//         Ok(())
//     }()
//     .is_ok());

//     assert!(other_thread.join().is_err());
// }

// pub fn cancel_send_works() {
//     let (other_thread, s) = fork_with_thread_id(move |s: Recv<(), End>| {
//         cancel(s);
//         Ok(())
//     });

//     assert!(|| -> Result<(), Box<dyn Error>> {
//         let s = send((), s);
//         close(s)
//     }()
//     .is_err());

//     assert!(other_thread.join().is_ok());
// }

// // Test cancellation of delegation.

// pub fn delegation_works() {
//     let (other_thread1, s) = fork_with_thread_id(nice_calc_server);
//     let (other_thread2, u) = fork_with_thread_id(move |u: Recv<NiceCalcClient<i32>, End>| {
//         cancel(u);
//         Ok(())
//     });

//     assert!(|| -> Result<(), Box<dyn Error>> {
//         let u = send(s, u);
//         close(u)
//     }()
//     .is_err());

//     assert!(other_thread1.join().is_err());
//     assert!(other_thread2.join().is_ok());
// }

// // Test cancellation of closures.

// pub fn closure_works() {
//     let (other_thread, s) = fork_with_thread_id(nice_calc_server);

//     assert!(|| -> Result<i32, Box<dyn Error>> {
//         // Create a closure which uses the session.
//         let _f = move |x: i32| -> Result<i32, Box<dyn Error>> {
//             let s = timed_choose!(CalcOp::Neg, s);
//             let s = send(x, s);
//             let (y, s) = recv(s)?;
//             close(s)?;
//             Ok(y)
//         };

//         // Let the closure go out of scope.
//         Err(Box::new(mpsc::RecvError))
//         // f(5)
//     }()
//     .is_err());

//     assert!(other_thread.join().is_err());
// }

// // Test recursive sessions.

// enum SumOp<N: marker::Send> {
//     More(Recv<i32,  NiceSumServer>),
//     Done(Send<i32,  End>),
// }
// type NiceSumServer = Recv<SumOp, End>;
// type NiceSumClient = <NiceSumServer as Session>::Dual;

// fn nice_sum_server(s: NiceSumServer<i32>) -> Result<(), Box<dyn Error>> {
//     nice_sum_server_accum(s, 0)
// }

// fn nice_sum_server_accum(s: NiceSumServer<i32>, x: i32) -> Result<(), Box<dyn Error>> {
//     timed_offer!(s, {
//         SumOp::More(s) => {
//             let (y, s) = recv(s)?;
//             nice_sum_server_accum(s, x.wrapping_add(y))
//         },
//         SumOp::Done(s) => {
//             let s = send(x, s);
//             close(s)
//         },
//     })
// }

// fn nice_sum_client_accum(s: NiceSumClient<i32>, mut xs: Vec<i32>) -> Result<i32, Box<dyn Error>> {
//     match xs.pop() {
//         Option::Some(x) => {
//             let s = timed_choose!(SumOp::More, s);
//             let s = send(x, s);
//             nice_sum_client_accum(s, xs)
//         }
//         Option::None => {
//             let s = timed_choose!(SumOp::Done, s);
//             let (sum, s) = recv(s)?;
//             close(s)?;
//             Ok(sum)
//         }
//     }
// }

// pub fn recursion_works() {
//     // Pick some random numbers.
//     let mut rng = thread_rng();
//     let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();
//     let sum1: i32 = xs.iter().fold(0, |sum, &x| sum.wrapping_add(x));

//     let (other_thread, s) = fork_with_thread_id(nice_sum_server);

//     assert!(|| -> Result<(), Box<dyn Error>> {
//         let sum2 = nice_sum_client_accum(s, xs)?;
//         assert_eq!(sum1, sum2);
//         Ok(())
//     }()
//     .is_ok());

//     assert!(other_thread.join().is_ok());
// }

// pub fn cancel_recursion() {
//     // Pick some random numbers.
//     let mut rng = thread_rng();
//     let xs: Vec<i32> = (1..100).map(|_| rng.gen()).collect();
//     let _sum1: i32 = xs.iter().fold(0, |sum, &x| sum.wrapping_add(x));

//     let (other_thread, s) = fork_with_thread_id(nice_sum_server);

//     assert!(|| -> Result<(), Box<dyn Error>> {
//         cancel(s);
//         Ok(())
//     }()
//     .is_ok());

//     assert!(other_thread.join().is_err());
// }

// // Test selection.

// pub fn selection_works() {
//     let mut other_threads = Vec::new();
//     let mut rs = Vec::new();

//     for i in 0..10 {
//         let (other_thread, s) = fork_with_thread_id(move |s: Send<u64, End>| {
//             sleep(Duration::from_millis(i * 1000));
//             let s = send(9 - i, s);
//             close(s)
//         });
//         other_threads.push(other_thread);
//         rs.push(s);
//     }

//     assert!(
//         || -> Result<(), Box<dyn Error>> {
//             let mut current_index = 9;
//             loop {
//                 if rs.is_empty() {
//                     break Ok(());
//                 } else {
//                     let (i, r) = select_mut(&mut rs)?;
//                     close(r)?;
//                     assert_eq!(current_index, i, "Messages were received out of order.");
//                     current_index = current_index.overflowing_sub(1).0;
//                     // decrement
//                 }
//             }
//         }()
//         .is_ok(),
//         "Main thread crashed."
//     );

//     for other_thread in other_threads {
//         let msg = format!("Thread {:?} crashed.", other_thread);
//         assert!(other_thread.join().is_ok(), "{}", msg);
//     }
// }

// #[allow(dead_code)]
// fn deadlock_loop() {
//     let s = fork(move |s: Send<(), End>| {
//         loop {
//             // Let's trick the reachability checker
//             if false {
//                 break;
//             }
//         }
//         let s = send((), s);
//         close(s)
//     });

//     || -> Result<(), Box<dyn Error>> {
//         let ((), s) = recv(s)?;
//         close(s)
//     }()
//     .unwrap();
// }

// #[allow(dead_code)]
// fn deadlock_forget() {
//     let s = fork(move |s: Send<(), End>| {
//         mem::forget(s);
//         Ok(())
//     });

//     || -> Result<(), Box<dyn Error>> {
//         let ((), s) = recv(s)?;
//         close(s)
//     }()
//     .unwrap();
// }

// #[allow(dead_code)]
// fn deadlock_new() {
//     let (s1, r1) = <Send<(), End>>::new();
//     let r2 = fork(move |s2: Send<(), End>| {
//         let (x, r1) = recv(r1)?;
//         let s2 = send(x, s2);
//         close(r1)?;
//         close(s2)
//     });

//     || -> Result<(), Box<dyn Error>> {
//         let (x, r2) = recv(r2)?;
//         let s1 = send(x, s1);
//         close(r2)?;
//         close(s1)
//     }()
//     .unwrap();
// }
