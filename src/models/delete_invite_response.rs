/*
 * The User API
 *
 * API to manage teams, members and tokens
 *
 * The version of the OpenAPI document: 1.3.7 frequent-amara
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteInviteResponse {
    #[serde(rename = "invite", skip_serializing_if = "Option::is_none")]
    pub invite: Option<Box<crate::models::Invite>>,
}

impl DeleteInviteResponse {
    pub fn new() -> DeleteInviteResponse {
        DeleteInviteResponse {
            invite: None,
        }
    }
}


