#![allow(clippy::type_complexity)]

mod binary_timed_mod;

#[test]
fn tests_binary_timed() {
    // Tests for sesh
    // binary_timed_mod::main::ping_works();
    binary_timed_mod::main::head_str();
    binary_timed_mod::main::tail_str();
    binary_timed_mod::main::self_head_str();
    binary_timed_mod::main::self_tail_str();
    binary_timed_mod::main::constraint_start_excluded();
    binary_timed_mod::main::constraint_start_included();
    binary_timed_mod::main::constraint_end_excluded();
    binary_timed_mod::main::constraint_end_included();
    binary_timed_mod::main::constraint_start_excluded_end_excluded();
    binary_timed_mod::main::constraint_start_excluded_end_included();
    binary_timed_mod::main::constraint_start_included_end_excluded();
    binary_timed_mod::main::constraint_start_included_end_included();
    binary_timed_mod::main::simple_calc_works();
}

#[test]
#[should_panic]
fn send_upper_timeout_panics() {
    binary_timed_mod::main::simple_calc_send_upper_timeout_panics();
}

#[test]
#[should_panic]
fn send_lower_timeout_panics() {
    binary_timed_mod::main::simple_calc_send_lower_timeout_panics();
}

#[test]
#[should_panic]
fn choose_lower_timeout_panics() {
    binary_timed_mod::main::simple_calc_choose_lower_timeout_panics();
}

#[test]
#[should_panic]
fn choose_upper_timeout_panics() {
    binary_timed_mod::main::simple_calc_choose_upper_timeout_panics();
}
