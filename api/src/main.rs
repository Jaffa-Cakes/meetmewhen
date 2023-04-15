use database::*;

mod api;
mod database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new();

    api::server(db).await?;

    Ok(())
}
