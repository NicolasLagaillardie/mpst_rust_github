#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-parse-enum.rs");
    t.compile_fail("tests/02-not-enum.rs");
    t.compile_fail("tests/03-out-of-order.rs");
    t.compile_fail("tests/04-variants-with-data.rs");
    t.pass("tests/05-variants-with-data.rs");
}
