mod cancel;
mod cases;
mod scribble;
mod unit;

#[test]
fn unit_tests() {
    // Checker
    unit::checker::test_checker();

    // Role
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

    // SessionMPST
    unit::sessionmpst::sessionmpst_fields();
    unit::sessionmpst::sessionmpst_methods();
}

#[test]
#[should_panic]
fn unit_tests_panic_test_checker_panic_stack() {
    unit::checker_panic::test_checker_panic_stack();
}

#[test]
#[should_panic]
fn unit_tests_panic_test_checker_panic_name() {
    unit::checker_panic::test_checker_panic_name();
}

#[test]
fn cases_tests_binary() {
    // Binary
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
    // cases::nested_choices;

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
    cases::a_usecase_recursive::run_a_usecase_recursive_checker();

    // Usecase recursive B
    cases::b_usecase_recursive::run_b_usecase_recursive();
    cases::b_usecase_recursive::run_b_usecase_recursive_checker();

    // Usecase recursive C
    cases::c_usecase_recursive::run_c_usecase_recursive();
    cases::c_usecase_recursive::run_c_usecase_recursive_checker();
}

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
    cases::macro_multi_sessionmpst::basic_macros();

    // Macro multi send-recv
    cases::macro_multi_send_recv_sessionmpst::test_new_send();

    // Macro multi choice
    cases::macro_multi_choice::test_new_choice_full();
    cases::macro_multi_choice::test_new_choice_close();

    // Macro multi recursion
    cases::macro_multi_recursion::new_run_usecase_recursive();
}

#[test]
fn scribble_tests() {
    scribble::top_down::top_down_approach();
    scribble::top_down_recursive::top_down_approach();
}

#[test]
fn canceling() {
    cancel::cancel_1::main();
    cancel::cancel_2::main();
    cancel::cancel_3::main();
    cancel::cancel_4::main();
    cancel::cancel_5::main();
    cancel::cancel_6::main();
    cancel::cancel_7::main();
    cancel::cancel_8::main();
    cancel::cancel_9::main();
    cancel::cancel_10::main();
}

#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    // Infinite types
    t.pass("tests/infinite_type/work.rs");
    t.compile_fail("tests/infinite_type/fail.rs");
    t.compile_fail("tests/infinite_type/fail_2.rs");
}

pub fn main() {}
