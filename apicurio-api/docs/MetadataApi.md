# \MetadataApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_artifact_version_meta_data**](MetadataApi.md#delete_artifact_version_meta_data) | **DELETE** /groups/{groupId}/artifacts/{artifactId}/versions/{version}/meta | Delete artifact version metadata
[**get_artifact_meta_data**](MetadataApi.md#get_artifact_meta_data) | **GET** /groups/{groupId}/artifacts/{artifactId}/meta | Get artifact metadata
[**get_artifact_version_meta_data**](MetadataApi.md#get_artifact_version_meta_data) | **GET** /groups/{groupId}/artifacts/{artifactId}/versions/{version}/meta | Get artifact version metadata
[**get_artifact_version_meta_data_by_content**](MetadataApi.md#get_artifact_version_meta_data_by_content) | **POST** /groups/{groupId}/artifacts/{artifactId}/meta | Get artifact version metadata by content
[**update_artifact_meta_data**](MetadataApi.md#update_artifact_meta_data) | **PUT** /groups/{groupId}/artifacts/{artifactId}/meta | Update artifact metadata
[**update_artifact_version_meta_data**](MetadataApi.md#update_artifact_version_meta_data) | **PUT** /groups/{groupId}/artifacts/{artifactId}/versions/{version}/meta | Update artifact version metadata



## delete_artifact_version_meta_data

> delete_artifact_version_meta_data(group_id, artifact_id, version)
Delete artifact version metadata

Deletes the user-editable metadata properties of the artifact version.  Any properties that are not user-editable are preserved.  This operation can fail for the following reasons:  * No artifact with this `artifactId` exists (HTTP error `404`) * No version with this `version` exists (HTTP error `404`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The artifact group ID.  Must be a string provided by the client, representing the name of the grouping of artifacts. | [required] |
**artifact_id** | **String** | The artifact ID.  Can be a string (client-provided) or UUID (server-generated), representing the unique artifact identifier. | [required] |
**version** | **String** | The unique identifier of a specific version of the artifact content. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artifact_meta_data

> crate::models::ArtifactMetaData get_artifact_meta_data(group_id, artifact_id)
Get artifact metadata

Gets the metadata for an artifact in the registry.  The returned metadata includes both generated (read-only) and editable metadata (such as name and description).  This operation can fail for the following reasons:  * No artifact with this `artifactId` exists (HTTP error `404`) * A server error occurred (HTTP error `500`)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The artifact group ID.  Must be a string provided by the client, representing the name of the grouping of artifacts. | [required] |
**artifact_id** | **String** | The artifact ID.  Can be a string (client-provided) or UUID (server-generated), representing the unique artifact identifier. | [required] |

### Return type

[**crate::models::ArtifactMetaData**](ArtifactMetaData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artifact_version_meta_data

> crate::models::VersionMetaData get_artifact_version_meta_data(group_id, artifact_id, version)
Get artifact version metadata

Retrieves the metadata for a single version of the artifact.  The version metadata is  a subset of the artifact metadata and only includes the metadata that is specific to the version (for example, this doesn't include `modifiedOn`).  This operation can fail for the following reasons:  * No artifact with this `artifactId` exists (HTTP error `404`) * No version with this `version` exists (HTTP error `404`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The artifact group ID.  Must be a string provided by the client, representing the name of the grouping of artifacts. | [required] |
**artifact_id** | **String** | The artifact ID.  Can be a string (client-provided) or UUID (server-generated), representing the unique artifact identifier. | [required] |
**version** | **String** | The unique identifier of a specific version of the artifact content. | [required] |

### Return type

[**crate::models::VersionMetaData**](VersionMetaData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artifact_version_meta_data_by_content

> crate::models::VersionMetaData get_artifact_version_meta_data_by_content(group_id, artifact_id, body, canonical)
Get artifact version metadata by content

Gets the metadata for an artifact that matches the raw content.  Searches the registry for a version of the given artifact matching the content provided in the body of the POST.  This operation can fail for the following reasons:  * Provided content (request body) was empty (HTTP error `400`) * No artifact with the `artifactId` exists (HTTP error `404`) * No artifact version matching the provided content exists (HTTP error `404`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The artifact group ID.  Must be a string provided by the client, representing the name of the grouping of artifacts. | [required] |
**artifact_id** | **String** | The artifact ID.  Can be a string (client-provided) or UUID (server-generated), representing the unique artifact identifier. | [required] |
**body** | Option<**serde_json::Value**> | The content of an artifact version. | [required] |
**canonical** | Option<**bool**> | Parameter that can be set to `true` to indicate that the server should \"canonicalize\" the content when searching for a matching version.  Canonicalization is unique to each artifact type, but typically involves removing any extra whitespace and formatting the content in a consistent manner. |  |

### Return type

[**crate::models::VersionMetaData**](VersionMetaData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_artifact_meta_data

> update_artifact_meta_data(group_id, artifact_id, editable_meta_data)
Update artifact metadata

Updates the editable parts of the artifact's metadata.  Not all metadata fields can be updated.  For example, `createdOn` and `createdBy` are both read-only properties.  This operation can fail for the following reasons:  * No artifact with the `artifactId` exists (HTTP error `404`) * A server error occurred (HTTP error `500`)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The artifact group ID.  Must be a string provided by the client, representing the name of the grouping of artifacts. | [required] |
**artifact_id** | **String** | The artifact ID.  Can be a string (client-provided) or UUID (server-generated), representing the unique artifact identifier. | [required] |
**editable_meta_data** | [**EditableMetaData**](EditableMetaData.md) | Updated artifact metadata. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_artifact_version_meta_data

> update_artifact_version_meta_data(group_id, artifact_id, version, editable_meta_data)
Update artifact version metadata

Updates the user-editable portion of the artifact version's metadata.  Only some of  the metadata fields are editable by the user.  For example, `description` is editable,  but `createdOn` is not.  This operation can fail for the following reasons:  * No artifact with this `artifactId` exists (HTTP error `404`) * No version with this `version` exists (HTTP error `404`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The artifact group ID.  Must be a string provided by the client, representing the name of the grouping of artifacts. | [required] |
**artifact_id** | **String** | The artifact ID.  Can be a string (client-provided) or UUID (server-generated), representing the unique artifact identifier. | [required] |
**version** | **String** | The unique identifier of a specific version of the artifact content. | [required] |
**editable_meta_data** | [**EditableMetaData**](EditableMetaData.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

