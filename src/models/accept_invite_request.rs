/*
 * The User API
 *
 * API to manage teams, members and tokens
 *
 * The version of the OpenAPI document: 1.3.6 crooked-daija
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcceptInviteRequest {
    /// The invite code to use.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl AcceptInviteRequest {
    pub fn new() -> AcceptInviteRequest {
        AcceptInviteRequest {
            code: None,
        }
    }
}


