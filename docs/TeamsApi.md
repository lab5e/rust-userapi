# \TeamsApi

All URIs are relative to *https://api.lab5e.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_invite**](TeamsApi.md#accept_invite) | **post** /user/teams/accept | Accept invite
[**create_team**](TeamsApi.md#create_team) | **post** /user/teams | Create team
[**delete_invite**](TeamsApi.md#delete_invite) | **delete** /user/teams/{teamId}/invites/{code} | Delete invite
[**delete_member**](TeamsApi.md#delete_member) | **delete** /user/teams/{teamId}/members/{userId} | Remove member
[**delete_team**](TeamsApi.md#delete_team) | **delete** /user/teams/{teamId} | Remove team
[**generate_invite**](TeamsApi.md#generate_invite) | **post** /user/teams/{teamId}/invites | Generate invite
[**list_invites**](TeamsApi.md#list_invites) | **get** /user/teams/{teamId}/invites | List invites
[**list_teams**](TeamsApi.md#list_teams) | **get** /user/teams | List teams
[**retrieve_invite**](TeamsApi.md#retrieve_invite) | **get** /user/teams/{teamId}/invites/{code} | Retrieve invite
[**retrieve_member**](TeamsApi.md#retrieve_member) | **get** /user/teams/{teamId}/members/{userId} | Retrieve member
[**retrieve_team**](TeamsApi.md#retrieve_team) | **get** /user/teams/{teamId} | Retrieve team
[**retrieve_team_members**](TeamsApi.md#retrieve_team_members) | **get** /user/teams/{teamId}/members | List members
[**update_member**](TeamsApi.md#update_member) | **patch** /user/teams/{teamId}/members/{userId} | Update member
[**update_team**](TeamsApi.md#update_team) | **patch** /user/teams/{teamId} | Update team



## accept_invite

> crate::models::Team accept_invite(body)
Accept invite

Accept an invite from another user. This will add the currently logged in user to the team as a regular memeber. When the invite is accepted it is removed from the team's invites and cannot be reused.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AcceptInviteRequest**](AcceptInviteRequest.md) |  | [required] |

### Return type

[**crate::models::Team**](Team.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_team

> crate::models::Team create_team(body)
Create team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Team**](Team.md) |  | [required] |

### Return type

[**crate::models::Team**](Team.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_invite

> crate::models::DeleteInviteResponse delete_invite(team_id, code)
Delete invite

Delete an invite created earlier. You must be an administrator of the team to perform this action

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | The team ID | [required] |
**code** | **String** | The invite code. | [required] |

### Return type

[**crate::models::DeleteInviteResponse**](DeleteInviteResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_member

> crate::models::Member delete_member(team_id, user_id)
Remove member

Remove a member from the team. You must be an administrator to do this. You can't remove yourself from the team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | The team ID | [required] |
**user_id** | **String** | The user ID | [required] |

### Return type

[**crate::models::Member**](Member.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_team

> crate::models::Team delete_team(team_id)
Remove team

Update the team. You must be an administrator of the team to edit it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | The team ID | [required] |

### Return type

[**crate::models::Team**](Team.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_invite

> crate::models::Invite generate_invite(team_id, body)
Generate invite

Update the team. You must be an administrator of the team to edit it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | The team ID | [required] |
**body** | [**InviteRequest**](InviteRequest.md) |  | [required] |

### Return type

[**crate::models::Invite**](Invite.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_invites

> crate::models::InviteList list_invites(team_id)
List invites

Update the team. You must be an administrator of the team to edit it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | The team ID | [required] |

### Return type

[**crate::models::InviteList**](InviteList.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_teams

> crate::models::TeamList list_teams()
List teams

Update the team. You must be an administrator of the team to edit it.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TeamList**](TeamList.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_invite

> crate::models::Invite retrieve_invite(team_id, code)
Retrieve invite

Read a single invite from the team's set of non-redeemed invites.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | The team ID | [required] |
**code** | **String** | The invite code. | [required] |

### Return type

[**crate::models::Invite**](Invite.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_member

> crate::models::Member retrieve_member(team_id, user_id)
Retrieve member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | The team ID | [required] |
**user_id** | **String** | The user ID | [required] |

### Return type

[**crate::models::Member**](Member.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_team

> crate::models::Team retrieve_team(team_id)
Retrieve team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | The team ID | [required] |

### Return type

[**crate::models::Team**](Team.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_team_members

> crate::models::MemberList retrieve_team_members(team_id)
List members

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | The team ID | [required] |

### Return type

[**crate::models::MemberList**](MemberList.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_member

> crate::models::Member update_member(team_id, user_id, body)
Update member

You must be an administrator of the team to update member settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |
**body** | [**Member**](Member.md) |  | [required] |

### Return type

[**crate::models::Member**](Member.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_team

> crate::models::Team update_team(team_id, body)
Update team

Update the team. You must be an administrator of the team to edit it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |
**body** | [**Team**](Team.md) |  | [required] |

### Return type

[**crate::models::Team**](Team.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

