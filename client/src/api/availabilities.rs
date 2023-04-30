use super::*;

use crate::availabilities::*;

pub struct Service;

impl Service {
    fn client() -> Client<WebClient> {
        let base_url = "http://localhost:50052".to_string();
        let wasm_client = WebClient::new(base_url);

        Client::new(wasm_client)
    }

    pub async fn create(
        req: api_types::availabilities::create::Req,
    ) -> api_types::availabilities::create::Package {
        use api_types::availabilities::create::*;

        let mut client = Service::client();

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
        req: api_types::availabilities::get::Req,
    ) -> api_types::availabilities::get::Package {
        use api_types::availabilities::get::*;

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

    pub async fn update(
        req: api_types::availabilities::update::Req,
    ) -> api_types::availabilities::update::Package {
        use api_types::availabilities::update::*;

        let mut client = Service::client();

        match client
            .update(Bytes {
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

    pub async fn delete(
        req: api_types::availabilities::delete::Req,
    ) -> api_types::availabilities::delete::Package {
        use api_types::availabilities::delete::*;

        let mut client = Service::client();

        match client
            .delete(Bytes {
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
