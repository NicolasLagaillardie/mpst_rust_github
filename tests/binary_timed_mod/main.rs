use mpstthree::binary::close::close;
use mpstthree::binary::struct_trait::end::*;
use mpstthree::binary::struct_trait::session::*;

use mpstthree::binary_timed::choose::{choose_left, choose_right};
use mpstthree::binary_timed::fork::fork;
use mpstthree::binary_timed::offer::{offer_either, OfferTimed};
use mpstthree::binary_timed::recv::recv;
use mpstthree::binary_timed::send::send;
use mpstthree::binary_timed::struct_trait::recv::RecvTimed;
use mpstthree::binary_timed::struct_trait::send::SendTimed;

use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn head_str() {
    assert_eq!(
        SendTimed::<i32, 'a', 0, true, 5, true, ' ', End>::head_str(),
        "Send".to_string()
    );
    assert_eq!(
        RecvTimed::<i32, 'a', 0, true, 5, true, ' ', End>::head_str(),
        "Recv".to_string()
    );
}

pub fn tail_str() {
    assert_eq!(
        SendTimed::<i32, 'a', 0, true, 5, true, ' ', End>::tail_str(),
        "End<>".to_string()
    );
    assert_eq!(
        RecvTimed::<i32, 'a', 0, true, 5, true, ' ', End>::tail_str(),
        "End<>".to_string()
    );
}

pub fn self_head_str() {
    let (send, recv) = SendTimed::<i32, 'a', 0, true, 5, true, ' ', End>::new();
    assert_eq!(send.self_head_str(), "Send".to_string());
    assert_eq!(recv.self_head_str(), "Recv".to_string());
}

pub fn self_tail_str() {
    let (send, recv) = SendTimed::<i32, 'a', 0, true, 5, true, ' ', End>::new();
    assert_eq!(send.self_tail_str(), "End<>".to_string());
    assert_eq!(recv.self_tail_str(), "End<>".to_string());
}

// Constraints

pub fn constraint_start_excluded() {
    let (send, recv) = SendTimed::<i32, 'a', 5, false, -5, true, ' ', End>::new();
    assert_eq!(send.constraint(), "5 < a".to_string());
    assert_eq!(recv.constraint(), "5 < a".to_string());
}

pub fn constraint_start_included() {
    let (send, recv) = SendTimed::<i32, 'a', 5, true, -5, true, ' ', End>::new();
    assert_eq!(send.constraint(), "5 <= a".to_string());
    assert_eq!(recv.constraint(), "5 <= a".to_string());
}

pub fn constraint_end_excluded() {
    let (send, recv) = SendTimed::<i32, 'a', -5, true, 5, false, ' ', End>::new();
    assert_eq!(send.constraint(), "a < 5".to_string());
    assert_eq!(recv.constraint(), "a < 5".to_string());
}

pub fn constraint_end_included() {
    let (send, recv) = SendTimed::<i32, 'a', -5, true, 5, true, ' ', End>::new();
    assert_eq!(send.constraint(), "a <= 5".to_string());
    assert_eq!(recv.constraint(), "a <= 5".to_string());
}

pub fn constraint_start_excluded_end_excluded() {
    let (send, recv) = SendTimed::<i32, 'a', 5, false, 10, false, ' ', End>::new();
    assert_eq!(send.constraint(), "5 < a < 10".to_string());
    assert_eq!(recv.constraint(), "5 < a < 10".to_string());
}

pub fn constraint_start_excluded_end_included() {
    let (send, recv) = SendTimed::<i32, 'a', 5, false, 10, true, ' ', End>::new();
    assert_eq!(send.constraint(), "5 < a <= 10".to_string());
    assert_eq!(recv.constraint(), "5 < a <= 10".to_string());
}

pub fn constraint_start_included_end_excluded() {
    let (send, recv) = SendTimed::<i32, 'a', 5, true, 10, false, ' ', End>::new();
    assert_eq!(send.constraint(), "5 <= a < 10".to_string());
    assert_eq!(recv.constraint(), "5 <= a < 10".to_string());
}

pub fn constraint_start_included_end_included() {
    let (send, recv) = SendTimed::<i32, 'a', 5, true, 10, true, ' ', End>::new();
    assert_eq!(send.constraint(), "5 <= a <= 10".to_string());
    assert_eq!(recv.constraint(), "5 <= a <= 10".to_string());
}

// Test a simple calculator server, implemented using timed binary
// choice.

type NegServer =
    RecvTimed<i32, 'a', 2, true, 4, true, ' ', SendTimed<i32, 'a', 4, true, 6, true, ' ', End>>;
type NegClient = <NegServer as Session>::Dual;

type AddServer = RecvTimed<
    i32,
    'a',
    2,
    true,
    4,
    true,
    ' ',
    RecvTimed<i32, 'a', 4, true, 6, true, ' ', SendTimed<i32, 'a', 6, true, 8, true, ' ', End>>,
>;
type AddClient = <AddServer as Session>::Dual;

type SimpleCalcServer = OfferTimed<NegServer, AddServer, 'a', 1, true, 2, true, ' '>;
type SimpleCalcClient = <SimpleCalcServer as Session>::Dual;

fn simple_calc_server(
    s: SimpleCalcServer,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    sleep(Duration::from_secs(1));
    offer_either(
        all_clocks,
        s,
        |all_clocks: &mut HashMap<char, Instant>, s: NegServer| {
            sleep(Duration::from_secs(2));
            let (x, s) = recv(all_clocks, s)?;
            sleep(Duration::from_secs(2));
            let s = send(-x, all_clocks, s)?;
            sleep(Duration::from_secs(2));
            close(s)
        },
        |all_clocks: &mut HashMap<char, Instant>, s: AddServer| {
            sleep(Duration::from_secs(2));
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

            sleep(Duration::from_secs(1));
            let s = choose_left::<_, AddClient, 'a', 1, true, 2, true, ' '>(&mut all_clocks, s)?;
            sleep(Duration::from_secs(2));
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

            sleep(Duration::from_secs(1));
            let s = choose_right::<NegClient, _, 'a', 1, true, 2, true, ' '>(&mut all_clocks, s)?;
            sleep(Duration::from_secs(2));
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
