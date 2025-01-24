use crate::service::text_search::TextSearch;

use reqwest::Client;
use crate::service::nearby_search::NearbySearch;
use crate::service::place_details::PlaceDetails;

pub struct PlaceSearch<'a> {
    api_key: String,
    client: &'a Client
}

impl<'a> PlaceSearch<'a> {
    
    pub fn new(api_key: &str, client: &'a Client ) -> Self {
        Self {
            api_key: String::from(api_key),
            client
        }
    }

    pub fn text_search(&mut self) -> TextSearch{
        let text_search_object: TextSearch = TextSearch::new(self.api_key.as_str(), self.client);
        text_search_object
    }

    pub fn nearby_search(&mut self) -> NearbySearch{
        let nearby_search_object: NearbySearch = NearbySearch::new(self.api_key.as_str(), self.client);
        nearby_search_object
    }

    pub fn place_details(&mut self) -> PlaceDetails{
        let details_object: PlaceDetails = PlaceDetails::new(self.api_key.as_str(), self.client);
        details_object
    }
}