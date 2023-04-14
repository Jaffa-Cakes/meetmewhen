fn main() -> std::io::Result<()> {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(&["basic_event.proto"], &["../api/protos"])?;

    Ok(())
}
