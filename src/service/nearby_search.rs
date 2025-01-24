use std::time::Duration;
use crate::models::constants::place::Location;
use crate::models::constants::{Language, PlaceSearchPlace, PlaceTypes};
use crate::models::NearbySearchResult;
use reqwest::Client;
use tokio::time::sleep;

pub struct NearbySearch<'a> {
    location: Option<Location>,
    radius: Option<f64>,
    keyword: Option<String>,
    language: Option<Language>,
    maxprice: Option<u8>,
    minprice: Option<u8>,
    opennow: Option<bool>,
    pagetoken: Option<String>,
    rankby: Option<String>,
    place_type: Option<String>,
    api_key: String,
    client: &'a Client,
    result: NearbySearchResult,
    current_index: usize,
}

impl<'a> NearbySearch<'a> {
    pub fn new(api_key: &str, client: &'a Client) -> Self {
        Self {
            location: None,
            radius: None,
            keyword: None,
            language: None,
            maxprice: None,
            minprice: None,
            opennow: None,
            pagetoken: None,
            rankby: None,
            place_type: None,
            api_key: String::from(api_key),
            client,
            result: Default::default(),
            current_index: 0,
        }
    }

    pub fn with_location(&mut self, location: Location) -> &mut Self {
        self.location = Some(location);
        self
    }

    pub fn with_radius(&mut self, radius: f64) -> &mut Self {
        self.radius = Some(radius);
        self
    }

    pub fn with_keyword(&mut self, keyword: &str) -> &mut Self {
        self.keyword = Some(String::from(keyword));
        self
    }

    pub fn with_language(&mut self, language: Language) -> &mut Self {
        self.language = Some(language);
        self
    }

    pub fn with_maxprice(&mut self, maxprice: u8) -> &mut Self {
        self.maxprice = Some(maxprice);
        self
    }

    pub fn with_minprice(&mut self, minprice: u8) -> &mut Self {
        self.minprice = Some(minprice);
        self
    }

    pub fn with_opennow(&mut self, opennow: bool) -> &mut Self {
        self.opennow = Some(opennow);
        self
    }

    pub fn with_pagetoken(&mut self, pagetoken: &str) -> &mut Self {
        self.pagetoken = Some(String::from(pagetoken));
        self
    }

    pub fn with_rankby(&mut self, rankby: &str) -> &mut Self {
        self.rankby = Some(String::from(rankby));
        self
    }

    pub fn with_type(&mut self, place_type: PlaceTypes) -> &mut Self {
        self.place_type = Some(place_type.to_string());
        self
    }

    fn build_params(&self) -> Vec<(&'static str, String)> {
        let mut params = vec![("key", self.api_key.clone())];

        if let Some(location) = &self.location {
            params.push(("location", location.to_string()));
        }

        if let Some(radius) = self.radius {
            params.push(("radius", radius.to_string()));
        }

        if let Some(keyword) = self.keyword.clone() {
            params.push(("keyword", keyword));
        }

        if let Some(language) = self.language {
            params.push(("language", language.to_string()));
        }

        if let Some(maxprice) = self.maxprice {
            params.push(("maxprice", maxprice.to_string()));
        }

        if let Some(minprice) = self.minprice {
            params.push(("minprice", minprice.to_string()));
        }

        if let Some(opennow) = self.opennow {
            params.push(("opennow", opennow.to_string()));
        }

        if let Some(pagetoken) = self.pagetoken.clone() {
            params.push(("pagetoken", pagetoken));
        }

        if let Some(rankby) = self.rankby.clone() {
            params.push(("rankby", rankby));
        }

        if let Some(place_type) = self.place_type.clone() {
            params.push(("type", place_type));
        }

        params
    }

    pub async fn execute(&mut self, max_pages: usize) -> Option<&mut Self> {
        if self.location.is_none() {
            panic!("Location must be provided for NearbySearch.");
        }

        let url = "https://maps.googleapis.com/maps/api/place/nearbysearch/json";
        let mut params = self.build_params();
        let mut page_count = 0;

        while page_count < max_pages {
            let resp = self.client.get(url).query(&params).send().await.unwrap();

            if let Ok(query_result) = resp.json::<NearbySearchResult>().await {
                if page_count == 0 {
                    self.result = query_result.clone();
                } else {
                    self.result.places.extend(query_result.places);
                }

                if let Some(next_page_token) = query_result.next_page_token {
                    params = vec![
                        ("key", self.api_key.clone()),
                        ("pagetoken", next_page_token),
                    ];
                    page_count += 1;
                    sleep(Duration::from_millis(2000)).await;

                } else {
                    break;
                }
            } else {
                return None;
            }
        }

        Some(self)
    }

    #[cfg(feature = "blocking")]
    pub fn execute_blocking(&mut self, max_pages: usize) -> Option<&mut Self> {
        if self.location.is_none() {
            panic!("Location must be provided for NearbySearch.");
        }

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.execute(max_pages))
    }

    pub fn iter(&mut self) -> NearbySearchIter<'_, 'a> {
        NearbySearchIter {
            nearby_search: self,
            current_index: 0,
        }
    }

    pub fn at(&self, index: usize) -> Option<&PlaceSearchPlace> {
        self.result.places.get(index)
    }

    pub fn get_result(&'a self) -> NearbySearchResult {
        self.result.clone()
    }
}

pub struct NearbySearchIter<'a, 'b> {
    nearby_search: &'b mut NearbySearch<'a>,
    current_index: usize,
}

impl<'a, 'b> Iterator for NearbySearchIter<'a, 'b> {
    type Item = &'b PlaceSearchPlace;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.nearby_search.result.places.len() {
            let place = &self.nearby_search.result.places[self.current_index];
            self.current_index += 1;
            Some(unsafe { std::mem::transmute(place) })
        } else {
            None
        }
    }
}
