#![allow(clippy::type_complexity)]

mod timed_binary_mod;

#[test]
fn tests_binary() {
    // Tests for sesh
    timed_binary_mod::main::ping_works();
    timed_binary_mod::main::head_str();
    timed_binary_mod::main::tail_str();
    timed_binary_mod::main::new_types();
    timed_binary_mod::main::new_types_cancel();
    timed_binary_mod::main::simple_calc_works();
    timed_binary_mod::main::nice_calc_works();
    timed_binary_mod::main::cancel_recv_works();
    timed_binary_mod::main::cancel_send_works();
    timed_binary_mod::main::delegation_works();
    timed_binary_mod::main::closure_works();
    timed_binary_mod::main::recursion_works();
    timed_binary_mod::main::selection_works();
    timed_binary_mod::main::cancel_recursion();
}
