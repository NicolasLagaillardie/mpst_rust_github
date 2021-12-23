#![allow(clippy::type_complexity)]

use criterion::{criterion_group, criterion_main, Criterion};

mod main_all;

criterion_group! {
    name = main_examples_litterature;
    config = Criterion::default().significance_level(0.1).sample_size(10000);
    targets =
        ////////// Benchmarks using basic functions
        main_all::basic::logging::logging_main,
        main_all::basic::circuit_breaker::circuit_breaker_main,
        main_all::basic::distributed_calc::distributed_calc_main,
        main_all::basic::fib::fibo_mpst,
        main_all::basic::fib::fibo_binary,
        main_all::basic::o_auth::o_auth_main,
        main_all::basic::online_wallet::online_wallet_main,
        main_all::basic::simple_voting::simple_voting_main,
        main_all::basic::smtp::smtp_main,
        main_all::basic::three_buyers::three_buyers_main,
        main_all::basic::travel_three::travel_main,
        main_all::basic::video_stream::video_stream_main,
        main_all::basic::dns_fowler::dns_fowler_main,
        main_all::basic::dns_imai::dns_imai_main,
        ////////// Benchmarks using generated methods
        main_all::baking::logging::logging_main,
        main_all::baking::circuit_breaker::circuit_breaker_main,
        main_all::baking::distributed_calc::distributed_calc_main,
        main_all::baking::fib::fibo_mpst,
        main_all::baking::o_auth::o_auth_main,
        main_all::baking::online_wallet::online_wallet_main,
        main_all::baking::simple_voting::simple_voting_main,
        main_all::baking::smtp::smtp_main,
        main_all::baking::three_buyers::three_buyers_main,
        main_all::baking::travel_three::travel_main,
        main_all::baking::video_stream::video_stream_main,
        main_all::baking::dns_fowler::dns_fowler_main,
        main_all::baking::dns_imai::dns_imai_main,
}

criterion_main! {
    main_examples_litterature
}
