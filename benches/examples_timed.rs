#![allow(clippy::type_complexity)]

use criterion::{criterion_group, criterion_main, Criterion};

mod examples_all_timed;

criterion_group! {
    name = main_timed_examples_litterature;
    config = Criterion::default().significance_level(0.1).sample_size(10000);
    targets =
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
    main_timed_examples_litterature
}
