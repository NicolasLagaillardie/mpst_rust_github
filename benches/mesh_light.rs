#![allow(dead_code, clippy::type_complexity, clippy::too_many_arguments)]

use criterion::{criterion_group, criterion_main, Criterion};

mod mesh_all;

criterion_group! {
    name = mesh_light;
    config = Criterion::default().significance_level(0.1).sample_size(100);
    targets =
        ////////// Benchmarks using basic functions with zero loops
        mesh_all::empty::mesh_two::mesh_protocol_mpst,
        mesh_all::empty::mesh_two::mesh_protocol_binary,
        mesh_all::empty::mesh_two::mesh_protocol_crossbeam,
        mesh_all::empty::mesh_three::mesh_protocol_mpst,
        mesh_all::empty::mesh_three::mesh_protocol_binary,
        mesh_all::empty::mesh_three::mesh_protocol_crossbeam,
        mesh_all::empty::mesh_four::mesh_protocol_mpst,
        mesh_all::empty::mesh_four::mesh_protocol_binary,
        mesh_all::empty::mesh_four::mesh_protocol_crossbeam,
        mesh_all::empty::mesh_five::mesh_protocol_mpst,
        mesh_all::empty::mesh_five::mesh_protocol_binary,
        mesh_all::empty::mesh_five::mesh_protocol_crossbeam,
        ////////// Benchmarks using basic functions with 100 loops
        mesh_all::normal::mesh_two::mesh_protocol_mpst,
        mesh_all::normal::mesh_two::mesh_protocol_binary,
        mesh_all::normal::mesh_two::mesh_protocol_crossbeam,
        mesh_all::normal::mesh_three::mesh_protocol_mpst,
        mesh_all::normal::mesh_three::mesh_protocol_binary,
        mesh_all::normal::mesh_three::mesh_protocol_crossbeam,
        mesh_all::normal::mesh_four::mesh_protocol_mpst,
        mesh_all::normal::mesh_four::mesh_protocol_binary,
        mesh_all::normal::mesh_four::mesh_protocol_crossbeam,
        mesh_all::normal::mesh_five::mesh_protocol_mpst,
        mesh_all::normal::mesh_five::mesh_protocol_binary,
        mesh_all::normal::mesh_five::mesh_protocol_crossbeam,
        ////////// Benchmarks using cancelling without a monitor to broadcast cancel using generated methods with 100 loops
        mesh_all::baking_cancel::mesh_two::mesh_protocol_mpst,
        mesh_all::baking_cancel::mesh_two::mesh_protocol_binary,
        mesh_all::baking_cancel::mesh_two::mesh_protocol_crossbeam,
        mesh_all::baking_cancel::mesh_three::mesh_protocol_mpst,
        mesh_all::baking_cancel::mesh_three::mesh_protocol_binary,
        mesh_all::baking_cancel::mesh_three::mesh_protocol_crossbeam,
        mesh_all::baking_cancel::mesh_four::mesh_protocol_mpst,
        mesh_all::baking_cancel::mesh_four::mesh_protocol_binary,
        mesh_all::baking_cancel::mesh_four::mesh_protocol_crossbeam,
        mesh_all::baking_cancel::mesh_five::mesh_protocol_mpst,
        mesh_all::baking_cancel::mesh_five::mesh_protocol_binary,
        mesh_all::baking_cancel::mesh_five::mesh_protocol_crossbeam,
}

criterion_main! {
    mesh_light
}
