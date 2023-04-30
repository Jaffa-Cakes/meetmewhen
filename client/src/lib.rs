pub use app::root::{App, Routes, ServerApp, ServerAppProps};

mod api;
mod app;

pub mod common {
    pub use hidden::Bytes;

    mod hidden {
        tonic::include_proto!("common");
    }
}

pub mod basic_event {
    use super::common;
    pub use common::Bytes;
    pub use hidden::basic_event_client::BasicEventClient as Client;

    mod hidden {
        tonic::include_proto!("basic_event");
    }
}

pub mod availabilities {
    use super::common;
    pub use common::Bytes;
    pub use hidden::availabilities_client::AvailabilitiesClient as Client;

    mod hidden {
        tonic::include_proto!("availabilities");
    }
}
