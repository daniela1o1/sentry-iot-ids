mod detection;
mod ingest;
mod models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ingest::mqtt::start_mqtt_ingest().await?;

    Ok(())
}
