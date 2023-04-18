pub use app::root::{App, Routes, ServerApp, ServerAppProps};

mod api;
mod app;

pub mod basic_event {
    pub use hidden::basic_event_client::BasicEventClient as Client;
    pub use hidden::Bytes;

    mod hidden {
        tonic::include_proto!("basic_event");
    }
}
