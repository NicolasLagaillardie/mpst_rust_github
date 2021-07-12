mod cancel;
mod cases;
mod cases_short;
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
fn cases_tests_binary() {
    // Tests for sesh
    cases::binary::ping_works();
    cases::binary::head_str();
    cases::binary::tail_str();
    cases::binary::new_types();
    cases::binary::new_types_cancel();
    cases::binary::simple_calc_works();
    cases::binary::nice_calc_works();
    cases::binary::cancel_recv_works();
    cases::binary::cancel_send_works();
    cases::binary::delegation_works();
    cases::binary::closure_works();
    cases::binary::recursion_works();
    cases::binary::selection_works();
    cases::binary::cancel_recursion();
}

// Tests for basic functions and macros with three participants
#[test]
fn cases_tests_mpst_simple() {
    // Simple
    cases::simple::simple_triple_endpoints();
    cases::simple::simple_triple_endpoints_checker();

    // Choose
    cases::choose::simple_choice();
    cases::choose::simple_choice_checker();

    // Choose 2 A
    cases::a_choose_2::double_choice();
    cases::a_choose_2::double_choice_checker();

    // Choose 2 A
    cases::b_choose_2::double_choice();
    cases::b_choose_2::double_choice_checker();

    // Choose 2 A
    cases::c_choose_2::double_choice();
    cases::c_choose_2::double_choice_checker();

    // Nested choice
    cases::nested_choices::nested_choice();

    // Usecase simple A
    cases::a_usecase::run_a_usecase_left();
    cases::a_usecase::run_a_usecase_right();
    cases::a_usecase::run_a_usecase_checker();

    // Usecase simple B
    cases::b_usecase::run_b_usecase_left();
    cases::b_usecase::run_b_usecase_right();
    cases::b_usecase::run_b_usecase_checker();

    // Usecase simple C
    cases::c_usecase::run_c_usecase_left();
    cases::c_usecase::run_c_usecase_right();
    cases::c_usecase::run_c_usecase_checker();

    // Usecase recursive A
    cases::a_usecase_recursive::run_a_usecase_recursive();
    // cases::a_usecase_recursive::run_a_usecase_recursive_checker();

    // Usecase recursive B
    cases::b_usecase_recursive::run_b_usecase_recursive();
    // cases::b_usecase_recursive::run_b_usecase_recursive_checker();

    // Usecase recursive C
    cases::c_usecase_recursive::run_c_usecase_recursive();
    // cases::c_usecase_recursive::run_c_usecase_recursive_checker();
}

// Tests for basic functions and macros with more than three participants
#[test]
fn cases_tests_mpst_macro() {
    // Macro basics
    cases::macro_basics::basic_macros_send();
    cases::macro_basics::basic_macros_recv();

    // Macro choice
    cases::macro_choice::run_usecase_right();
    cases::macro_choice::run_usecase_left();

    // Macro recursive
    cases::macro_recursive::run_macro_recursive();

    // Macro multi basics
    cases::macro_multi_meshedchannels::basic_macros();

    // Macro multi send-recv
    cases::macro_multi_send_recv_meshedchannels::test_new_send();

    // Macro multi choice
    cases::macro_multi_choice::test_new_choice_full();
    cases::macro_multi_choice::test_new_choice_close();

    // Macro multi recursion
    cases::macro_multi_recursion::new_run_usecase_recursive();

    // Macro multi recursion for shorting
    cases::long_simple_three_mpst_short::shorten_main();
}

// Tests for baking with three participants
#[test]
fn cases_tests_mpst_simple_short() {
    // Simple
    cases_short::simple::simple_triple_endpoints();
    cases_short::simple::simple_triple_endpoints_checker();

    // Choose
    cases_short::choose::simple_choice();
    cases_short::choose::simple_choice_checker();

    // Choose 2 A
    cases_short::a_choose_2::double_choice();
    cases_short::a_choose_2::double_choice_checker();

    // Choose 2 A
    cases_short::b_choose_2::double_choice();
    cases_short::b_choose_2::double_choice_checker();

    // Choose 2 A
    cases_short::c_choose_2::double_choice();
    cases_short::c_choose_2::double_choice_checker();

    // Nested choice
    // cases_short::nested_choices;

    // Usecase simple A
    cases_short::a_usecase::run_a_usecase_left();
    cases_short::a_usecase::run_a_usecase_right();
    cases_short::a_usecase::run_a_usecase_checker();

    // Usecase simple B
    cases_short::b_usecase::run_b_usecase_left();
    cases_short::b_usecase::run_b_usecase_right();
    cases_short::b_usecase::run_b_usecase_checker();

    // Usecase simple C
    cases_short::c_usecase::run_c_usecase_left();
    cases_short::c_usecase::run_c_usecase_right();
    cases_short::c_usecase::run_c_usecase_checker();

    // Usecase recursive A
    cases_short::a_usecase_recursive::run_a_usecase_recursive();
    // cases_short::a_usecase_recursive::run_a_usecase_recursive_checker();

    // Usecase recursive B
    cases_short::b_usecase_recursive::run_b_usecase_recursive();
    // cases_short::b_usecase_recursive::run_b_usecase_recursive_checker();

    // Usecase recursive C
    cases_short::c_usecase_recursive::run_c_usecase_recursive();
    // cases_short::c_usecase_recursive::run_c_usecase_recursive_checker();
}

// Tests for baking with more than three participants
#[test]
fn cases_tests_mpst_macro_short() {
    // Macro basics
    cases_short::macro_basics::basic_macros_send();
    cases_short::macro_basics::basic_macros_recv();

    // Macro choice
    cases_short::macro_choice::run_usecase_right();
    cases_short::macro_choice::run_usecase_left();

    // Macro recursive
    cases_short::macro_recursive::run_macro_recursive();

    // Macro multi basics
    cases_short::macro_multi_meshedchannels::basic_macros();

    // Macro multi send-recv
    cases_short::macro_multi_send_recv_meshedchannels::test_new_send();

    // Macro multi choice
    cases_short::macro_multi_choice::test_new_choice_full();
    cases_short::macro_multi_choice::test_new_choice_close();

    // Macro multi recursion
    cases_short::macro_multi_recursion::new_run_usecase_recursive();

    // Macro multi choice with macro of macro
    cases_short::macro_multi_recursion_macro_of_macro::new_run_usecase_recursive();

    // Macro multi choice with macro of macro directly in the baking generation
    cases_short::macro_multi_recursion_short::new_run_usecase_recursive();

    // Macro multi recursion for shorting
    cases_short::long_simple_three_mpst_short::shorten_main();
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
    t.compile_fail("tests/cases/long_simple_three_mpst_short_fail.rs");
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
