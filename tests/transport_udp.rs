#![allow(clippy::type_complexity)]

mod udp;

use ntest::timeout;

#[test]
#[timeout(60000)]
fn transport_udp() {
    udp::binary::main();
    udp::binary_fail::main();
}
