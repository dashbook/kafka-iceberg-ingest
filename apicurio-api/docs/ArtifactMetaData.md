# ArtifactMetaData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**created_by** | **String** |  | 
**created_on** | **String** |  | 
**modified_by** | **String** |  | 
**modified_on** | **String** |  | 
**id** | **String** | The ID of a single artifact. | 
**version** | **String** |  | 
**r#type** | [**crate::models::ArtifactType**](ArtifactType.md) |  | 
**global_id** | **i64** |  | 
**state** | [**crate::models::ArtifactState**](ArtifactState.md) |  | 
**labels** | Option<**Vec<String>**> |  | [optional]
**properties** | Option<**::std::collections::HashMap<String, String>**> | User-defined name-value pairs. Name and value must be strings. | [optional]
**group_id** | Option<**String**> | An ID of a single artifact group. | [optional]
**content_id** | **i64** |  | 
**references** | Option<[**Vec<crate::models::ArtifactReference>**](ArtifactReference.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


