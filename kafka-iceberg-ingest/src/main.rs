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

    let mut json =
        artifacts_api::get_latest_artifact(&configuration, "default", &topic, Some(true))
            .await
            .expect("Failed to get schema of topic.");

    if let Some(serde_json::Value::Array(fields)) = json.get_mut("fields") {
        if let Some(source) = fields.get_mut(2) {
            if let Some(source_type) = source.get_mut("type") {
                *source_type = serde_json::Value::String("string".to_string());
            }
        }
    }

    let schema = Schema::parse_str(&json.to_string()).expect("Failed to parse schema.");
    dbg!(&schema);
}
