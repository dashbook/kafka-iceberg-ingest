# \GroupsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_group**](GroupsApi.md#create_group) | **PUT** /schemagroups/{group-id} | Create schema group
[**delete_group**](GroupsApi.md#delete_group) | **DELETE** /schemagroups/{group-id} | Delete schema group
[**delete_schemas_by_group**](GroupsApi.md#delete_schemas_by_group) | **DELETE** /schemagroups/{group-id}/schemas | Deletes all schemas in group
[**get_group**](GroupsApi.md#get_group) | **GET** /schemagroups/{group-id} | Get schema group
[**get_groups**](GroupsApi.md#get_groups) | **GET** /schemagroups | Get list of schema groups
[**get_schemas_by_group**](GroupsApi.md#get_schemas_by_group) | **GET** /schemagroups/{group-id}/schemas | List schemas for group id



## create_group

> create_group(group_id, schema_group)
Create schema group

Create schema group with specified format in registry namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | schema group | [required] |
**schema_group** | [**SchemaGroup**](SchemaGroup.md) | schema group description | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group

> delete_group(group_id)
Delete schema group

Delete schema group in schema registry namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | schema group | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_schemas_by_group

> delete_schemas_by_group(group_id)
Deletes all schemas in group

Deletes all schemas under specified group id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | schema group | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group

> crate::models::SchemaGroup get_group(group_id)
Get schema group

Get schema group description in registry namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | schema group | [required] |

### Return type

[**crate::models::SchemaGroup**](SchemaGroup.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_groups

> Vec<String> get_groups()
Get list of schema groups

Get all schema groups in namespace.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schemas_by_group

> Vec<String> get_schemas_by_group(group_id)
List schemas for group id

Returns schema by group id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | schema group | [required] |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

