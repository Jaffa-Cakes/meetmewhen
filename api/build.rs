fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=migrations");

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&["basic_event.proto"], &["../api/protos"])?;

    Ok(())
}
