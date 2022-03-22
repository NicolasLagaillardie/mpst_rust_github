use mpstthree::binary::struct_trait::end::*;
use mpstthree::binary::struct_trait::session::*;

use mpstthree::binary_timed::choose::{choose_left, choose_right, ChooseTimed};

use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn choose_left_upper_timeout_panics() {
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

pub fn choose_left_lower_timeout_panics() {
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

pub fn choose_right_upper_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 3 seconds to exceed upper timeout
        sleep(Duration::from_secs(3));

        // Send
        let (sender, _) = ChooseTimed::<End, End, 'a', 1, true, 2, true, false>::new();
        let _ = choose_right::<End, End, 'a', 1, true, 2, true, false>(&mut all_clocks, sender)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn choose_right_lower_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 0 seconds to exceed lower timeout
        sleep(Duration::from_secs(0));

        // Send
        let (sender, _) = ChooseTimed::<End, End, 'a', 1, true, 2, true, false>::new();
        let _ = choose_right::<End, End, 'a', 1, true, 2, true, false>(&mut all_clocks, sender)?; // will fail

        Ok(())
    }()
    .is_err());
}
