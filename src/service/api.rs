use crate::service::place_search::PlaceSearch;
use dotenv::dotenv;
use reqwest::Client;

pub struct GooglePlacesAPI {
    api_key: String,
    client: Client,
}

impl GooglePlacesAPI {
    /// Creates a new instance of `GooglePlacesAPI`.
    ///
    /// Loads environment variables using `dotenv` and retrieves the
    /// `GOOGLE_PLACES_API_KEY` from the environment to initialize the API key.
    /// Initializes a new `reqwest::Client` for HTTP requests.
    ///
    /// # Panics
    ///
    /// Panics if the `GOOGLE_PLACES_API_KEY` environment variable is not set.
    pub fn new() -> Self {
        dotenv().ok(); // This line loads the environment variables from the ".env" file.
        Self {
            api_key: String::from(
                std::env::var("GOOGLE_PLACES_API_KEY").expect("GOOGLE_PLACES_API_KEY must be set."),
            ),
            client: Client::new(),
        }
    }

    /// Returns a new `PlaceSearch` instance with the API key and client.

    pub fn place_search(&self) -> PlaceSearch {
        PlaceSearch::new(self.api_key.as_str(), &self.client)
    }
}
