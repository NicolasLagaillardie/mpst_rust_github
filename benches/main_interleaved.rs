#![allow(clippy::type_complexity)]

use criterion::{criterion_group, criterion_main, Criterion};

mod main_all_interleaved;

criterion_group! {
    name = main_interleaved;
    config = Criterion::default().significance_level(0.1).sample_size(10000);
    targets =
        main_all_interleaved::basic::logging_solo::logging_solo_main,
        main_all_interleaved::basic::circuit_breaker_solo::circuit_breaker_solo_main,
}

criterion_main! {
    main_interleaved
}
