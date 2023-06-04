#![allow(clippy::type_complexity)]

mod infinite_type;
mod scribble;

#[test]
fn scribble_tests() {
    // Test code generated from Scribble
    scribble::top_down::top_down_approach();
    scribble::top_down_recursive::top_down_approach();
}

#[test]
fn infinite_type_pass() {
    infinite_type::work::main();
}

#[test]
fn infinite_type_fail() {
    let t = trybuild::TestCases::new();

    // Infinite types
    t.compile_fail("tests/infinite_type/infinite_size.rs");
    t.compile_fail("tests/infinite_type/overflow_evaluation.rs");
    t.compile_fail("tests/infinite_type/overflow_evaluation_double_trouble.rs");
    t.compile_fail("tests/infinite_type/overflow_evaluation_double_trouble_with_test_struct.rs");
    t.compile_fail("tests/infinite_type/fail_0.rs");
    t.compile_fail("tests/infinite_type/fail_1.rs");
    t.compile_fail("tests/infinite_type/fail_2.rs");
    t.compile_fail("tests/infinite_type/fail_3.rs");
}
