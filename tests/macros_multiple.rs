mod macros_multiple_mod;

#[test]
fn unit_tests_macros_roles() {
    // Role methods and fields

    // RoleA
    macros_multiple_mod::unit_roles::role_a_fields();

    // RoleB
    macros_multiple_mod::unit_roles::role_b_fields();

    // RoleC
    macros_multiple_mod::unit_roles::role_c_fields();

    // RoleAtoAll
    macros_multiple_mod::unit_roles::role_a_to_all_fields();
    macros_multiple_mod::unit_roles::role_all_to_a_fields();

    // RoleBtoAll
    macros_multiple_mod::unit_roles::role_b_to_all_fields();
    macros_multiple_mod::unit_roles::role_all_to_b_fields();

    // RoleCtoAll
    macros_multiple_mod::unit_roles::role_c_to_all_fields();
    macros_multiple_mod::unit_roles::role_all_to_c_fields();

    // head_str and tail_str
    macros_multiple_mod::unit_roles::role_head_str();
    macros_multiple_mod::unit_roles::role_tail_str();
}

#[test]
fn unit_tests_meshedchannels() {
    // MeshedChannels methods and fields
    macros_multiple_mod::unit_meshedchannels::meshedchannels_fields();
    macros_multiple_mod::unit_meshedchannels::meshedchannels_methods();
    macros_multiple_mod::unit_meshedchannels::meshedchannels_self_methods();
}

// Tests for macros with more than two participants
#[test]
fn macro_basics() {
    // Macro multi macros_multiple_mod
    macros_multiple_mod::macro_multi_meshedchannels::basic_macros_three();
    macros_multiple_mod::macro_multi_meshedchannels::basic_macros_four();

    // Macro multi send-recv
    macros_multiple_mod::macro_multi_send_recv_meshedchannels::test_new_send();

    // Macro multi choice
    macros_multiple_mod::macro_multi_choice::test_new_choice_full();
    macros_multiple_mod::macro_multi_choice::test_new_choice_close();

    // Macro multi recursion
    macros_multiple_mod::macro_multi_recursion::new_run_usecase_recursive();

    // Macro recursive
    macros_multiple_mod::macro_recursive::run_macro_recursive();
}
