pub use app::App;
pub use app::Route;
pub use app::ServerApp;
pub use app::ServerAppProps;

mod api;
mod app;

pub mod basic_event {
    pub use hidden::basic_event_client::BasicEventClient as Client;
    pub use hidden::Bytes;

    pub use hidden::*;

    mod hidden {
        tonic::include_proto!("basic_event");
    }
}
