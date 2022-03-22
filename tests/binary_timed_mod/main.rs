use mpstthree::binary::close::close;
use mpstthree::binary::struct_trait::end::*;
use mpstthree::binary::struct_trait::session::*;

use mpstthree::binary_timed::choose::{choose_left, choose_right, ChooseTimed};
use mpstthree::binary_timed::fork::fork;
use mpstthree::binary_timed::offer::{offer_either, OfferTimed};
use mpstthree::binary_timed::recv::recv;
use mpstthree::binary_timed::send::send;
use mpstthree::binary_timed::struct_trait::recv::RecvTimed;
use mpstthree::binary_timed::struct_trait::send::SendTimed;

use rand::{thread_rng, Rng};
use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;
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

type SimpleCalcServer = OfferTimed<NegServer, AddServer, 'a', 1, true, 2, true, false>;
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
            let s = choose_left::<_, AddClient, 'a', 1, true, 2, true, false>(&mut all_clocks, s)?;
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
            let s = choose_right::<NegClient, _, 'a', 1, true, 2, true, false>(&mut all_clocks, s)?;
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

pub fn send_upper_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Create a random payload
        let mut rng = thread_rng();
        let x: i32 = rng.gen();

        // Sleep for 3 seconds to exceed upper timeout
        sleep(Duration::from_secs(3));

        // Try to send
        let (sender, _receiver) = SendTimed::<i32, End, 'a', 1, true, 2, true, false>::new();
        let _ = send(x, &mut all_clocks, sender)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn send_lower_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Create a random payload
        let mut rng = thread_rng();
        let x: i32 = rng.gen();

        // Sleep for 0 seconds to exceed lower timeout
        // sleep(Duration::from_secs(3));

        // Try to send
        let (sender, _receiver) = SendTimed::<i32, End, 'a', 1, true, 2, true, false>::new();
        let _ = send(x, &mut all_clocks, sender)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_upper_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Create a random payload
        let mut rng = thread_rng();
        let x: i32 = rng.gen();

        // Sleep for 1 seconds to remain in the correct time window
        sleep(Duration::from_secs(1));

        // Send
        let (sender, receiver) = SendTimed::<i32, End, 'a', 1, true, 2, true, false>::new();
        let _ = send(x, &mut all_clocks, sender)?;

        // Sleep for 2 seconds to exceed upper timeout
        sleep(Duration::from_secs(2));

        // Try to recv
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_lower_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Create a random payload
        let mut rng = thread_rng();
        let x: i32 = rng.gen();

        // Sleep for 1 seconds to remain in the correct time window
        sleep(Duration::from_secs(1));

        // Send
        let (sender, receiver) = SendTimed::<i32, End, 'a', 1, true, 2, true, false>::new();
        let _ = send(x, &mut all_clocks, sender)?;

        // reset clock 'a' to exceed lower timeout
        let clock_a = all_clocks.get_mut(&'a').unwrap();
        *clock_a = Instant::now();

        // Try to recv
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn choose_lower_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 0 seconds to exceed lower timeout
        sleep(Duration::from_secs(0));

        // Send
        let (sender, _) = ChooseTimed::<End, End, 'a', 1, true, 2, true, false>::new();
        let _ = choose_left::<End, End, 'a', 1, true, 2, true, false>(&mut all_clocks, sender)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn choose_upper_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 3 seconds to exceed upper timeout
        sleep(Duration::from_secs(3));

        // Send
        let (sender, _) = ChooseTimed::<End, End, 'a', 1, true, 2, true, false>::new();
        let _ = choose_left::<End, End, 'a', 1, true, 2, true, false>(&mut all_clocks, sender)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn offer_upper_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in the correct time window
        sleep(Duration::from_secs(1));

        // Send
        let (sender, receiver) = ChooseTimed::<End, End, 'a', 1, true, 2, true, false>::new();
        let _ = choose_left::<End, End, 'a', 1, true, 2, true, false>(&mut all_clocks, sender)?; // will fail

        // Sleep for 3 seconds to exceed upper timeout
        sleep(Duration::from_secs(3));

        offer_either(
            &mut all_clocks,
            receiver,
            |_all_clocks: &mut HashMap<char, Instant>, s: End| {
                close(s)
            },
            |_all_clocks: &mut HashMap<char, Instant>, s: End| {
                close(s)
            },
        )
    }()
    .is_err());
}

pub fn offer_lower_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in the correct time window
        sleep(Duration::from_secs(1));

        // Send
        let (sender, receiver) = ChooseTimed::<End, End, 'a', 1, true, 2, true, false>::new();
        let _ = choose_left::<End, End, 'a', 1, true, 2, true, false>(&mut all_clocks, sender)?; // will fail

        // reset clock 'a' to exceed lower timeout
        let clock_a = all_clocks.get_mut(&'a').unwrap();
        *clock_a = Instant::now();

        offer_either(
            &mut all_clocks,
            receiver,
            |_all_clocks: &mut HashMap<char, Instant>, s: End| {
                close(s)
            },
            |_all_clocks: &mut HashMap<char, Instant>, s: End| {
                close(s)
            },
        )
    }()
    .is_err());
}
