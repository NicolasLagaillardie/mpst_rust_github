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
    t.compile_fail("tests/infinite_type/fail_3.rs");
}
