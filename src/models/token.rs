/*
 * The User API
 *
 * API to manage teams, members and tokens
 *
 * The version of the OpenAPI document: 1.3.8 indivisible-esau
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Token {
    /// The resource of the token.  The token applies to the specified resource and any resources below this so the resource `/` applies to the root resource and any resource below the root resource. In the same manner `/collections` will apply to all collectins while `/collections/{id}` will apply to that particular collection.
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// Write flag for token.  If this is set to `true` the token can be used for write operations in the API such as POST, DELETE and PATCH.
    #[serde(rename = "write", skip_serializing_if = "Option::is_none")]
    pub write: Option<bool>,
    /// Use this in the `X-API-Token` header when using the API.
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Tags for the token.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// The last time the token was used. Time in ms since epoch.
    #[serde(rename = "lastUse", skip_serializing_if = "Option::is_none")]
    pub last_use: Option<String>,
    #[serde(rename = "uses", skip_serializing_if = "Option::is_none")]
    pub uses: Option<String>,
}

impl Token {
    pub fn new() -> Token {
        Token {
            resource: None,
            write: None,
            token: None,
            tags: None,
            last_use: None,
            uses: None,
        }
    }
}


