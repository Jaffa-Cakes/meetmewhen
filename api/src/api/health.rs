use super::*;

use api::health::*;

pub struct Service;

impl Service {
    pub fn server() -> Server<Service> {
        Server::new(Service)
    }
}

#[tonic::async_trait]
impl Trait for Service {
    async fn is_ok(&self, _request: Request<IsOkReq>) -> Result<Response<IsOkRes>, Status> {
        println!("Health checked");
        Ok(Response::new(IsOkRes { healthy: true }))
    }
}
