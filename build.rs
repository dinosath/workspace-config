fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto");
    // protox compiles the proto in pure Rust, so no system protoc is required.
    let descriptors = protox::compile(["modulaga_config_api.proto"], ["proto"])?;
    tonic_prost_build::configure()
        .build_client(false)
        .compile_fds(descriptors)?;
    Ok(())
}
