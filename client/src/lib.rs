pub use app::App;

#[cfg(target_arch = "wasm32")]
mod api;
mod app;

pub mod health {
    pub use hidden::health_client::HealthClient as Client;
    pub use hidden::{IsOkReq, IsOkRes};

    mod hidden {
        tonic::include_proto!("health");
    }
}