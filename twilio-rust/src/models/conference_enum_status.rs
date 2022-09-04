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
pub enum ConferenceEnumStatus {
    #[serde(rename = "init")]
    Init,
    #[serde(rename = "in-progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,

}

impl ToString for ConferenceEnumStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Init => String::from("init"),
            Self::InProgress => String::from("in-progress"),
            Self::Completed => String::from("completed"),
        }
    }
}

impl Default for ConferenceEnumStatus {
    fn default() -> ConferenceEnumStatus {
        Self::Init
    }
}




