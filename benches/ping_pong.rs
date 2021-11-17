use criterion::criterion_main;

mod ping_pong_all;

criterion_main! {
    ping_pong_all::ping_pong_1::ping_pong,
    ping_pong_all::ping_pong_baking_cancel_1::ping_pong,
    ping_pong_all::ping_pong_cancel_1::ping_pong,
    ping_pong_all::ping_pong_cancel_broadcast_1::ping_pong,
}
