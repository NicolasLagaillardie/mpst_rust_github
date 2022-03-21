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
    // binary_timed_mod::main::new_types();
    // binary_timed_mod::main::new_types_cancel();
    binary_timed_mod::main::simple_calc_works();
    // binary_timed_mod::main::nice_calc_works();
    // binary_timed_mod::main::cancel_recv_works();
    // binary_timed_mod::main::cancel_send_works();
    // binary_timed_mod::main::delegation_works();
    // binary_timed_mod::main::closure_works();
    // binary_timed_mod::main::recursion_works();
    // binary_timed_mod::main::selection_works();
    // binary_timed_mod::main::cancel_recursion();
}
