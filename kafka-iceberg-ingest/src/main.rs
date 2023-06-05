use std::env;

use kafka_iceberg_ingest::schema::get_schema;

#[tokio::main]
pub async fn main() {
    let topic = env::var("TOPIC").expect("No topic is specified.");
    let schema_registry_host =
        env::var("SCHEMA_REGISTRY_HOST").unwrap_or("http://localhost".to_string());

    let schema = get_schema(&schema_registry_host, &topic).await;
    dbg!(&schema);
}
