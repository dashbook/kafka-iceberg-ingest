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

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`create_artifact_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateArtifactRuleError {
    Status400(crate::models::Error),
    Status404(crate::models::Error),
    Status500(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_artifact_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteArtifactRuleError {
    Status404(crate::models::Error),
    Status500(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_artifact_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteArtifactRulesError {
    Status404(crate::models::Error),
    Status500(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_artifact_rule_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetArtifactRuleConfigError {
    Status404(crate::models::Error),
    Status500(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_artifact_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListArtifactRulesError {
    Status404(crate::models::Error),
    Status500(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`test_update_artifact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestUpdateArtifactError {
    Status404(crate::models::Error),
    Status409(crate::models::RuleViolationError),
    Status500(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_artifact_rule_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateArtifactRuleConfigError {
    Status404(crate::models::Error),
    Status500(crate::models::Error),
    UnknownValue(serde_json::Value),
}


/// Adds a rule to the list of rules that get applied to the artifact when adding new versions.  All configured rules must pass to successfully add a new artifact version.  This operation can fail for the following reasons:  * No artifact with this `artifactId` exists (HTTP error `404`) * Rule (named in the request body) is unknown (HTTP error `400`) * A server error occurred (HTTP error `500`)
pub async fn create_artifact_rule(configuration: &configuration::Configuration, group_id: &str, artifact_id: &str, rule: crate::models::Rule) -> Result<(), Error<CreateArtifactRuleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/groups/{groupId}/artifacts/{artifactId}/rules", local_var_configuration.base_path, groupId=crate::apis::urlencode(group_id), artifactId=crate::apis::urlencode(artifact_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&rule);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CreateArtifactRuleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a rule from the artifact.  This results in the rule no longer applying for this artifact.  If this is the only rule configured for the artifact, this is the  same as deleting **all** rules, and the globally configured rules now apply to this artifact.  This operation can fail for the following reasons:  * No artifact with this `artifactId` exists (HTTP error `404`) * No rule with this name/type is configured for this artifact (HTTP error `404`) * Invalid rule type (HTTP error `400`) * A server error occurred (HTTP error `500`)
pub async fn delete_artifact_rule(configuration: &configuration::Configuration, group_id: &str, artifact_id: &str, rule: &str) -> Result<(), Error<DeleteArtifactRuleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/groups/{groupId}/artifacts/{artifactId}/rules/{rule}", local_var_configuration.base_path, groupId=crate::apis::urlencode(group_id), artifactId=crate::apis::urlencode(artifact_id), rule=crate::apis::urlencode(rule));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteArtifactRuleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes all of the rules configured for the artifact.  After this is done, the global rules apply to the artifact again.  This operation can fail for the following reasons:  * No artifact with this `artifactId` exists (HTTP error `404`) * A server error occurred (HTTP error `500`)
pub async fn delete_artifact_rules(configuration: &configuration::Configuration, group_id: &str, artifact_id: &str) -> Result<(), Error<DeleteArtifactRulesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/groups/{groupId}/artifacts/{artifactId}/rules", local_var_configuration.base_path, groupId=crate::apis::urlencode(group_id), artifactId=crate::apis::urlencode(artifact_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteArtifactRulesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns information about a single rule configured for an artifact.  This is useful when you want to know what the current configuration settings are for a specific rule.  This operation can fail for the following reasons:  * No artifact with this `artifactId` exists (HTTP error `404`) * No rule with this name/type is configured for this artifact (HTTP error `404`) * Invalid rule type (HTTP error `400`) * A server error occurred (HTTP error `500`)
pub async fn get_artifact_rule_config(configuration: &configuration::Configuration, group_id: &str, artifact_id: &str, rule: &str) -> Result<crate::models::Rule, Error<GetArtifactRuleConfigError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/groups/{groupId}/artifacts/{artifactId}/rules/{rule}", local_var_configuration.base_path, groupId=crate::apis::urlencode(group_id), artifactId=crate::apis::urlencode(artifact_id), rule=crate::apis::urlencode(rule));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetArtifactRuleConfigError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a list of all rules configured for the artifact.  The set of rules determines how the content of an artifact can evolve over time.  If no rules are configured for an artifact, the set of globally configured rules are used.  If no global rules  are defined, there are no restrictions on content evolution.  This operation can fail for the following reasons:  * No artifact with this `artifactId` exists (HTTP error `404`) * A server error occurred (HTTP error `500`)
pub async fn list_artifact_rules(configuration: &configuration::Configuration, group_id: &str, artifact_id: &str) -> Result<Vec<crate::models::RuleType>, Error<ListArtifactRulesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/groups/{groupId}/artifacts/{artifactId}/rules", local_var_configuration.base_path, groupId=crate::apis::urlencode(group_id), artifactId=crate::apis::urlencode(artifact_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListArtifactRulesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Tests whether an update to the artifact's content *would* succeed for the provided content. Ultimately, this applies any rules configured for the artifact against the given content to determine whether the rules would pass or fail, but without actually updating the artifact content.  The body of the request should be the raw content of the artifact.  This is typically in  JSON format for *most* of the supported types, but may be in another format for a few  (for example, `PROTOBUF`).  The update could fail for a number of reasons including:  * Provided content (request body) was empty (HTTP error `400`) * No artifact with the `artifactId` exists (HTTP error `404`) * The new content violates one of the rules configured for the artifact (HTTP error `409`) * The provided artifact type is not recognized (HTTP error `404`) * A server error occurred (HTTP error `500`)  When successful, this operation simply returns a *No Content* response.  This response indicates that the content is valid against the configured content rules for the  artifact (or the global rules if no artifact rules are enabled).
pub async fn test_update_artifact(configuration: &configuration::Configuration, group_id: &str, artifact_id: &str, body: Option<serde_json::Value>) -> Result<(), Error<TestUpdateArtifactError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/groups/{groupId}/artifacts/{artifactId}/test", local_var_configuration.base_path, groupId=crate::apis::urlencode(group_id), artifactId=crate::apis::urlencode(artifact_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<TestUpdateArtifactError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates the configuration of a single rule for the artifact.  The configuration data is specific to each rule type, so the configuration of the `COMPATIBILITY` rule  is in a different format from the configuration of the `VALIDITY` rule.  This operation can fail for the following reasons:  * No artifact with this `artifactId` exists (HTTP error `404`) * No rule with this name/type is configured for this artifact (HTTP error `404`) * Invalid rule type (HTTP error `400`) * A server error occurred (HTTP error `500`) 
pub async fn update_artifact_rule_config(configuration: &configuration::Configuration, group_id: &str, artifact_id: &str, rule: &str, rule2: crate::models::Rule) -> Result<crate::models::Rule, Error<UpdateArtifactRuleConfigError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/groups/{groupId}/artifacts/{artifactId}/rules/{rule}", local_var_configuration.base_path, groupId=crate::apis::urlencode(group_id), artifactId=crate::apis::urlencode(artifact_id), rule=crate::apis::urlencode(rule));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&rule2);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateArtifactRuleConfigError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

