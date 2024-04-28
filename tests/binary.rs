mod binary_mod;

#[test]
fn tests_binary() {
    // Tests for sesh
    binary_mod::main::ping_works();
    binary_mod::main::head_str();
    binary_mod::main::tail_str();
    binary_mod::main::self_head_str();
    binary_mod::main::self_tail_str();
    binary_mod::main::new_types();
    binary_mod::main::new_types_cancel();
    binary_mod::main::simple_calc_works();
    binary_mod::main::nice_calc_works();
    binary_mod::main::cancel_recv_works();
    binary_mod::main::cancel_send_works();
    binary_mod::main::delegation_works();
    binary_mod::main::closure_works();
    binary_mod::main::recursion_works();
    binary_mod::main::selection_works();
    binary_mod::main::cancel_recursion();
}
