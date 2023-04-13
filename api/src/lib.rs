pub mod health {
    pub use hidden::health_server::Health as Trait;
    pub use hidden::health_server::HealthServer as Server;
    pub use hidden::{IsOkReq, IsOkRes};

    mod hidden {
        tonic::include_proto!("health");
    }
}