use super::*;

use api::basic_event::*;

pub struct Service {
    db: Database
}

impl Service {
    pub fn server(db: Database) -> Server<Service> {
        Server::new(Service {
            db
        })
    }
}
// test
#[tonic::async_trait]
impl Trait for Service {
    async fn create(&self, _request: Request<CreateReq>) -> Result<Response<CreateRes>, Status> {

        let mut conn = self.db.get_conn();

        {
            use diesel::dsl::*;
            use schema::basic_event::dsl::*;

            conn.transaction(|conn| {

                let tz = {
                    let (hours, minutes, seconds) = time::UtcOffset::UTC.as_hms();
                    format!("{}:{}:{}", hours, minutes, seconds)
                };

                insert_into(basic_event)
                    .values((
                        id.eq("123abc"),
                        name.eq("event name"),
                        when.eq("M|W|T"),
                        no_ealier.eq(time::Time::from_hms(0, 0, 0).unwrap()),
                        no_later.eq(time::Time::from_hms(23, 59, 59).unwrap()),
                        timezone.eq(tz),
                    ))
                    .execute(conn)
            }).unwrap();
        }

        Ok(Response::new(CreateRes { id: 1 }))
    }
}
