use crate::models::constants::{Language, PlaceDetailsPlaceFields, ReviewSort};
use crate::models::PlaceDetailsResult;
use isocountry::CountryCode;
use reqwest::Client;
use std::collections::HashSet;
pub struct PlaceDetails<'a> {
    place_id: Option<String>,
    fields: Option<HashSet<PlaceDetailsPlaceFields>>,
    language: Option<Language>,
    region: Option<CountryCode>,
    review_no_translation: Option<bool>,
    review_sort: Option<ReviewSort>,
    session_token: Option<String>,
    api_key: String,
    client: &'a Client,
    result: PlaceDetailsResult,
}

impl<'a> PlaceDetails<'a> {
    pub fn new(api_key: &str, client: &'a Client) -> Self {
        Self {
            place_id: None,
            fields: None,
            language: None,
            review_no_translation: None,
            review_sort: None,
            session_token: None,
            region: None,
            api_key: String::from(api_key),
            client,
            result: Default::default(),
        }
    }

    /**
    Assign the place_id for a PlaceDetails call.

    place_id -> The place id.
    */
    pub fn with_place_id(&mut self, place_id: &str) -> &mut PlaceDetails<'a> {
        self.place_id = Some(String::from(place_id));
        self
    }

    /**
    Assign the fields for a PlaceDetails call.

    fields -> The fields parameter.
    */
    pub fn with_fields(
        &mut self,
        fields: HashSet<PlaceDetailsPlaceFields>,
    ) -> &mut PlaceDetails<'a> {
        self.fields = Some(fields);
        self
    }

    /**
    Assign the language for a PlaceDetails call.

    language -> The language parameter.
    */
    pub fn with_language(&mut self, language: Language) -> &mut PlaceDetails<'a> {
        self.language = Some(language);
        self
    }

    /**
    Assign the review_no_translations for a PlaceDetails call.

    review_no_translations -> The review_no_translations parameter.
    */
    pub fn with_review_no_translations(
        &mut self,
        review_no_translations: bool,
    ) -> &mut PlaceDetails<'a> {
        self.review_no_translation = Some(review_no_translations);
        self
    }

    /**
    Assign the review_sort for a PlaceDetails call.

    review_sort -> The review_sort parameter.
    */
    pub fn with_review_sort(&mut self, review_sort: ReviewSort) -> &mut PlaceDetails<'a> {
        self.review_sort = Some(review_sort);
        self
    }

    /**
    Assign the session_token for a PlaceDetails call.

    session_token -> The session_token parameter.
    */
    pub fn with_session_token(&mut self, session_token: &str) -> &mut PlaceDetails<'a> {
        self.session_token = Some(String::from(session_token));
        self
    }

    /**
    Assign the region for a PlaceDetails call.

    region -> The region parameter.
    */
    pub fn with_region(&mut self, region: CountryCode) -> &mut PlaceDetails<'a> {
        self.region = Some(region);
        self
    }

    fn build_params(&self) -> Vec<(&'static str, String)> {
        let mut params = vec![("key", self.api_key.clone())];

        if let Some(place_id) = &self.place_id {
            params.push(("placeid", place_id.clone()));
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

        if let Some(review_no_translations) = &self.review_no_translation {
            params.push(("review_no_translations", review_no_translations.to_string()));
        }

        if let Some(review_sort) = &self.review_sort {
            params.push(("review_sort", review_sort.to_string()));
        }

        if let Some(session_token) = &self.session_token {
            params.push(("sessiontoken", session_token.clone()));
        }

        if let Some(region) = &self.region {
            params.push(("region", region.to_string()));
        }

        params
    }

    /**
    Execute the call in an asynchronous fashion.
    */
    pub async fn execute(&mut self) -> Option<&mut PlaceDetails<'a>> {
        match self.place_id.clone() {
            Some(_) => {
                let url = "https://maps.googleapis.com/maps/api/place/details/json";
                let params = self.build_params();

                let resp = self.client.get(url).query(&params).send().await.unwrap();

                match resp.json::<PlaceDetailsResult>().await {
                    Ok(query_result) => {
                        self.result = query_result;
                        Some(self)
                    }
                    Err(err) => {
                        println!("Error parsing response: {:?}", err);
                        println!("Maps API: Did you maybe forget to add the PlaceId to the request's fields filter?");

                        None
                    }
                }
            }
            None => {
                panic!("Provide place_id for details query!");
            }
        }
    }

    /**
    Execute the call in a blocking fashion.
    */
    #[cfg(feature = "blocking")]
    pub fn execute_blocking(&mut self) -> Option<&mut PlaceDetails<'a>> {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.execute())
    }

    pub fn get_details(&self) -> PlaceDetailsResult {
        self.result.clone()
    }
}
