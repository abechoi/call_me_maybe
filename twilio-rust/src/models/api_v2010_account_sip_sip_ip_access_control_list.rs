/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.34.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiV2010AccountSipSipIpAccessControlList {
    /// A string that uniquely identifies this resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The unique sid that identifies this account
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// A human readable description of this resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The date this resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The date this resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The IP addresses associated with this resource.
    #[serde(rename = "subresource_uris", skip_serializing_if = "Option::is_none")]
    pub subresource_uris: Option<serde_json::Value>,
    /// The URI for this resource
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl ApiV2010AccountSipSipIpAccessControlList {
    pub fn new() -> ApiV2010AccountSipSipIpAccessControlList {
        ApiV2010AccountSipSipIpAccessControlList {
            sid: None,
            account_sid: None,
            friendly_name: None,
            date_created: None,
            date_updated: None,
            subresource_uris: None,
            uri: None,
        }
    }
}


