use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    // benchmarks::long_five::long_protocols,
    benchmarks::long_simple::long_simple_protocols,
}
