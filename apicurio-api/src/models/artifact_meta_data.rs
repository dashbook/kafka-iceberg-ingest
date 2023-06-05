/*
 * Apicurio Registry API [v2]
 *
 * Apicurio Registry is a datastore for standard event schemas and API designs. Apicurio Registry enables developers to manage and share the structure of their data using a REST interface. For example, client applications can dynamically push or pull the latest updates to or from the registry without needing to redeploy. Apicurio Registry also enables developers to create rules that govern how registry content can evolve over time. For example, this includes rules for content validation and version compatibility.  The Apicurio Registry REST API enables client applications to manage the artifacts in the registry. This API provides create, read, update, and delete operations for schema and API artifacts, rules, versions, and metadata.   The supported artifact types include: - Apache Avro schema - AsyncAPI specification - Google protocol buffers - GraphQL schema - JSON Schema - Kafka Connect schema - OpenAPI specification - Web Services Description Language - XML Schema Definition   **Important**: The Apicurio Registry REST API is available from `https://MY-REGISTRY-URL/apis/registry/v2` by default. Therefore you must prefix all API operation paths with `../apis/registry/v2` in this case. For example: `../apis/registry/v2/ids/globalIds/{globalId}`. 
 *
 * The version of the OpenAPI document: 2.2.5.Final
 * Contact: apicurio@lists.jboss.org
 * Generated by: https://openapi-generator.tech
 */

/// ArtifactMetaData : 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArtifactMetaData {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "createdBy")]
    pub created_by: String,
    #[serde(rename = "createdOn")]
    pub created_on: String,
    #[serde(rename = "modifiedBy")]
    pub modified_by: String,
    #[serde(rename = "modifiedOn")]
    pub modified_on: String,
    /// The ID of a single artifact.
    #[serde(rename = "id")]
    pub id: String,
    /// 
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "type")]
    pub r#type: crate::models::ArtifactType,
    /// 
    #[serde(rename = "globalId")]
    pub global_id: i64,
    #[serde(rename = "state")]
    pub state: crate::models::ArtifactState,
    /// 
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// User-defined name-value pairs. Name and value must be strings.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, String>>,
    /// An ID of a single artifact group.
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// 
    #[serde(rename = "contentId")]
    pub content_id: i64,
    /// 
    #[serde(rename = "references", skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<crate::models::ArtifactReference>>,
}

impl ArtifactMetaData {
    /// 
    pub fn new(created_by: String, created_on: String, modified_by: String, modified_on: String, id: String, version: String, r#type: crate::models::ArtifactType, global_id: i64, state: crate::models::ArtifactState, content_id: i64) -> ArtifactMetaData {
        ArtifactMetaData {
            name: None,
            description: None,
            created_by,
            created_on,
            modified_by,
            modified_on,
            id,
            version,
            r#type,
            global_id,
            state,
            labels: None,
            properties: None,
            group_id: None,
            content_id,
            references: None,
        }
    }
}


