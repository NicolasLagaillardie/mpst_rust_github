mod basics_mod;

#[test]
fn unit_tests_basics_roles() {
    // Role methods and fields
    // RoleEnd
    basics_mod::unit_roles::role_end_fields_1();
    basics_mod::unit_roles::role_end_fields_2();

    // RoleA
    basics_mod::unit_roles::role_a_fields();

    // RoleB
    basics_mod::unit_roles::role_b_fields();

    // RoleC
    basics_mod::unit_roles::role_c_fields();

    // RoleAtoAll
    basics_mod::unit_roles::role_a_to_all_fields();
    basics_mod::unit_roles::role_all_to_a_fields();

    // RoleBtoAll
    basics_mod::unit_roles::role_b_to_all_fields();
    basics_mod::unit_roles::role_all_to_b_fields();

    // RoleCtoAll
    basics_mod::unit_roles::role_c_to_all_fields();
    basics_mod::unit_roles::role_all_to_c_fields();

    // head_str and tail_str
    basics_mod::unit_roles::role_head_str();
    basics_mod::unit_roles::role_tail_str();

    // RoleBroadcast
    basics_mod::unit_roles::role_broadcast_fields_1();
    basics_mod::unit_roles::role_broadcast_fields_2();
}

#[test]
fn from_str() {
    basics_mod::from_str::binary_sessions();
    basics_mod::from_str::meshedchannels();
    basics_mod::from_str::roles();
}

// Tests for basic functions
#[test]
fn simple_basics() {
    // Simple
    basics_mod::simple_basics::simple::simple_triple_endpoints();
    basics_mod::simple_basics::simple::simple_triple_endpoints_checker();

    // Choose
    basics_mod::simple_basics::choose::simple_choice();
    basics_mod::simple_basics::choose::simple_choice_checker();

    // Choose 2 A
    basics_mod::simple_basics::a_choose::double_choice();
    basics_mod::simple_basics::a_choose::double_choice_checker();

    // Choose 2 A
    basics_mod::simple_basics::b_choose::double_choice();
    basics_mod::simple_basics::b_choose::double_choice_checker();

    // Choose 2 A
    basics_mod::simple_basics::c_choose::double_choice();
    basics_mod::simple_basics::c_choose::double_choice_checker();

    // Usecase simple A
    basics_mod::simple_basics::a_usecase::run_a_usecase_left();
    basics_mod::simple_basics::a_usecase::run_a_usecase_right();
    basics_mod::simple_basics::a_usecase::run_a_usecase_checker();

    // Usecase simple B
    basics_mod::simple_basics::b_usecase::run_b_usecase_left();
    basics_mod::simple_basics::b_usecase::run_b_usecase_right();
    basics_mod::simple_basics::b_usecase::run_b_usecase_checker();

    // Usecase simple C
    basics_mod::simple_basics::c_usecase::run_c_usecase_left();
    basics_mod::simple_basics::c_usecase::run_c_usecase_right();
    basics_mod::simple_basics::c_usecase::run_c_usecase_checker();

    // Usecase recursive A
    basics_mod::simple_basics::a_usecase_recursive::run_a_usecase_recursive();
    basics_mod::simple_basics::a_usecase_recursive::run_a_usecase_recursive_checker();

    // Usecase recursive B
    basics_mod::simple_basics::b_usecase_recursive::run_b_usecase_recursive();
    basics_mod::simple_basics::b_usecase_recursive::run_b_usecase_recursive_checker();

    // Usecase recursive C
    basics_mod::simple_basics::c_usecase_recursive::run_c_usecase_recursive();
    basics_mod::simple_basics::c_usecase_recursive::run_c_usecase_recursive_checker();
}
