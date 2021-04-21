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
pub struct Team {
    #[serde(rename = "teamId", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(rename = "isPrivate", skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<crate::models::Member>>,
}

impl Team {
    pub fn new() -> Team {
        Team {
            team_id: None,
            is_private: None,
            tags: None,
            members: None,
        }
    }
}


