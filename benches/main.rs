use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::long_five::long_five_protocols,
    // benchmarks::long_simple_three_empty::long_three_empty_simple_protocols,
    // benchmarks::long_simple_three::long_three_simple_protocols,
    // benchmarks::long_simple_four_empty::long_four_empty_simple_protocols,
    // benchmarks::long_simple_four::long_four_simple_protocols,
    // benchmarks::long_simple_five_empty::long_five_empty_simple_protocols,
    // benchmarks::long_simple_five::long_five_simple_protocols,
    // benchmarks::long_simple_six_empty::long_six_empty_simple_protocols,
    // benchmarks::long_simple_six::long_six_simple_protocols,
    // benchmarks::long_simple_seven_empty::long_seven_empty_simple_protocols,
    // benchmarks::long_simple_seven::long_seven_simple_protocols,
    // benchmarks::long_simple_eight_empty::long_eight_empty_simple_protocols,
    // benchmarks::long_simple_eight::long_eight_simple_protocols,
    // benchmarks::long_simple_nine_empty::long_nine_empty_simple_protocols,
    // benchmarks::long_simple_nine::long_nine_simple_protocols,
    // benchmarks::long_simple_ten_empty::long_ten_empty_simple_protocols,
    // benchmarks::long_simple_ten::long_ten_simple_protocols,
    // benchmarks::long_simple_eleven_empty::long_eleven_empty_simple_protocols,
    // benchmarks::long_simple_eleven::long_eleven_simple_protocols,
    // benchmarks::long_simple_twenty_empty::long_twenty_empty_simple_protocols,
    // benchmarks::long_simple_twenty::long_twenty_simple_protocols,
    // benchmarks::fib::fibo,
    // benchmarks::o_auth::o_auth,
    // benchmarks::simple_voting_three::simple_voting,
    // benchmarks::travel_three::travel,
    // benchmarks::three_buyer::three_buyer,
    // benchmarks::smtp::smtp,
}
