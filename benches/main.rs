#![allow(clippy::type_complexity)]

use criterion::{criterion_group, criterion_main, Criterion};

mod main_all;

criterion_group! {
    name = main_examples_litterature;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(20000);
    targets =
        ////////// Benchmarks using basic functions
        main_all::basic::logging::logging_main,
}

criterion_main! {
    main_examples_litterature
}
