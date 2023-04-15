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

    pub async fn create(
        req: api_types::basic_event::create::Req,
    ) -> Result<api_types::basic_event::create::Res, ()> {
        let mut client = Service::client();

        if !req.is_valid() {
            log::info!("Data is invalid.");
            todo!("Handle error");
        }

        match client
            .create(Bytes {
                value: req.to_bincode(),
            })
            .await
        {
            Ok(res) => {
                match api_types::basic_event::create::Res::from_bincode(&res.into_inner().value) {
                    Ok(res) => Ok(res),
                    Err(_) => todo!("Handle error"),
                }
            }
            Err(_) => todo!("Handle error"),
        }
    }

    pub async fn get(
        req: api_types::basic_event::get::Req,
    ) -> Result<api_types::basic_event::get::Res, ()> {
        let mut client = Service::client();

        match client
            .get(Bytes {
                value: req.to_bincode(),
            })
            .await
        {
            Ok(res) => {
                match api_types::basic_event::get::Res::from_bincode(&res.into_inner().value) {
                    Ok(res) => Ok(res),
                    Err(_) => todo!("Handle error"),
                }
            }
            Err(err) => todo!("Handle error: {:#?}", err),
        }
    }
}
