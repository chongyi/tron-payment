use std::env::current_dir;
use std::fs::create_dir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = current_dir()?.join("src").join("apis");
    if !out_dir.exists() {
        create_dir(&out_dir);
    }

    tonic_build::configure()
        .build_server(false)
        .format(true)
        .out_dir(out_dir)
        .compile(
            &["protocol/api/api.proto"],
            &["protocol"],
        )?;
    Ok(())
}