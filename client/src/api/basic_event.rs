use super::*;

use crate::basic_event::*;

pub enum Type {
    Dates = 0,
    Days = 1,
}

pub struct Service;

impl Service {
    fn client() -> Client<WebClient> {
        let base_url = "http://localhost:50052".to_string();
        let wasm_client = WebClient::new(base_url);

        Client::new(wasm_client)
    }

    pub async fn create(req: api_types::basic_event::create::Req) -> bool {
        let mut client = Service::client();

        if !req.is_valid() {
            log::info!("Data is invalid.");
            return false;
        }

        match client.create(Bytes { value: req.to_bincode()}).await {
            Ok(_response) => true,
            Err(_) => false,
        }
    }
}
