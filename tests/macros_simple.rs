#![allow(clippy::type_complexity)]

mod macros_simple_mod;

#[test]
fn unit_tests_meshedchannels() {
    // MeshedChannels methods and fields
    macros_simple_mod::unit_meshedchannels::meshedchannels_fields();
    macros_simple_mod::unit_meshedchannels::meshedchannels_methods();
    macros_simple_mod::unit_meshedchannels::meshedchannels_self_methods();
}

// Tests for macros with three participants but any role
#[test]
fn macro_basics() {
    // Macro macros_simple_mod
    macros_simple_mod::macro_basics::basic_macros_send();
    macros_simple_mod::macro_basics::basic_macros_recv();

    // Macro choice
    macros_simple_mod::macro_choice::run_usecase_right();
    macros_simple_mod::macro_choice::run_usecase_left();
}
