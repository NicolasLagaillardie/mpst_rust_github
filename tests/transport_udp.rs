mod udp;

use ntest::timeout;

#[test]
#[timeout(120000)]
fn transport_udp() {
    udp::binary::main();
    udp::binary_fail::main();
}
