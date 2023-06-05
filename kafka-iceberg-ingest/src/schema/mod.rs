use apache_avro::Schema;
use apicurio_api::apis::{artifacts_api, configuration};

pub async fn get_schema(schema_registry_host: &str, topic: &str) -> Result<Schema, anyhow::Error> {
    let mut configuration = configuration::Configuration::default();

    configuration.base_path = schema_registry_host.to_string() + "/apis/registry/v2";

    let mut json =
        artifacts_api::get_latest_artifact(&configuration, "default", &topic, Some(true)).await?;

    if let Some(serde_json::Value::Array(fields)) = json.get_mut("fields") {
        if let Some(source) = fields.get_mut(2) {
            if let Some(source_type) = source.get_mut("type") {
                *source_type = serde_json::Value::String("string".to_string());
            }
        }
    }

    Schema::parse_str(&json.to_string()).map_err(anyhow::Error::msg)
}
