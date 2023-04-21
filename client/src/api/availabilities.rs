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
    ) -> Result<api_types::availabilities::create::Res, ()> {
        let mut client = Service::client();

        match client
            .create(Bytes {
                value: req.to_bincode(),
            })
            .await
        {
            Ok(res) => {
                match api_types::availabilities::create::Res::from_bincode(&res.into_inner().value)
                {
                    Ok(res) => Ok(res),
                    Err(_) => todo!("Handle error"),
                }
            }
            Err(_) => todo!("Handle error"),
        }
    }

    pub async fn get(
        req: api_types::availabilities::get::Req,
    ) -> Result<api_types::availabilities::get::Res, ()> {
        let mut client = Service::client();

        match client
            .get(Bytes {
                value: req.to_bincode(),
            })
            .await
        {
            Ok(res) => {
                match api_types::availabilities::get::Res::from_bincode(&res.into_inner().value) {
                    Ok(res) => Ok(res),
                    Err(_) => todo!("Handle error"),
                }
            }
            Err(err) => todo!("Handle error: {:#?}", err),
        }
    }

    pub async fn update(
        req: api_types::availabilities::update::Req,
    ) -> Result<api_types::availabilities::update::Res, ()> {
        let mut client = Service::client();

        match client
            .update(Bytes {
                value: req.to_bincode(),
            })
            .await
        {
            Ok(res) => {
                match api_types::availabilities::update::Res::from_bincode(&res.into_inner().value)
                {
                    Ok(res) => Ok(res),
                    Err(_) => todo!("Handle error"),
                }
            }
            Err(_) => todo!("Handle error"),
        }
    }
}
