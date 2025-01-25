use crate::models::constants::place::Location;
use crate::models::constants::{Language, PlaceSearchPlace, PlaceTypes};
use crate::models::NearbySearchResult;
use reqwest::Client;
use std::time::Duration;
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
}

impl<'a> NearbySearch<'a> {
    /// Construct a new `NearbySearch` instance.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A Google Places API key.
    /// * `client` - A `reqwest::Client` instance.
    ///
    /// # Example
    ///
    ///
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
        }
    }

    /// Set the location for the nearby search.
    ///
    /// # Arguments
    ///
    /// * `location` - The location to use for the nearby search. This can be
    ///   a specific latitude/longitude, or a query string that will be
    ///   geocoded.
    ///
    /// # Example
    ///
    ///
    pub fn with_location(&mut self, location: Location) -> &mut Self {
        self.location = Some(location);
        self
    }

    /// Set the radius for the nearby search.
    ///
    /// # Arguments
    ///
    /// * `radius` - The search radius in meters.
    ///
    /// # Returns
    ///
    /// A mutable reference to the current instance of `NearbySearch` to allow for method chaining.
    pub fn with_radius(&mut self, radius: f64) -> &mut Self {
        self.radius = Some(radius);
        self
    }

    /// Set the keyword for the nearby search.
    ///
    /// # Arguments
    ///
    /// * `keyword` - The keyword to use for the nearby search.
    ///
    /// # Returns
    ///
    /// A mutable reference to the current instance of `NearbySearch` to allow for method chaining.
    pub fn with_keyword(&mut self, keyword: &str) -> &mut Self {
        self.keyword = Some(String::from(keyword));
        self
    }

    /// Set the language for the nearby search.
    ///
    /// # Arguments
    ///
    /// * `language` - The language to use for the nearby search.
    ///
    /// # Returns
    ///
    /// A mutable reference to the current instance of `NearbySearch` to allow for method chaining.
    pub fn with_language(&mut self, language: Language) -> &mut Self {
        self.language = Some(language);
        self
    }


    /// Set the maximum price level for the nearby search.
    ///
    /// # Arguments
    ///
    /// * `maxprice` - The maximum price level to filter the results by.
    ///   The maximum value is 4.
    ///
    /// # Returns
    ///
    /// A mutable reference to the current instance of `NearbySearch` to allow for method chaining.
    pub fn with_maxprice(&mut self, maxprice: u8) -> &mut Self {
        self.maxprice = Some(maxprice);
        self
    }

    /// Set the minimum price level for the nearby search.
    ///
    /// # Arguments
    ///
    /// * `minprice` - The minimum price level to filter the results by.
    ///   The minimum value is 0.
    ///
    /// # Returns
    ///
    /// A mutable reference to the current instance of `NearbySearch` to allow for method chaining.
    pub fn with_minprice(&mut self, minprice: u8) -> &mut Self {
        self.minprice = Some(minprice);
        self
    }

    /// Set whether the search should only include places that are open now.
    ///
    /// # Arguments
    ///
    /// * `opennow` - Whether the search should only include places that are open now.
    ///
    /// # Returns
    ///
    /// A mutable reference to the current instance of `NearbySearch` to allow for method chaining.
    pub fn with_opennow(&mut self, opennow: bool) -> &mut Self {
        self.opennow = Some(opennow);
        self
    }

    /// Set the pagination token for the nearby search.
    ///
    /// This token is a string provided in the response of the previous call to `execute`.
    /// It is used to page through the results of the query.
    ///
    /// # Arguments
    ///
    /// * `pagetoken` - The pagination token to use for the search.
    ///
    /// # Returns
    ///
    /// A mutable reference to the current instance of `NearbySearch` to allow for method chaining.
    pub fn with_pagetoken(&mut self, pagetoken: &str) -> &mut Self {
        self.pagetoken = Some(String::from(pagetoken));
        self
    }

    /// Set the ranking criteria for the nearby search.
    ///
    /// # Arguments
    ///
    /// * `rankby` - The ranking criteria to use for the nearby search results.
    ///   Valid values are "prominence" or "distance".
    ///
    /// # Returns
    ///
    /// A mutable reference to the current instance of `NearbySearch` to allow for method chaining.
    pub fn with_rankby(&mut self, rankby: &str) -> &mut Self {
        self.rankby = Some(String::from(rankby));
        self
    }

    /// Set the type of places to search for.
    ///
    /// # Arguments
    ///
    /// * `place_type` - The type of places to search for.
    ///
    /// # Returns
    ///
    /// A mutable reference to the current instance of `NearbySearch` to allow for method chaining.
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

    /// Execute the call in an asynchronous fashion.
    ///
    /// This function will return `None` if the response from the API cannot be parsed.
    ///
    /// # Arguments
    ///
    /// * `max_pages` - The maximum number of pages of results to fetch.
    ///
    /// # Errors
    ///
    /// * The `Location` must be set for the `NearbySearch` before calling this function. If it is not set, a panic will occur.
    /// * If the response from the API cannot be parsed, an error will be printed and `None` will be returned.
    ///
    /// # Examples
    ///
    ///
    pub async fn execute(&mut self, max_pages: usize) -> Option<&mut Self> {
        if self.location.is_none() {
            panic!("Location must be provided for NearbySearch.");
        }

        let url = "https://maps.googleapis.com/maps/api/place/nearbysearch/json";
        let mut params = self.build_params();
        let mut page_count = 0;

        while page_count < max_pages {
            let resp = self.client.get(url).query(&params).send().await.unwrap();

            match resp.json::<NearbySearchResult>().await {
                Ok(query_result) => {
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
                        if page_count != max_pages {
                            sleep(Duration::from_millis(2000)).await;
                        }
                    } else {
                        break;
                    }
                }
                Err(err) => {
                    println!("Error parsing response: {:?}", err);
                    return None;
                }
            }
        }

        Some(self)
    }

    /// Execute the call in a blocking fashion.
    ///
    /// This function will return `None` if the response from the API cannot be parsed.
    ///
    /// # Arguments
    ///
    /// * `max_pages` - The maximum number of pages of results to fetch.
    ///
    /// # Errors
    ///
    /// * The `Location` must be set for the `NearbySearch` before calling this function. If it is not set, a panic will occur.
    /// * If the response from the API cannot be parsed, an error will be printed and `None` will be returned.
    ///
    /// # Examples
    ///
    ///
    #[cfg(feature = "blocking")]
    pub fn execute_blocking(&mut self, max_pages: usize) -> Option<&mut Self> {
        if self.location.is_none() {
            panic!("Location must be provided for NearbySearch.");
        }

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.execute(max_pages))
    }

    /// Returns an iterator over the places in the `NearbySearch`.
    ///
    /// This function allows you to iterate over the places in the `NearbySearch` result without having to manually keep track of the index.
    ///
    /// # Examples
    ///
    ///
    pub fn iter(&mut self) -> NearbySearchIter<'_, 'a> {
        NearbySearchIter {
            nearby_search: self,
            current_index: 0,
        }
    }

    /// Retrieve the place at the specified index.
    ///
    /// `index` - The index of the place to retrieve.
    ///
    /// Returns an `Option` with the place if it exists, or `None` if the index is out of range.
    pub fn at(&self, index: usize) -> Option<&PlaceSearchPlace> {
        self.result.places.get(index)
    }

    /// Retrieve the `NearbySearchResult`.
    ///
    /// This function returns a clone of the `NearbySearchResult` contained in the `NearbySearch`.
    ///
    /// # Returns
    ///
    /// A `NearbySearchResult` object representing the current state of the search results.
    ///
    /// # Examples
    ///
    /// ```
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

    /// Advances the iterator and returns the next place in the search results.
    ///
    /// Returns `None` when the iterator reaches the end of the collection.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it uses `std::mem::transmute` to cast
    /// a reference to `PlaceSearchPlace` to a reference with a different lifetime.
    /// This is safe in this context because the `NearbySearchIter` is created
    /// from a `&'b mut NearbySearch<'a>`, ensuring that the lifetime of
    /// `NearbySearch` is at least as long as `NearbySearchIter`.
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
