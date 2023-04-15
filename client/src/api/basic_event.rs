use super::*;

use crate::basic_event::*;

pub struct Service;

impl Service {
    fn client() -> Client<WebClient> {
        let base_url = "http://localhost:50052".to_string();
        let wasm_client = WebClient::new(base_url);

        Client::new(wasm_client)
    }

    pub async fn create() -> bool {
        let mut client = Service::client();

        match client.create(CreateReq {
            name: "Test".to_string(),
            r#type: 0,
            when: vec![],
            no_earlier: 1,
            no_later: 2
        }).await {
            Ok(_response) => true,
            Err(_) => false,
        }
    }
}
