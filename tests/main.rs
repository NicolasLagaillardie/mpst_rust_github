mod baking;
mod basics;
mod binary;
mod cancel;
mod graph;
mod http;
mod infinite_type;
mod scribble;
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
    basics::simple_basics::simple::simple_triple_endpoints();
    basics::simple_basics::simple::simple_triple_endpoints_checker();

    // Choose
    basics::simple_basics::choose::simple_choice();
    basics::simple_basics::choose::simple_choice_checker();

    // Choose 2 A
    basics::simple_basics::a_choose::double_choice();
    basics::simple_basics::a_choose::double_choice_checker();

    // Choose 2 A
    basics::simple_basics::b_choose::double_choice();
    basics::simple_basics::b_choose::double_choice_checker();

    // Choose 2 A
    basics::simple_basics::c_choose::double_choice();
    basics::simple_basics::c_choose::double_choice_checker();

    // Usecase simple A
    basics::simple_basics::a_usecase::run_a_usecase_left();
    basics::simple_basics::a_usecase::run_a_usecase_right();
    basics::simple_basics::a_usecase::run_a_usecase_checker();

    // Usecase simple B
    basics::simple_basics::b_usecase::run_b_usecase_left();
    basics::simple_basics::b_usecase::run_b_usecase_right();
    basics::simple_basics::b_usecase::run_b_usecase_checker();

    // Usecase simple C
    basics::simple_basics::c_usecase::run_c_usecase_left();
    basics::simple_basics::c_usecase::run_c_usecase_right();
    basics::simple_basics::c_usecase::run_c_usecase_checker();

    // Usecase recursive A
    basics::simple_basics::a_usecase_recursive::run_a_usecase_recursive();
    basics::simple_basics::a_usecase_recursive::run_a_usecase_recursive_checker();

    // Usecase recursive B
    basics::simple_basics::b_usecase_recursive::run_b_usecase_recursive();
    basics::simple_basics::b_usecase_recursive::run_b_usecase_recursive_checker();

    // Usecase recursive C
    basics::simple_basics::c_usecase_recursive::run_c_usecase_recursive();
    basics::simple_basics::c_usecase_recursive::run_c_usecase_recursive_checker();
}

// Tests for macros with three participants but any role
#[test]
fn macro_basics() {
    // Macro basics
    basics::macros_basics::macro_basics::basic_macros_send();
    basics::macros_basics::macro_basics::basic_macros_recv();

    // Macro choice
    basics::macros_basics::macro_choice::run_usecase_right();
    basics::macros_basics::macro_choice::run_usecase_left();

    // Macro recursive
    basics::macros_basics::macro_recursive::run_macro_recursive();

    // Macro multi basics
    basics::macros_basics::macro_multi_meshedchannels::basic_macros();

    // Macro multi send-recv
    basics::macros_basics::macro_multi_send_recv_meshedchannels::test_new_send();

    // Macro multi choice
    basics::macros_basics::macro_multi_choice::test_new_choice_full();
    basics::macros_basics::macro_multi_choice::test_new_choice_close();

    // Macro multi recursion
    basics::macros_basics::macro_multi_recursion::new_run_usecase_recursive();

    // Macro multi recursion for shorting
    basics::macros_basics::long_simple_three_mpst_short::shorten_main();
}

// Tests for baking with three participants
#[test]
fn simple_baking() {
    // Simple
    baking::simple_baking::simple::simple_triple_endpoints();
    baking::simple_baking::simple::simple_triple_endpoints_checker();

    // Choose
    baking::simple_baking::choose::simple_choice();
    baking::simple_baking::choose::simple_choice_checker();

    // Choose 2 A
    baking::simple_baking::a_choose::double_choice();
    baking::simple_baking::a_choose::double_choice_checker();

    // Choose 2 A
    baking::simple_baking::b_choose::double_choice();
    baking::simple_baking::b_choose::double_choice_checker();

    // Choose 2 A
    baking::simple_baking::c_choose::double_choice();
    baking::simple_baking::c_choose::double_choice_checker();

    // Usecase simple A
    baking::simple_baking::a_usecase::run_a_usecase_left();
    baking::simple_baking::a_usecase::run_a_usecase_right();
    baking::simple_baking::a_usecase::run_a_usecase_checker();

    // Usecase simple B
    baking::simple_baking::b_usecase::run_b_usecase_left();
    baking::simple_baking::b_usecase::run_b_usecase_right();
    baking::simple_baking::b_usecase::run_b_usecase_checker();

    // Usecase simple C
    baking::simple_baking::c_usecase::run_c_usecase_left();
    baking::simple_baking::c_usecase::run_c_usecase_right();
    baking::simple_baking::c_usecase::run_c_usecase_checker();

    // Usecase recursive A
    baking::simple_baking::a_usecase_recursive::run_a_usecase_recursive();
    baking::simple_baking::a_usecase_recursive::run_a_usecase_recursive_checker();

    // Usecase recursive B
    baking::simple_baking::b_usecase_recursive::run_b_usecase_recursive();
    baking::simple_baking::b_usecase_recursive::run_b_usecase_recursive_checker();

    // Usecase recursive C
    baking::simple_baking::c_usecase_recursive::run_c_usecase_recursive();
    baking::simple_baking::c_usecase_recursive::run_c_usecase_recursive_checker();
}

// Tests for baking with more than three participants
#[test]
fn macro_baking() {
    // Macro basics
    baking::macros_baking::macro_basics::basic_macros_send();
    baking::macros_baking::macro_basics::basic_macros_recv();

    // Macro choice
    baking::macros_baking::macro_choice::run_usecase_right();
    baking::macros_baking::macro_choice::run_usecase_left();

    // Macro recursive
    baking::macros_baking::macro_recursive::run_macro_recursive();

    // Macro multi basics
    baking::macros_baking::macro_multi_meshedchannels::basic_macros();

    // Macro multi send-recv
    baking::macros_baking::macro_multi_send_recv_meshedchannels::test_new_send();

    // Macro multi choice
    baking::macros_baking::macro_multi_choice::test_new_choice_full();
    baking::macros_baking::macro_multi_choice::test_new_choice_close();

    // Macro multi recursion
    baking::macros_baking::macro_multi_recursion::new_run_usecase_recursive();

    // Macro multi choice with macro of macro
    baking::macros_baking::macro_multi_recursion_macro_of_macro::new_run_usecase_recursive();

    // Macro multi choice with macro of macro directly in the baking generation
    baking::macros_baking::macro_multi_recursion_short::new_run_usecase_recursive();

    // Macro multi recursion for shorting
    baking::macros_baking::long_simple_three_mpst_short::shorten_main();
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
fn infinite_type_pass() {
    infinite_type::work::main();
}

#[test]
fn infinite_type_fail() {
    let t = trybuild::TestCases::new();

    // Infinite types
    t.compile_fail("tests/infinite_type/fail.rs");
    t.compile_fail("tests/infinite_type/fail_2.rs");
}

#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();

    // Macro multi recursion for shorting
    t.compile_fail("tests/basics/macros_basics/long_simple_three_mpst_short_fail.rs");

    // Macro multi recursion for shorting
    t.compile_fail("tests/baking/macros_baking/long_simple_three_mpst_short_fail.rs");
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
pub fn main() {
    assert!(true);
}
