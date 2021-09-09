mod tcp;

#[test]
fn tcp() {
    tcp::binary::main();
    tcp::binary_fail::main();
}
