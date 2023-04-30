use super::*;
use api_types::Bincoded;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use api::basic_event::*;

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
        use api_types::basic_event::create::*;

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

        let tz = req.timezone.as_hms();
        let tz = format!(
            "{:0width$}:{:0width$}:{:0width$}",
            tz.0,
            tz.1,
            tz.2,
            width = 2
        );

        let mut conn = self.db.get_conn();

        let mut id_constructed: String;

        loop {
            use diesel::dsl::*;
            use schema::basic_event::dsl::*;

            id_constructed = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(8)
                .map(char::from)
                .collect();

            let result = conn.transaction(|conn| {
                insert_into(basic_event)
                    .values((
                        id.eq(&id_constructed),
                        name.eq(&req.name),
                        when.eq(req.when.to_bincode()),
                        no_ealier.eq(req.no_earlier),
                        no_later.eq(req.no_later),
                        timezone.eq(&tz),
                    ))
                    .execute(conn)
            });

            // If the key is not unique, we will need to try again
            if result.is_ok() {
                break;
            }
        }

        package = Ok(Res { id: id_constructed });

        prepare(package)
    }

    async fn get(&self, req: Request<Bytes>) -> Result<Response<Bytes>, Status> {
        use api_types::basic_event::get::*;

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

        {
            match conn.transaction(|conn| {
                use schema::basic_event::dsl::*;

                basic_event.filter(id.eq(req.id)).first::<(
                    String,
                    String,
                    Vec<u8>,
                    time::Time,
                    time::Time,
                    String,
                    time::OffsetDateTime,
                )>(conn)
            }) {
                Ok((id, name, when, no_earlier, no_later, timezone, created)) => {
                    let when = match api_types::basic_event::When::from_bincode(&when) {
                        Ok(when) => when,
                        Err(_) => panic!("Error handling for invalid bincode"),
                    };
                    let timezone = {
                        let hours = match timezone.split(':').next() {
                            Some(hours) => match hours.parse::<i8>() {
                                Ok(hours) => hours,
                                Err(_) => panic!("Error handling for invalid timezone"),
                            },
                            None => panic!("Error handling for invalid timezone"),
                        };

                        match time::UtcOffset::from_hms(hours, 0, 0) {
                            Ok(timezone) => timezone,
                            Err(_) => panic!("Error handling for invalid bincode"),
                        }
                    };

                    package = Ok(Res {
                        id,
                        name,
                        when,
                        no_earlier,
                        no_later,
                        timezone,
                        created,
                    });

                    prepare(package)
                }
                Err(_) => {
                    package = Err(Error::NotFound);
                    return prepare(package);
                }
            }
        }
    }
}
