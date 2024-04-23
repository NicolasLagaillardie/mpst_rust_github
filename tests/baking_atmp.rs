mod baking_atmp_mod;

#[test]
fn baking_atmp_timeless_tests() {
    baking_atmp_mod::timeless::main();
}

#[test]
fn baking_atmp_timers_tests() {
    baking_atmp_mod::timers::main();
}

#[test]
fn baking_atmp_timers_err_tests() {
    baking_atmp_mod::timers_err::main();
}
