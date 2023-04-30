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
        use api_types::availabilities::create::*;

        let Bytes { value: req } = req.into_inner();
        let package: Package;

        let req = match Req::from_bincode(&req) {
            Ok(req) => match req.is_valid() {
                true => req,
                false => {
                    package = Err(Error::InvalidRequest);
                    return prepare(package);
                }
            },
            Err(_) => {
                package = Err(Error::InvalidBincode);
                return prepare(package);
            }
        };

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
                Err(_) => {
                    package = Err(Error::BasicEventNotFoundOrDuplicateName);
                    return prepare(package);
                }
                Ok(id) => id,
            }
        };

        package = Ok(Res { id });

        prepare(package)
    }

    async fn get(&self, req: Request<Bytes>) -> Result<Response<Bytes>, Status> {
        use api_types::availabilities::get::*;

        let Bytes { value: req } = req.into_inner();
        let package: Package;

        let req = match Req::from_bincode(&req) {
            Ok(req) => req,
            Err(_) => {
                package = Err(Error::InvalidBincode);
                return prepare(package);
            }
        };

        let mut conn = self.db.get_conn();

        let respondents = {
            match conn.transaction(|conn| {
                use schema::availability::dsl::*;

                availability
                    .filter(basic_event.eq(req.basic_event))
                    .get_results::<(i32, String, String, Vec<u8>)>(conn)
            }) {
                Err(_) => {
                    package = Err(Error::BasicEventNotFound);
                    return prepare(package);
                }
                Ok(results) => results
                    .into_iter()
                    .map(|r| {
                        let id = r.0;
                        let name = r.2;
                        let availabilities =
                            api_types::availabilities::Availabilities::from_bincode(&r.3).unwrap();

                        Respondent {
                            id,
                            name,
                            availabilities,
                        }
                    })
                    .collect::<Vec<Respondent>>(),
            }
        };

        package = Ok(Res { respondents });

        prepare(package)
    }

    async fn update(&self, req: Request<Bytes>) -> Result<Response<Bytes>, Status> {
        use api_types::availabilities::update::*;

        let Bytes { value: req } = req.into_inner();
        let package: Package;

        let req = match Req::from_bincode(&req) {
            Ok(req) => match req.is_valid() {
                true => req,
                false => {
                    package = Err(Error::InvalidRequest);
                    return prepare(package);
                }
            },
            Err(_) => {
                package = Err(Error::InvalidBincode);
                return prepare(package);
            }
        };

        let mut conn = self.db.get_conn();

        let id = {
            match conn.transaction(|conn| {
                use diesel::dsl::*;
                use schema::availability::dsl::*;

                update(availability)
                    .filter(id.eq(req.id))
                    .filter(basic_event.eq(req.basic_event))
                    .set((
                        name.eq(req.name),
                        availabilities.eq(req.availabilities.to_bincode()),
                    ))
                    .returning(id)
                    .get_result::<i32>(conn)
            }) {
                Err(_) => {
                    package = Err(Error::BasicEventOrRespondentNotFound);
                    return prepare(package);
                }
                Ok(id) => id,
            }
        };

        package = Ok(Res { id });

        prepare(package)
    }

    async fn delete(&self, req: Request<Bytes>) -> Result<Response<Bytes>, Status> {
        use api_types::availabilities::delete::*;

        let Bytes { value: req } = req.into_inner();
        let package: Package;

        let req = match Req::from_bincode(&req) {
            Ok(req) => req,
            Err(_) => {
                package = Err(Error::InvalidBincode);
                return prepare(package);
            }
        };

        let mut conn = self.db.get_conn();

        if let Err(_) = conn.transaction(|conn| {
            use diesel::dsl::*;
            use schema::availability::dsl::*;

            delete(availability)
                .filter(id.eq(req.id))
                .filter(basic_event.eq(req.basic_event))
                .execute(conn)
        }) {
            package = Err(Error::BasicEventOrRespondentNotFound);
            return prepare(package);
        }

        package = Ok(Res {});

        prepare(package)
    }
}
