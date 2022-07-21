// use protobuf;
// use protobuf_codegen;
// use protoc_bin_vendored;

fn main() {
    println!("testing proto");

    // Use this in build.rs
    protobuf_codegen::Codegen::new()
        // Use `protoc` parser, optional.
        .protoc()
        // Use `protoc-bin-vendored` bundled protoc command, optional.
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(&["protos_input"])
        // Inputs must reside in some of include paths.
        .input("protos_input/person.proto")
        // Specify output directory relative to Cargo output directory.
        .cargo_out_dir("protos_output")
        .run_from_script();

    println!("done");
}
