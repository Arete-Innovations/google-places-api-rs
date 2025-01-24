use crate::models::constants::PlaceSearchPlace;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NearbySearchResult {
    pub html_attributions: Vec<String>,
    #[serde(rename = "results")]
    pub places: Vec<PlaceSearchPlace>,
    pub status: PlaceSearchStatus,
    pub error_message: Option<String>,
    pub info_messages: Option<Vec<String>>,
    pub next_page_token: Option<String>,
    #[serde(skip)]
    pub total_results: u32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct FindPlaceSearchResult {
    #[serde(rename = "candidates")]
    pub places: Vec<PlaceSearchPlace>,
    pub status: PlaceSearchStatus,
    pub error_message: Option<String>,
    pub info_messages: Option<Vec<String>>,
    #[serde(skip)]
    pub total_results: u32,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TextSearchResult {
    pub html_attributions: Vec<String>,
    #[serde(rename = "results")]
    pub places: Vec<PlaceSearchPlace>,
    pub status: PlaceSearchStatus,
    pub error_message: Option<String>,
    pub info_messages: Option<Vec<String>>,
    pub next_page_token: Option<String>,
    #[serde(skip)]
    pub total_results: u32,
}

impl NearbySearchResult {
    pub fn calculate_total_results(&mut self) {
        self.total_results = self.places.len() as u32;
    }
    pub fn display(&self) -> String {
        let places = self
            .places
            .iter()
            .map(|p| p.display())
            .collect::<Vec<String>>()
            .join(", ");
        let html_attributions = self.html_attributions.join(", ");
        let info_messages = self
            .info_messages
            .as_ref()
            .map(|v| v.join(", "))
            .unwrap_or_default();
        format!("NearbySearchResult {{ html_attributions: [{}], places: [{}], status: {}, error_message: {}, info_messages: [{}], next_page_token: {}, total_results: {} }}",
                html_attributions, places, self.status.to_string(),
                self.error_message.as_ref().unwrap_or(&"".to_string()),
                info_messages, self.next_page_token.as_ref().unwrap_or(&"".to_string()), self.total_results)
    }
}

impl FindPlaceSearchResult {
    pub fn calculate_total_results(&mut self) {
        self.total_results = self.places.len() as u32;
    }
    pub fn display(&self) -> String {
        let results = self
            .places
            .iter()
            .map(|p| p.display())
            .collect::<Vec<String>>()
            .join(", ");
        let info_messages = self
            .info_messages
            .as_ref()
            .map(|v| v.join(", "))
            .unwrap_or_default();
        format!("FindPlaceSearchResult {{ results: [{}], status: {}, error_message: {}, info_messages: [{}], total_results: {} }}",
                results, self.status.to_string(), self.error_message.as_ref().unwrap_or(&"".to_string()), info_messages, self.total_results)
    }
}

impl TextSearchResult {
    pub fn calculate_total_results(&mut self) {
        self.total_results = self.places.len() as u32;
    }
    pub fn display(&self) -> String {
        let places = self
            .places
            .iter()
            .map(|p| p.display())
            .collect::<Vec<String>>()
            .join(", ");
        let html_attributions = self.html_attributions.join(", ");
        let info_messages = self
            .info_messages
            .as_ref()
            .map(|v| v.join(", "))
            .unwrap_or_default();
        format!("TextSearchResult {{ html_attributions: [{}], places: [{}], status: {}, error_message: {}, info_messages: [{}], next_page_token: {} }}, total_results: {}",
                html_attributions, places, self.status.to_string(),
                self.error_message.as_ref().unwrap_or(&"".to_string()),
                info_messages, self.next_page_token.as_ref().unwrap_or(&"".to_string()), self.total_results)
    }
}

#[derive(Debug, Serialize, PartialEq, Eq, Deserialize, Display, EnumString, Clone, Default)]
pub enum PlaceSearchStatus {
    #[serde(rename = "OK")]
    #[strum(serialize = "OK")]
    Ok,
    #[serde(rename = "ZERO_RESULTS")]
    #[strum(serialize = "ZERO_RESULTS")]
    ZeroResults,
    #[serde(rename = "INVALID_REQUEST")]
    #[strum(serialize = "INVALID_REQUEST")]
    InvalidRequest,
    #[serde(rename = "OVER_QUERY_LIMIT")]
    #[strum(serialize = "OVER_QUERY_LIMIT")]
    OverQueryLimit,
    #[serde(rename = "REQUEST_DENIED")]
    #[strum(serialize = "REQUEST_DENIED")]
    RequestDenied,
    #[serde(rename = "UNKNOWN_ERROR")]
    #[strum(serialize = "UNKNOWN_ERROR")]
    #[default]
    UnknownError,
}
