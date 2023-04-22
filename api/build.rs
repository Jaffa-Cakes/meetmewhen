fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=migrations");

    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(
            &["basic_event.proto", "availabilities.proto", "common.proto"],
            &["../api/protos"],
        )?;

    Ok(())
}
