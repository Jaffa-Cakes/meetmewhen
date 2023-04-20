use super::*;
use api_types::Bincoded;

use api::availabilities::*;

pub struct Service {
    db: Database,
}

impl Service {
    pub fn server(db: Database) -> Server<Service> {
        Server::new(Service { db })
    }
}

#[tonic::async_trait]
impl Trait for Service {
    async fn create(&self, req: Request<Bytes>) -> Result<Response<Bytes>, Status> {
        let Bytes { value: req } = req.into_inner();

        let req = api_types::availabilities::create::Req::from_bincode(&req).unwrap();

        let mut conn = self.db.get_conn();

        let id = {
            match conn.transaction(|conn| {
                use diesel::dsl::*;
                use schema::availability::dsl::*;

                insert_into(availability)
                    .values((
                        basic_event.eq(req.basic_event),
                        name.eq(req.name),
                        availabilities.eq(req.availabilities.to_bincode()),
                    ))
                    .returning(id)
                    .get_result::<i32>(conn)
            }) {
                Err(_) => todo!("Error handling for database error"),
                Ok(id) => id,
            }
        };

        Ok(Response::new(Bytes {
            value: api_types::availabilities::create::Res { id }.to_bincode(),
        }))
    }

    async fn get(&self, req: Request<Bytes>) -> Result<Response<Bytes>, Status> {
        let Bytes { value: req } = req.into_inner();

        let req = api_types::availabilities::get::Req::from_bincode(&req).unwrap();

        let mut conn = self.db.get_conn();

        let respondents = {
            match conn.transaction(|conn| {
                use schema::availability::dsl::*;

                availability
                    .filter(basic_event.eq(req.basic_event))
                    .get_results::<(i32, String, String, Vec<u8>)>(conn)
            }) {
                Err(_) => todo!("Error handling for database error"),
                Ok(results) => results
                    .into_iter()
                    .map(|r| {
                        let id = r.0;
                        let name = r.2;
                        let availabilities =
                            api_types::availabilities::Availabilities::from_bincode(&r.3).unwrap();

                        api_types::availabilities::get::Respondent {
                            id,
                            name,
                            availabilities,
                        }
                    })
                    .collect::<Vec<api_types::availabilities::get::Respondent>>(),
            }
        };

        Ok(Response::new(Bytes {
            value: api_types::availabilities::get::Res { respondents }.to_bincode(),
        }))
    }
}
