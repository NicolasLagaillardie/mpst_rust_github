#![allow(clippy::type_complexity)]

use criterion::{criterion_group, criterion_main, Criterion};

mod examples_all_basic;

criterion_group! {
    name = main_examples_litterature;
    config = Criterion::default().significance_level(0.1).sample_size(10000);
    targets =
        ////////// Benchmarks using basic functions
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
}

criterion_main! {
    main_examples_litterature
}
