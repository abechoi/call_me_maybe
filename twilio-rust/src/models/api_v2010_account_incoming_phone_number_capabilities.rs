/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.34.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

/// ApiV2010AccountIncomingPhoneNumberCapabilities : Indicate if a phone can receive calls or messages



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiV2010AccountIncomingPhoneNumberCapabilities {
    #[serde(rename = "mms", skip_serializing_if = "Option::is_none")]
    pub mms: Option<bool>,
    #[serde(rename = "sms", skip_serializing_if = "Option::is_none")]
    pub sms: Option<bool>,
    #[serde(rename = "voice", skip_serializing_if = "Option::is_none")]
    pub voice: Option<bool>,
    #[serde(rename = "fax", skip_serializing_if = "Option::is_none")]
    pub fax: Option<bool>,
}

impl ApiV2010AccountIncomingPhoneNumberCapabilities {
    /// Indicate if a phone can receive calls or messages
    pub fn new() -> ApiV2010AccountIncomingPhoneNumberCapabilities {
        ApiV2010AccountIncomingPhoneNumberCapabilities {
            mms: None,
            sms: None,
            voice: None,
            fax: None,
        }
    }
}


