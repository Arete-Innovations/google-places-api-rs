use crate::models::Photo;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PlaceDetailsPlace {
    // Basic
    #[serde(rename = "place_id")]
    pub id: String,
    pub name: Option<String>,
    pub address_components: Option<Vec<AddressComponent>>,
    pub adr_address: Option<String>,
    pub business_status: Option<String>,
    pub formatted_address: Option<String>,
    pub geometry: Option<Geometry>,
    pub icon: Option<String>,
    pub icon_mask_base_uri: Option<String>,
    pub icon_background_color: Option<String>,
    pub photos: Option<Vec<Photo>>,
    pub plus_code: Option<PlusCode>,
    pub types: Option<Vec<String>>,
    pub url: Option<String>,
    pub utc_offset: Option<i32>,
    pub vicinity: Option<String>,
    pub wheelchair_accessible_entrance: Option<bool>,

    // Contact
    pub current_opening_hours: Option<PlaceOpeningHours>,
    pub formatted_phone_number: Option<String>,
    pub international_phone_number: Option<String>,
    pub opening_hours: Option<OpeningHours>,
    pub secondary_opening_hours: Option<Vec<PlaceOpeningHours>>,
    pub website: Option<String>,

    // Atmosphere
    pub curbside_pickup: Option<bool>,
    pub delivery: Option<bool>,
    pub dine_in: Option<bool>,
    pub editorial_summary: Option<PlaceEditorialSummary>,
    pub price_level: Option<i32>,
    pub rating: Option<f32>,
    pub reservable: Option<bool>,
    pub reviews: Option<Vec<Review>>,
    pub serves_beer: Option<bool>,
    pub serves_breakfast: Option<bool>,
    pub serves_brunch: Option<bool>,
    pub serves_dinner: Option<bool>,
    pub serves_lunch: Option<bool>,
    pub serves_vegetarian_food: Option<bool>,
    pub serves_wine: Option<bool>,
    pub takeout: Option<bool>,
    pub user_ratings_total: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlaceSearchPlace {
    // Basic
    #[serde(rename = "place_id")]
    pub id: String,
    pub name: Option<String>,
    pub business_status: Option<String>,
    pub formatted_address: Option<String>,
    pub geometry: Option<Geometry>,
    pub icon: Option<String>,
    pub icon_mask_base_uri: Option<String>,
    pub icon_background_color: Option<String>,
    pub photos: Option<Vec<Photo>>,
    pub plus_code: Option<PlusCode>,
    pub types: Option<Vec<String>>,
    pub vicinity: Option<String>,

    // Contact
    pub opening_hours: Option<OpeningHours>,

    // Atmosphere
    pub price_level: Option<i32>,
    pub rating: Option<f32>,
    pub user_ratings_total: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressComponent {
    pub long_name: Option<String>,
    pub short_name: Option<String>,
    pub types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geometry {
    pub location: Option<Location>,
    pub viewport: Option<Viewport>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

impl Location {
    pub fn new(lat: f64, lon: f64) -> Self {
        Location {
            lat: Some(lat),
            lon: Some(lon),
        }
    }

    pub fn to_string(&self) -> String {
        if let (Some(lat), Some(lon)) = (self.lat, self.lon) {
            format!("{},{}", lat, lon)
        } else {
            panic!("Cannot convert Location to string: one of lat, lon is None");
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Viewport {
    pub northeast: Option<Location>,
    pub southwest: Option<Location>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpeningHours {
    pub open_now: Option<bool>,
    pub periods: Option<Vec<Period>>,
    pub weekday_text: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Period {
    pub open: Option<DayTime>,
    pub close: Option<DayTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayTime {
    pub day: Option<i32>,
    pub time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlusCode {
    pub compound_code: Option<String>,
    pub global_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub author_name: String,
    pub author_url: Option<String>,
    pub language: Option<String>,
    pub original_language: Option<String>,
    pub translated: Option<bool>,
    pub profile_photo_url: Option<String>,
    pub rating: i32,
    pub relative_time_description: String,
    pub text: Option<String>,
    pub time: i64,
}

// Additional structs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceOpeningHours {
    pub open_now: Option<bool>,
    pub periods: Option<Vec<OpeningHoursPeriod>>,
    pub special_days: Option<Vec<PlaceSpecialDay>>,
    #[serde(rename = "type")]
    pub place_opening_hours_type: Option<String>,
    pub weekday_text: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceSpecialDay {
    pub date: Option<String>,
    pub exceptional_hours: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpeningHoursPeriod {
    pub open: Option<OpeningHoursTime>,
    pub close: Option<OpeningHoursTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpeningHoursTime {
    pub day: Option<i32>,
    pub time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceEditorialSummary {
    pub body: Option<String>,
    pub attribution: Option<String>,
    pub language: Option<String>,
    pub overview: Option<String>,
}

impl PlaceDetailsPlace {
    pub fn display(&self) -> String {
        let json_value: Value = json!(self);
        let cleaned_value = remove_empty_fields(&json_value);
        serde_json::to_string_pretty(&cleaned_value)
            .unwrap_or_else(|_| String::from("Error formatting Place"))
    }
}

impl PlaceSearchPlace {
    pub fn display(&self) -> String {
        let json_value: Value = json!(self);
        let cleaned_value = remove_empty_fields(&json_value);
        serde_json::to_string_pretty(&cleaned_value)
            .unwrap_or_else(|_| String::from("Error formatting Place"))
    }
}

fn remove_empty_fields(value: &Value) -> Value {
    match value {
        Value::Object(obj) => {
            let cleaned_fields: serde_json::Map<String, Value> = obj
                .iter()
                .filter(|(_, v)| !v.is_null())
                .map(|(k, v)| (k.clone(), remove_empty_fields(v)))
                .collect();

            Value::Object(cleaned_fields)
        }
        Value::Array(arr) => {
            let cleaned_array: Vec<Value> = arr.iter().map(|v| remove_empty_fields(v)).collect();

            Value::Array(cleaned_array)
        }
        _ => value.clone(),
    }
}
