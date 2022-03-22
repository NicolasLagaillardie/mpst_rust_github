use mpstthree::binary::struct_trait::end::*;
use mpstthree::binary::struct_trait::session::*;

use mpstthree::binary_timed::send::send;
use mpstthree::binary_timed::struct_trait::send::SendTimed;

use rand::{thread_rng, Rng};
use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn send_both_included_upper_timeout_panics() {
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

pub fn send_both_included_lower_timeout_panics() {
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

pub fn send_upper_included_upper_timeout_panics() {
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
        let (sender, _receiver) = SendTimed::<i32, End, 'a', 1, false, 2, true, false>::new();
        let _ = send(x, &mut all_clocks, sender)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn send_upper_included_lower_timeout_panics() {
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
        let (sender, _receiver) = SendTimed::<i32, End, 'a', 1, false, 2, true, false>::new();
        let _ = send(x, &mut all_clocks, sender)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn send_lower_included_upper_timeout_panics() {
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
        let (sender, _receiver) = SendTimed::<i32, End, 'a', 1, true, 2, false, false>::new();
        let _ = send(x, &mut all_clocks, sender)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn send_lower_included_lower_timeout_panics() {
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
        let (sender, _receiver) = SendTimed::<i32, End, 'a', 1, true, 2, false, false>::new();
        let _ = send(x, &mut all_clocks, sender)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn send_none_included_upper_timeout_panics() {
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
        let (sender, _receiver) = SendTimed::<i32, End, 'a', 1, false, 2, false, false>::new();
        let _ = send(x, &mut all_clocks, sender)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn send_none_included_lower_timeout_panics() {
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
        let (sender, _receiver) = SendTimed::<i32, End, 'a', 1, false, 2, false, false>::new();
        let _ = send(x, &mut all_clocks, sender)?; // will fail

        Ok(())
    }()
    .is_err());
}
