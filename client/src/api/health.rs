use super::*;

use crate::health::*;

pub struct Service;

impl Service {
    fn client() -> Client<WebClient> {
        let base_url = "http://localhost:50052".to_string();
        let wasm_client = WebClient::new(base_url);

        Client::new(wasm_client)
    }

    pub async fn is_ok() -> bool {
        let mut client = Service::client();

        match client.is_ok(IsOkReq {}).await {
            Ok(response) => {
                let response = response.into_inner();
                response.healthy
            }
            Err(_) => false,
        }
    }
}
