use mpstthree::top_down_nuscr::generator::generator;

fn main() {
    generator(
        "scribble_protocols/atmp/remote_data.nuscr",
        "examples/artifact_atmp/generated_files/",
        true,
    )
    .unwrap();
}
