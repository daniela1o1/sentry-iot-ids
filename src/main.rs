mod api;
mod database;
mod detection;
mod ingest;
mod models;

use api::routes::create_router;
use database::pool::create_pool;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let pool = create_pool(&database_url).await?;

    println!("Connected to PostgreSQL");

    let app = create_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("API listening on port 3000");

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    ingest::mqtt::start_mqtt_ingest(pool).await?;

    Ok(())
}
