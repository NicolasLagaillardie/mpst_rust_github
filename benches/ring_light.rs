#![allow(dead_code, clippy::type_complexity, clippy::too_many_arguments)]

use criterion::{criterion_group, criterion_main, Criterion};

mod ring_all;

criterion_group! {
    name = ring_light;
    config = Criterion::default().significance_level(0.1).sample_size(100);
    targets =
        ////////// Benchmarks using basic functions with zero loops
        ring_all::empty::ring_two::ring_protocol_mpst,
        ring_all::empty::ring_two::ring_protocol_binary,
        ring_all::empty::ring_two::ring_protocol_crossbeam,
        ring_all::empty::ring_three::ring_protocol_mpst,
        ring_all::empty::ring_three::ring_protocol_binary,
        ring_all::empty::ring_three::ring_protocol_crossbeam,
        ring_all::empty::ring_four::ring_protocol_mpst,
        ring_all::empty::ring_four::ring_protocol_binary,
        ring_all::empty::ring_four::ring_protocol_crossbeam,
        ring_all::empty::ring_five::ring_protocol_mpst,
        ring_all::empty::ring_five::ring_protocol_binary,
        ring_all::empty::ring_five::ring_protocol_crossbeam,
        ////////// Benchmarks using basic functions with 100 loops
        ring_all::normal::ring_two::ring_protocol_mpst,
        ring_all::normal::ring_two::ring_protocol_binary,
        ring_all::normal::ring_two::ring_protocol_crossbeam,
        ring_all::normal::ring_three::ring_protocol_mpst,
        ring_all::normal::ring_three::ring_protocol_binary,
        ring_all::normal::ring_three::ring_protocol_crossbeam,
        ring_all::normal::ring_four::ring_protocol_mpst,
        ring_all::normal::ring_four::ring_protocol_binary,
        ring_all::normal::ring_four::ring_protocol_crossbeam,
        ring_all::normal::ring_five::ring_protocol_mpst,
        ring_all::normal::ring_five::ring_protocol_binary,
        ring_all::normal::ring_five::ring_protocol_crossbeam,
        ////////// Benchmarks using cancelling without a monitor to broadcast cancel using generated methods with 100 loops
        ring_all::baking::ring_two::ring_protocol_mpst,
        ring_all::baking::ring_two_ampst::ring_protocol_ampst,
        ring_all::baking::ring_two::ring_protocol_binary,
        ring_all::baking::ring_two::ring_protocol_crossbeam,
        ring_all::baking::ring_three::ring_protocol_mpst,
        ring_all::baking::ring_three_ampst::ring_protocol_ampst,
        ring_all::baking::ring_three::ring_protocol_binary,
        ring_all::baking::ring_three::ring_protocol_crossbeam,
        ring_all::baking::ring_four::ring_protocol_mpst,
        ring_all::baking::ring_four_ampst::ring_protocol_ampst,
        ring_all::baking::ring_four::ring_protocol_binary,
        ring_all::baking::ring_four::ring_protocol_crossbeam,
        ring_all::baking::ring_five::ring_protocol_mpst,
        ring_all::baking::ring_five_ampst::ring_protocol_ampst,
        ring_all::baking::ring_five::ring_protocol_binary,
        ring_all::baking::ring_five::ring_protocol_crossbeam,
}

criterion_main! {
    ring_light
}
