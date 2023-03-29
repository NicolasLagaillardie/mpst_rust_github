#![allow(clippy::type_complexity)]

use criterion::{criterion_group, criterion_main, Criterion};

mod examples_all_baking;

criterion_group! {
    name = main_examples_litterature;
    config = Criterion::default().significance_level(0.1).sample_size(10000);
    targets =
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
}

criterion_main! {
    main_examples_litterature
}
