use mpstthree::top_down_nuscr::generator::generator;

fn main() {
    generator(
        "scribble_protocols/atmp/servo.nuscr",
        "examples/artifact_atmp/generated_files/",
    )
    .unwrap();
}
