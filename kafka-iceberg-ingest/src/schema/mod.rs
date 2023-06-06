use apache_avro::Schema;
use apicurio_api::apis::{artifacts_api, configuration};

pub async fn get_value_schema(
    schema_registry_host: &str,
    group_id: &str,
    topic: &str,
) -> Result<Schema, anyhow::Error> {
    let mut configuration = configuration::Configuration::default();

    configuration.base_path = schema_registry_host.to_string() + "/apis/registry/v2";

    let mut json = artifacts_api::get_latest_artifact(
        &configuration,
        group_id,
        &(topic.to_string() + "-value"),
        None,
    )
    .await
    .expect("Failed to get artifact.");

    if let Some(serde_json::Value::Array(fields)) = json.get_mut("fields") {
        if let Some(source) = fields.get_mut(2) {
            if let Some(source_type) = source.get_mut("type") {
                *source_type = {
                    if let serde_json::Value::String(source_type) = source_type {
                        artifacts_api::get_latest_artifact(
                            &configuration,
                            group_id,
                            &source_type,
                            None,
                        )
                        .await
                        .expect("Failed to get artifact.")
                    } else {
                        serde_json::Value::String("Value".to_string())
                    }
                };
            }
        }
    }

    Schema::parse_str(&json.to_string()).map_err(anyhow::Error::msg)
}

pub async fn get_key_schema(
    schema_registry_host: &str,
    group_id: &str,
    topic: &str,
) -> Result<Schema, anyhow::Error> {
    let mut configuration = configuration::Configuration::default();

    configuration.base_path = schema_registry_host.to_string() + "/apis/registry/v2";

    let json = artifacts_api::get_latest_artifact(
        &configuration,
        group_id,
        &(topic.to_string() + "-key"),
        None,
    )
    .await
    .expect("Failed to get artifact.");

    Schema::parse_str(&json.to_string()).map_err(anyhow::Error::msg)
}
