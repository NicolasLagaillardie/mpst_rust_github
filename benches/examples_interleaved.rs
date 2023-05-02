#![allow(clippy::type_complexity)]

use criterion::{criterion_group, criterion_main, Criterion};

mod examples_all_interleaved;

criterion_group! {
    name = main_interleaved;
    config = Criterion::default().significance_level(0.1).sample_size(10000);
    targets =
        examples_all_interleaved::basic::logging_solo::logging_solo_main,
        examples_all_interleaved::basic::circuit_breaker_solo::circuit_breaker_solo_main,
}

criterion_main! {
    main_interleaved
}
