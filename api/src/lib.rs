pub mod common {
    pub use hidden::Bytes;

    mod hidden {
        tonic::include_proto!("common");
    }
}

pub mod basic_event {
    use super::common;
    pub use common::Bytes;
    pub use hidden::basic_event_server::BasicEvent as Trait;
    pub use hidden::basic_event_server::BasicEventServer as Server;

    mod hidden {
        tonic::include_proto!("basic_event");
    }
}

pub mod availabilities {
    use super::common;
    pub use common::Bytes;
    pub use hidden::availabilities_server::Availabilities as Trait;
    pub use hidden::availabilities_server::AvailabilitiesServer as Server;

    mod hidden {
        tonic::include_proto!("availabilities");
    }
}
