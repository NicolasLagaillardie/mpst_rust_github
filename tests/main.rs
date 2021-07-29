mod binary;
mod cancel;
mod checking;
mod graph;
mod http;
mod macros_baking;
mod macros_basics;
mod scribble;
mod simple_baking;
mod simple_basics;
mod tcp;
mod unit;

use ntest::timeout;

#[test]
fn unit_tests_basics_roles() {
    // Role methods and fields
    // RoleEnd
    unit::basics_roles::role_end_fields_1();
    unit::basics_roles::role_end_fields_2();

    // RoleA
    unit::basics_roles::role_a_fields();

    // RoleB
    unit::basics_roles::role_b_fields();

    // RoleC
    unit::basics_roles::role_c_fields();

    // RoleAtoAll
    unit::basics_roles::role_a_to_all_fields();
    unit::basics_roles::role_all_to_a_fields();

    // RoleBtoAll
    unit::basics_roles::role_b_to_all_fields();
    unit::basics_roles::role_all_to_b_fields();

    // RoleCtoAll
    unit::basics_roles::role_c_to_all_fields();
    unit::basics_roles::role_all_to_c_fields();

    // head_str and tail_str
    unit::basics_roles::role_head_str();
    unit::basics_roles::role_tail_str();

    // RoleBroadcast
    unit::basics_roles::role_broadcast_fields_1();
    unit::basics_roles::role_broadcast_fields_2();
}

#[test]
fn unit_tests_macros_roles() {
    // Role methods and fields

    // RoleA
    unit::macros_roles::role_a_fields();

    // RoleB
    unit::macros_roles::role_b_fields();

    // RoleC
    unit::macros_roles::role_c_fields();

    // RoleAtoAll
    unit::macros_roles::role_a_to_all_fields();
    unit::macros_roles::role_all_to_a_fields();

    // RoleBtoAll
    unit::macros_roles::role_b_to_all_fields();
    unit::macros_roles::role_all_to_b_fields();

    // RoleCtoAll
    unit::macros_roles::role_c_to_all_fields();
    unit::macros_roles::role_all_to_c_fields();

    // head_str and tail_str
    unit::macros_roles::role_head_str();
    unit::macros_roles::role_tail_str();
}

#[test]
fn unit_tests_meshedchannels() {
    // MeshedChannels methods and fields
    unit::basics_meshedchannels::meshedchannels_fields();
    unit::basics_meshedchannels::meshedchannels_methods();
    unit::basics_meshedchannels::meshedchannels_self_methods();

    // MeshedChannels methods and fields
    unit::macros_meshedchannels::meshedchannels_fields();
    unit::macros_meshedchannels::meshedchannels_methods();
    unit::macros_meshedchannels::meshedchannels_self_methods();

    // MeshedChannels methods and fields
    unit::baking_meshedchannels::meshedchannels_fields();
    unit::baking_meshedchannels::meshedchannels_methods();
    unit::baking_meshedchannels::meshedchannels_self_methods();
}

#[test]
fn tests_binary() {
    // Tests for sesh
    binary::ping_works();
    binary::head_str();
    binary::tail_str();
    binary::new_types();
    binary::new_types_cancel();
    binary::simple_calc_works();
    binary::nice_calc_works();
    binary::cancel_recv_works();
    binary::cancel_send_works();
    binary::delegation_works();
    binary::closure_works();
    binary::recursion_works();
    binary::selection_works();
    binary::cancel_recursion();
}

// Tests for basic functions
#[test]
fn simple_basics() {
    // Simple
    simple_basics::simple::simple_triple_endpoints();
    simple_basics::simple::simple_triple_endpoints_checker();

    // Choose
    simple_basics::choose::simple_choice();
    simple_basics::choose::simple_choice_checker();

    // Choose 2 A
    simple_basics::a_choose::double_choice();
    simple_basics::a_choose::double_choice_checker();

    // Choose 2 A
    simple_basics::b_choose::double_choice();
    simple_basics::b_choose::double_choice_checker();

    // Choose 2 A
    simple_basics::c_choose::double_choice();
    simple_basics::c_choose::double_choice_checker();

    // Usecase simple A
    simple_basics::a_usecase::run_a_usecase_left();
    simple_basics::a_usecase::run_a_usecase_right();
    simple_basics::a_usecase::run_a_usecase_checker();

    // Usecase simple B
    simple_basics::b_usecase::run_b_usecase_left();
    simple_basics::b_usecase::run_b_usecase_right();
    simple_basics::b_usecase::run_b_usecase_checker();

    // Usecase simple C
    simple_basics::c_usecase::run_c_usecase_left();
    simple_basics::c_usecase::run_c_usecase_right();
    simple_basics::c_usecase::run_c_usecase_checker();

    // Usecase recursive A
    simple_basics::a_usecase_recursive::run_a_usecase_recursive();
    simple_basics::a_usecase_recursive::run_a_usecase_recursive_checker();

    // Usecase recursive B
    simple_basics::b_usecase_recursive::run_b_usecase_recursive();
    simple_basics::b_usecase_recursive::run_b_usecase_recursive_checker();

    // Usecase recursive C
    simple_basics::c_usecase_recursive::run_c_usecase_recursive();
    simple_basics::c_usecase_recursive::run_c_usecase_recursive_checker();
}

// Tests for macros with three participants but any role
#[test]
fn macro_basics() {
    // Macro basics
    macros_basics::macro_basics::basic_macros_send();
    macros_basics::macro_basics::basic_macros_recv();

    // Macro choice
    macros_basics::macro_choice::run_usecase_right();
    macros_basics::macro_choice::run_usecase_left();

    // Macro recursive
    macros_basics::macro_recursive::run_macro_recursive();

    // Macro multi basics
    macros_basics::macro_multi_meshedchannels::basic_macros();

    // Macro multi send-recv
    macros_basics::macro_multi_send_recv_meshedchannels::test_new_send();

    // Macro multi choice
    macros_basics::macro_multi_choice::test_new_choice_full();
    macros_basics::macro_multi_choice::test_new_choice_close();

    // Macro multi recursion
    macros_basics::macro_multi_recursion::new_run_usecase_recursive();

    // Macro multi recursion for shorting
    macros_basics::long_simple_three_mpst_short::shorten_main();
}

// Tests for baking with three participants
#[test]
fn simple_baking() {
    // Simple
    simple_baking::simple::simple_triple_endpoints();
    simple_baking::simple::simple_triple_endpoints_checker();

    // Choose
    simple_baking::choose::simple_choice();
    simple_baking::choose::simple_choice_checker();

    // Choose 2 A
    simple_baking::a_choose::double_choice();
    simple_baking::a_choose::double_choice_checker();

    // Choose 2 A
    simple_baking::b_choose::double_choice();
    simple_baking::b_choose::double_choice_checker();

    // Choose 2 A
    simple_baking::c_choose::double_choice();
    simple_baking::c_choose::double_choice_checker();

    // Usecase simple A
    simple_baking::a_usecase::run_a_usecase_left();
    simple_baking::a_usecase::run_a_usecase_right();
    simple_baking::a_usecase::run_a_usecase_checker();

    // Usecase simple B
    simple_baking::b_usecase::run_b_usecase_left();
    simple_baking::b_usecase::run_b_usecase_right();
    simple_baking::b_usecase::run_b_usecase_checker();

    // Usecase simple C
    simple_baking::c_usecase::run_c_usecase_left();
    simple_baking::c_usecase::run_c_usecase_right();
    simple_baking::c_usecase::run_c_usecase_checker();

    // Usecase recursive A
    simple_baking::a_usecase_recursive::run_a_usecase_recursive();
    simple_baking::a_usecase_recursive::run_a_usecase_recursive_checker();

    // Usecase recursive B
    simple_baking::b_usecase_recursive::run_b_usecase_recursive();
    simple_baking::b_usecase_recursive::run_b_usecase_recursive_checker();

    // Usecase recursive C
    simple_baking::c_usecase_recursive::run_c_usecase_recursive();
    simple_baking::c_usecase_recursive::run_c_usecase_recursive_checker();
}

// Tests for baking with more than three participants
#[test]
fn macro_baking() {
    // Macro basics
    macros_baking::macro_basics::basic_macros_send();
    macros_baking::macro_basics::basic_macros_recv();

    // Macro choice
    macros_baking::macro_choice::run_usecase_right();
    macros_baking::macro_choice::run_usecase_left();

    // Macro recursive
    macros_baking::macro_recursive::run_macro_recursive();

    // Macro multi basics
    macros_baking::macro_multi_meshedchannels::basic_macros();

    // Macro multi send-recv
    macros_baking::macro_multi_send_recv_meshedchannels::test_new_send();

    // Macro multi choice
    macros_baking::macro_multi_choice::test_new_choice_full();
    macros_baking::macro_multi_choice::test_new_choice_close();

    // Macro multi recursion
    macros_baking::macro_multi_recursion::new_run_usecase_recursive();

    // Macro multi choice with macro of macro
    macros_baking::macro_multi_recursion_macro_of_macro::new_run_usecase_recursive();

    // Macro multi choice with macro of macro directly in the baking generation
    macros_baking::macro_multi_recursion_short::new_run_usecase_recursive();

    // Macro multi recursion for shorting
    macros_baking::long_simple_three_mpst_short::shorten_main();
}

#[test]
fn scribble_tests() {
    // Test code generated from Scribble
    scribble::top_down::top_down_approach();
    scribble::top_down_recursive::top_down_approach();
}

#[test]
fn canceling() {
    cancel::cancel_01::main();
    cancel::cancel_02::main();
    cancel::cancel_03::main();
    cancel::cancel_04::main();
    cancel::cancel_05::main();
    cancel::cancel_06::main();
    cancel::cancel_07::main();
    cancel::cancel_08::main();
    cancel::cancel_09::main();
    cancel::cancel_10::main();
    cancel::cancel_11::main();
    cancel::cancel_12::main();
    cancel::cancel_13::main();
    cancel::cancel_14::main();
}

#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    // Infinite types
    t.pass("tests/infinite_type/work.rs");
    t.compile_fail("tests/infinite_type/fail.rs");
    t.compile_fail("tests/infinite_type/fail_2.rs");

    // Macro multi recursion for shorting
    t.compile_fail("tests/macros_basics/long_simple_three_mpst_short_fail.rs");
}

#[test]
fn tcp() {
    tcp::binary::main();
    tcp::binary_fail::main();
}

#[test]
#[timeout(60000)]
fn http() {
    http::simple_http_get::main();
    http::simple_http_post::main();
    http::simple_https_get::main();
    http::complex_https_get::main();
    http::binary_http_get::main();
    http::o_auth::main();
    http::o_auth_fail_too_true::main();
    http::o_auth_fail_too_false::main();
}

#[test]
fn graph() {
    graph::simple::simple_triple_endpoints();
}

#[test]
fn from_str() {
    unit::from_str::binary_sessions();
    unit::from_str::meshedchannels();
    unit::from_str::roles();
}

#[test]
fn checking_basics() {
    checking::basics::checking_simple::main();
    checking::basics::checking_choice::main();
    checking::basics::checking_recursion::main();
}

#[test]
fn checking_complex() {
    checking::complex::commit_protocol::main();
}

#[test]
pub fn main() {
    assert!(true);
}
