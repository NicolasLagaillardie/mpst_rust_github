#![allow(clippy::type_complexity)]

use criterion::{criterion_group, criterion_main, Criterion};

mod examples_all;

criterion_group! {
    name = main_examples_litterature;
    config = Criterion::default().significance_level(0.1).sample_size(10000);
    targets =
        ////////// Benchmarks using basic functions
        examples_all::basic::logging::logging_main,
        examples_all::basic::circuit_breaker::circuit_breaker_main,
        examples_all::basic::distributed_calc::distributed_calc_main,
        examples_all::basic::fib::fibo_mpst,
        examples_all::basic::fib::fibo_binary,
        examples_all::basic::o_auth::o_auth_main,
        examples_all::basic::online_wallet::online_wallet_main,
        examples_all::basic::simple_voting::simple_voting_main,
        examples_all::basic::smtp::smtp_main,
        examples_all::basic::three_buyers::three_buyers_main,
        examples_all::basic::three_travel::travel_main,
        examples_all::basic::video_stream::video_stream_main,
        examples_all::basic::dns_fowler::dns_fowler_main,
        examples_all::basic::dns_imai::dns_imai_main,
        ////////// Benchmarks using generated methods
        examples_all::baking::logging::logging_main,
        examples_all::baking::circuit_breaker::circuit_breaker_main,
        examples_all::baking::distributed_calc::distributed_calc_main,
        examples_all::baking::fib::fibo_mpst,
        examples_all::baking::o_auth::o_auth_main,
        examples_all::baking::online_wallet::online_wallet_main,
        examples_all::baking::simple_voting::simple_voting_main,
        examples_all::baking::smtp::smtp_main,
        examples_all::baking::three_buyers::three_buyers_main,
        examples_all::baking::three_travel::travel_main,
        examples_all::baking::video_stream::video_stream_main,
        examples_all::baking::dns_fowler::dns_fowler_main,
        examples_all::baking::dns_imai::dns_imai_main,
}

criterion_main! {
    main_examples_litterature
}
