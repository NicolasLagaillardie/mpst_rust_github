//! This module contains the functions for
//! forking binary sessions.

use crate::binary::struct_trait::session::Session;

use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;
use std::marker;
use std::panic;
use std::thread::{Builder, JoinHandle};
use std::time::Instant;

#[doc(hidden)]
pub fn fork_with_thread_id<'a, S, P>(
    p: P,
    all_clocks: &mut HashMap<char, Instant>,
) -> (JoinHandle<()>, S::Dual)
where
    S: Session + 'static,
    P: FnOnce(S, &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
{
    let (there, here) = Session::new();
    let other_thread = Builder::new()
        .name(String::from("Thread P"))
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            panic::set_hook(Box::new(|_info| {
                // do nothing
            }));

            match p(there, all_clocks) {
                Ok(()) => (),
                Err(e) => panic!("{}", e.to_string()),
            }
        })
        .unwrap();
    (other_thread, here)
}

/// Creates a child process, and a session with two dual
/// endpoints of type `S` and `S::Dual`. The first endpoint
/// is given to the child process. Returns the
/// second endpoint.
pub fn fork<S, P>(p: P) -> (S::Dual, HashMap<char, Instant>)
where
    S: Session + 'static,
    P: FnOnce(S, &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
{
    let mut all_clocks: HashMap<char, Instant> = HashMap::<char, Instant>::new();
    all_clocks.insert('a', Instant::now());
    all_clocks.insert('b', Instant::now());

    (fork_with_thread_id(p, &mut all_clocks).1, all_clocks)
}
