use mpstthree::top_down_nuscr::generator::generator;

fn main() {
    generator("tests/generator/basic.nuscr", "tests/generator/").unwrap();

    generator("tests/generator/choice.nuscr", "tests/generator/").unwrap();

    assert!(generator("tests/generator/nested_choice.nuscr", "tests/generator/").is_err());

    generator("tests/generator/recursive.nuscr", "tests/generator/").unwrap();
}
