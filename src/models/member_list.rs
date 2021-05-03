/*
 * The User API
 *
 * API to manage teams, members and tokens
 *
 * The version of the OpenAPI document: 1.3.10 constant-champ
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemberList {
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<crate::models::Member>>,
}

impl MemberList {
    pub fn new() -> MemberList {
        MemberList { members: None }
    }
}
