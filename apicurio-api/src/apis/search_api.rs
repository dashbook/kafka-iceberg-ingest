/*
 * Apicurio Registry API [v2]
 *
 * Apicurio Registry is a datastore for standard event schemas and API designs. Apicurio Registry enables developers to manage and share the structure of their data using a REST interface. For example, client applications can dynamically push or pull the latest updates to or from the registry without needing to redeploy. Apicurio Registry also enables developers to create rules that govern how registry content can evolve over time. For example, this includes rules for content validation and version compatibility.  The Apicurio Registry REST API enables client applications to manage the artifacts in the registry. This API provides create, read, update, and delete operations for schema and API artifacts, rules, versions, and metadata.   The supported artifact types include: - Apache Avro schema - AsyncAPI specification - Google protocol buffers - GraphQL schema - JSON Schema - Kafka Connect schema - OpenAPI specification - Web Services Description Language - XML Schema Definition   **Important**: The Apicurio Registry REST API is available from `https://MY-REGISTRY-URL/apis/registry/v2` by default. Therefore you must prefix all API operation paths with `../apis/registry/v2` in this case. For example: `../apis/registry/v2/ids/globalIds/{globalId}`.
 *
 * The version of the OpenAPI document: 2.2.5.Final
 * Contact: apicurio@lists.jboss.org
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::{
    apis::ResponseContent,
    models::{ArtifactType, SortBy, SortOrder},
};

/// struct for typed errors of method [`search_artifacts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchArtifactsError {
    Status500(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`search_artifacts_by_content`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchArtifactsByContentError {
    Status500(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// Returns a paginated list of all artifacts that match the provided filter criteria.
pub async fn search_artifacts(
    configuration: &configuration::Configuration,
    name: Option<&str>,
    offset: Option<i32>,
    limit: Option<i32>,
    order: Option<SortOrder>,
    orderby: Option<SortBy>,
    labels: Option<Vec<String>>,
    properties: Option<Vec<String>>,
    description: Option<&str>,
    group: Option<&str>,
    global_id: Option<i64>,
    content_id: Option<i64>,
) -> Result<crate::models::ArtifactSearchResults, Error<SearchArtifactsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/search/artifacts", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = name {
        local_var_req_builder =
            local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder =
            local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order {
        local_var_req_builder =
            local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = orderby {
        local_var_req_builder =
            local_var_req_builder.query(&[("orderby", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = labels {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("labels".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "labels",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = properties {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("properties".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "properties",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = description {
        local_var_req_builder =
            local_var_req_builder.query(&[("description", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = group {
        local_var_req_builder =
            local_var_req_builder.query(&[("group", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = global_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("globalId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = content_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("contentId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SearchArtifactsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a paginated list of all artifacts with at least one version that matches the posted content.
pub async fn search_artifacts_by_content(
    configuration: &configuration::Configuration,
    body: serde_json::Value,
    canonical: Option<bool>,
    artifact_type: Option<ArtifactType>,
    offset: Option<i32>,
    limit: Option<i32>,
    order: Option<&str>,
    orderby: Option<&str>,
) -> Result<crate::models::ArtifactSearchResults, Error<SearchArtifactsByContentError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/search/artifacts", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = canonical {
        local_var_req_builder =
            local_var_req_builder.query(&[("canonical", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = artifact_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("artifactType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder =
            local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order {
        local_var_req_builder =
            local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = orderby {
        local_var_req_builder =
            local_var_req_builder.query(&[("orderby", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SearchArtifactsByContentError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
