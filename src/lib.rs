mod models;
mod service;

use std::fs::File;
use std::io::{BufWriter, Write};

#[cfg(test)]
mod tests {
    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>());
    }

    use super::*;
    use crate::models::constants::place::Location;
    use crate::models::constants::place_types::PlaceTypes;
    use crate::models::constants::{Language, PlaceDetailsPlaceFields};
    use crate::service::api::GooglePlacesAPI;
    use futures::future::join_all;
    use std::collections::HashSet;
    use std::time::Instant;

    fn write_report(report: &str) {
        let file = File::create(format!(
            "test_report_{}.txt",
            std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ))
            .expect("Unable to create test report file");
        let mut writer = BufWriter::new(file);
        writer
            .write_all(report.as_bytes())
            .expect("Unable to write to file");
    }

    #[tokio::test]
    async fn test_text_search() {
        let mut report = String::new();
        let start = Instant::now();
        let places_api = GooglePlacesAPI::new("AIzaSyCKUJExGG9OVjHqK2JhPo87ti6DFiwJLms");
        let result = places_api
            .details()
            .text_search()
            .with_query("coffee")
            .execute(3)
            .await
            .expect("Failed test_text_search")
            .get_result();

        report.push_str(&format!(
            "Test: test_text_search\nStatus: {:?}\nPlaces Count: {:?}\nError: {:?}\nInfo: {:?}\nDuration: {:?}\n\n",
            result.status,
            result.places.len(),
            result.error_message,
            result.info_messages,
            start.elapsed()
        ));

        assert_eq!(result.error_message, None);
        
    }

    #[tokio::test]
    async fn test_text_search_with_radius() {
        let mut report = String::new();
        let start = Instant::now();
        let places_api = GooglePlacesAPI::new("AIzaSyCKUJExGG9OVjHqK2JhPo87ti6DFiwJLms");
        let result = places_api
            .details()
            .text_search()
            .with_query("coffee")
            .with_radius(1000.0)
            .execute(3)
            .await
            .expect("Failed test_text_search_with_radius")
            .get_result();

        report.push_str(&format!(
            "Test: test_text_search_with_radius\nStatus: {:?}\nPlaces Count: {:?}\nError: {:?}\nInfo: {:?}\nDuration: {:?}\n\n",
            result.status,
            result.places.len(),
            result.error_message,
            result.info_messages,
            start.elapsed()
        ));

        assert_eq!(result.error_message, None);
        
    }

    #[tokio::test]
    async fn test_nearby_search() {
        let mut report = String::new();
        let start = Instant::now();
        let places_api = GooglePlacesAPI::new("AIzaSyCKUJExGG9OVjHqK2JhPo87ti6DFiwJLms");
        let result = places_api
            .details()
            .nearby_search()
            .with_location(Location::new(46.7749, 23.62))
            .with_radius(1000.0)
            .with_type(PlaceTypes::Cafe)
            .execute(3)
            .await
            .expect("Failed test_nearby_search")
            .get_result();

        report.push_str(&format!(
            "Test: test_nearby_search\nStatus: {:?}\nPlaces Count: {:?}\nError: {:?}\nInfo: {:?}\nDuration: {:?}\n\n",
            result.status,
            result.places.len(),
            result.error_message,
            result.info_messages,
            start.elapsed()
        ));

        assert_eq!(result.error_message, None);
        
    }

    #[tokio::test]
    async fn test_text_search_with_language() {
        let mut report = String::new();
        let start = Instant::now();
        let places_api = GooglePlacesAPI::new("AIzaSyCKUJExGG9OVjHqK2JhPo87ti6DFiwJLms");
        let result = places_api
            .details()
            .text_search()
            .with_query("coffee")
            .with_language(Language::En)
            .execute(3)
            .await
            .expect("Failed test_text_search_with_language")
            .get_result();

        report.push_str(&format!(
            "Test: test_text_search_with_language\nStatus: {:?}\nPlaces Count: {:?}\nError: {:?}\nInfo: {:?}\nDuration: {:?}\n\n",
            result.status,
            result.places.len(),
            result.error_message,
            result.info_messages,
            start.elapsed()
        ));

        assert_eq!(result.error_message, None);
        
    }

    async fn test_place_details() {
        let mut report = String::new();
        let start = std::time::Instant::now();

        let places_api = GooglePlacesAPI::new("AIzaSyCKUJExGG9OVjHqK2JhPo87ti6DFiwJLms");

        let results: Vec<_> = places_api
            .details()
            .text_search()
            .with_query("coffee")
            .with_language(Language::En)
            .execute(3)
            .await
            .expect("Failed test_text_search_with_language")
            .iter()
            .cloned()
            .collect();

        for place in results {
            let place_id = place.id.clone();

            if let Some(detail_query_result) = places_api
                .details()
                .place_details()
                .with_place_id(&place_id)
                .execute()
                .await
            {
                let details = detail_query_result.get_details();
                println!("Processed: {:?}", details);
            }
        }


    }

    #[tokio::test]
    async fn test_place_details_map_mode() {
        let mut report = String::new();
        let start = std::time::Instant::now();

        let places_api = GooglePlacesAPI::new("AIzaSyCKUJExGG9OVjHqK2JhPo87ti6DFiwJLms");

        let tasks: Vec<_> = places_api
            .details()
            .text_search()
            .with_query("coffee")
            .with_language(Language::En)
            .execute(3)
            .await
            .expect("Failed test_text_search_with_language")
            .iter()
            .cloned()
            .collect::<Vec<_>>().into_iter()
            .map(|place| {
                let places_api = &places_api;
                let place_id = place.id.clone();

                async move {
                    if let Some(detail_query_result) = places_api
                        .details()
                        .place_details()
                        .with_place_id(&place_id)
                        .execute()
                        .await
                    {
                        let details = detail_query_result.get_details();
                        println!("Processed: {:?}", details.place.website);
                    }else {
                        println!("Failed");
                    }
                }
            })
            .collect();

        join_all(tasks).await;

    }

    #[tokio::test]
    async fn test_place_details_map_mode_nearby() {
        let mut report = String::new();
        let start = std::time::Instant::now();

        let places_api = GooglePlacesAPI::new("AIzaSyCKUJExGG9OVjHqK2JhPo87ti6DFiwJLms");

        let tasks: Vec<_> = places_api
            .details()
            .nearby_search()
            .with_location(Location::new(46.7749, 23.62))
            .with_radius(10000.0)
            .with_type(PlaceTypes::Accounting)
            .execute(3)
            .await
            .expect("Failed test_text_search_with_language")
            .iter()
            .cloned()
            .collect::<Vec<_>>().into_iter()
            .map(|place| {
                let places_api = &places_api;
                let place_id = place.id.clone();

                async move {
                    if let Some(detail_query_result) = places_api
                        .details()
                        .place_details()
                        .with_place_id(&place_id)
                        .with_fields(HashSet::from([PlaceDetailsPlaceFields::Website, PlaceDetailsPlaceFields::PlaceId]))
                        .execute()
                        .await
                    {
                        let details = detail_query_result.get_details();
                        println!("Processed: {:?}", details.place.website);
                    }else {
                        println!("Failed");
                    }
                }
            })
            .collect();

        join_all(tasks).await;

    }
}
