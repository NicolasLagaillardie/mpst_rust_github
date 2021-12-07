#![allow(clippy::type_complexity)]

use criterion::criterion_main;

mod main_all_interleaved;

criterion_main! {
    main_all_interleaved::basic::logging_solo::logging_solo,
    main_all_interleaved::basic::circuit_breaker_solo::circuit_breaker_solo,
}
