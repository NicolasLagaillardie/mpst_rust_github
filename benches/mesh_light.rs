#![allow(dead_code, clippy::type_complexity, clippy::too_many_arguments)]

use criterion::criterion_main;

mod mesh_all;

criterion_main! {
    ////////// Benchmarks using basic functions with zero loops
    mesh_all::empty::mesh_two::mesh_two,
    mesh_all::empty::mesh_three::mesh_three,
    mesh_all::empty::mesh_four::mesh_four,
    mesh_all::empty::mesh_five::mesh_five,
    ////////// Benchmarks using basic functions with 100 loops
    mesh_all::normal::mesh_two::mesh_two,
    mesh_all::normal::mesh_three::mesh_three,
    mesh_all::normal::mesh_four::mesh_four,
    mesh_all::normal::mesh_five::mesh_five,
    ////////// Benchmarks using cancelling without a monitor to broadcast cancel using generated methods with 100 loops
    mesh_all::baking_cancel_inline::mesh_two::mesh_two,
    mesh_all::baking_cancel_inline::mesh_three::mesh_three,
    mesh_all::baking_cancel_inline::mesh_four::mesh_four,
    mesh_all::baking_cancel_inline::mesh_five::mesh_five,
}
