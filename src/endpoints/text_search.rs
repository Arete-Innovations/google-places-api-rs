use crate::types::constants::place::Location;
use crate::types::constants::{Language, PlaceSearchPlace, PlaceTypes};
use crate::types::TextSearchResult;
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;

pub struct TextSearch<'a> {
    text_query: Option<String>,
    radius: Option<f64>,
    language: Option<Language>,
    location: Option<Location>,
    maxprice: Option<u8>,
    minprice: Option<u8>,
    opennow: Option<bool>,
    pagetoken: Option<String>,
    region: Option<String>,
    place_type: Option<String>,
    api_key: String,
    client: &'a Client,
    result: TextSearchResult,
}

impl<'a> TextSearch<'a> {
    pub fn new(api_key: &str, client: &'a Client) -> Self {
        Self {
            text_query: None,
            radius: None,
            language: None,
            location: None,
            maxprice: None,
            minprice: None,
            opennow: None,
            pagetoken: None,
            region: None,
            place_type: None,
            api_key: String::from(api_key),
            client: client,
            result: Default::default(),
        }
    }

    /**
    Assign the query string for a TextSearch call.

    text_query -> The query text.
    */
    pub fn with_query(&mut self, text_query: &str) -> &mut TextSearch<'a> {
        self.text_query = Some(String::from(text_query));
        self
    }

    /**
    Assign the radius for a TextSearch call.

    radius -> The search radius.
    */
    pub fn with_radius(&mut self, radius: f64) -> &mut TextSearch<'a> {
        self.radius = Some(radius);
        self
    }

    /**
    Assign the language for a TextSearch call.

    language -> The language parameter.
    */
    pub fn with_language(&mut self, language: Language) -> &mut TextSearch<'a> {
        self.language = Some(language);
        self
    }

    /**
    Assign the location for a TextSearch call.

    location -> The location parameter.
    */
    pub fn with_location(&mut self, location: Location) -> &mut TextSearch<'a> {
        self.location = Some(location);
        self
    }

    /**
    Assign the max price for a TextSearch call.

    maxprice -> The maximum price level.
    */
    pub fn with_maxprice(&mut self, maxprice: u8) -> &mut TextSearch<'a> {
        self.maxprice = Some(maxprice);
        self
    }

    /**
    Assign the min price for a TextSearch call.

    minprice -> The minimum price level.
    */
    pub fn with_minprice(&mut self, minprice: u8) -> &mut TextSearch<'a> {
        self.minprice = Some(minprice);
        self
    }

    /**
    Assign the open now filter for a TextSearch call.

    opennow -> Whether the search should only include places that are open now.
    */
    pub fn with_opennow(&mut self, opennow: bool) -> &mut TextSearch<'a> {
        self.opennow = Some(opennow);
        self
    }

    /**
    Assign the page token for a TextSearch call.

    pagetoken -> The page token for the results.
    */
    pub fn with_pagetoken(&mut self, pagetoken: &str) -> &mut TextSearch<'a> {
        self.pagetoken = Some(String::from(pagetoken));
        self
    }

    /**
    Assign the region for a TextSearch call.

    region -> The region parameter.
    */
    pub fn with_region(&mut self, region: &str) -> &mut TextSearch<'a> {
        self.region = Some(String::from(region));
        self
    }

    /**
    Assign the place type for a TextSearch call.

    place_type -> The type of place.
    */
    pub fn with_type(&mut self, place_type: PlaceTypes) -> &mut TextSearch<'a> {
        self.place_type = Some(place_type.to_string());
        self
    }

    fn build_params(&self) -> Vec<(&'static str, String)> {
        let mut params = vec![("key", self.api_key.clone())];

        if let Some(text_query) = self.text_query.clone() {
            params.push(("query", text_query));
        }

        if let Some(radius) = self.radius {
            params.push(("radius", radius.to_string()));
        }

        if let Some(language) = self.language {
            params.push(("language", language.to_string()));
        }

        if let Some(location) = self.location.clone() {
            params.push(("location", location.to_string()));
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

        if let Some(region) = self.region.clone() {
            params.push(("region", region));
        }

        if let Some(place_type) = self.place_type.clone() {
            params.push(("type", place_type));
        }

        params
    }


    /// Execute the TextSearch call in a non-blocking fashion.
    ///
    /// This will make a request to the Google Places API and retrieve the results. The results will be stored in the `result` field of the `TextSearch` struct.
    ///
    /// If the query is successful, the method will return `Some(self)`. If the query fails, the method will return `None`.
    ///
    /// # Panics
    ///
    /// Panics if the query_text and type fields are both `None`.
    ///
    /// # Errors
    ///
    /// If the query fails, the method will return `None`. In this case, you should check the error message contained in the `result` field of the `TextSearch` struct.
    ///
    /// # Examples
    ///
    ///
    pub async fn execute(&mut self, max_pages: usize) -> Option<&mut TextSearch<'a>> {
        match (self.text_query.clone(), self.place_type.clone()) {
            (Some(_), _) | (_, Some(_)) => {
                let url = "https://maps.googleapis.com/maps/api/place/textsearch/json";
                let mut params = self.build_params();
                let mut page_count = 0;

                while page_count < max_pages {
                    let resp = self.client.get(url).query(&params).send().await.unwrap();

                    match resp.json::<TextSearchResult>().await {
                        Ok(query_result) => {
                            if page_count == 0 {
                                // First page, initialize result
                                self.result = query_result.clone();
                            } else {
                                // Append subsequent pages
                                self.result.places.extend(query_result.places);
                            }

                            if let Some(next_page_token) = query_result.next_page_token {
                                self.pagetoken = Some(next_page_token);
                                params = self.build_params();

                                page_count += 1;
                                if page_count != max_pages {
                                    sleep(Duration::from_millis(2000)).await;
                                }
                            } else {
                                break; // No more pages to fetch
                            }
                        }
                        Err(err) => {
                            println!("Failed to parse API response: {:?}", err);
                            return None;
                        }
                    }
                }

                Some(self)
            }
            (None, None) => {
                panic!("Provide either query_text or type or both for query!");
            }
        }
    }


    /// Execute the call in a blocking fashion.
    ///
    /// This function will return `None` if the response from the API cannot be parsed.
    ///
    /// # Arguments
    ///
    /// * `max_pages` - The maximum number of pages of results to fetch.
    ///
    /// # Examples
    ///
    ///
    #[cfg(feature = "blocking")]
    pub fn execute_blocking(&mut self, max_pages: usize) -> Option<&mut TextSearch<'a>> {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.execute(max_pages))
    }

    /**
    This function returns an iterator (TextSearchIter) over the places in a TextSearch object. The iterator is initialized to start at the first place (index 0).

    It allows you to iterate over the places in the TextSearch result without having to manually keep track of the index.

    For example, you can use it like this:
    */
    pub fn iter(&mut self) -> TextSearchIter<'_, 'a> {
        TextSearchIter {
            text_search: self,
            current_index: 0,
        }
    }

    /**
    Retrieve the place at the specified index.

    index -> The index of the place to retrieve.

    Returns an `Option` with the place if it exists, or `None` if the index is out of range.
    */
    pub fn at(&self, index: usize) -> Option<&PlaceSearchPlace> {
        self.result.places.get(index)
    }
    /**
    Retrieve a cloned `TextSearchResult`.

    This function returns a clone of the `TextSearchResult` associated with the `TextSearch` instance.

    # Returns
    A `TextSearchResult` object that contains the results of the text search operation.
    */
    pub fn get_result(&'a self) -> TextSearchResult {
        self.result.clone()
    }
}
pub struct TextSearchIter<'a, 'b> {
    text_search: &'b mut TextSearch<'a>,
    current_index: usize,
}

impl<'a, 'b> Iterator for TextSearchIter<'a, 'b> {
    type Item = &'b PlaceSearchPlace;

    /// Advances the iterator and returns the next value.
    ///
    /// Returns `None` when iteration is finished.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it uses `std::mem::transmute` to cast a reference to `PlaceSearchPlace` to a reference to `&'b PlaceSearchPlace`.
    /// This is safe because the `TextSearchIterator` is only ever created from a `&'b mut TextSearch<'a>`, so we know that the lifetime of the `TextSearch`
    /// is at least as long as the lifetime of the `TextSearchIterator`.
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.text_search.result.places.len() {
            let place = &self.text_search.result.places[self.current_index];
            self.current_index += 1;
            Some(unsafe { std::mem::transmute(place) })
        } else {
            None
        }
    }
}
