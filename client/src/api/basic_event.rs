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

    pub async fn create(name: String, r#type: Type, when: Vec<String>, no_earlier: u32, no_later: u32, timezone: String) -> bool {
        let mut client = Service::client();

        match client.create(CreateReq {
            name,
            r#type: r#type as i32,
            when,
            no_earlier,
            no_later,
            timezone
        }).await {
            Ok(_response) => true,
            Err(_) => false,
        }
    }
}
