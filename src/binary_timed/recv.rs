#![allow(clippy::collapsible_else_if)]

//! This module contains the functions for receiving
//! a payload for binary sessions.

use crate::binary::cancel::cancel;
use crate::binary::struct_trait::session::Session;
use crate::binary_timed::struct_trait::recv::RecvTimed;

use std::boxed::Box;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::marker;
use std::time::{Duration, Instant};

/// Receive a value of type `T`. Can fail. Returns either a
/// pair of the received value and the continuation of the
/// session `S` or an error.
pub fn recv<
    T,
    const CLOCK: char,
    const START: i128,
    const INCLUDE_START: bool,
    const END: i128,
    const INCLUDE_END: bool,
    const RESET: char,
    S,
>(
    all_clocks: &mut HashMap<char, Instant>,
    s: RecvTimed<T, CLOCK, START, INCLUDE_START, END, INCLUDE_END, RESET, S>,
) -> Result<(T, S), Box<dyn Error>>
where
    T: marker::Send,
    S: Session,
{
    // if there is no lower bound
    if s.start < 0 {
        // if there is an upper bound
        if s.end >= 0 {
            // if this upper bound is included in the time constraint
            if s.include_end {
                // if the clock is available among all clocks
                if let Some(own_clock) = all_clocks.get_mut(&s.clock) {
                    // if the clock respects the time constraint and the clock must be reset
                    if own_clock.elapsed().as_secs() <= u64::try_from(s.end)? && s.reset == ' ' {
                        // receive with timeout
                        match s.channel.recv_timeout(
                            Duration::from_secs(u64::try_from(s.end)?) - own_clock.elapsed(),
                        ) {
                            Ok((v, s)) => {
                                *own_clock = Instant::now();
                                Ok((v, s))
                            }
                            Err(e) => {
                                cancel(s);
                                Err(Box::new(e))
                            }
                        }
                    // if the clock respects the time constraint and the clock must not be reset
                    } else if own_clock.elapsed().as_secs() <= u64::try_from(s.end)? {
                        // receive with timeout
                        match s.channel.recv_timeout(
                            Duration::from_secs(u64::try_from(s.end)?) - own_clock.elapsed(),
                        ) {
                            Ok((v, s)) => Ok((v, s)),
                            Err(e) => {
                                cancel(s);
                                Err(Box::new(e))
                            }
                        }
                    // if the clock does not respect the time constraint
                    } else {
                        panic!("Timeout for clock {}", s.clock);
                    }
                // if the clock is not available among all clocks
                } else {
                    panic!("The clock {} is not available in {:?}", s.clock, all_clocks);
                }
            // if this upper bound is not included in the time constraint. In this case, we remove
            // one time from the upper bound
            } else {
                // if the clock is available among all clocks
                if let Some(own_clock) = all_clocks.get_mut(&s.clock) {
                    // if the clock respects the time constraint and the clock must be reset
                    if own_clock.elapsed().as_secs() < u64::try_from(s.end)? && s.reset == ' ' {
                        // receive with timeout
                        match s.channel.recv_timeout(
                            Duration::from_secs(u64::try_from(s.end)?) - own_clock.elapsed(),
                        ) {
                            Ok((v, s)) => {
                                *own_clock = Instant::now();
                                Ok((v, s))
                            }
                            Err(e) => {
                                cancel(s);
                                Err(Box::new(e))
                            }
                        }
                    // if the clock respects the time constraint and the clock must not be reset
                    } else if own_clock.elapsed().as_secs() < u64::try_from(s.end)? {
                        // receive with timeout
                        match s.channel.recv_timeout(
                            Duration::from_secs(u64::try_from(s.end)?) - own_clock.elapsed(),
                        ) {
                            Ok((v, s)) => Ok((v, s)),
                            Err(e) => {
                                cancel(s);
                                Err(Box::new(e))
                            }
                        }
                    // if the clock does not respect the time constraint
                    } else {
                        panic!("Timeout for clock {}", s.clock);
                    }
                // if the clock is not available among all clocks
                } else {
                    panic!("The clock {} is not available in {:?}", s.clock, all_clocks);
                }
            }
        // if there is are not correct bounds: both are negative
        } else {
            panic!("Both start and end parameters are negative")
        }
    // if there is a lower bound
    } else {
        // if there is no upper bound
        if s.end < 0 {
            // if this lower bound is included in the time constraint
            if s.include_start {
                // if the clock is available among all clocks
                if let Some(own_clock) = all_clocks.get_mut(&s.clock) {
                    // if the clock respects the time constraint and the clock must be reset
                    if u64::try_from(s.start)? <= own_clock.elapsed().as_secs() && s.reset == ' ' {
                        // blocking receive
                        match s.channel.recv() {
                            Ok((v, s)) => {
                                *own_clock = Instant::now();
                                Ok((v, s))
                            }
                            Err(e) => {
                                cancel(s);
                                Err(Box::new(e))
                            }
                        }
                    // if the clock respects the time constraint and the clock must not be reset
                    } else if u64::try_from(s.start)? <= own_clock.elapsed().as_secs() {
                        // blocking receive
                        match s.channel.recv() {
                            Ok((v, s)) => Ok((v, s)),
                            Err(e) => {
                                cancel(s);
                                Err(Box::new(e))
                            }
                        }
                    // if the clock does not respect the time constraint
                    } else {
                        panic!("Timeout for clock {}", s.clock);
                    }
                // if the clock is not available among all clocks
                } else {
                    panic!("The clock {} is not available in {:?}", s.clock, all_clocks);
                }
            // if this lower bound is not included in the time constraint. In this case, we add one
            // unit time from the upper bound
            } else {
                // if the clock is available among all clocks
                if let Some(own_clock) = all_clocks.get_mut(&s.clock) {
                    // if the clock respects the time constraint and the clock must be reset
                    if u64::try_from(s.start)? < own_clock.elapsed().as_secs() && s.reset == ' ' {
                        // blocking receive
                        match s.channel.recv() {
                            Ok((v, s)) => {
                                *own_clock = Instant::now();
                                Ok((v, s))
                            }
                            Err(e) => {
                                cancel(s);
                                Err(Box::new(e))
                            }
                        }
                    // if the clock respects the time constraint and the clock must not be reset
                    } else if u64::try_from(s.start)? < own_clock.elapsed().as_secs() {
                        // blocking receive
                        match s.channel.recv() {
                            Ok((v, s)) => Ok((v, s)),
                            Err(e) => {
                                cancel(s);
                                Err(Box::new(e))
                            }
                        }
                    // if the clock does not respect the time constraint
                    } else {
                        panic!("Timeout for clock {}", s.clock);
                    }
                // if the clock is not available among all clocks
                } else {
                    panic!("The clock {} is not available in {:?}", s.clock, all_clocks);
                }
            }
        // if both bounds are correct (positive)
        } else {
            // if the time constraint does not make sense
            if s.start > s.end {
                panic!(
                    "Start and End parameters cannot match: start = {} > {} = end",
                    s.start, s.end
                );
            }
            // match on the possible inclusion of the bounds
            match (s.include_start, s.include_end) {
                // if both bounds are included
                (true, true) => {
                    if let Some(own_clock) = all_clocks.get_mut(&s.clock) {
                        if u64::try_from(s.start)? <= own_clock.elapsed().as_secs()
                            && own_clock.elapsed().as_secs() <= u64::try_from(s.end)?
                            && s.reset == ' '
                        {
                            // receive with timeout
                            match s.channel.recv_timeout(
                                Duration::from_secs(u64::try_from(s.end)?) - own_clock.elapsed(),
                            ) {
                                Ok((v, s)) => {
                                    *own_clock = Instant::now();
                                    Ok((v, s))
                                }
                                Err(e) => {
                                    cancel(s);
                                    Err(Box::new(e))
                                }
                            }
                        } else if u64::try_from(s.start)? <= own_clock.elapsed().as_secs()
                            && own_clock.elapsed().as_secs() <= u64::try_from(s.end)?
                        {
                            // receive with timeout
                            match s.channel.recv_timeout(
                                Duration::from_secs(u64::try_from(s.end)?) - own_clock.elapsed(),
                            ) {
                                Ok((v, s)) => Ok((v, s)),
                                Err(e) => {
                                    cancel(s);
                                    Err(Box::new(e))
                                }
                            }
                        } else {
                            panic!("Timeout for clock {}", s.clock);
                        }
                    } else {
                        panic!("The clock {} is not available in {:?}", s.clock, all_clocks);
                    }
                }
                // if only the lower bound is included
                (true, false) => {
                    if let Some(own_clock) = all_clocks.get_mut(&s.clock) {
                        if u64::try_from(s.start)? <= own_clock.elapsed().as_secs()
                            && own_clock.elapsed().as_secs() < u64::try_from(s.end)?
                            && s.reset == ' '
                        {
                            // receive with timeout
                            match s.channel.recv_timeout(
                                Duration::from_secs(u64::try_from(s.end)?) - own_clock.elapsed(),
                            ) {
                                Ok((v, s)) => {
                                    *own_clock = Instant::now();
                                    Ok((v, s))
                                }
                                Err(e) => {
                                    cancel(s);
                                    Err(Box::new(e))
                                }
                            }
                        } else if u64::try_from(s.start)? <= own_clock.elapsed().as_secs()
                            && own_clock.elapsed().as_secs() < u64::try_from(s.end)?
                        {
                            // receive with timeout
                            match s.channel.recv_timeout(
                                Duration::from_secs(u64::try_from(s.end)?) - own_clock.elapsed(),
                            ) {
                                Ok((v, s)) => Ok((v, s)),
                                Err(e) => {
                                    cancel(s);
                                    Err(Box::new(e))
                                }
                            }
                        } else {
                            panic!("Timeout for clock {}", s.clock);
                        }
                    } else {
                        panic!("The clock {} is not available in {:?}", s.clock, all_clocks);
                    }
                }
                // if only the upper bound is included
                (false, true) => {
                    if let Some(own_clock) = all_clocks.get_mut(&s.clock) {
                        if u64::try_from(s.start)? < own_clock.elapsed().as_secs()
                            && own_clock.elapsed().as_secs() <= u64::try_from(s.end)?
                            && s.reset == ' '
                        {
                            // receive with timeout
                            match s.channel.recv_timeout(
                                Duration::from_secs(u64::try_from(s.end)?) - own_clock.elapsed(),
                            ) {
                                Ok((v, s)) => {
                                    *own_clock = Instant::now();
                                    Ok((v, s))
                                }
                                Err(e) => {
                                    cancel(s);
                                    Err(Box::new(e))
                                }
                            }
                        } else if u64::try_from(s.start)? < own_clock.elapsed().as_secs()
                            && own_clock.elapsed().as_secs() <= u64::try_from(s.end)?
                        {
                            // receive with timeout
                            match s.channel.recv_timeout(
                                Duration::from_secs(u64::try_from(s.end)?) - own_clock.elapsed(),
                            ) {
                                Ok((v, s)) => Ok((v, s)),
                                Err(e) => {
                                    cancel(s);
                                    Err(Box::new(e))
                                }
                            }
                        } else {
                            panic!("Timeout for clock {}", s.clock);
                        }
                    } else {
                        panic!("The clock {} is not available in {:?}", s.clock, all_clocks);
                    }
                }
                // if none of the bounds are included
                (false, false) => {
                    if let Some(own_clock) = all_clocks.get_mut(&s.clock) {
                        if u64::try_from(s.start)? < own_clock.elapsed().as_secs()
                            && own_clock.elapsed().as_secs() < u64::try_from(s.end)?
                            && s.reset == ' '
                        {
                            // receive with timeout
                            match s.channel.recv_timeout(
                                Duration::from_secs(u64::try_from(s.end)?) - own_clock.elapsed(),
                            ) {
                                Ok((v, s)) => {
                                    *own_clock = Instant::now();
                                    Ok((v, s))
                                }
                                Err(e) => {
                                    cancel(s);
                                    Err(Box::new(e))
                                }
                            }
                        } else if u64::try_from(s.start)? < own_clock.elapsed().as_secs()
                            && own_clock.elapsed().as_secs() < u64::try_from(s.end)?
                        {
                            // receive with timeout
                            match s.channel.recv_timeout(
                                Duration::from_secs(u64::try_from(s.end)?) - own_clock.elapsed(),
                            ) {
                                Ok((v, s)) => Ok((v, s)),
                                Err(e) => {
                                    cancel(s);
                                    Err(Box::new(e))
                                }
                            }
                        } else {
                            panic!("Timeout for clock {}", s.clock);
                        }
                    } else {
                        panic!("The clock {} is not available in {:?}", s.clock, all_clocks);
                    }
                }
            }
        }
    }
}
