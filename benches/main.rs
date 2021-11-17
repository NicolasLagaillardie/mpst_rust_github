use criterion::criterion_main;

mod main_all;

criterion_main!(
    main_all::basic::logging::logging,
    main_all::basic::circuit_breaker::circuit_breaker,
    main_all::basic::logging_solo::logging_solo,
    main_all::basic::circuit_breaker_solo::circuit_breaker_solo,
    main_all::basic::distributed_calc::distributed_calc,
    main_all::basic::fib::fib,
    main_all::basic::o_auth::o_auth,
    main_all::basic::online_wallet::online_wallet,
    main_all::basic::simple_voting::simple_voting,
    main_all::basic::smtp::smtp,
    main_all::basic::three_buyers::three_buyers,
    main_all::basic::travel_three::travel_three,
    main_all::basic::video_stream::video_stream,
    main_all::basic::dns_fowler::dns_fowler,
    main_all::basic::dns_imai::dns_imai,
    // //////////
    // main_all::baking::logging::logging,
    // main_all::baking::circuit_breaker::circuit_breaker,
    // main_all::baking::distributed_calc::distributed_calc,
    // main_all::baking::fib::fib,
    // main_all::baking::o_auth::o_auth,
    // main_all::baking::online_wallet::online_wallet,
    // main_all::baking::simple_voting::simple_voting,
    // main_all::baking::smtp::smtp,
    // main_all::baking::three_buyers::three_buyers,
    // main_all::baking::travel_three::travel_three,
    // main_all::baking::video_stream::video_stream,
    // main_all::baking::dns_fowler::dns_fowler,
    // main_all::baking::dns_imai::dns_imai,
);
