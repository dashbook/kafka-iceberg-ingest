/*
 * Apicurio Registry API [v2]
 *
 * Apicurio Registry is a datastore for standard event schemas and API designs. Apicurio Registry enables developers to manage and share the structure of their data using a REST interface. For example, client applications can dynamically push or pull the latest updates to or from the registry without needing to redeploy. Apicurio Registry also enables developers to create rules that govern how registry content can evolve over time. For example, this includes rules for content validation and version compatibility.  The Apicurio Registry REST API enables client applications to manage the artifacts in the registry. This API provides create, read, update, and delete operations for schema and API artifacts, rules, versions, and metadata.   The supported artifact types include: - Apache Avro schema - AsyncAPI specification - Google protocol buffers - GraphQL schema - JSON Schema - Kafka Connect schema - OpenAPI specification - Web Services Description Language - XML Schema Definition   **Important**: The Apicurio Registry REST API is available from `https://MY-REGISTRY-URL/apis/registry/v2` by default. Therefore you must prefix all API operation paths with `../apis/registry/v2` in this case. For example: `../apis/registry/v2/ids/globalIds/{globalId}`. 
 *
 * The version of the OpenAPI document: 2.2.5.Final
 * Contact: apicurio@lists.jboss.org
 * Generated by: https://openapi-generator.tech
 */

/// ConfigurationProperty : 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConfigurationProperty {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: String,
    /// 
    #[serde(rename = "type")]
    pub r#type: String,
    /// 
    #[serde(rename = "label")]
    pub label: String,
    /// 
    #[serde(rename = "description")]
    pub description: String,
}

impl ConfigurationProperty {
    /// 
    pub fn new(name: String, value: String, r#type: String, label: String, description: String) -> ConfigurationProperty {
        ConfigurationProperty {
            name,
            value,
            r#type,
            label,
            description,
        }
    }
}


