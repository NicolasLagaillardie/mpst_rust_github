mod baking_message_mod;

// Tests for baking_mod with more than three participants
#[test]
fn macro_baking() {
    // Macro multi choice with macro of macro and message struct
    baking_message_mod::macro_multi_recursion_message::new_run_usecase_recursive();

    // Macro multi choice with macro of macro and message struct and optional payload
    baking_message_mod::macro_multi_recursion_message_option::new_run_usecase_recursive();
}
