use tonic::{Request, Response, Status};
use crate::Database;
use crate::schema;
use diesel::prelude::*;

pub use server::server;

mod basic_event;

mod server {
    use crate::database::Database;

    use super::basic_event;

    use std::time::Duration;

    use http::header::HeaderName;
    use tonic::transport::Server;
    use tonic_web::GrpcWebLayer;
    use tower_http::cors::{AllowOrigin, CorsLayer};

    const DEFAULT_MAX_AGE: Duration = Duration::from_secs(24 * 60 * 60);
    const DEFAULT_EXPOSED_HEADERS: [&str; 3] =
        ["grpc-status", "grpc-message", "grpc-status-details-bin"];
    const DEFAULT_ALLOW_HEADERS: [&str; 4] =
        ["x-grpc-web", "content-type", "x-user-agent", "grpc-timeout"];

    pub async fn server(db: Database) -> Result<(), Box<dyn std::error::Error>> {
        let addr = "0.0.0.0:50052".parse().unwrap();

        Server::builder()
            .accept_http1(true)
            .layer(
                CorsLayer::new()
                    .allow_origin(AllowOrigin::mirror_request())
                    .allow_credentials(true)
                    .max_age(DEFAULT_MAX_AGE)
                    .expose_headers(
                        DEFAULT_EXPOSED_HEADERS
                            .iter()
                            .cloned()
                            .map(HeaderName::from_static)
                            .collect::<Vec<HeaderName>>(),
                    )
                    .allow_headers(
                        DEFAULT_ALLOW_HEADERS
                            .iter()
                            .cloned()
                            .map(HeaderName::from_static)
                            .collect::<Vec<HeaderName>>(),
                    ),
            )
            .layer(GrpcWebLayer::new())
            .add_service(basic_event::Service::server(db.clone()))
            .serve(addr)
            .await?;

        Ok(())
    }
}
