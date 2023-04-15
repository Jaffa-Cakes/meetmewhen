pub mod basic_event {
    pub use hidden::basic_event_server::BasicEvent as Trait;
    pub use hidden::basic_event_server::BasicEventServer as Server;
    pub use hidden::Bytes;

    mod hidden {
        tonic::include_proto!("basic_event");
    }
}
