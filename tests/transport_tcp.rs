mod tcp;

#[test]
fn transport_tcp() {
    tcp::binary::main();
    tcp::binary_cancel::main();
    tcp::binary_fail_connect::main();
}
