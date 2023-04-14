use super::*;

use api::basic_event::*;

pub struct Service;

impl Service {
    pub fn server() -> Server<Service> {
        Server::new(Service)
    }
}
// test
#[tonic::async_trait]
impl Trait for Service {
    async fn create(&self, _request: Request<CreateReq>) -> Result<Response<CreateRes>, Status> {
        Ok(Response::new(CreateRes { id: 1 }))
    }
}
