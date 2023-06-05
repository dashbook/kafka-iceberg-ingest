# \VersionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_schema_version**](VersionsApi.md#delete_schema_version) | **DELETE** /schemagroups/{group-id}/schemas/{schema-id}/versions/{version-number} | Delete specified version of schema
[**get_schema_version**](VersionsApi.md#get_schema_version) | **GET** /schemagroups/{group-id}/schemas/{schema-id}/versions/{version-number} | Get specified version of schema
[**get_schema_versions**](VersionsApi.md#get_schema_versions) | **GET** /schemagroups/{group-id}/schemas/{schema-id}/versions | Get list of versions



## delete_schema_version

> delete_schema_version(group_id, schema_id, version_number)
Delete specified version of schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | schema group | [required] |
**schema_id** | **String** | schema id | [required] |
**version_number** | **i32** | version number | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schema_version

> std::path::PathBuf get_schema_version(group_id, schema_id, version_number)
Get specified version of schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | schema group | [required] |
**schema_id** | **String** | schema id | [required] |
**version_number** | **i32** | version number | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;format=avro

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schema_versions

> Vec<i32> get_schema_versions(group_id, schema_id)
Get list of versions

Get list of versions for specified schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | schema group | [required] |
**schema_id** | **String** | schema id | [required] |

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;format=avro

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

