# \VersionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_artifact_version**](VersionsApi.md#create_artifact_version) | **POST** /groups/{groupId}/artifacts/{artifactId}/versions | Create artifact version
[**get_artifact_version**](VersionsApi.md#get_artifact_version) | **GET** /groups/{groupId}/artifacts/{artifactId}/versions/{version} | Get artifact version
[**get_artifact_version_references**](VersionsApi.md#get_artifact_version_references) | **GET** /groups/{groupId}/artifacts/{artifactId}/versions/{version}/references | Get artifact version
[**list_artifact_versions**](VersionsApi.md#list_artifact_versions) | **GET** /groups/{groupId}/artifacts/{artifactId}/versions | List artifact versions
[**update_artifact_version_state**](VersionsApi.md#update_artifact_version_state) | **PUT** /groups/{groupId}/artifacts/{artifactId}/versions/{version}/state | Update artifact version state



## create_artifact_version

> crate::models::VersionMetaData create_artifact_version(group_id, artifact_id, body, x_registry_version, x_registry_name, x_registry_description, x_registry_description_encoded, x_registry_name_encoded)
Create artifact version

Creates a new version of the artifact by uploading new content.  The configured rules for the artifact are applied, and if they all pass, the new content is added as the most recent  version of the artifact.  If any of the rules fail, an error is returned.  The body of the request can be the raw content of the new artifact version, or the raw content  and a set of references pointing to other artifacts, and the type of that content should match the artifact's type (for example if the artifact type is `AVRO` then the content of the request should be an Apache Avro document).  This operation can fail for the following reasons:  * Provided content (request body) was empty (HTTP error `400`) * No artifact with this `artifactId` exists (HTTP error `404`) * The new content violates one of the rules configured for the artifact (HTTP error `409`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The artifact group ID.  Must be a string provided by the client, representing the name of the grouping of artifacts. | [required] |
**artifact_id** | **String** | The artifact ID.  Can be a string (client-provided) or UUID (server-generated), representing the unique artifact identifier. | [required] |
**body** | Option<**serde_json::Value**> | The content of the artifact version being created or the content and a set of references to other artifacts. This is often, but not always, JSON data representing one of the supported artifact types:  * Avro (`AVRO`) * Protobuf (`PROTOBUF`) * JSON Schema (`JSON`) * Kafka Connect (`KCONNECT`) * OpenAPI (`OPENAPI`) * AsyncAPI (`ASYNCAPI`) * GraphQL (`GRAPHQL`) * Web Services Description Language (`WSDL`) * XML Schema (`XSD`)  | [required] |
**x_registry_version** | Option<**String**> | Specifies the version number of this new version of the artifact content.  This would typically be a simple integer or a SemVer value.  It must be unique within the artifact.  If this is not provided, the server will generate a new, unique version number for this new updated content. |  |
**x_registry_name** | Option<**String**> | Specifies the artifact name of this new version of the artifact content. Name must be ASCII-only string. If this is not provided, the server will extract the name from the artifact content. |  |
**x_registry_description** | Option<**String**> | Specifies the artifact description of this new version of the artifact content. Description must be ASCII-only string. If this is not provided, the server will extract the description from the artifact content. |  |
**x_registry_description_encoded** | Option<**String**> | Specifies the artifact description of this new version of the artifact content. Value of this must be Base64 encoded string. If this is not provided, the server will extract the description from the artifact content. |  |
**x_registry_name_encoded** | Option<**String**> | Specifies the artifact name of this new version of the artifact content. Value of this must be Base64 encoded string. If this is not provided, the server will extract the name from the artifact content. |  |

### Return type

[**crate::models::VersionMetaData**](VersionMetaData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/create.extended+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artifact_version

> serde_json::Value get_artifact_version(group_id, artifact_id, version, dereference)
Get artifact version

Retrieves a single version of the artifact content.  Both the `artifactId` and the unique `version` number must be provided.  The `Content-Type` of the response depends  on the artifact type.  In most cases, this is `application/json`, but for some types  it may be different (for example, `PROTOBUF`).  This operation can fail for the following reasons:  * No artifact with this `artifactId` exists (HTTP error `404`) * No version with this `version` exists (HTTP error `404`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The artifact group ID.  Must be a string provided by the client, representing the name of the grouping of artifacts. | [required] |
**artifact_id** | **String** | The artifact ID.  Can be a string (client-provided) or UUID (server-generated), representing the unique artifact identifier. | [required] |
**version** | **String** | The unique identifier of a specific version of the artifact content. | [required] |
**dereference** | Option<**bool**> | Allows the user to specify if the content should be dereferenced when being returned |  |

### Return type

[**serde_json::Value**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artifact_version_references

> Vec<crate::models::ArtifactReference> get_artifact_version_references(group_id, artifact_id, version)
Get artifact version

Retrieves a single version of the artifact content.  Both the `artifactId` and the unique `version` number must be provided.  The `Content-Type` of the response depends  on the artifact type.  In most cases, this is `application/json`, but for some types  it may be different (for example, `PROTOBUF`).  This operation can fail for the following reasons:  * No artifact with this `artifactId` exists (HTTP error `404`) * No version with this `version` exists (HTTP error `404`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The artifact group ID.  Must be a string provided by the client, representing the name of the grouping of artifacts. | [required] |
**artifact_id** | **String** | The artifact ID.  Can be a string (client-provided) or UUID (server-generated), representing the unique artifact identifier. | [required] |
**version** | **String** | The unique identifier of a specific version of the artifact content. | [required] |

### Return type

[**Vec<crate::models::ArtifactReference>**](ArtifactReference.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_artifact_versions

> crate::models::VersionSearchResults list_artifact_versions(group_id, artifact_id, offset, limit)
List artifact versions

Returns a list of all versions of the artifact.  The result set is paged.  This operation can fail for the following reasons:  * No artifact with this `artifactId` exists (HTTP error `404`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The artifact group ID.  Must be a string provided by the client, representing the name of the grouping of artifacts. | [required] |
**artifact_id** | **String** | The artifact ID.  Can be a string (client-provided) or UUID (server-generated), representing the unique artifact identifier. | [required] |
**offset** | Option<**i32**> | The number of versions to skip before starting to collect the result set.  Defaults to 0. |  |
**limit** | Option<**i32**> | The number of versions to return.  Defaults to 20. |  |

### Return type

[**crate::models::VersionSearchResults**](VersionSearchResults.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_artifact_version_state

> update_artifact_version_state(group_id, artifact_id, version, update_state)
Update artifact version state

Updates the state of a specific version of an artifact.  For example, you can use  this operation to disable a specific version.  This operation can fail for the following reasons:  * No artifact with this `artifactId` exists (HTTP error `404`) * No version with this `version` exists (HTTP error `404`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The artifact group ID.  Must be a string provided by the client, representing the name of the grouping of artifacts. | [required] |
**artifact_id** | **String** | The artifact ID.  Can be a string (client-provided) or UUID (server-generated), representing the unique artifact identifier. | [required] |
**version** | **String** | The unique identifier of a specific version of the artifact content. | [required] |
**update_state** | [**UpdateState**](UpdateState.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

