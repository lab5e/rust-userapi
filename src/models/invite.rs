/*
 * The User API
 *
 * API to manage teams, members and tokens
 *
 * The version of the OpenAPI document: 1.3.5 everlasting-ashanti
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// Invite : Invite. Invites are used to share access to teams. You must be an administrator to generate invites for a team.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Invite {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Time stamp when the invite was generated.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl Invite {
    /// Invite. Invites are used to share access to teams. You must be an administrator to generate invites for a team.
    pub fn new() -> Invite {
        Invite {
            code: None,
            created_at: None,
        }
    }
}
