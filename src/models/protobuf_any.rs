/*
 * The User API
 *
 * API to manage teams, members and tokens
 *
 * The version of the OpenAPI document: 1.3.13 bordering-jerilyn
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtobufAny {
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl ProtobufAny {
    pub fn new() -> ProtobufAny {
        ProtobufAny {
            _type: None,
        }
    }
}


