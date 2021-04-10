/*
 * The User API
 *
 * API to manage teams, members and tokens
 *
 * The version of the OpenAPI document: 1.3.5 everlasting-ashanti
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// UserProfile : Your user profile. You can change the contents of the user profile via the log in service you are using.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserProfile {
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "avatarUrl", skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "profileUrl", skip_serializing_if = "Option::is_none")]
    pub profile_url: Option<String>,
    #[serde(rename = "githubLogin", skip_serializing_if = "Option::is_none")]
    pub github_login: Option<String>,
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "logoutUrl", skip_serializing_if = "Option::is_none")]
    pub logout_url: Option<String>,
}

impl UserProfile {
    /// Your user profile. You can change the contents of the user profile via the log in service you are using.
    pub fn new() -> UserProfile {
        UserProfile {
            user_id: None,
            email: None,
            avatar_url: None,
            name: None,
            profile_url: None,
            github_login: None,
            provider: None,
            logout_url: None,
        }
    }
}


