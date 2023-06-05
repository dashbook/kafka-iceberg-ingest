/*
 * Cloud Native Data Schema Registry
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SchemaGroup {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "createdtimeutc", skip_serializing_if = "Option::is_none")]
    pub createdtimeutc: Option<String>,
    #[serde(rename = "updatedtimeutc", skip_serializing_if = "Option::is_none")]
    pub updatedtimeutc: Option<String>,
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Set of properties for a schemagroup.
    #[serde(rename = "groupProperties", skip_serializing_if = "Option::is_none")]
    pub group_properties: Option<::std::collections::HashMap<String, String>>,
}

impl SchemaGroup {
    pub fn new() -> SchemaGroup {
        SchemaGroup {
            id: None,
            description: None,
            createdtimeutc: None,
            updatedtimeutc: None,
            format: None,
            group_properties: None,
        }
    }
}


