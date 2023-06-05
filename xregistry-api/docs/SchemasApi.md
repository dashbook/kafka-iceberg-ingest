# \SchemasApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_schema**](SchemasApi.md#delete_schema) | **DELETE** /schemagroups/{group-id}/schemas/{schema-id} | Delete schema
[**get_latest_schema**](SchemasApi.md#get_latest_schema) | **GET** /schemagroups/{group-id}/schemas/{schema-id} | Get latest version of schema



## delete_schema

> delete_schema(group_id, schema_id)
Delete schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | schema group | [required] |
**schema_id** | **String** | schema id | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_latest_schema

> std::path::PathBuf get_latest_schema(group_id, schema_id)
Get latest version of schema

Get latest version of schema.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | schema group | [required] |
**schema_id** | **String** | schema id | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;format=avro

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

