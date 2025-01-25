use crate::models::constants::{Language, LocationBias, PlaceSearchPlace, PlaceSearchPlaceFields};
use crate::models::FindPlaceSearchResult;
use reqwest::Client;
use std::collections::HashSet;

pub struct FindPlace<'a> {
    input: Option<String>,
    input_type: Option<String>,
    language: Option<Language>,
    fields: Option<HashSet<PlaceSearchPlaceFields>>,
    location_bias: Option<LocationBias>,
    api_key: String,
    client: &'a Client,
    result: FindPlaceSearchResult,
}

impl<'a> FindPlace<'a> {
    /// Construct a new `FindPlace` instance.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The Google Places API key.
    /// * `client` - The reqwest client instance.
    ///
    /// # Returns
    ///
    /// A new instance of `FindPlace`.
    pub fn new(api_key: &str, client: &'a Client) -> Self {
        Self {
            input: None,
            fields: None,
            language: None,
            location_bias: None,
            input_type: None,
            api_key: String::from(api_key),
            client,
            result: Default::default(),
        }
    }

    /**
    Assign the input string for a FindPlace call.

    input -> The input text.
    */
    pub fn with_input(&mut self, input: &str) -> &mut FindPlace<'a> {
        self.input = Some(String::from(input));
        self
    }

    /**
    Assign the fields for a FindPlace call.

    fields -> The fields parameter.
    */
    pub fn with_fields(&mut self, fields: HashSet<PlaceSearchPlaceFields>) -> &mut FindPlace<'a> {
        self.fields = Some(fields);
        self
    }

    /**
    Assign the language for a FindPlace call.

    language -> The language parameter.
    */
    pub fn with_language(&mut self, language: Language) -> &mut FindPlace<'a> {
        self.language = Some(language);
        self
    }

    /**
    Assign the location_bias for a FindPlace call.

    location_bias -> The location_bias parameter.
    */
    pub fn with_location_bias(&mut self, location_bias: LocationBias) -> &mut FindPlace<'a> {
        self.location_bias = Some(location_bias);
        self
    }

    /**
    Assign the input_type for a FindPlace call.

    input_type -> The input_type parameter.
    */
    pub fn with_input_type(&mut self, input_type: &str) -> &mut FindPlace<'a> {
        self.input_type = Some(String::from(input_type));
        self
    }

    fn build_params(&self) -> Vec<(&'static str, String)> {
        let mut params = vec![("key", self.api_key.clone())];

        if let Some(input) = &self.input {
            params.push(("input", input.clone()));
        }

        if let Some(fields) = &self.fields {
            params.push((
                "fields",
                fields
                    .iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ));
        }

        if let Some(language) = &self.language {
            params.push(("language", language.to_string()));
        }

        if let Some(location_bias) = &self.location_bias {
            params.push(("locationbias", location_bias.to_string()));
        }

        if let Some(input_type) = &self.input_type {
            params.push(("inputtype", input_type.clone()));
        }

        params
    }

    /**
    Execute the call in an asynchronous fashion.
    */
    pub async fn execute(&mut self) -> Option<&mut FindPlace<'a>> {
        match (self.input.clone(), self.input_type.clone()) {
            (Some(_), Some(_)) => {
                let url = "https://maps.googleapis.com/maps/api/place/findplacefromtext/json";
                let params = self.build_params();

                let resp = self.client.get(url).query(&params).send().await.unwrap();

                match resp.json::<FindPlaceSearchResult>().await {
                    Ok(query_result) => {
                        self.result = query_result;
                        Some(self)
                    }
                    Err(err) => {
                        println!("Error parsing response: {:?}", err);

                        None
                    }
                }
            }

            _ => {panic!("Provide input and inputtype for find_place query!");}
        }
    }

    /**
    Execute the call in a blocking fashion.
    */
    #[cfg(feature = "blocking")]
    pub fn execute_blocking(&mut self) -> Option<&mut FindPlace<'a>> {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.execute())
    }

    /**
    This function returns an iterator (FindPlaceIterator) over the places in a FindPlace object. The iterator is initialized to start at the first place (index 0).

    It allows you to iterate over the places in the FindPlace result without having to manually keep track of the index.

    For example, you can use it like this:
    */
    pub fn iter(&mut self) -> FindPlaceIterator<'_, 'a> {
        FindPlaceIterator {
            find_place: self,
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
    pub fn get_result(&self) -> FindPlaceSearchResult {
        self.result.clone()
    }
}


pub struct FindPlaceIterator<'a, 'b> {
    find_place: &'b mut FindPlace<'a>,
    current_index: usize,
}

impl<'a, 'b> Iterator for FindPlaceIterator<'a, 'b> {
    type Item = &'b PlaceSearchPlace;

    /// Advances the iterator and returns the next value.
    ///
    /// Returns `None` when iteration is finished.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it uses `std::mem::transmute` to cast a reference to `PlaceSearchPlace` to a reference to `&'b PlaceSearchPlace`.
    /// This is safe because the `FindPlaceIterator` is only ever created from a `&'b mut FindPlace<'a>`, so we know that the lifetime of the `FindPlace`
    /// is at least as long as the lifetime of the `FindPlaceIterator`.
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.find_place.result.places.len() {
            let place = &self.find_place.result.places[self.current_index];
            self.current_index += 1;
            Some(unsafe { std::mem::transmute(place) })
        } else {
            None
        }
    }
}