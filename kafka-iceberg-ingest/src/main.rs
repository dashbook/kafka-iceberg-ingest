use std::env;

use apache_avro::Schema;
use apicurio_api::apis::{artifacts_api, configuration};

#[tokio::main]
pub async fn main() {
    let topic = env::var("TOPIC").expect("No topic is specified.");
    let schema_registry_host =
        env::var("SCHEMA_REGISTRY_HOST").unwrap_or("http://localhost".to_string());

    let mut configuration = configuration::Configuration::default();

    configuration.base_path = schema_registry_host + "/apis/registry/v2";

    let schema = Schema::parse_str(
        &artifacts_api::get_latest_artifact(&configuration, "default", &topic, Some(true))
            .await
            .expect("Failed to get schema of topic.")
            .to_string(),
    )
    .expect("Failed to parse schema.");
    dbg!(&schema);
}
