# Rust API client for userapi

API to manage teams, members and tokens

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.3.6 crooked-daija
- Package version: 1.3.6
- Build package: org.openapitools.codegen.languages.RustClientCodegen
For more information, please visit [https://docs.lab5e.com](https://docs.lab5e.com)

## Installation

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:

```
    openapi = { path = "./generated" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.lab5e.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ProfileApi* | [**get_user_profile**](docs/ProfileApi.md#get_user_profile) | **get** /user/profile | Logged in profile
*TeamsApi* | [**accept_invite**](docs/TeamsApi.md#accept_invite) | **post** /user/teams/accept | Accept invite
*TeamsApi* | [**create_team**](docs/TeamsApi.md#create_team) | **post** /user/teams | Create team
*TeamsApi* | [**delete_invite**](docs/TeamsApi.md#delete_invite) | **delete** /user/teams/{teamId}/invites/{code} | Delete invite
*TeamsApi* | [**delete_member**](docs/TeamsApi.md#delete_member) | **delete** /user/teams/{teamId}/members/{userId} | Remove member
*TeamsApi* | [**delete_team**](docs/TeamsApi.md#delete_team) | **delete** /user/teams/{teamId} | Remove team
*TeamsApi* | [**generate_invite**](docs/TeamsApi.md#generate_invite) | **post** /user/teams/{teamId}/invites | Generate invite
*TeamsApi* | [**list_invites**](docs/TeamsApi.md#list_invites) | **get** /user/teams/{teamId}/invites | List invites
*TeamsApi* | [**list_teams**](docs/TeamsApi.md#list_teams) | **get** /user/teams | List teams
*TeamsApi* | [**retrieve_invite**](docs/TeamsApi.md#retrieve_invite) | **get** /user/teams/{teamId}/invites/{code} | Retrieve invite
*TeamsApi* | [**retrieve_member**](docs/TeamsApi.md#retrieve_member) | **get** /user/teams/{teamId}/members/{userId} | Retrieve member
*TeamsApi* | [**retrieve_team**](docs/TeamsApi.md#retrieve_team) | **get** /user/teams/{teamId} | Retrieve team
*TeamsApi* | [**retrieve_team_members**](docs/TeamsApi.md#retrieve_team_members) | **get** /user/teams/{teamId}/members | List members
*TeamsApi* | [**update_member**](docs/TeamsApi.md#update_member) | **patch** /user/teams/{teamId}/members/{userId} | Update member
*TeamsApi* | [**update_team**](docs/TeamsApi.md#update_team) | **patch** /user/teams/{teamId} | Update team
*TokensApi* | [**create_token**](docs/TokensApi.md#create_token) | **post** /user/tokens | Create token
*TokensApi* | [**delete_token**](docs/TokensApi.md#delete_token) | **delete** /user/tokens/{token} | Remove token
*TokensApi* | [**list_tokens**](docs/TokensApi.md#list_tokens) | **get** /user/tokens | List tokens
*TokensApi* | [**retrieve_token**](docs/TokensApi.md#retrieve_token) | **get** /user/tokens/{token} | Retrieve token
*TokensApi* | [**update_token**](docs/TokensApi.md#update_token) | **patch** /user/tokens/{token} | Update token


## Documentation For Models

 - [AcceptInviteRequest](docs/AcceptInviteRequest.md)
 - [DeleteInviteResponse](docs/DeleteInviteResponse.md)
 - [DeleteTokenResponse](docs/DeleteTokenResponse.md)
 - [Invite](docs/Invite.md)
 - [InviteList](docs/InviteList.md)
 - [InviteRequest](docs/InviteRequest.md)
 - [Member](docs/Member.md)
 - [MemberList](docs/MemberList.md)
 - [ProtobufAny](docs/ProtobufAny.md)
 - [RpcStatus](docs/RpcStatus.md)
 - [Team](docs/Team.md)
 - [TeamList](docs/TeamList.md)
 - [Token](docs/Token.md)
 - [TokenList](docs/TokenList.md)
 - [UserProfile](docs/UserProfile.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

dev@lab5e.com

