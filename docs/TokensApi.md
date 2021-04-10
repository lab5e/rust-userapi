# \TokensApi

All URIs are relative to *https://api.lab5e.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_create_token**](TokensApi.md#user_create_token) | **post** /user/tokens | Create token
[**user_delete_token**](TokensApi.md#user_delete_token) | **delete** /user/tokens/{token} | Remove token
[**user_list_tokens**](TokensApi.md#user_list_tokens) | **get** /user/tokens | List tokens
[**user_retrieve_token**](TokensApi.md#user_retrieve_token) | **get** /user/tokens/{token} | Retrieve token
[**user_update_token**](TokensApi.md#user_update_token) | **patch** /user/tokens/{token} | Update token



## user_create_token

> crate::models::Token user_create_token(body)
Create token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Token**](Token.md) |  | [required] |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_delete_token

> serde_json::Value user_delete_token(token)
Remove token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | The token to delete. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_list_tokens

> crate::models::TokenList user_list_tokens()
List tokens

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TokenList**](TokenList.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_retrieve_token

> crate::models::Token user_retrieve_token(token)
Retrieve token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_update_token

> crate::models::Token user_update_token(token, body)
Update token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | The token  Use this in the `X-API-Token` header when using the API. | [required] |
**body** | [**Token**](Token.md) |  | [required] |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

