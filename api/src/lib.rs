pub mod basic_event {
    pub use hidden::basic_event_server::BasicEvent as Trait;
    pub use hidden::basic_event_server::BasicEventServer as Server;
    pub use hidden::Bytes;

    mod hidden {
        tonic::include_proto!("basic_event");
    }
}

pub mod availabilities {
    pub use hidden::availabilities_server::Availabilities as Trait;
    pub use hidden::availabilities_server::AvailabilitiesServer as Server;
    pub use hidden::Bytes;

    mod hidden {
        tonic::include_proto!("availabilities");
    }
}
