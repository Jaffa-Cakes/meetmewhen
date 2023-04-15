use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[derive(Clone)]
pub struct Database(Pool<ConnectionManager<PgConnection>>);

impl Database {
    // Cuts down some boilerplate when wanting a new database connection
    pub fn get_conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.0.get().expect("Couldn't get DB connection from pool")
    }

    pub fn new() -> Database {
        // Double check if this needs any changes to the number of connections and threads
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");

        let manager = ConnectionManager::<PgConnection>::new(&database_url);

        let database = Database(
            Pool::builder()
                .max_size(num_cpus::get().try_into().unwrap())
                .build(manager)
                .unwrap_or_else(|err| {
                    panic!("Error connecting to '{}': '{:?}'", database_url, err)
                }),
        );

        {
            database
                .get_conn()
                .run_pending_migrations(MIGRATIONS)
                .expect("Failed to run migrations");
        }

        database
    }
}