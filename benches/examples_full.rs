#![allow(clippy::type_complexity)]

use criterion::{criterion_group, criterion_main, Criterion};

mod examples_all_basic;

mod examples_all_baking;

mod main_all_interleaved;

mod examples_all_timed;

criterion_group! {
    name = main_examples_litterature;
    config = Criterion::default().significance_level(0.1).sample_size(10000);
    targets =
        ////////// Benchmarks using basic methods
        examples_all_basic::logging::logging_main,
        examples_all_basic::circuit_breaker::circuit_breaker_main,
        examples_all_basic::distributed_calc::distributed_calc_main,
        examples_all_basic::fib::fibo_mpst,
        examples_all_basic::fib::fibo_binary,
        examples_all_basic::o_auth::o_auth_main,
        examples_all_basic::online_wallet::online_wallet_main,
        examples_all_basic::simple_voting::simple_voting_main,
        examples_all_basic::smtp::smtp_main,
        examples_all_basic::three_buyers::three_buyers_main,
        examples_all_basic::three_travel::travel_main,
        examples_all_basic::video_stream::video_stream_main,
        examples_all_basic::dns_fowler::dns_fowler_main,
        examples_all_basic::dns_imai::dns_imai_main,
        ////////// Benchmarks using generated methods
        examples_all_baking::logging::logging_main,
        examples_all_baking::circuit_breaker::circuit_breaker_main,
        examples_all_baking::distributed_calc::distributed_calc_main,
        examples_all_baking::fib::fibo_mpst,
        examples_all_baking::o_auth::o_auth_main,
        examples_all_baking::online_wallet::online_wallet_main,
        examples_all_baking::simple_voting::simple_voting_main,
        examples_all_baking::smtp::smtp_main,
        examples_all_baking::three_buyers::three_buyers_main,
        examples_all_baking::three_travel::travel_main,
        examples_all_baking::video_stream::video_stream_main,
        examples_all_baking::dns_fowler::dns_fowler_main,
        examples_all_baking::dns_imai::dns_imai_main,
        examples_all_baking::servo::servo_main,
        examples_all_baking::remote_data::remote_data_main,
        examples_all_baking::http::http_main,
        ////////// Benchmarks using interleaved methods
        examples_all_interleaved::basic::logging_solo::logging_solo_main,
        examples_all_interleaved::basic::circuit_breaker_solo::circuit_breaker_solo_main,
        ////////// Benchmarks using timed generated methods
        examples_all_timed::distributed_calc::distributed_calc_main,
        examples_all_timed::fib::fibo_mpst,
        examples_all_timed::o_auth::o_auth_main,
        examples_all_timed::online_wallet::online_wallet_main,
        examples_all_timed::simple_voting::simple_voting_main,
        examples_all_timed::smtp::smtp_main,
        examples_all_timed::three_buyers::three_buyers_main,
        examples_all_timed::three_travel::travel_main,
        examples_all_timed::servo::servo_main,
        examples_all_timed::remote_data::remote_data_main,
        examples_all_timed::http::http_main,
}

criterion_main! {
    main_examples_litterature
}
