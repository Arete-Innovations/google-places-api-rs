use crate::types::constants::{Language, ReviewSort};
use crate::types::{Photo, PlaceDetailsResult};
use isocountry::CountryCode;
use reqwest::Client;
use std::collections::HashSet;
pub struct PlacePhotos<'a> {
    photo_reference: Option<String>,
    maxwidht: Option<u16>,
    maxheight: Option<u16>,
    api_key: String,
    client: &'a Client,
    result: Vec<u8>,
}

impl<'a> PlacePhotos<'a> {
    pub fn new(api_key: &str, client: &'a Client) -> Self {
        Self {
            photo_reference: None,
            maxwidht: None,
            maxheight: None,
            api_key: String::from(api_key),
            client,
            result: Default::default(),
        }
    }

    /// Assign the photo_reference for a PlacePhotos call.
    ///
    /// # Arguments
    ///
    /// * `photo_reference` - A string slice that holds the reference of the photo.
    ///
    /// # Returns
    ///
    /// A mutable reference to the same `PlacePhotos` instance.
    ///
    /// This method allows chaining of multiple configuration methods.
    pub fn with_photo_reference(&mut self, photo_reference: &str) -> &mut PlacePhotos<'a> {
        self.photo_reference = Some(String::from(photo_reference));
        self
    }

    /// Assign the maximum width for the requested photo.
    ///
    /// # Arguments
    ///
    /// * `maxwidth` - The maximum width of the image.
    ///
    /// # Returns
    ///
    /// A mutable reference to the same `PlacePhotos` instance.
    ///
    /// This method allows chaining of multiple configuration methods.
    pub fn with_maxwidth(&mut self, maxwidth: u16) -> &mut PlacePhotos<'a> {
        self.maxwidht = Some(maxwidth);
        self
    }

    /// Assign the maximum height for the requested photo.
    ///
    /// # Arguments
    ///
    /// * `maxheight` - The maximum height of the image.
    ///
    /// # Returns
    ///
    /// A mutable reference to the same `PlacePhotos` instance.
    ///
    /// This method allows chaining of multiple configuration methods.
    pub fn with_maxheight(&mut self, maxheight: u16) -> &mut PlacePhotos<'a> {
        self.maxheight = Some(maxheight);
        self
    }

    pub fn build_params(&self) -> Vec<(&'static str, String)> {
        let mut params = vec![("key", self.api_key.clone())];

        if let Some(photo_reference) = &self.photo_reference {
            params.push(("photoreference", photo_reference.clone()));
        }

        if let Some(maxwidth) = &self.maxwidht {
            params.push(("maxwidth", maxwidth.to_string()));
        }

        if let Some(maxheight) = &self.maxheight {
            params.push(("maxheight", maxheight.to_string()));
        }

        params
    }


    pub async fn execute(&mut self) -> Option<&mut PlacePhotos<'a>> {
        match self.photo_reference.clone() {
            Some(_) => {
                let url = "https://maps.googleapis.com/maps/api/place/photo";
                let params = self.build_params();

                let resp = self.client.get(url).query(&params).send().await.unwrap();

                match resp.bytes().await {
                    Ok(query_result) => {
                        self.result = query_result.to_vec();
                        Some(self)
                    }
                    Err(err) => {
                        println!("Error parsing response: {:?}", err);
                        None
                    }
                }
            }
            None => {
                panic!("Provide photo_refernece to get a place's pohoto!");
            }
        }
    }

    #[cfg(feature = "blocking")]
    pub fn execute_blocking(&mut self) -> Option<&mut PlacePhotos<'a>> {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.execute())
    }

    pub fn get_photo(&self) -> Vec<u8> {
        self.result.clone()
    }
}
