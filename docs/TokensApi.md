# \TokensApi

All URIs are relative to *https://api.lab5e.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_token**](TokensApi.md#create_token) | **post** /user/tokens | Create token
[**delete_token**](TokensApi.md#delete_token) | **delete** /user/tokens/{token} | Remove token
[**list_tokens**](TokensApi.md#list_tokens) | **get** /user/tokens | List tokens
[**retrieve_token**](TokensApi.md#retrieve_token) | **get** /user/tokens/{token} | Retrieve token
[**update_token**](TokensApi.md#update_token) | **patch** /user/tokens/{token} | Update token



## create_token

> crate::models::Token create_token(body)
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


## delete_token

> crate::models::DeleteTokenResponse delete_token(token)
Remove token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | The token to delete. | [required] |

### Return type

[**crate::models::DeleteTokenResponse**](DeleteTokenResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tokens

> crate::models::TokenList list_tokens()
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


## retrieve_token

> crate::models::Token retrieve_token(token)
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


## update_token

> crate::models::Token update_token(token, body)
Update token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | The token  Use this in the `X-API-Token` header when using the API. | [required] |
**body** | [**AnApiToken**](AnApiToken.md) |  | [required] |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

