/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.34.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MessageEnumStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "sending")]
    Sending,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "undelivered")]
    Undelivered,
    #[serde(rename = "receiving")]
    Receiving,
    #[serde(rename = "received")]
    Received,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "partially_delivered")]
    PartiallyDelivered,
    #[serde(rename = "canceled")]
    Canceled,

}

impl ToString for MessageEnumStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Queued => String::from("queued"),
            Self::Sending => String::from("sending"),
            Self::Sent => String::from("sent"),
            Self::Failed => String::from("failed"),
            Self::Delivered => String::from("delivered"),
            Self::Undelivered => String::from("undelivered"),
            Self::Receiving => String::from("receiving"),
            Self::Received => String::from("received"),
            Self::Accepted => String::from("accepted"),
            Self::Scheduled => String::from("scheduled"),
            Self::Read => String::from("read"),
            Self::PartiallyDelivered => String::from("partially_delivered"),
            Self::Canceled => String::from("canceled"),
        }
    }
}

impl Default for MessageEnumStatus {
    fn default() -> MessageEnumStatus {
        Self::Queued
    }
}




