fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .compile(
            &["protocol/api/api.proto"],
            &["protocol"],
        )?;
    Ok(())
}