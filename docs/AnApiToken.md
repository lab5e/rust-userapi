# AnApiToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**resource** | Option<**String**> | The resource of the token.  The token applies to the specified resource and any resources below this so the resource `/` applies to the root resource and any resource below the root resource. In the same manner `/collections` will apply to all collectins while `/collections/{id}` will apply to that particular collection. | [optional]
**write** | Option<**bool**> | Write flag for token.  If this is set to `true` the token can be used for write operations in the API such as POST, DELETE and PATCH. | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | Tags for the token. | [optional]
**last_use** | Option<**String**> | The last time the token was used. Time in ms since epoch. | [optional]
**uses** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


