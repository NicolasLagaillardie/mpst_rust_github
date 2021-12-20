#![allow(dead_code, clippy::type_complexity, clippy::too_many_arguments)]

use criterion::criterion_main;

mod ring_all;

criterion_main! {
    ////////// Benchmarks using basic functions with zero loops
    ring_all::empty::ring_two::ring_two,
    ring_all::empty::ring_three::ring_three,
    ring_all::empty::ring_four::ring_four,
    ring_all::empty::ring_five::ring_five,
    ////////// Benchmarks using basic functions with 100 loops
    ring_all::normal::ring_two::ring_two,
    ring_all::normal::ring_three::ring_three,
    ring_all::normal::ring_four::ring_four,
    ring_all::normal::ring_five::ring_five,
    ////////// Benchmarks using cancelling without a monitor to broadcast cancel using generated methods with 100 loops
    ring_all::baking_cancel_inline::ring_two::ring_two,
    ring_all::baking_cancel_inline::ring_three::ring_three,
    ring_all::baking_cancel_inline::ring_four::ring_four,
    ring_all::baking_cancel_inline::ring_five::ring_five,
}