mod unit;

#[test]
fn tests(){
    let t = trybuild::TestCases::new();
    // Unit tests
    t.pass("tests/unit/checker.rs");
    t.pass("tests/unit/roles.rs");
    t.pass("tests/unit/sessionmpst.rs");
    // Cases tests
    t.pass("tests/cases/binary.rs");
    t.pass("tests/cases/simple.rs");
    t.pass("tests/cases/choose.rs");
    t.pass("tests/cases/a_choose_2.rs");
    t.pass("tests/cases/b_choose_2.rs");
    t.pass("tests/cases/c_choose_2.rs");
    t.pass("tests/cases/a_usecase.rs");
    t.pass("tests/cases/b_usecase.rs");
    t.pass("tests/cases/c_usecase.rs");
    t.pass("tests/cases/a_usecase_recursive.rs");
    t.pass("tests/cases/b_usecase_recursive.rs");
    t.pass("tests/cases/c_usecase_recursive.rs");
    t.pass("tests/cases/macro_basics.rs");
    t.pass("tests/cases/macro_choice.rs");
    t.pass("tests/cases/macro_recursive.rs");
    t.pass("tests/cases/macro_multi_sessionmpst.rs");
    t.pass("tests/cases/macro_multi_send_recv_sessionmpst.rs");
    t.pass("tests/cases/macro_multi_choice.rs");
    t.pass("tests/cases/macro_multi_recursion.rs");
    // Scribble tests
    t.pass("tests/scribble/top_down.rs");
    t.pass("tests/scribble/top_down_recursive.rs");
    // Recursive types bug
    t.pass("tests/infinite_type/work.rs");
    t.compile_fail("tests/infinite_type/fail.rs");
    t.compile_fail("tests/infinite_type/fail_2.rs");

}

#[test]
#[should_panic]
fn should_panic() {
    unit::checker_panic::test_checker_panic_name();
    unit::checker_panic::test_checker_panic_stack();
}
