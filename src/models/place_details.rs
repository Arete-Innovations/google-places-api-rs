use serde::{Deserialize, Serialize};
use crate::models::constants::PlaceDetailsPlace;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PlaceDetailsResult {
    pub html_attributions: Vec<String>,
    #[serde(rename = "result")]
    pub place: PlaceDetailsPlace,
    pub status: PlaceDetailsStatus,
    pub info_messages: Option<Vec<String>>,
}

impl PlaceDetailsResult {
    pub fn display(&self) -> String {
        let html_attributions = self.html_attributions.join(", ");
        let info_messages = self.info_messages.as_ref().map(|v| v.join(", ")).unwrap_or_default();
        format!("PlaceDetails {{ html_attributions: [{}], place: {}, status: {}, info_messages: [{}] }}", 
            html_attributions, self.place.display(), self.status.as_str(), info_messages)
    }
}

#[derive(Debug, Serialize, PartialEq, Eq, Deserialize, Default, Clone)]
pub enum PlaceDetailsStatus {
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "ZERO_RESULTS")]
    ZeroResults,
    #[serde(rename = "OVER_QUERY_LIMIT")]
    OverQueryLimit,
    #[serde(rename = "REQUEST_DENIED")]
    RequestDenied,
    #[serde(rename = "INVALID_REQUEST")]
    InvalidRequest,
    #[serde(rename = "UNKNOWN_ERROR")]
    #[default]
    UnknownError,
    #[serde(rename = "NOT_FOUND")]
    NotFound,
}

impl PlaceDetailsStatus {
    pub fn as_str(&self) -> &str {
        match self {
            PlaceDetailsStatus::Ok => "OK",
            PlaceDetailsStatus::ZeroResults => "ZERO_RESULTS",
            PlaceDetailsStatus::OverQueryLimit => "OVER_QUERY_LIMIT",
            PlaceDetailsStatus::RequestDenied => "REQUEST_DENIED",
            PlaceDetailsStatus::InvalidRequest => "INVALID_REQUEST",
            PlaceDetailsStatus::UnknownError => "UNKNOWN_ERROR",
            PlaceDetailsStatus::NotFound => "NOT_FOUND",
        }
    }
}


