mod udp;

#[test]
fn udp() {
    udp::binary::main();
    udp::binary_fail::main();
}
