/*
 * The User API
 *
 * API to manage teams, members and tokens
 *
 * The version of the OpenAPI document: 1.3.10 constant-champ
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method `accept_invite`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AcceptInviteError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    Status409(serde_json::Value),
    Status500(serde_json::Value),
    DefaultResponse(crate::models::RpcStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `create_team`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTeamError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    Status409(serde_json::Value),
    Status500(serde_json::Value),
    DefaultResponse(crate::models::RpcStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_invite`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteInviteError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    Status409(serde_json::Value),
    Status500(serde_json::Value),
    DefaultResponse(crate::models::RpcStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_member`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMemberError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    Status409(serde_json::Value),
    Status500(serde_json::Value),
    DefaultResponse(crate::models::RpcStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_team`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTeamError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    Status409(serde_json::Value),
    Status500(serde_json::Value),
    DefaultResponse(crate::models::RpcStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `generate_invite`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenerateInviteError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    Status409(serde_json::Value),
    Status500(serde_json::Value),
    DefaultResponse(crate::models::RpcStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_invites`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListInvitesError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    Status409(serde_json::Value),
    Status500(serde_json::Value),
    DefaultResponse(crate::models::RpcStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_teams`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTeamsError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    Status409(serde_json::Value),
    Status500(serde_json::Value),
    DefaultResponse(crate::models::RpcStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `retrieve_invite`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveInviteError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    Status409(serde_json::Value),
    Status500(serde_json::Value),
    DefaultResponse(crate::models::RpcStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `retrieve_member`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveMemberError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    Status409(serde_json::Value),
    Status500(serde_json::Value),
    DefaultResponse(crate::models::RpcStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `retrieve_team`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveTeamError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    Status409(serde_json::Value),
    Status500(serde_json::Value),
    DefaultResponse(crate::models::RpcStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `retrieve_team_members`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveTeamMembersError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    Status409(serde_json::Value),
    Status500(serde_json::Value),
    DefaultResponse(crate::models::RpcStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update_member`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMemberError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    Status409(serde_json::Value),
    Status500(serde_json::Value),
    DefaultResponse(crate::models::RpcStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update_team`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTeamError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    Status409(serde_json::Value),
    Status500(serde_json::Value),
    DefaultResponse(crate::models::RpcStatus),
    UnknownValue(serde_json::Value),
}

/// Accept an invite from another user. This will add the currently logged in user to the team as a regular memeber. When the invite is accepted it is removed from the team's invites and cannot be reused.
pub async fn accept_invite(
    configuration: &configuration::Configuration,
    body: crate::models::AcceptInviteRequest,
) -> Result<crate::models::Team, Error<AcceptInviteError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/user/teams/accept", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Token", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AcceptInviteError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create_team(
    configuration: &configuration::Configuration,
    body: crate::models::Team,
) -> Result<crate::models::Team, Error<CreateTeamError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/user/teams", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Token", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateTeamError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete an invite created earlier. You must be an administrator of the team to perform this action
pub async fn delete_invite(
    configuration: &configuration::Configuration,
    team_id: &str,
    code: &str,
) -> Result<crate::models::DeleteInviteResponse, Error<DeleteInviteError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/user/teams/{teamId}/invites/{code}",
        configuration.base_path,
        teamId = crate::apis::urlencode(team_id),
        code = crate::apis::urlencode(code)
    );
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteInviteError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Remove a member from the team. You must be an administrator to do this. You can't remove yourself from the team.
pub async fn delete_member(
    configuration: &configuration::Configuration,
    team_id: &str,
    user_id: &str,
) -> Result<crate::models::Member, Error<DeleteMemberError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/user/teams/{teamId}/members/{userId}",
        configuration.base_path,
        teamId = crate::apis::urlencode(team_id),
        userId = crate::apis::urlencode(user_id)
    );
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteMemberError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update the team. You must be an administrator of the team to edit it.
pub async fn delete_team(
    configuration: &configuration::Configuration,
    team_id: &str,
) -> Result<crate::models::Team, Error<DeleteTeamError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/user/teams/{teamId}",
        configuration.base_path,
        teamId = crate::apis::urlencode(team_id)
    );
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteTeamError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update the team. You must be an administrator of the team to edit it.
pub async fn generate_invite(
    configuration: &configuration::Configuration,
    team_id: &str,
    body: crate::models::InviteRequest,
) -> Result<crate::models::Invite, Error<GenerateInviteError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/user/teams/{teamId}/invites",
        configuration.base_path,
        teamId = crate::apis::urlencode(team_id)
    );
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Token", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GenerateInviteError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update the team. You must be an administrator of the team to edit it.
pub async fn list_invites(
    configuration: &configuration::Configuration,
    team_id: &str,
) -> Result<crate::models::InviteList, Error<ListInvitesError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/user/teams/{teamId}/invites",
        configuration.base_path,
        teamId = crate::apis::urlencode(team_id)
    );
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListInvitesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update the team. You must be an administrator of the team to edit it.
pub async fn list_teams(
    configuration: &configuration::Configuration,
) -> Result<crate::models::TeamList, Error<ListTeamsError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/user/teams", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListTeamsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Read a single invite from the team's set of non-redeemed invites.
pub async fn retrieve_invite(
    configuration: &configuration::Configuration,
    team_id: &str,
    code: &str,
) -> Result<crate::models::Invite, Error<RetrieveInviteError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/user/teams/{teamId}/invites/{code}",
        configuration.base_path,
        teamId = crate::apis::urlencode(team_id),
        code = crate::apis::urlencode(code)
    );
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RetrieveInviteError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn retrieve_member(
    configuration: &configuration::Configuration,
    team_id: &str,
    user_id: &str,
) -> Result<crate::models::Member, Error<RetrieveMemberError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/user/teams/{teamId}/members/{userId}",
        configuration.base_path,
        teamId = crate::apis::urlencode(team_id),
        userId = crate::apis::urlencode(user_id)
    );
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RetrieveMemberError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn retrieve_team(
    configuration: &configuration::Configuration,
    team_id: &str,
) -> Result<crate::models::Team, Error<RetrieveTeamError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/user/teams/{teamId}",
        configuration.base_path,
        teamId = crate::apis::urlencode(team_id)
    );
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RetrieveTeamError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn retrieve_team_members(
    configuration: &configuration::Configuration,
    team_id: &str,
) -> Result<crate::models::MemberList, Error<RetrieveTeamMembersError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/user/teams/{teamId}/members",
        configuration.base_path,
        teamId = crate::apis::urlencode(team_id)
    );
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RetrieveTeamMembersError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// You must be an administrator of the team to update member settings
pub async fn update_member(
    configuration: &configuration::Configuration,
    team_id: &str,
    user_id: &str,
    body: crate::models::Member,
) -> Result<crate::models::Member, Error<UpdateMemberError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/user/teams/{teamId}/members/{userId}",
        configuration.base_path,
        teamId = crate::apis::urlencode(team_id),
        userId = crate::apis::urlencode(user_id)
    );
    let mut local_var_req_builder = local_var_client.patch(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Token", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateMemberError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update the team. You must be an administrator of the team to edit it.
pub async fn update_team(
    configuration: &configuration::Configuration,
    team_id: &str,
    body: crate::models::Team,
) -> Result<crate::models::Team, Error<UpdateTeamError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/user/teams/{teamId}",
        configuration.base_path,
        teamId = crate::apis::urlencode(team_id)
    );
    let mut local_var_req_builder = local_var_client.patch(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Token", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateTeamError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
