#![allow(clippy::type_complexity)]

mod baking_timed_mod;

#[test]
fn baking_timed_timeless_tests() {
    baking_timed_mod::timeless::main();
}

#[test]
fn baking_timed_timers_tests() {
    baking_timed_mod::timers::main();
}

#[test]
fn baking_timed_timers_err_tests() {
    baking_timed_mod::timers_err::main();
}
