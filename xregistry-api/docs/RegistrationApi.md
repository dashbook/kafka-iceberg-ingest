# \RegistrationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_schema**](RegistrationApi.md#create_schema) | **POST** /schemagroups/{group-id}/schemas/{schema-id} | Register schema



## create_schema

> crate::models::SchemaId create_schema(group_id, schema_id, body)
Register schema

Register schema. If schema of specified name does not exist in specified group, schema is created at version 1. If schema of specified name exists already in specified group, schema is created at latest version + 1. If schema with identical content already exists, existing schema's ID is returned.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | schema group | [required] |
**schema_id** | **String** | schema id | [required] |
**body** | **std::path::PathBuf** | schema content | [required] |

### Return type

[**crate::models::SchemaId**](SchemaId.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json;format=avro
- **Accept**: application/json;format=avro, application/json;format=protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

