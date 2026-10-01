
fn main() {
    let manifest_dir = std::path::PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
    );
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    grpc_protobuf_build::CodeGen::new()
        .output_dir(&out_dir)
        .input("modulaga_config_api.proto")
        .include(manifest_dir.join("../proto"))
        .client_only()
        .compile()
        .unwrap();
}
