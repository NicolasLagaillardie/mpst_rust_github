#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/session_mpst.rs");
}