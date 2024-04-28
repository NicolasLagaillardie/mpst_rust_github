use mpstthree::top_down_nuscr::generator::generator;

mod nuscr_mod;

#[test]
fn nuscr_generation_basic() {
    assert!(generator(
        "tests/nuscr_mod/correct/basic.nuscr",
        "tests/nuscr_mod/correct/",
        false
    )
    .is_ok());
}

#[test]
fn nuscr_generation_choice() {
    assert!(generator(
        "tests/nuscr_mod/correct/choice.nuscr",
        "tests/nuscr_mod/correct/",
        false
    )
    .is_ok());
}

// #[test]
// fn nuscr_generation_nested_choice() {
//     assert!(generator(
//         "tests/nuscr_mod/correct/nested_choice.nuscr",
//         "tests/nuscr_mod/correct/",
// false
//     )
//     .is_err());
// }

#[test]
fn nuscr_generation_recursion() {
    assert!(generator(
        "tests/nuscr_mod/correct/recursion.nuscr",
        "tests/nuscr_mod/correct/",
        false
    )
    .is_ok());
}

#[test]
fn correct_run_generated_files() {
    nuscr_mod::correct::basic_correct::main();
    nuscr_mod::correct::choice_correct::main();
    nuscr_mod::correct::recursion_correct::main();
}

#[test]
fn failing_generated_files_wrong_interval() {
    assert!(generator(
        "tests/nuscr_mod/failing/wrong_interval.nuscr",
        "tests/nuscr_mod/failing/",
        false
    )
    .is_err());
}

#[test]
fn failing_generated_files_wrong_consecutive_intervals() {
    assert!(generator(
        "tests/nuscr_mod/failing/wrong_consecutive_intervals.nuscr",
        "tests/nuscr_mod/failing/",
        false
    )
    .is_err());
}
