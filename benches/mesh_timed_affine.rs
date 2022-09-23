#![allow(dead_code, clippy::type_complexity, clippy::too_many_arguments)]

use criterion::{criterion_group, criterion_main, Criterion};

mod mesh_all;

criterion_group! {
    name = mesh_timed_affine;
    config = Criterion::default().significance_level(0.1).sample_size(10000);
    targets =
    ////////// Benchmarks using cancelling without a monitor to broadcast cancel using generated methods with 100 loops
    mesh_all::baking_cancel::mesh_two::mesh_protocol_mpst,
    mesh_all::baking_cancel::mesh_three::mesh_protocol_mpst,
    mesh_all::baking_cancel::mesh_four::mesh_protocol_mpst,
    mesh_all::baking_cancel::mesh_five::mesh_protocol_mpst,
    mesh_all::baking_cancel::mesh_six::mesh_protocol_mpst,
    mesh_all::baking_cancel::mesh_seven::mesh_protocol_mpst,
    mesh_all::baking_cancel::mesh_eight::mesh_protocol_mpst,
    mesh_all::baking_cancel::mesh_nine::mesh_protocol_mpst,
    mesh_all::baking_cancel::mesh_ten::mesh_protocol_mpst,
    ////////// Benchmarks using timed cancelling without a monitor to broadcast cancel using generated methods with 100 loops
    mesh_all::timed_baking_cancel::mesh_two::mesh_protocol_mpst,
    mesh_all::timed_baking_cancel::mesh_three::mesh_protocol_mpst,
    mesh_all::timed_baking_cancel::mesh_four::mesh_protocol_mpst,
    mesh_all::timed_baking_cancel::mesh_five::mesh_protocol_mpst,
    mesh_all::timed_baking_cancel::mesh_six::mesh_protocol_mpst,
    mesh_all::timed_baking_cancel::mesh_seven::mesh_protocol_mpst,
    mesh_all::timed_baking_cancel::mesh_eight::mesh_protocol_mpst,
    mesh_all::timed_baking_cancel::mesh_nine::mesh_protocol_mpst,
    mesh_all::timed_baking_cancel::mesh_ten::mesh_protocol_mpst,
}

criterion_main! {
    mesh_timed_affine
}
