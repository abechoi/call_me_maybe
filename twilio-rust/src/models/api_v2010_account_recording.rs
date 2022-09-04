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
pub struct ApiV2010AccountRecording {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The API version used during the recording.
    #[serde(rename = "api_version", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// The SID of the Call the resource is associated with
    #[serde(rename = "call_sid", skip_serializing_if = "Option::is_none")]
    pub call_sid: Option<String>,
    /// The unique ID for the conference associated with the recording.
    #[serde(rename = "conference_sid", skip_serializing_if = "Option::is_none")]
    pub conference_sid: Option<String>,
    /// The RFC 2822 date and time in GMT that the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT that the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The start time of the recording, given in RFC 2822 format
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// The length of the recording in seconds.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The one-time cost of creating the recording.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// The currency used in the price property.
    #[serde(rename = "price_unit", skip_serializing_if = "Option::is_none")]
    pub price_unit: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::RecordingEnumStatus>,
    /// The number of channels in the final recording file as an integer.
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<i32>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::models::RecordingEnumSource>,
    /// More information about why the recording is missing, if status is `absent`.
    #[serde(rename = "error_code", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    /// The URI of the resource, relative to `https://api.twilio.com`
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// How to decrypt the recording.
    #[serde(rename = "encryption_details", skip_serializing_if = "Option::is_none")]
    pub encryption_details: Option<serde_json::Value>,
    /// A list of related resources identified by their relative URIs
    #[serde(rename = "subresource_uris", skip_serializing_if = "Option::is_none")]
    pub subresource_uris: Option<serde_json::Value>,
    /// The URL of the media file.
    #[serde(rename = "media_url", skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
}

impl ApiV2010AccountRecording {
    pub fn new() -> ApiV2010AccountRecording {
        ApiV2010AccountRecording {
            account_sid: None,
            api_version: None,
            call_sid: None,
            conference_sid: None,
            date_created: None,
            date_updated: None,
            start_time: None,
            duration: None,
            sid: None,
            price: None,
            price_unit: None,
            status: None,
            channels: None,
            source: None,
            error_code: None,
            uri: None,
            encryption_details: None,
            subresource_uris: None,
            media_url: None,
        }
    }
}


