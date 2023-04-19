fn main() -> std::io::Result<()> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(
            &["basic_event.proto", "availabilities.proto"],
            &["../api/protos"],
        )
}
