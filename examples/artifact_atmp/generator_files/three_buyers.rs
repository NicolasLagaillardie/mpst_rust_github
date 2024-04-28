use mpstthree::top_down_nuscr::generator::generator;

fn main() {
    generator(
        "scribble_protocols/atmp/three_buyers.nuscr",
        "examples/artifact_atmp/generated_files/",
        true,
    )
    .unwrap();
}
