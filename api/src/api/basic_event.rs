use super::*;
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
// test
#[tonic::async_trait]
impl Trait for Service {
    async fn create(&self, request: Request<CreateReq>) -> Result<Response<CreateRes>, Status> {
        let r = request.into_inner();

        let time_format = time::macros::format_description!("[hour]:[minute]:[second]");

        let no_ealier_constructed =
            match time::Time::parse(&format!("{:0width$}:00:00", r.no_earlier, width = 2), time_format) {
                Ok(time) => time,
                Err(_) => {
                    println!("{}", format!("{:0width$}:00:00", r.no_earlier, width = 2));
                    todo!("Error handling for invalid time")
                },
            };
        let no_later_constructed =
            match time::Time::parse(&format!("{:0width$}:00:00", r.no_later, width = 2), time_format) {
                Ok(time) => time,
                Err(_) => todo!("Error handling for invalid time"),
            };

        if r.when.len() == 0 {
            todo!("Error handling for invalid when")
        }

        if r.name.len() == 0 {
            todo!("Error handling for invalid name")
        }

        let when_constructed = r.when.join("|");

        let id_constructed: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();

        if r.timezone.len() == 0 {
            todo!("Error handling for invalid timezone")
        }

        let mut conn = self.db.get_conn();

        {
            use diesel::dsl::*;
            use schema::basic_event::dsl::*;

            if let Err(_) = conn.transaction(|conn| {
                insert_into(basic_event)
                    .values((
                        id.eq(&id_constructed),
                        name.eq(r.name),
                        when.eq(when_constructed),
                        no_ealier.eq(no_ealier_constructed),
                        no_later.eq(no_later_constructed),
                        timezone.eq(r.timezone),
                    ))
                    .execute(conn)
            }) {
                todo!("Error handling for duplicate id insertion")
            }
        }

        Ok(Response::new(CreateRes { id: id_constructed }))
    }
}