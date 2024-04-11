use mpstthree::binary::cancel::cancel;
use mpstthree::binary::struct_trait::end::*;
use mpstthree::binary::struct_trait::session::*;

use mpstthree::binary_timed::recv::recv;
use mpstthree::binary_timed::struct_trait::recv::RecvTimed;

use std::collections::HashMap;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

/////////////////////////

pub fn recv_both_positive_both_included_wrong_order_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 2 seconds to stay in correct time window
        sleep(Duration::from_secs(2));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', 2, true, 1, true, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

/////////////////////////

pub fn recv_both_negative_both_included_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to stay in "correct" time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', -1, true, -2, true, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

///////////////////////// both positive / both included

pub fn recv_both_positive_both_included_upper_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 3 seconds to exceed upper timeout
        sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', 1, true, 2, true, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_both_positive_both_included_lower_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 0 seconds to exceed lower timeout
        // sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', 1, true, 2, true, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_both_positive_both_included_reset_clock_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', 1, true, 2, true, 'a', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_both_positive_both_included_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', 1, true, 2, true, ' ', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_both_positive_both_included_wrong_reset_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to stay in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'b', 1, true, 2, true, 'b', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

///////////////////////// both positive / upper included

pub fn recv_both_positive_upper_included_upper_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 3 seconds to exceed upper timeout
        sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', 1, false, 2, true, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_both_positive_upper_included_lower_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 0 seconds to exceed lower timeout
        // sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', 1, false, 2, true, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_both_positive_upper_included_reset_clock_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', 1, false, 2, true, 'a', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_both_positive_upper_included_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', 1, false, 2, true, ' ', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_both_positive_upper_included_wrong_reset_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to stay in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'b', 1, false, 2, true, 'b', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

///////////////////////// both positive / lower included

pub fn recv_both_positive_lower_included_upper_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 3 seconds to exceed upper timeout
        sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', 1, true, 2, false, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_both_positive_lower_included_lower_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 0 seconds to exceed lower timeout
        // sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', 1, true, 2, false, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_both_positive_lower_included_lower_reset_clock_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', 1, true, 2, false, 'a', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_both_positive_lower_included_lower_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', 1, true, 2, false, ' ', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_both_positive_lower_included_wrong_reset_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to stay in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'b', 1, true, 2, false, 'b', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

///////////////////////// both positive / none included

pub fn recv_both_positive_none_included_upper_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 3 seconds to exceed upper timeout
        sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', 1, false, 2, false, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_both_positive_none_included_lower_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 0 seconds to exceed lower timeout
        // sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', 1, false, 2, false, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_both_positive_none_included_reset_clock_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', 1, false, 2, false, 'a', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_both_positive_none_included_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', 1, false, 2, false, ' ', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_both_positive_none_included_wrong_reset_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to stay in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'b', 1, false, 2, false, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

///////////////////////// lower positive / both included

pub fn recv_lower_positive_both_included_upper_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 3 seconds to exceed upper timeout
        sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', 1, true, -2, true, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_lower_positive_both_included_lower_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 0 seconds to exceed lower timeout
        // sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', 1, true, -2, true, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_lower_positive_both_included_reset_clock_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', 1, true, -2, true, 'a', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_lower_positive_both_included_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', 1, true, -2, true, ' ', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_lower_positive_both_included_wrong_reset_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to stay in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'b', 1, true, -2, true, 'b', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

///////////////////////// lower positive / upper included

pub fn recv_lower_positive_upper_included_upper_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 3 seconds to exceed upper timeout
        sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', 1, false, -2, true, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_lower_positive_upper_included_lower_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 0 seconds to exceed lower timeout
        // sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', 1, false, -2, true, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_lower_positive_upper_included_reset_clock_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', 1, false, -2, true, 'a', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_lower_positive_upper_included_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', 1, false, -2, true, ' ', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_lower_positive_upper_included_wrong_reset_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to stay in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'b', 1, false, -2, true, 'b', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

///////////////////////// lower positive / lower included

pub fn recv_lower_positive_lower_included_upper_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 3 seconds to exceed upper timeout
        sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', 1, true, -2, false, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_lower_positive_lower_included_lower_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 0 seconds to exceed lower timeout
        // sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', 1, true, -2, false, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_lower_positive_lower_included_lower_reset_clock_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', 1, true, -2, false, 'a', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_lower_positive_lower_included_lower_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', 1, true, -2, false, ' ', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_lower_positive_lower_included_wrong_reset_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to stay in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'b', 1, true, -2, false, 'b', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

///////////////////////// lower positive / none included

pub fn recv_lower_positive_none_included_upper_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 3 seconds to exceed upper timeout
        sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', 1, false, -2, false, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_lower_positive_none_included_lower_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 0 seconds to exceed lower timeout
        // sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', 1, false, -2, false, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_lower_positive_none_included_reset_clock_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', 1, false, -2, false, 'a', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_lower_positive_none_included_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', 1, false, -2, false, ' ', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_lower_positive_none_included_wrong_reset_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to stay in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'b', 1, false, -2, false, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

///////////////////////// upper positive / both included

pub fn recv_upper_positive_both_included_upper_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 3 seconds to exceed upper timeout
        sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', -1, true, 2, true, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_upper_positive_both_included_lower_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 0 seconds to exceed lower timeout
        // sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', -1, true, 2, true, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_upper_positive_both_included_reset_clock_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', -1, true, 2, true, 'a', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_upper_positive_both_included_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', -1, true, 2, true, ' ', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_upper_positive_both_included_wrong_reset_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to stay in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'b', -1, true, 2, true, 'b', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

///////////////////////// upper positive / upper included

pub fn recv_upper_positive_upper_included_upper_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 3 seconds to exceed upper timeout
        sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', -1, false, 2, true, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_upper_positive_upper_included_lower_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 0 seconds to exceed lower timeout
        // sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', -1, false, 2, true, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_upper_positive_upper_included_reset_clock_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', -1, false, 2, true, 'a', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_upper_positive_upper_included_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', -1, false, 2, true, ' ', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_upper_positive_upper_included_wrong_reset_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to stay in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'b', -1, false, 2, true, 'b', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

///////////////////////// upper positive / lower included

pub fn recv_upper_positive_lower_included_upper_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 3 seconds to exceed upper timeout
        sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', -1, true, 2, false, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_upper_positive_lower_included_lower_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 0 seconds to exceed lower timeout
        // sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', -1, true, 2, false, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_upper_positive_lower_included_lower_reset_clock_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', -1, true, 2, false, 'a', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_upper_positive_lower_included_lower_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', -1, true, 2, false, ' ', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_upper_positive_lower_included_wrong_reset_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to stay in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'b', -1, true, 2, false, 'b', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

///////////////////////// upper positive / none included

pub fn recv_upper_positive_none_included_upper_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 3 seconds to exceed upper timeout
        sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', -1, false, 2, false, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}

pub fn recv_upper_positive_none_included_lower_timeout_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 0 seconds to exceed lower timeout
        // sleep(Duration::from_secs(3));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'a', -1, false, 2, false, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_upper_positive_none_included_reset_clock_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', -1, false, 2, false, 'a', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_upper_positive_none_included_send_missing_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to remain in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, sender) = RecvTimed::<i32, 'a', -1, false, 2, false, ' ', End>::new();
        cancel(sender);
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_err());
}

pub fn recv_upper_positive_none_included_wrong_reset_panics() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Create the new clock ledger
        let mut all_clocks = HashMap::<char, Instant>::new();
        all_clocks.insert('a', Instant::now());

        // Sleep for 1 seconds to stay in correct time window
        sleep(Duration::from_secs(1));

        // Try to send
        let (receiver, _sender) = RecvTimed::<i32, 'b', -1, false, 2, false, ' ', End>::new();
        let _ = recv(&mut all_clocks, receiver)?; // will fail

        Ok(())
    }()
    .is_ok());
}
