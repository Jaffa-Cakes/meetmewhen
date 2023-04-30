use super::*;

use crate::basic_event::*;

pub struct Service;

impl Service {
    fn client() -> Client<WebClient> {
        let base_url = "http://localhost:50052".to_string();
        let wasm_client = WebClient::new(base_url);

        Client::new(wasm_client)
    }

    pub async fn create(
        req: api_types::basic_event::create::Req,
    ) -> api_types::basic_event::create::Package {
        use api_types::basic_event::create::*;

        let mut client = Service::client();

        if !req.is_valid() {
            todo!("Invalid request data");
        }

        match client
            .create(Bytes {
                value: req.to_bincode(),
            })
            .await
        {
            Ok(res) => match Package::from_bincode(&res.into_inner().value) {
                Ok(res) => res,
                Err(_) => todo!("Failed to deserialize bincoded response from API"),
            },
            Err(_) => todo!("API failed to respond"),
        }
    }

    pub async fn get(
        req: api_types::basic_event::get::Req,
    ) -> api_types::basic_event::get::Package {
        use api_types::basic_event::get::*;

        let mut client = Service::client();

        match client
            .get(Bytes {
                value: req.to_bincode(),
            })
            .await
        {
            Ok(res) => match Package::from_bincode(&res.into_inner().value) {
                Ok(res) => res,
                Err(_) => todo!("Failed to deserialize bincoded response from API"),
            },
            Err(_) => todo!("API failed to respond"),
        }
    }
}
