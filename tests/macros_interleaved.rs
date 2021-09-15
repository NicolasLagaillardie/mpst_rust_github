mod macros_interleaved_mod;

// Tests for macros with more than two participants
#[test]
fn macro_basics() {
    // Macro multi send-recv
    macros_interleaved_mod::macro_multi_send_recv_meshedchannels::test_new_send();

    // Macro multi choice
    macros_interleaved_mod::macro_multi_choice::test_new_choice_full();
    macros_interleaved_mod::macro_multi_choice::test_new_choice_close();

    // Macro multi recursion
    macros_interleaved_mod::macro_multi_recursion::new_run_usecase_recursive();

    // Macro multi recursion for shorting
    macros_interleaved_mod::long_simple_three_mpst_short::main();

    // Macro recursive
    macros_interleaved_mod::macro_recursive::run_macro_recursive();
}
