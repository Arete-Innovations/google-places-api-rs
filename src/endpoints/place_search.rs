use crate::endpoints::text_search::TextSearch;

use crate::endpoints::nearby_search::NearbySearch;
use crate::endpoints::place_details::PlaceDetails;
use reqwest::Client;
use crate::endpoints::find_place::FindPlace;

pub struct PlaceSearch<'a> {
    api_key: String,
    client: &'a Client,
}

impl<'a> PlaceSearch<'a> {
    /// Constructs a new `PlaceSearch` instance.
    ///
    /// ## DO NOT USE THIS ALONE, USE THE `GooglePlacesAPI` STRUCT.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A string slice that holds the API key for accessing the Google Places API.
    /// * `client` - A reference to a `reqwest::Client` for executing HTTP requests.
    ///
    /// # Returns
    ///
    /// A new instance of `PlaceSearch`.
    pub fn new(api_key: &str, client: &'a Client) -> Self {
        Self {
            api_key: String::from(api_key),
            client,
        }
    }

    /// Returns a new `TextSearch` instance that can be used to execute a
    /// Text Search request.
    ///
    /// # Returns
    ///
    /// A new instance of `TextSearch`.
    pub fn text_search(&mut self) -> TextSearch {
        let text_search_object: TextSearch = TextSearch::new(self.api_key.as_str(), self.client);
        text_search_object
    }

    /// Returns a new `NearbySearch` instance that can be used to execute a
    /// Nearby Search request.
    ///
    /// # Returns
    ///
    /// A new instance of `NearbySearch`.
    pub fn nearby_search(&mut self) -> NearbySearch {
        let nearby_search_object: NearbySearch =
            NearbySearch::new(self.api_key.as_str(), self.client);
        nearby_search_object
    }

    /// Returns a new `PlaceDetails` instance that can be used to execute a
    /// Place Details request.
    ///
    /// # Returns
    ///
    /// A new instance of `PlaceDetails`.
    pub fn place_details(&mut self) -> PlaceDetails {
        let details_object: PlaceDetails = PlaceDetails::new(self.api_key.as_str(), self.client);
        details_object
    }

    /// Returns a new `FindPlace` instance that can be used to execute a
    /// Find Place request.
    ///
    /// # Returns
    ///
    /// A new instance of `FindPlace`.
    pub fn find_place(&mut self) -> FindPlace {
        let find_place_object: FindPlace = FindPlace::new(self.api_key.as_str(), self.client);
        find_place_object
    }
}
