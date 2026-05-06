mod database;
mod detection;
mod ingest;
mod models;

use database::pool::create_pool;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let pool = create_pool(&database_url).await?;

    println!("Connected to PostgreSQL");

    ingest::mqtt::start_mqtt_ingest(pool).await?;

    Ok(())
}
