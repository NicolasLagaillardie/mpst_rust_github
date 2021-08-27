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
    t.compile_fail("tests/infinite_type/fail.rs");
    t.compile_fail("tests/infinite_type/fail_2.rs");
}

#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();

    // Macro multi recursion for shorting
    t.compile_fail("tests/extra_mod/macros/long_simple_three_mpst_short_fail.rs");

    // Macro multi recursion for shorting
    t.compile_fail("tests/extra_mod/baking/long_simple_three_mpst_short_fail.rs");
}
