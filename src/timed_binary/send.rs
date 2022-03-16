//! This module contains the functions for sending
//! a payload for binary sessions.

use crate::binary::cancel::cancel;
use crate::binary::struct_trait::session::Session;
use crate::timed_binary::struct_trait::send_timed::SendTimed;

use std::boxed::Box;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::marker;
use std::panic;
use std::time::Instant;

/// Send a value of type `T`. Always succeeds. Returns the
/// continuation of the session `S`.
pub fn send<
    T,
    S,
    const CLOCK: char,
    const START: i128,
    const INCLUDE_START: bool,
    const END: i128,
    const INCLUDE_END: bool,
    const RESET: bool,
>(
    x: T,
    mut all_clocks: HashMap<char, Instant>,
    s: SendTimed<T, S, CLOCK, START, INCLUDE_START, END, INCLUDE_END, RESET>,
) -> Result<(S, HashMap<char, Instant>), Box<dyn Error>>
where
    T: marker::Send,
    S: Session,
{
    if s.start < 0 {
        if s.end >= 0 {
            if s.include_end {
                if let Some(own_clock) = all_clocks.get_mut(&s.clock) {
                    if own_clock.elapsed().as_secs() <= u64::try_from(s.end)? && s.reset {
                        let (here, there) = S::new();
                        match s.channel.send((x, there)) {
                            Ok(_) => {
                                *own_clock = Instant::now();
                                Ok((here, all_clocks))
                            }
                            Err(e) => {
                                cancel(s);
                                panic!("{}", e.to_string())
                            }
                        }
                    } else if own_clock.elapsed().as_secs() <= u64::try_from(s.end)? {
                        let (here, there) = S::new();
                        match s.channel.send((x, there)) {
                            Ok(_) => Ok((here, all_clocks)),
                            Err(e) => {
                                cancel(s);
                                panic!("{}", e.to_string())
                            }
                        }
                    } else {
                        panic!("Timeout for clock {}", s.clock);
                    }
                } else {
                    panic!("The clock {} is not available in {:?}", s.clock, all_clocks);
                }
            } else {
                if let Some(own_clock) = all_clocks.get_mut(&s.clock) {
                    if own_clock.elapsed().as_secs() < u64::try_from(s.end)? && s.reset {
                        let (here, there) = S::new();
                        match s.channel.send((x, there)) {
                            Ok(_) => {
                                *own_clock = Instant::now();
                                Ok((here, all_clocks))
                            }
                            Err(e) => {
                                cancel(s);
                                panic!("{}", e.to_string())
                            }
                        }
                    } else if own_clock.elapsed().as_secs() < u64::try_from(s.end)? {
                        let (here, there) = S::new();
                        match s.channel.send((x, there)) {
                            Ok(_) => Ok((here, all_clocks)),
                            Err(e) => {
                                cancel(s);
                                panic!("{}", e.to_string())
                            }
                        }
                    } else {
                        panic!("Timeout for clock {}", s.clock);
                    }
                } else {
                    panic!("The clock {} is not available in {:?}", s.clock, all_clocks);
                }
            }
        } else {
            panic!("Both start and end parameters are negative")
        }
    } else {
        if s.end < 0 {
            if s.include_start {
                if let Some(own_clock) = all_clocks.get_mut(&s.clock) {
                    if own_clock.elapsed().as_secs() >= u64::try_from(s.start)? && s.reset {
                        let (here, there) = S::new();
                        match s.channel.send((x, there)) {
                            Ok(_) => {
                                *own_clock = Instant::now();
                                Ok((here, all_clocks))
                            }
                            Err(e) => {
                                cancel(s);
                                panic!("{}", e.to_string())
                            }
                        }
                    } else if own_clock.elapsed().as_secs() >= u64::try_from(s.start)? {
                        let (here, there) = S::new();
                        match s.channel.send((x, there)) {
                            Ok(_) => Ok((here, all_clocks)),
                            Err(e) => {
                                cancel(s);
                                panic!("{}", e.to_string())
                            }
                        }
                    } else {
                        panic!("Timeout for clock {}", s.clock);
                    }
                } else {
                    panic!("The clock {} is not available in {:?}", s.clock, all_clocks);
                }
            } else {
                if let Some(own_clock) = all_clocks.get_mut(&s.clock) {
                    if own_clock.elapsed().as_secs() > u64::try_from(s.start)? && s.reset {
                        let (here, there) = S::new();
                        match s.channel.send((x, there)) {
                            Ok(_) => {
                                *own_clock = Instant::now();
                                Ok((here, all_clocks))
                            }
                            Err(e) => {
                                cancel(s);
                                panic!("{}", e.to_string())
                            }
                        }
                    } else if own_clock.elapsed().as_secs() > u64::try_from(s.start)? {
                        let (here, there) = S::new();
                        match s.channel.send((x, there)) {
                            Ok(_) => Ok((here, all_clocks)),
                            Err(e) => {
                                cancel(s);
                                panic!("{}", e.to_string())
                            }
                        }
                    } else {
                        panic!("Timeout for clock {}", s.clock);
                    }
                } else {
                    panic!("The clock {} is not available in {:?}", s.clock, all_clocks);
                }
            }
        } else {
            if s.start > s.end {
                panic!(
                    "Start and End parameters cannot match: start = {} > {} = end",
                    s.start, s.end
                );
            }

            match (s.include_start, s.include_end) {
                (true, true) => {
                    if let Some(own_clock) = all_clocks.get_mut(&s.clock) {
                        if u64::try_from(s.start)? <= own_clock.elapsed().as_secs()
                            && own_clock.elapsed().as_secs() <= u64::try_from(s.end)?
                            && s.reset
                        {
                            let (here, there) = S::new();
                            match s.channel.send((x, there)) {
                                Ok(_) => {
                                    *own_clock = Instant::now();
                                    Ok((here, all_clocks))
                                }
                                Err(e) => {
                                    cancel(s);
                                    panic!("{}", e.to_string())
                                }
                            }
                        } else if u64::try_from(s.start)? <= own_clock.elapsed().as_secs()
                            && own_clock.elapsed().as_secs() <= u64::try_from(s.end)?
                        {
                            let (here, there) = S::new();
                            match s.channel.send((x, there)) {
                                Ok(_) => Ok((here, all_clocks)),
                                Err(e) => {
                                    cancel(s);
                                    panic!("{}", e.to_string())
                                }
                            }
                        } else {
                            panic!("Timeout for clock {}", s.clock);
                        }
                    } else {
                        panic!("The clock {} is not available in {:?}", s.clock, all_clocks);
                    }
                }
                (true, false) => {
                    if let Some(own_clock) = all_clocks.get_mut(&s.clock) {
                        if u64::try_from(s.start)? <= own_clock.elapsed().as_secs()
                            && own_clock.elapsed().as_secs() < u64::try_from(s.end)?
                            && s.reset
                        {
                            let (here, there) = S::new();
                            match s.channel.send((x, there)) {
                                Ok(_) => {
                                    *own_clock = Instant::now();
                                    Ok((here, all_clocks))
                                }
                                Err(e) => {
                                    cancel(s);
                                    panic!("{}", e.to_string())
                                }
                            }
                        } else if u64::try_from(s.start)? <= own_clock.elapsed().as_secs()
                            && own_clock.elapsed().as_secs() < u64::try_from(s.end)?
                        {
                            let (here, there) = S::new();
                            match s.channel.send((x, there)) {
                                Ok(_) => Ok((here, all_clocks)),
                                Err(e) => {
                                    cancel(s);
                                    panic!("{}", e.to_string())
                                }
                            }
                        } else {
                            panic!("Timeout for clock {}", s.clock);
                        }
                    } else {
                        panic!("The clock {} is not available in {:?}", s.clock, all_clocks);
                    }
                }
                (false, true) => {
                    if let Some(own_clock) = all_clocks.get_mut(&s.clock) {
                        if u64::try_from(s.start)? < own_clock.elapsed().as_secs()
                            && own_clock.elapsed().as_secs() <= u64::try_from(s.end)?
                            && s.reset
                        {
                            let (here, there) = S::new();
                            match s.channel.send((x, there)) {
                                Ok(_) => {
                                    *own_clock = Instant::now();
                                    Ok((here, all_clocks))
                                }
                                Err(e) => {
                                    cancel(s);
                                    panic!("{}", e.to_string())
                                }
                            }
                        } else if u64::try_from(s.start)? < own_clock.elapsed().as_secs()
                            && own_clock.elapsed().as_secs() <= u64::try_from(s.end)?
                        {
                            let (here, there) = S::new();
                            match s.channel.send((x, there)) {
                                Ok(_) => Ok((here, all_clocks)),
                                Err(e) => {
                                    cancel(s);
                                    panic!("{}", e.to_string())
                                }
                            }
                        } else {
                            panic!("Timeout for clock {}", s.clock);
                        }
                    } else {
                        panic!("The clock {} is not available in {:?}", s.clock, all_clocks);
                    }
                }
                (false, false) => {
                    if let Some(own_clock) = all_clocks.get_mut(&s.clock) {
                        if u64::try_from(s.start)? < own_clock.elapsed().as_secs()
                            && own_clock.elapsed().as_secs() < u64::try_from(s.end)?
                            && s.reset
                        {
                            let (here, there) = S::new();
                            match s.channel.send((x, there)) {
                                Ok(_) => {
                                    *own_clock = Instant::now();
                                    Ok((here, all_clocks))
                                }
                                Err(e) => {
                                    cancel(s);
                                    panic!("{}", e.to_string())
                                }
                            }
                        } else if u64::try_from(s.start)? < own_clock.elapsed().as_secs()
                            && own_clock.elapsed().as_secs() < u64::try_from(s.end)?
                        {
                            let (here, there) = S::new();
                            match s.channel.send((x, there)) {
                                Ok(_) => Ok((here, all_clocks)),
                                Err(e) => {
                                    cancel(s);
                                    panic!("{}", e.to_string())
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
