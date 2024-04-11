use mpstthree::binary::close::close;
use mpstthree::binary::struct_trait::end::*;
use mpstthree::binary::struct_trait::session::*;

use mpstthree::binary_timed::choose::{choose_left, ChooseTimed};
use mpstthree::binary_timed::offer::offer_either;

use std::collections::HashMap;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn offer_upper_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in the correct time window
        sleep(Duration::from_secs(1));

        // Send
        let (sender, receiver) = ChooseTimed::<End, End, 'a', 1, true, 2, true, ' '>::new();
        let _ = choose_left::<End, End, 'a', 1, true, 2, true, ' '>(&mut all_clocks, sender)?; // will fail

        // Sleep for 3 seconds to exceed upper timeout
        sleep(Duration::from_secs(3));

        offer_either(
            &mut all_clocks,
            receiver,
            |_all_clocks: &mut HashMap<char, Instant>, s: End| close(s),
            |_all_clocks: &mut HashMap<char, Instant>, s: End| close(s),
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
        let (sender, receiver) = ChooseTimed::<End, End, 'a', 1, true, 2, true, ' '>::new();
        let _ = choose_left::<End, End, 'a', 1, true, 2, true, ' '>(&mut all_clocks, sender)?; // will fail

        // reset clock 'a' to exceed lower timeout
        let clock_a = all_clocks.get_mut(&'a').unwrap();
        *clock_a = Instant::now();

        offer_either(
            &mut all_clocks,
            receiver,
            |_all_clocks: &mut HashMap<char, Instant>, s: End| close(s),
            |_all_clocks: &mut HashMap<char, Instant>, s: End| close(s),
        )
    }()
    .is_err());
}
