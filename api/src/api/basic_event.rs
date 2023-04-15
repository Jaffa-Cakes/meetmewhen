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
// test
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

        // let time_format = time::macros::format_description!("[hour]:[minute]:[second]");

        // let no_ealier_constructed =
        //     match time::Time::parse(&format!("{:0width$}:00:00", r.no_earlier, width = 2), time_format) {
        //         Ok(time) => time,
        //         Err(_) => {
        //             println!("{}", format!("{:0width$}:00:00", r.no_earlier, width = 2));
        //             todo!("Error handling for invalid time")
        //         },
        //     };
        // let no_later_constructed =
        //     match time::Time::parse(&format!("{:0width$}:00:00", r.no_later, width = 2), time_format) {
        //         Ok(time) => time,
        //         Err(_) => todo!("Error handling for invalid time"),
        //     };

        // if r.when.len() == 0 {
        //     todo!("Error handling for invalid when")
        // }

        // if r.name.len() == 0 {
        //     todo!("Error handling for invalid name")
        // }

        // let when_constructed = r.when.join("|");

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
}
