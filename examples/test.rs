use mpstthree::top_down_nuscr::generator::generator;

fn main() {
    generator(
        "scribble_protocols/atmp/servo.nuscr",
        "examples/artifact_atmp/generated_files/",
    )
    .unwrap();

    generator(
        "scribble_protocols/atmp/three_buyers.nuscr",
        "examples/artifact_atmp/generated_files/",
    )
    .unwrap();

    generator(
        "scribble_protocols/atmp/remote_data.nuscr",
        "examples/artifact_atmp/generated_files/",
    )
    .unwrap();
}
