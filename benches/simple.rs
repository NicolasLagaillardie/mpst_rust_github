use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    // benchmarks::long_five::long_five_protocols,
    benchmarks::long_simple::long_three_simple_protocols,
}
