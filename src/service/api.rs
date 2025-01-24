use dotenv::dotenv;
use crate::service::place_search::PlaceSearch;
use reqwest::Client;


pub struct GooglePlacesAPI {
    api_key: String,
    client: Client
}

impl GooglePlacesAPI {

    pub fn new() -> Self {
        dotenv().ok(); // This line loads the environment variables from the ".env" file.
        Self {
            api_key: String::from(std::env::var("GOOGLE_PLACES_API_KEY").expect("GOOGLE_PLACES_API_KEY must be set.")),
            client: Client::new()
        }
    }

    pub fn details(&self) -> PlaceSearch {
        PlaceSearch::new(self.api_key.as_str(), &self.client)
    }
}