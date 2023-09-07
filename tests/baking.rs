#![allow(clippy::type_complexity)]

mod baking_mod;

#[test]
fn unit_tests_meshedchannels() {
    // MeshedChannels methods and fields
    baking_mod::unit_meshedchannels::meshedchannels_fields();
    baking_mod::unit_meshedchannels::meshedchannels_methods();
    baking_mod::unit_meshedchannels::meshedchannels_self_methods();
}

// Tests for baking_mod with three participants
#[test]
fn simple_baking() {
    // Simple
    baking_mod::simple_baking::simple::simple_triple_endpoints();
    baking_mod::simple_baking::simple::simple_triple_endpoints_checker();

    // Choose
    baking_mod::simple_baking::choose::simple_choice_left();
    baking_mod::simple_baking::choose::simple_choice_right();
    baking_mod::simple_baking::choose::simple_choice_checker();

    // Choose 2 A
    baking_mod::simple_baking::a_choose::double_choice_left();
    baking_mod::simple_baking::a_choose::double_choice_right();
    baking_mod::simple_baking::a_choose::double_choice_checker();

    // Choose 2 B
    baking_mod::simple_baking::b_choose::double_choice_left();
    baking_mod::simple_baking::b_choose::double_choice_right();
    baking_mod::simple_baking::b_choose::double_choice_checker();

    // Choose 2 C
    baking_mod::simple_baking::c_choose::double_choice_left();
    baking_mod::simple_baking::c_choose::double_choice_right();
    baking_mod::simple_baking::c_choose::double_choice_checker();

    // Usecase simple A
    baking_mod::simple_baking::a_usecase::run_a_usecase_left();
    baking_mod::simple_baking::a_usecase::run_a_usecase_right();
    baking_mod::simple_baking::a_usecase::run_a_usecase_checker();

    // Usecase simple B
    baking_mod::simple_baking::b_usecase::run_b_usecase_left();
    baking_mod::simple_baking::b_usecase::run_b_usecase_right();
    baking_mod::simple_baking::b_usecase::run_b_usecase_checker();

    // Usecase simple C
    baking_mod::simple_baking::c_usecase::run_c_usecase_left();
    baking_mod::simple_baking::c_usecase::run_c_usecase_right();
    baking_mod::simple_baking::c_usecase::run_c_usecase_checker();

    // Usecase recursive A
    baking_mod::simple_baking::a_usecase_recursive::run_a_usecase_recursive();
    baking_mod::simple_baking::a_usecase_recursive::run_a_usecase_recursive_checker();

    // Usecase recursive B
    baking_mod::simple_baking::b_usecase_recursive::run_b_usecase_recursive();
    baking_mod::simple_baking::b_usecase_recursive::run_b_usecase_recursive_checker();

    // Usecase recursive C
    baking_mod::simple_baking::c_usecase_recursive::run_c_usecase_recursive();
    baking_mod::simple_baking::c_usecase_recursive::run_c_usecase_recursive_checker();
}

// Tests for baking_mod with more than three participants
#[test]
fn macro_baking() {
    // Macro basics
    baking_mod::macros_baking::macro_basics::basic_macros_send();
    baking_mod::macros_baking::macro_basics::basic_macros_recv();

    // Macro choice
    baking_mod::macros_baking::macro_choice::run_usecase_right();
    baking_mod::macros_baking::macro_choice::run_usecase_left();

    // Macro recursive
    baking_mod::macros_baking::macro_recursive::run_macro_recursive();

    // Macro multi basics
    baking_mod::macros_baking::macro_multi_meshedchannels::basic_macros_three();
    baking_mod::macros_baking::macro_multi_meshedchannels::basic_macros_four();

    // Macro multi send-recv
    baking_mod::macros_baking::macro_multi_send_recv_meshedchannels::test_new_send();

    // Macro multi choice
    baking_mod::macros_baking::macro_multi_choice::test_new_choice_full();
    baking_mod::macros_baking::macro_multi_choice::test_new_choice_close();

    // Macro multi recursion
    baking_mod::macros_baking::macro_multi_recursion::new_run_usecase_recursive();

    // Macro multi choice with macro of macro
    baking_mod::macros_baking::macro_multi_recursion_macro_of_macro::new_run_usecase_recursive();

    // Macro multi choice with macro of macro directly in the baking_mod generation
    baking_mod::macros_baking::macro_multi_recursion_short::new_run_usecase_recursive();
}
