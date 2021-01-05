use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    // benchmarks::long_five::long_five_protocols,
    benchmarks::long_simple_three::long_three_simple_protocols,
    benchmarks::long_simple_four::long_four_simple_protocols,
    benchmarks::long_simple_five::long_five_simple_protocols,
    benchmarks::long_simple_six::long_six_simple_protocols,
    benchmarks::long_simple_seven::long_seven_simple_protocols,
    benchmarks::long_simple_eight::long_eight_simple_protocols,
}
