# \GlobalRulesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_global_rule**](GlobalRulesApi.md#create_global_rule) | **POST** /admin/rules | Create global rule
[**delete_all_global_rules**](GlobalRulesApi.md#delete_all_global_rules) | **DELETE** /admin/rules | Delete all global rules
[**delete_global_rule**](GlobalRulesApi.md#delete_global_rule) | **DELETE** /admin/rules/{rule} | Delete global rule
[**get_global_rule_config**](GlobalRulesApi.md#get_global_rule_config) | **GET** /admin/rules/{rule} | Get global rule configuration
[**list_global_rules**](GlobalRulesApi.md#list_global_rules) | **GET** /admin/rules | List global rules
[**update_global_rule_config**](GlobalRulesApi.md#update_global_rule_config) | **PUT** /admin/rules/{rule} | Update global rule configuration



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

