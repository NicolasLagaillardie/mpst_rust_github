mod udp;

use ntest::timeout;

#[test]
#[timeout(60000)]
fn udp() {
    udp::binary::main();
    udp::binary_fail::main();
}
