use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Photo {
    // Define the fields that represent a place photo
    #[serde(rename = "photo_reference")]
    pub id: String,
    pub height: u32,
    pub width: u32,
    pub html_attributions: Vec<String>,
}