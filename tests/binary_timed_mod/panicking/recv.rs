use mpstthree::binary::struct_trait::end::*;
use mpstthree::binary::struct_trait::session::*;

use mpstthree::binary_timed::recv::recv;
use mpstthree::binary_timed::send::send;
use mpstthree::binary_timed::struct_trait::send::SendTimed;

use rand::{thread_rng, Rng};
use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

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
