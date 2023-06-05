# \SearchApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_artifacts**](SearchApi.md#search_artifacts) | **GET** /search/artifacts | Search for artifacts
[**search_artifacts_by_content**](SearchApi.md#search_artifacts_by_content) | **POST** /search/artifacts | Search for artifacts by content



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

