mod baking;
mod basics;
pub mod basics_macros;
mod binary;
mod cancel;
mod http;
mod scribble;
mod tcp;
mod unit;

use ntest::timeout;

#[test]
fn unit_tests() {
    // Checker result
    unit::checker::test_checker();

    // Role methods and fields
    unit::roles::role_end_fields_1();
    unit::roles::role_end_fields_2();
    unit::roles::role_a_to_all_fields();
    unit::roles::role_all_to_a_fields();
    unit::roles::role_b_to_all_fields();
    unit::roles::role_all_to_b_fields();
    unit::roles::role_c_to_all_fields();
    unit::roles::role_all_to_c_fields();
    unit::roles::role_head_str();
    unit::roles::role_tail_str();

    // MeshedChannels methods and fields
    unit::meshedchannels::meshedchannels_fields();
    unit::meshedchannels::meshedchannels_methods();
}

#[test]
#[should_panic]
fn unit_tests_panic_test_checker_panic_stack() {
    // Test panic with wrong stack
    unit::checker_panic::test_checker_panic_stack();
}

#[test]
#[should_panic]
fn unit_tests_panic_test_checker_panic_name() {
    // Test panic with wrong name
    unit::checker_panic::test_checker_panic_name();
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
fn basics() {
    // Simple
    basics::simple::simple_triple_endpoints();
    basics::simple::simple_triple_endpoints_checker();

    // Choose
    basics::choose::simple_choice();
    basics::choose::simple_choice_checker();

    // Choose 2 A
    basics::a_choose_2::double_choice();
    basics::a_choose_2::double_choice_checker();

    // Choose 2 A
    basics::b_choose_2::double_choice();
    basics::b_choose_2::double_choice_checker();

    // Choose 2 A
    basics::c_choose_2::double_choice();
    basics::c_choose_2::double_choice_checker();

    // Usecase simple A
    basics::a_usecase::run_a_usecase_left();
    basics::a_usecase::run_a_usecase_right();
    basics::a_usecase::run_a_usecase_checker();

    // Usecase simple B
    basics::b_usecase::run_b_usecase_left();
    basics::b_usecase::run_b_usecase_right();
    basics::b_usecase::run_b_usecase_checker();

    // Usecase simple C
    basics::c_usecase::run_c_usecase_left();
    basics::c_usecase::run_c_usecase_right();
    basics::c_usecase::run_c_usecase_checker();

    // Usecase recursive A
    basics::a_usecase_recursive::run_a_usecase_recursive();
    // a_usecase_recursive::run_a_usecase_recursive_checker();

    // Usecase recursive B
    basics::b_usecase_recursive::run_b_usecase_recursive();
    // b_usecase_recursive::run_b_usecase_recursive_checker();

    // Usecase recursive C
    basics::c_usecase_recursive::run_c_usecase_recursive();
    // c_usecase_recursive::run_c_usecase_recursive_checker();
}

// Tests for macros with three participants but any role
#[test]
fn cases_tests_mpst_macro() {
    // Macro basics
    basics_macros::macro_basics::basic_macros_send();
    basics_macros::macro_basics::basic_macros_recv();

    // Macro choice
    basics_macros::macro_choice::run_usecase_right();
    basics_macros::macro_choice::run_usecase_left();

    // Macro recursive
    basics_macros::macro_recursive::run_macro_recursive();

    // Macro multi basics
    basics_macros::macro_multi_meshedchannels::basic_macros();

    // Macro multi send-recv
    basics_macros::macro_multi_send_recv_meshedchannels::test_new_send();

    // Macro multi choice
    basics_macros::macro_multi_choice::test_new_choice_full();
    basics_macros::macro_multi_choice::test_new_choice_close();

    // Macro multi recursion
    basics_macros::macro_multi_recursion::new_run_usecase_recursive();

    // Macro multi recursion for shorting
    basics_macros::long_simple_three_mpst_short::shorten_main();
}

// Tests for baking with three participants
#[test]
fn cases_tests_mpst_simple_short() {
    // Simple
    baking::simple::simple_triple_endpoints();
    baking::simple::simple_triple_endpoints_checker();

    // Choose
    baking::basics::choose::simple_choice();
    baking::basics::choose::simple_choice_checker();

    // Choose 2 A
    baking::basics::a_choose_2::double_choice();
    baking::basics::a_choose_2::double_choice_checker();

    // Choose 2 A
    baking::basics::b_choose_2::double_choice();
    baking::basics::b_choose_2::double_choice_checker();

    // Choose 2 A
    baking::basics::c_choose_2::double_choice();
    baking::basics::c_choose_2::double_choice_checker();

    // Usecase simple A
    baking::basics::a_usecase::run_a_usecase_left();
    baking::basics::a_usecase::run_a_usecase_right();
    baking::basics::a_usecase::run_a_usecase_checker();

    // Usecase simple B
    baking::basics::b_usecase::run_b_usecase_left();
    baking::basics::b_usecase::run_b_usecase_right();
    baking::basics::b_usecase::run_b_usecase_checker();

    // Usecase simple C
    baking::basics::c_usecase::run_c_usecase_left();
    baking::basics::c_usecase::run_c_usecase_right();
    baking::basics::c_usecase::run_c_usecase_checker();

    // Usecase recursive A
    baking::basics::a_usecase_recursive::run_a_usecase_recursive();
    // baking::a_usecase_recursive::run_a_usecase_recursive_checker();

    // Usecase recursive B
    baking::basics::b_usecase_recursive::run_b_usecase_recursive();
    // baking::b_usecase_recursive::run_b_usecase_recursive_checker();

    // Usecase recursive C
    baking::basics::c_usecase_recursive::run_c_usecase_recursive();
    // baking::c_usecase_recursive::run_c_usecase_recursive_checker();
}

// Tests for baking with more than three participants
#[test]
fn cases_tests_mpst_macro_short() {
    // Macro basics
    baking::macros::macro_basics::basic_macros_send();
    baking::macros::macro_basics::basic_macros_recv();

    // Macro choice
    baking::macros::macro_choice::run_usecase_right();
    baking::macros::macro_choice::run_usecase_left();

    // Macro recursive
    baking::macros::macro_recursive::run_macro_recursive();

    // Macro multi basics
    baking::macros::macro_multi_meshedchannels::basic_macros();

    // Macro multi send-recv
    baking::macros::macro_multi_send_recv_meshedchannels::test_new_send();

    // Macro multi choice
    baking::macros::macro_multi_choice::test_new_choice_full();
    baking::macros::macro_multi_choice::test_new_choice_close();

    // Macro multi recursion
    baking::macros::macro_multi_recursion::new_run_usecase_recursive();

    // Macro multi choice with macro of macro
    baking::macros::macro_multi_recursion_macro_of_macro::new_run_usecase_recursive();

    // Macro multi choice with macro of macro directly in the baking generation
    baking::macros::macro_multi_recursion_short::new_run_usecase_recursive();

    // Macro multi recursion for shorting
    baking::macros::long_simple_three_mpst_short::shorten_main();
}

#[test]
fn scribble_tests() {
    // Test code generated from Scribble
    scribble::top_down::top_down_approach();
    scribble::top_down_recursive::top_down_approach();
}

#[test]
#[timeout(30000)]
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
    t.compile_fail("tests/basics_macros/long_simple_three_mpst_short_fail.rs");
}

#[test]
fn tcp() {
    tcp::binary::main();
    tcp::binary_fail::main();
}

#[test]
#[timeout(60000)]
fn http() {
    println!("Starting http");
    http::simple_http_get::main();
    println!("simple_http_get done");
    http::simple_http_post::main();
    println!("simple_http_post done");
    http::simple_https_get::main();
    println!("simple_https_get done");
    http::complex_https_get::main();
    println!("complex_https_get done");
    http::binary_http_get::main();
    println!("binary_http_get done");
    http::o_auth::main();
    println!("o_auth done");
    http::o_auth_fail_too_true::main();
    println!("o_auth_fail_too_true done");
    http::o_auth_fail_too_false::main();
    println!("o_auth_fail_too_false done");
}

pub fn main() {}
