mod udp;

use ntest::timeout;

#[test]
#[timeout(10000)]
fn udp() {
    udp::binary::main();
    udp::binary_fail::main();
}
