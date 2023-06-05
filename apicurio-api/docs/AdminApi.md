# \AdminApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_global_rule**](AdminApi.md#create_global_rule) | **POST** /admin/rules | Create global rule
[**create_role_mapping**](AdminApi.md#create_role_mapping) | **POST** /admin/roleMappings | Create a new role mapping
[**delete_all_global_rules**](AdminApi.md#delete_all_global_rules) | **DELETE** /admin/rules | Delete all global rules
[**delete_global_rule**](AdminApi.md#delete_global_rule) | **DELETE** /admin/rules/{rule} | Delete global rule
[**delete_role_mapping**](AdminApi.md#delete_role_mapping) | **DELETE** /admin/roleMappings/{principalId} | Delete a role mapping
[**export_data**](AdminApi.md#export_data) | **GET** /admin/export | Export registry data
[**get_config_property**](AdminApi.md#get_config_property) | **GET** /admin/config/properties/{propertyName} | Get the value of a configuration property
[**get_global_rule_config**](AdminApi.md#get_global_rule_config) | **GET** /admin/rules/{rule} | Get global rule configuration
[**get_log_configuration**](AdminApi.md#get_log_configuration) | **GET** /admin/loggers/{logger} | Get a single logger configuration
[**get_role_mapping**](AdminApi.md#get_role_mapping) | **GET** /admin/roleMappings/{principalId} | Return a single role mapping
[**import_data**](AdminApi.md#import_data) | **POST** /admin/import | Import registry data
[**list_config_properties**](AdminApi.md#list_config_properties) | **GET** /admin/config/properties | List all configuration properties
[**list_global_rules**](AdminApi.md#list_global_rules) | **GET** /admin/rules | List global rules
[**list_log_configurations**](AdminApi.md#list_log_configurations) | **GET** /admin/loggers | List logging configurations
[**list_role_mappings**](AdminApi.md#list_role_mappings) | **GET** /admin/roleMappings | List all role mappings
[**remove_log_configuration**](AdminApi.md#remove_log_configuration) | **DELETE** /admin/loggers/{logger} | Removes logger configuration
[**reset_config_property**](AdminApi.md#reset_config_property) | **DELETE** /admin/config/properties/{propertyName} | Reset a configuration property
[**set_log_configuration**](AdminApi.md#set_log_configuration) | **PUT** /admin/loggers/{logger} | Set a logger's configuration
[**update_config_property**](AdminApi.md#update_config_property) | **PUT** /admin/config/properties/{propertyName} | Update a configuration property
[**update_global_rule_config**](AdminApi.md#update_global_rule_config) | **PUT** /admin/rules/{rule} | Update global rule configuration
[**update_role_mapping**](AdminApi.md#update_role_mapping) | **PUT** /admin/roleMappings/{principalId} | Update a role mapping



## create_global_rule

> create_global_rule(rule)
Create global rule

Adds a rule to the list of globally configured rules.  This operation can fail for the following reasons:  * The rule type is unknown (HTTP error `400`) * The rule already exists (HTTP error `409`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule** | [**Rule**](Rule.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_role_mapping

> create_role_mapping(role_mapping)
Create a new role mapping

Creates a new mapping between a user/principal and a role.  This operation can fail for the following reasons:  * A server error occurred (HTTP error `500`)  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_mapping** | [**RoleMapping**](RoleMapping.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_global_rules

> delete_all_global_rules()
Delete all global rules

Deletes all globally configured rules.  This operation can fail for the following reasons:  * A server error occurred (HTTP error `500`) 

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_global_rule

> delete_global_rule(rule)
Delete global rule

Deletes a single global rule.  If this is the only rule configured, this is the same as deleting **all** rules.  This operation can fail for the following reasons:  * Invalid rule name/type (HTTP error `400`) * No rule with name/type `rule` exists (HTTP error `404`) * Rule cannot be deleted (HTTP error `409`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule** | [**RuleType**](.md) | The unique name/type of a rule. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_role_mapping

> delete_role_mapping(principal_id)
Delete a role mapping

Deletes a single role mapping, effectively denying access to a user/principal.  This operation can fail for the following reasons:  * No role mapping for the principalId exists (HTTP error `404`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**principal_id** | **String** | Unique id of a principal (typically either a user or service account). | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_data

> serde_json::Value export_data(for_browser)
Export registry data

Exports registry data as a ZIP archive.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**for_browser** | Option<**bool**> | Indicates if the operation is done for a browser.  If true, the response will be a JSON payload with a property called `href`.  This `href` will be a single-use, naked download link suitable for use by a web browser to download the content. |  |

### Return type

[**serde_json::Value**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/zip, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_config_property

> crate::models::ConfigurationProperty get_config_property(property_name)
Get the value of a configuration property

Returns the value of a single configuration property.  This operation may fail for one of the following reasons:  * Property not found or not configured (HTTP error `404`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**property_name** | **String** | The name of a configuration property. | [required] |

### Return type

[**crate::models::ConfigurationProperty**](ConfigurationProperty.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_global_rule_config

> crate::models::Rule get_global_rule_config(rule)
Get global rule configuration

Returns information about the named globally configured rule.  This operation can fail for the following reasons:  * Invalid rule name/type (HTTP error `400`) * No rule with name/type `rule` exists (HTTP error `404`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule** | [**RuleType**](.md) | The unique name/type of a rule. | [required] |

### Return type

[**crate::models::Rule**](Rule.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_log_configuration

> crate::models::NamedLogConfiguration get_log_configuration(logger)
Get a single logger configuration

Returns the configured logger configuration for the provided logger name, if no logger configuration is persisted it will return the current default log configuration in the system.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**logger** | **String** | The name of a single logger. | [required] |

### Return type

[**crate::models::NamedLogConfiguration**](NamedLogConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_mapping

> crate::models::RoleMapping get_role_mapping(principal_id)
Return a single role mapping

Gets the details of a single role mapping (by principalId).  This operation can fail for the following reasons:  * No role mapping for the principalId exists (HTTP error `404`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**principal_id** | **String** | Unique id of a principal (typically either a user or service account). | [required] |

### Return type

[**crate::models::RoleMapping**](RoleMapping.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_data

> import_data(body, x_registry_preserve_global_id, x_registry_preserve_content_id)
Import registry data

Imports registry data that was previously exported using the `/admin/export` operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** | The ZIP file representing the previously exported registry data. | [required] |
**x_registry_preserve_global_id** | Option<**bool**> | If this header is set to false, global ids of imported data will be ignored and replaced by next id in global id sequence. This allows to import any data even thought the global ids would cause a conflict. |  |
**x_registry_preserve_content_id** | Option<**bool**> | If this header is set to false, content ids of imported data will be ignored and replaced by next id in content id sequence. The mapping between content and artifacts will be preserved. This allows to import any data even thought the content ids would cause a conflict. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/zip
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_config_properties

> Vec<crate::models::ConfigurationProperty> list_config_properties()
List all configuration properties

Returns a list of all configuration properties that have been set.  The list is not paged.  This operation may fail for one of the following reasons:  * A server error occurred (HTTP error `500`) 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ConfigurationProperty>**](ConfigurationProperty.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_global_rules

> Vec<crate::models::RuleType> list_global_rules()
List global rules

Gets a list of all the currently configured global rules (if any).  This operation can fail for the following reasons:  * A server error occurred (HTTP error `500`) 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::RuleType>**](RuleType.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_log_configurations

> Vec<crate::models::NamedLogConfiguration> list_log_configurations()
List logging configurations

List all of the configured logging levels.  These override the default logging configuration.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::NamedLogConfiguration>**](NamedLogConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_role_mappings

> Vec<crate::models::RoleMapping> list_role_mappings()
List all role mappings

Gets a list of all role mappings configured in the registry (if any).  This operation can fail for the following reasons:  * A server error occurred (HTTP error `500`) 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::RoleMapping>**](RoleMapping.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_log_configuration

> crate::models::NamedLogConfiguration remove_log_configuration(logger)
Removes logger configuration

Removes the configured logger configuration (if any) for the given logger.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**logger** | **String** | The name of a single logger. | [required] |

### Return type

[**crate::models::NamedLogConfiguration**](NamedLogConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_config_property

> reset_config_property(property_name)
Reset a configuration property

Resets the value of a single configuration property.  This will return the property to its default value (see external documentation for supported properties and their default values).  This operation may fail for one of the following reasons:  * Property not found or not configured (HTTP error `404`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**property_name** | **String** | The name of a configuration property. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_log_configuration

> crate::models::NamedLogConfiguration set_log_configuration(logger, log_configuration)
Set a logger's configuration

Configures the logger referenced by the provided logger name with the given configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**logger** | **String** | The name of a single logger. | [required] |
**log_configuration** | [**LogConfiguration**](LogConfiguration.md) | The new logger configuration. | [required] |

### Return type

[**crate::models::NamedLogConfiguration**](NamedLogConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_config_property

> update_config_property(property_name, update_configuration_property)
Update a configuration property

Updates the value of a single configuration property.  This operation may fail for one of the following reasons:  * Property not found or not configured (HTTP error `404`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**property_name** | **String** | The name of a configuration property. | [required] |
**update_configuration_property** | [**UpdateConfigurationProperty**](UpdateConfigurationProperty.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_global_rule_config

> crate::models::Rule update_global_rule_config(rule, rule2)
Update global rule configuration

Updates the configuration for a globally configured rule.  This operation can fail for the following reasons:  * Invalid rule name/type (HTTP error `400`) * No rule with name/type `rule` exists (HTTP error `404`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule** | [**RuleType**](.md) | The unique name/type of a rule. | [required] |
**rule2** | [**Rule**](Rule.md) |  | [required] |

### Return type

[**crate::models::Rule**](Rule.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_role_mapping

> update_role_mapping(principal_id, update_role)
Update a role mapping

Updates a single role mapping for one user/principal.  This operation can fail for the following reasons:  * No role mapping for the principalId exists (HTTP error `404`) * A server error occurred (HTTP error `500`) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**principal_id** | **String** | Unique id of a principal (typically either a user or service account). | [required] |
**update_role** | [**UpdateRole**](UpdateRole.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

