# \ArtifactsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_artifact**](ArtifactsApi.md#create_artifact) | **POST** /groups/{groupId}/artifacts | Create artifact
[**delete_artifact**](ArtifactsApi.md#delete_artifact) | **DELETE** /groups/{groupId}/artifacts/{artifactId} | Delete artifact
[**delete_artifacts_in_group**](ArtifactsApi.md#delete_artifacts_in_group) | **DELETE** /groups/{groupId}/artifacts | Deletes all artifacts in a group
[**get_content_by_global_id**](ArtifactsApi.md#get_content_by_global_id) | **GET** /ids/globalIds/{globalId} | Get artifact by global ID
[**get_content_by_hash**](ArtifactsApi.md#get_content_by_hash) | **GET** /ids/contentHashes/{contentHash}/ | Get artifact content by SHA-256 hash
[**get_content_by_id**](ArtifactsApi.md#get_content_by_id) | **GET** /ids/contentIds/{contentId}/ | Get artifact content by ID
[**get_latest_artifact**](ArtifactsApi.md#get_latest_artifact) | **GET** /groups/{groupId}/artifacts/{artifactId} | Get latest artifact
[**list_artifacts_in_group**](ArtifactsApi.md#list_artifacts_in_group) | **GET** /groups/{groupId}/artifacts | List artifacts in group
[**references_by_content_hash**](ArtifactsApi.md#references_by_content_hash) | **GET** /ids/contentHashes/{contentHash}/references | Returns a list with all the references for the artifact with the given hash
[**references_by_content_id**](ArtifactsApi.md#references_by_content_id) | **GET** /ids/contentIds/{contentId}/references | Returns a list with all the references for the artifact with the given content id.
[**references_by_global_id**](ArtifactsApi.md#references_by_global_id) | **GET** /ids/globalIds/{globalId}/references | Returns a list with all the references for the artifact with the given global id.
[**search_artifacts**](ArtifactsApi.md#search_artifacts) | **GET** /search/artifacts | Search for artifacts
[**search_artifacts_by_content**](ArtifactsApi.md#search_artifacts_by_content) | **POST** /search/artifacts | Search for artifacts by content
[**update_artifact**](ArtifactsApi.md#update_artifact) | **PUT** /groups/{groupId}/artifacts/{artifactId} | Update artifact
[**update_artifact_state**](ArtifactsApi.md#update_artifact_state) | **PUT** /groups/{groupId}/artifacts/{artifactId}/state | Update artifact state



## create_artifact

> crate::models::ArtifactMetaData create_artifact(group_id, body, x_registry_artifact_type, x_registry_artifact_id, x_registry_version, if_exists, canonical, x_registry_description, x_registry_description_encoded, x_registry_name, x_registry_name_encoded)
Create artifact

Creates a new artifact by posting the artifact content.  The body of the request should be the raw content of the artifact.  This is typically in JSON format for *most* of the  supported types, but may be in another format for a few (for example, `PROTOBUF`).  The registry attempts to figure out what kind of artifact is being added from the following supported list:  * Avro (`AVRO`) * Protobuf (`PROTOBUF`) * JSON Schema (`JSON`) * Kafka Connect (`KCONNECT`) * OpenAPI (`OPENAPI`) * AsyncAPI (`ASYNCAPI`) * GraphQL (`GRAPHQL`) * Web Services Description Language (`WSDL`) * XML Schema (`XSD`)  Alternatively, you can specify the artifact type using the `X-Registry-ArtifactType`  HTTP request header, or include a hint in the request's `Content-Type`.  For example:  ``` Content-Type: application/json; artifactType=AVRO ```  An artifact is created using the content provided in the body of the request.  This content is created under a unique artifact ID that can be provided in the request using the `X-Registry-ArtifactId` request header.  If not provided in the request, the server generates a unique ID for the artifact.  It is typically recommended that callers provide the ID, because this is typically a meaningful identifier,  and for most use cases should be supplied by the caller.  If an artifact with the provided artifact ID already exists, the default behavior is for the server to reject the content with a 409 error.  However, the caller can supply the `ifExists` query parameter to alter this default behavior. The `ifExists` query parameter can have one of the following values:  * `FAIL` (*default*) - server rejects the content with a 409 error * `UPDATE` - server updates the existing artifact and returns the new metadata * `RETURN` - server does not create or add content to the server, but instead  returns the metadata for the existing artifact * `RETURN_OR_UPDATE` - server returns an existing **version** that matches the  provided content if such a version exists, otherwise a new version is created  This operation may fail for one of the following reasons:  * An invalid `ArtifactType` was indicated (HTTP error `400`) * No `ArtifactType` was indicated and the server could not determine one from the content (HTTP error `400`) * Provided content (request body) was empty (HTTP error `400`) * An artifact with the provided ID already exists (HTTP error `409`) * The content violates one of the configured global rules (HTTP error `409`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Unique ID of an artifact group. | [required] |
**body** | Option<**serde_json::Value**> | The content of the artifact being created. This is often, but not always, JSON data representing one of the supported artifact types:  * Avro (`AVRO`) * Protobuf (`PROTOBUF`) * JSON Schema (`JSON`) * Kafka Connect (`KCONNECT`) * OpenAPI (`OPENAPI`) * AsyncAPI (`ASYNCAPI`) * GraphQL (`GRAPHQL`) * Web Services Description Language (`WSDL`) * XML Schema (`XSD`)  | [required] |
**x_registry_artifact_type** | Option<[**ArtifactType**](.md)> | Specifies the type of the artifact being added. Possible values include:  * Avro (`AVRO`) * Protobuf (`PROTOBUF`) * JSON Schema (`JSON`) * Kafka Connect (`KCONNECT`) * OpenAPI (`OPENAPI`) * AsyncAPI (`ASYNCAPI`) * GraphQL (`GRAPHQL`) * Web Services Description Language (`WSDL`) * XML Schema (`XSD`) |  |
**x_registry_artifact_id** | Option<**String**> | A client-provided, globally unique identifier for the new artifact. |  |
**x_registry_version** | Option<**String**> | Specifies the version number of this initial version of the artifact content.  This would typically be a simple integer or a SemVer value.  If not provided, the server will assign a version number automatically (starting with version `1`). |  |
**if_exists** | Option<[**IfExists**](.md)> | Set this option to instruct the server on what to do if the artifact already exists. |  |
**canonical** | Option<**bool**> | Used only when the `ifExists` query parameter is set to `RETURN_OR_UPDATE`, this parameter can be set to `true` to indicate that the server should \"canonicalize\" the content when searching for a matching version.  The canonicalization algorithm is unique to each artifact type, but typically involves removing extra whitespace and formatting the content in a consistent manner. |  |
**x_registry_description** | Option<**String**> | Specifies the description of artifact being added. Description must be ASCII-only string. If this is not provided, the server will extract the description from the artifact content. |  |
**x_registry_description_encoded** | Option<**String**> | Specifies the description of artifact being added. Value of this must be Base64 encoded string. If this is not provided, the server will extract the description from the artifact content. |  |
**x_registry_name** | Option<**String**> | Specifies the name of artifact being added. Name must be ASCII-only string. If this is not provided, the server will extract the name from the artifact content. |  |
**x_registry_name_encoded** | Option<**String**> | Specifies the name of artifact being added. Value of this must be Base64 encoded string. If this is not provided, the server will extract the name from the artifact content. |  |

### Return type

[**crate::models::ArtifactMetaData**](ArtifactMetaData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/create.extended+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_artifact

> delete_artifact(group_id, artifact_id)
Delete artifact

Deletes an artifact completely, resulting in all versions of the artifact also being deleted.  This may fail for one of the following reasons:  * No artifact with the `artifactId` exists (HTTP error `404`) * A server error occurred (HTTP error `500`)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The artifact group ID.  Must be a string provided by the client, representing the name of the grouping of artifacts. | [required] |
**artifact_id** | **String** | The artifact ID.  Can be a string (client-provided) or UUID (server-generated), representing the unique artifact identifier. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_artifacts_in_group

> delete_artifacts_in_group(group_id)
Deletes all artifacts in a group

Deletes all of the artifacts that exist in a given group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Unique ID of an artifact group. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_content_by_global_id

> serde_json::Value get_content_by_global_id(global_id, dereference)
Get artifact by global ID

Gets the content for an artifact version in the registry using its globally unique identifier.  This operation may fail for one of the following reasons:  * No artifact version with this `globalId` exists (HTTP error `404`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**global_id** | **i64** | Global identifier for an artifact version. | [required] |
**dereference** | Option<**bool**> | Allows the user to specify if the content should be dereferenced when being returned |  |

### Return type

[**serde_json::Value**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_content_by_hash

> serde_json::Value get_content_by_hash(content_hash)
Get artifact content by SHA-256 hash

Gets the content for an artifact version in the registry using the  SHA-256 hash of the content.  This content hash may be shared by multiple artifact versions in the case where the artifact versions have identical content.  This operation may fail for one of the following reasons:  * No content with this `contentHash` exists (HTTP error `404`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_hash** | **String** | SHA-256 content hash for a single artifact content. | [required] |

### Return type

[**serde_json::Value**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_content_by_id

> serde_json::Value get_content_by_id(content_id)
Get artifact content by ID

Gets the content for an artifact version in the registry using the unique content identifier for that content.  This content ID may be shared by multiple artifact versions in the case where the artifact versions are identical.  This operation may fail for one of the following reasons:  * No content with this `contentId` exists (HTTP error `404`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_id** | **i64** | Global identifier for a single artifact content. | [required] |

### Return type

[**serde_json::Value**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_latest_artifact

> serde_json::Value get_latest_artifact(group_id, artifact_id, dereference)
Get latest artifact

Returns the latest version of the artifact in its raw form.  The `Content-Type` of the response depends on the artifact type.  In most cases, this is `application/json`, but  for some types it may be different (for example, `PROTOBUF`).  This operation may fail for one of the following reasons:  * No artifact with this `artifactId` exists (HTTP error `404`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The artifact group ID.  Must be a string provided by the client, representing the name of the grouping of artifacts. | [required] |
**artifact_id** | **String** | The artifact ID.  Can be a string (client-provided) or UUID (server-generated), representing the unique artifact identifier. | [required] |
**dereference** | Option<**bool**> | Allows the user to specify if the content should be dereferenced when being returned |  |

### Return type

[**serde_json::Value**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_artifacts_in_group

> crate::models::ArtifactSearchResults list_artifacts_in_group(group_id, limit, offset, order, orderby)
List artifacts in group

Returns a list of all artifacts in the group.  This list is paged.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Unique ID of an artifact group. | [required] |
**limit** | Option<**i32**> | The number of artifacts to return.  Defaults to 20. |  |
**offset** | Option<**i32**> | The number of artifacts to skip before starting the result set.  Defaults to 0. |  |
**order** | Option<[**SortOrder**](.md)> | Sort order, ascending (`asc`) or descending (`desc`). |  |
**orderby** | Option<[**SortBy**](.md)> | The field to sort by.  Can be one of:  * `name` * `createdOn`  |  |

### Return type

[**crate::models::ArtifactSearchResults**](ArtifactSearchResults.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## references_by_content_hash

> Vec<crate::models::ArtifactReference> references_by_content_hash(content_hash)
Returns a list with all the references for the artifact with the given hash

Returns a list containing all the artifact references using the artifact content hash.  This operation may fail for one of the following reasons:  * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_hash** | **String** | SHA-256 content hash for a single artifact content. | [required] |

### Return type

[**Vec<crate::models::ArtifactReference>**](ArtifactReference.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## references_by_content_id

> Vec<crate::models::ArtifactReference> references_by_content_id(content_id)
Returns a list with all the references for the artifact with the given content id.

Returns a list containing all the artifact references using the artifact contentId.  This operation may fail for one of the following reasons:  * A server error occurred (HTTP error `500`)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_id** | **i64** | Global identifier for a single artifact content. | [required] |

### Return type

[**Vec<crate::models::ArtifactReference>**](ArtifactReference.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## references_by_global_id

> Vec<crate::models::ArtifactReference> references_by_global_id(global_id)
Returns a list with all the references for the artifact with the given global id.

Returns a list containing all the artifact references using the artifact global id.  This operation may fail for one of the following reasons:  * A server error occurred (HTTP error `500`)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**global_id** | **i64** | Global identifier for an artifact version. | [required] |

### Return type

[**Vec<crate::models::ArtifactReference>**](ArtifactReference.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_artifacts

> crate::models::ArtifactSearchResults search_artifacts(name, offset, limit, order, orderby, labels, properties, description, group, global_id, content_id)
Search for artifacts

Returns a paginated list of all artifacts that match the provided filter criteria. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Filter by artifact name. |  |
**offset** | Option<**i32**> | The number of artifacts to skip before starting to collect the result set.  Defaults to 0. |  |[default to 0]
**limit** | Option<**i32**> | The number of artifacts to return.  Defaults to 20. |  |[default to 20]
**order** | Option<[**SortOrder**](.md)> | Sort order, ascending (`asc`) or descending (`desc`). |  |
**orderby** | Option<[**SortBy**](.md)> | The field to sort by.  Can be one of:  * `name` * `createdOn`  |  |
**labels** | Option<[**Vec<String>**](String.md)> | Filter by label.  Include one or more label to only return artifacts containing all of the specified labels. |  |
**properties** | Option<[**Vec<String>**](String.md)> | Filter by one or more name/value property.  Separate each name/value pair using a colon.  For example `properties=foo:bar` will return only artifacts with a custom property named `foo` and value `bar`. |  |
**description** | Option<**String**> | Filter by description. |  |
**group** | Option<**String**> | Filter by artifact group. |  |
**global_id** | Option<**i64**> | Filter by globalId. |  |
**content_id** | Option<**i64**> | Filter by contentId. |  |

### Return type

[**crate::models::ArtifactSearchResults**](ArtifactSearchResults.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_artifacts_by_content

> crate::models::ArtifactSearchResults search_artifacts_by_content(body, canonical, artifact_type, offset, limit, order, orderby)
Search for artifacts by content

Returns a paginated list of all artifacts with at least one version that matches the posted content. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** | The content to search for. | [required] |
**canonical** | Option<**bool**> | Parameter that can be set to `true` to indicate that the server should \"canonicalize\" the content when searching for matching artifacts.  Canonicalization is unique to each artifact type, but typically involves removing any extra whitespace and formatting the content in a consistent manner.  Must be used along with the `artifactType` query parameter. |  |
**artifact_type** | Option<[**ArtifactType**](.md)> | Indicates the type of artifact represented by the content being used for the search.  This is only needed when using the `canonical` query parameter, so that the server knows how to canonicalize the content prior to searching for matching artifacts. |  |
**offset** | Option<**i32**> | The number of artifacts to skip before starting to collect the result set.  Defaults to 0. |  |[default to 0]
**limit** | Option<**i32**> | The number of artifacts to return.  Defaults to 20. |  |[default to 20]
**order** | Option<**String**> | Sort order, ascending (`asc`) or descending (`desc`). |  |
**orderby** | Option<**String**> | The field to sort by.  Can be one of:  * `name` * `createdOn`  |  |

### Return type

[**crate::models::ArtifactSearchResults**](ArtifactSearchResults.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_artifact

> crate::models::ArtifactMetaData update_artifact(group_id, artifact_id, body, x_registry_version, x_registry_name, x_registry_name_encoded, x_registry_description, x_registry_description_encoded)
Update artifact

Updates an artifact by uploading new content.  The body of the request can be the raw content of the artifact or a JSON object containing both the raw content and a set of references to other artifacts..  This is typically in JSON format for *most* of the supported types, but may be in another format for a few (for example, `PROTOBUF`). The type of the content should be compatible with the artifact's type (it would be an error to update an `AVRO` artifact with new `OPENAPI` content, for example).  The update could fail for a number of reasons including:  * Provided content (request body) was empty (HTTP error `400`) * No artifact with the `artifactId` exists (HTTP error `404`) * The new content violates one of the rules configured for the artifact (HTTP error `409`) * A server error occurred (HTTP error `500`)  When successful, this creates a new version of the artifact, making it the most recent (and therefore official) version of the artifact.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The artifact group ID.  Must be a string provided by the client, representing the name of the grouping of artifacts. | [required] |
**artifact_id** | **String** | The artifact ID.  Can be a string (client-provided) or UUID (server-generated), representing the unique artifact identifier. | [required] |
**body** | Option<**serde_json::Value**> | The new content of the artifact being updated. This is often, but not always, JSON data representing one of the supported artifact types:  * Avro (`AVRO`) * Protobuf (`PROTOBUF`) * JSON Schema (`JSON`) * Kafka Connect (`KCONNECT`) * OpenAPI (`OPENAPI`) * AsyncAPI (`ASYNCAPI`) * GraphQL (`GRAPHQL`) * Web Services Description Language (`WSDL`) * XML Schema (`XSD`)  | [required] |
**x_registry_version** | Option<**String**> | Specifies the version number of this new version of the artifact content.  This would typically be a simple integer or a SemVer value.  If not provided, the server will assign a version number automatically. |  |
**x_registry_name** | Option<**String**> | Specifies the artifact name of this new version of the artifact content. Name must be ASCII-only string. If this is not provided, the server will extract the name from the artifact content. |  |
**x_registry_name_encoded** | Option<**String**> | Specifies the artifact name of this new version of the artifact content. Value of this must be Base64 encoded string. If this is not provided, the server will extract the name from the artifact content. |  |
**x_registry_description** | Option<**String**> | Specifies the artifact description of this new version of the artifact content. Description must be ASCII-only string. If this is not provided, the server will extract the description from the artifact content. |  |
**x_registry_description_encoded** | Option<**String**> | Specifies the artifact description of this new version of the artifact content. Value of this must be Base64 encoded string. If this is not provided, the server will extract the description from the artifact content. |  |

### Return type

[**crate::models::ArtifactMetaData**](ArtifactMetaData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/create.extended+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_artifact_state

> update_artifact_state(group_id, artifact_id, update_state)
Update artifact state

Updates the state of the artifact.  For example, you can use this to mark the latest version of an artifact as `DEPRECATED`.  The operation changes the state of the latest  version of the artifact.  If multiple versions exist, only the most recent is changed.  This operation can fail for the following reasons:  * No artifact with this `artifactId` exists (HTTP error `404`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The artifact group ID.  Must be a string provided by the client, representing the name of the grouping of artifacts. | [required] |
**artifact_id** | **String** | The artifact ID.  Can be a string (client-provided) or UUID (server-generated), representing the unique artifact identifier. | [required] |
**update_state** | [**UpdateState**](UpdateState.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

