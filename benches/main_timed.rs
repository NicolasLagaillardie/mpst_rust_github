#![allow(clippy::type_complexity)]

use criterion::{criterion_group, criterion_main, Criterion};

mod main_all;

criterion_group! {
    name = main_timed_examples_litterature;
    config = Criterion::default().significance_level(0.1).sample_size(10000);
    targets =
        ////////// Benchmarks using timed generated methods
        main_all::timed::distributed_calc::distributed_calc_main,
        main_all::timed::fib::fibo_mpst,
        main_all::timed::o_auth::o_auth_main,
        main_all::timed::online_wallet::online_wallet_main,
        main_all::timed::simple_voting::simple_voting_main,
        main_all::timed::smtp::smtp_main,
        main_all::timed::three_buyers::three_buyers_main,
        main_all::timed::three_travel::travel_main,
        main_all::timed::remote_data::remote_data_main,
        main_all::timed::servo::servo_main,
}

criterion_main! {
    main_timed_examples_litterature
}
