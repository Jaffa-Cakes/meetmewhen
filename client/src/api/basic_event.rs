use super::*;

use crate::basic_event::*;

#[cfg(target_arch = "wasm32")]
type ClientReturn = Client<WebClient>;
#[cfg(not(target_arch = "wasm32"))]
type ClientReturn = Client<tonic::transport::Channel>;

mod hidden {
    tonic::include_proto!("basic_event");
}

pub enum Type {
    Dates = 0,
    Days = 1,
}

pub struct Service;

impl Service {
    async fn client() -> ClientReturn {
        #[cfg(target_arch = "wasm32")]
        let out = Client::new(WebClient::new("http://localhost:50052".to_string()));
        #[cfg(not(target_arch = "wasm32"))]
        let out = Client::connect("http://localhost:50053".to_string()).await.unwrap();

        out
    }

    pub async fn create(
        req: api_types::basic_event::create::Req,
    ) -> Result<api_types::basic_event::create::Res, ()> {
        let mut client = Service::client().await;

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
        println!("Creating Client");
        let mut client = Service::client().await;

        println!("Sending Request");
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
