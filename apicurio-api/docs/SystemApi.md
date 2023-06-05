# \SystemApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_resource_limits**](SystemApi.md#get_resource_limits) | **GET** /system/limits | Get resource limits information
[**get_system_info**](SystemApi.md#get_system_info) | **GET** /system/info | Get system information



## get_resource_limits

> crate::models::Limits get_resource_limits()
Get resource limits information

This operation retrieves the list of limitations on used resources, that are applied on the current instance of Registry.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Limits**](Limits.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_info

> crate::models::SystemInfo get_system_info()
Get system information

This operation retrieves information about the running registry system, such as the version of the software and when it was built.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SystemInfo**](SystemInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

