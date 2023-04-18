use super::*;
use api_types::Bincoded;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use api::basic_event::*;
use api_types::prelude::*;

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

        let req = match api_types::basic_event::create::Req::from_bincode(&req) {
            Ok(req) => match req.is_valid() {
                true => req,
                false => todo!("Error handling for invalid request"),
            },
            Err(_) => todo!("Error handling for invalid bincode"),
        };

        let id_constructed: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();

        let tz = req.timezone.as_hms();
        let tz = format!(
            "{:0width$}:{:0width$}:{:0width$}",
            tz.0,
            tz.1,
            tz.2,
            width = 2
        );

        let mut conn = self.db.get_conn();

        {
            use diesel::dsl::*;
            use schema::basic_event::dsl::*;

            if let Err(_) = conn.transaction(|conn| {
                insert_into(basic_event)
                    .values((
                        id.eq(&id_constructed),
                        name.eq(req.name),
                        when.eq(req.when.to_bincode()),
                        no_ealier.eq(req.no_earlier),
                        no_later.eq(req.no_later),
                        timezone.eq(tz),
                    ))
                    .execute(conn)
            }) {
                todo!("Error handling for duplicate id insertion")
            }
        }

        Ok(Response::new(Bytes {
            value: api_types::basic_event::create::Res { id: id_constructed }.to_bincode(),
        }))
    }

    async fn get(&self, req: Request<Bytes>) -> Result<Response<Bytes>, Status> {
        let Bytes { value: req } = req.into_inner();

        let req = match api_types::basic_event::get::Req::from_bincode(&req) {
            Ok(req) => req,
            Err(_) => todo!("Error handling for invalid bincode"),
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
                        Err(_) => todo!("Error handling for invalid bincode"),
                    };
                    let timezone = {
                        let hours = match timezone.split(':').next() {
                            Some(hours) => match hours.parse::<i8>() {
                                Ok(hours) => hours,
                                Err(_) => todo!("Error handling for invalid timezone"),
                            },
                            None => todo!("Error handling for invalid timezone"),
                        };

                        match time::UtcOffset::from_hms(hours, 0, 0) {
                            Ok(timezone) => timezone,
                            Err(_) => todo!("Error handling for invalid bincode"),
                        }
                    };

                    Ok(Response::new(Bytes {
                        value: api_types::basic_event::get::Res {
                            id,
                            name,
                            when,
                            no_earlier,
                            no_later,
                            timezone,
                            created,
                        }
                        .to_bincode(),
                    }))
                }
                Err(_) => todo!("Error handling for not found"),
            }
        }
    }
}
