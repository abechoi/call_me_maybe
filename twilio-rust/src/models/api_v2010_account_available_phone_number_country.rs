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
pub struct ApiV2010AccountAvailablePhoneNumberCountry {
    /// The ISO-3166-1 country code of the country.
    #[serde(rename = "country_code", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// The name of the country
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The URI of the Country resource, relative to `https://api.twilio.com`
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// Whether all phone numbers available in the country are new to the Twilio platform.
    #[serde(rename = "beta", skip_serializing_if = "Option::is_none")]
    pub beta: Option<bool>,
    /// A list of related resources identified by their relative URIs
    #[serde(rename = "subresource_uris", skip_serializing_if = "Option::is_none")]
    pub subresource_uris: Option<serde_json::Value>,
}

impl ApiV2010AccountAvailablePhoneNumberCountry {
    pub fn new() -> ApiV2010AccountAvailablePhoneNumberCountry {
        ApiV2010AccountAvailablePhoneNumberCountry {
            country_code: None,
            country: None,
            uri: None,
            beta: None,
            subresource_uris: None,
        }
    }
}


