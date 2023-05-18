#![allow(clippy::type_complexity)]

use criterion::{criterion_group, criterion_main, Criterion};

mod ping_pong_all;

criterion_main! {
    ping_pong
}

criterion_group! {
    name = ping_pong;
    config = Criterion::default().significance_level(0.1).sample_size(1000);
    targets =
        // Basic
        ping_pong_all::ping_pong_1::ping_pong_protocol_mpst,
        ping_pong_all::ping_pong_1::ping_pong_protocol_binary,
        ping_pong_all::ping_pong_1::ping_pong_protocol_crossbeam,
        // Baking
        ping_pong_all::ping_pong_baking_mpst_1::ping_pong_protocol_mpst,
        // Baking_cancel
        ping_pong_all::ping_pong_baking_ampst_1::ping_pong_protocol_ampst,
        // Cancel
        ping_pong_all::ping_pong_cancel_1::ping_pong_protocol_mpst,
        // Cancel_broadcast
        ping_pong_all::ping_pong_cancel_broadcast_1::ping_pong_protocol_mpst,
}
