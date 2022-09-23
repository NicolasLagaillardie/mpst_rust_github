#![allow(dead_code, clippy::type_complexity, clippy::too_many_arguments)]

use criterion::{criterion_group, criterion_main, Criterion};

mod ring_all;

criterion_group! {
    name = ring_timed_baking;
    config = Criterion::default().significance_level(0.1).sample_size(10000);
    targets =
    ////////// Benchmarks using cancelling without a monitor to broadcast cancel using generated methods with 100 loops
    ring_all::baking_cancel::ring_two::ring_protocol_mpst,
    ring_all::baking_cancel::ring_three::ring_protocol_mpst,
    ring_all::baking_cancel::ring_four::ring_protocol_mpst,
    ring_all::baking_cancel::ring_five::ring_protocol_mpst,
    ring_all::baking_cancel::ring_six::ring_protocol_mpst,
    ring_all::baking_cancel::ring_seven::ring_protocol_mpst,
    ring_all::baking_cancel::ring_eight::ring_protocol_mpst,
    ring_all::baking_cancel::ring_nine::ring_protocol_mpst,
    ring_all::baking_cancel::ring_ten::ring_protocol_mpst,
    ////////// Benchmarks using timed cancelling without a monitor to broadcast cancel using generated methods with 100 loops
    ring_all::timed_baking_cancel::ring_two::ring_protocol_mpst,
    ring_all::timed_baking_cancel::ring_three::ring_protocol_mpst,
    ring_all::timed_baking_cancel::ring_four::ring_protocol_mpst,
    ring_all::timed_baking_cancel::ring_five::ring_protocol_mpst,
    ring_all::timed_baking_cancel::ring_six::ring_protocol_mpst,
    ring_all::timed_baking_cancel::ring_seven::ring_protocol_mpst,
    ring_all::timed_baking_cancel::ring_eight::ring_protocol_mpst,
    ring_all::timed_baking_cancel::ring_nine::ring_protocol_mpst,
    ring_all::timed_baking_cancel::ring_ten::ring_protocol_mpst,
}

criterion_main! {
    ring_timed_baking
}
