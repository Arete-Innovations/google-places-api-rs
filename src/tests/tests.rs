#[cfg(test)]
mod tests {

    
    use crate::types::constants::place::Location;
    use crate::types::constants::place_types::PlaceTypes;
    use crate::types::constants::{Language, PlaceDetailsPlaceFields};
    use crate::endpoints::api::GooglePlacesAPI;
    use futures::future::join_all;
    use std::collections::HashSet;
    use std::fs::{copy, File};
    use std::io::{BufReader, Read, Write};
    use std::time::Instant;


    #[tokio::test]
    async fn test_text_search() {
        let mut report = String::new();
        let start = Instant::now();
        let places_api = GooglePlacesAPI::from_env();
        let result = places_api
            .place_search()
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
        println!("{}",report);

        assert_eq!(result.error_message, None);
    }

    #[tokio::test]
    async fn test_text_search_with_radius() {
        let mut report = String::new();
        let start = Instant::now();
        let places_api = GooglePlacesAPI::from_env();
        let result = places_api
            .place_search()
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
        println!("{}",report);

        assert_eq!(result.error_message, None);
    }

    #[tokio::test]
    async fn test_nearby_search() {
        let mut report = String::new();
        let start = Instant::now();
        let places_api = GooglePlacesAPI::from_env();
        let result = places_api
            .place_search()
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
        println!("{}",report);

        assert_eq!(result.error_message, None);
    }

    #[tokio::test]
    async fn test_text_search_with_language() {
        let mut report = String::new();
        let start = Instant::now();
        let places_api = GooglePlacesAPI::from_env();
        let result = places_api
            .place_search()
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
        println!("{}",report);
        assert_eq!(result.error_message, None);
    }
    #[tokio::test]
    async fn test_place_details() {
        let report = String::new();
        let start = std::time::Instant::now();

        let places_api = GooglePlacesAPI::from_env();

        let results: Vec<_> = places_api
            .place_search()
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
                .place_search()
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
        let report = String::new();
        let start = std::time::Instant::now();

        let places_api = GooglePlacesAPI::from_env();

        let tasks: Vec<_> = places_api
            .place_search()
            .text_search()
            .with_query("coffee")
            .with_language(Language::En)
            .execute(3)
            .await
            .expect("Failed test_text_search_with_language")
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|place| {
                let places_api = &places_api;
                let place_id = place.id.clone();

                async move {
                    if let Some(detail_query_result) = places_api
                        .place_search()
                        .place_details()
                        .with_place_id(&place_id)
                        .execute()
                        .await
                    {
                        let details = detail_query_result.get_details();
                        println!("Processed: {:?}", details.place.website);
                    } else {
                        println!("Failed");
                    }
                }
            })
            .collect();

        join_all(tasks).await;
    }

    #[tokio::test]
    async fn test_place_details_map_mode_nearby() {
        let report = String::new();
        let start = std::time::Instant::now();

        let places_api = GooglePlacesAPI::from_env();

        let tasks: Vec<_> = places_api
            .place_search()
            .nearby_search()
            .with_location(Location::new(46.7749, 23.62))
            .with_radius(10000.0)
            .with_type(PlaceTypes::Accounting)
            .execute(3)
            .await
            .expect("Failed test_text_search_with_language")
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|place| {
                let places_api = &places_api;
                let place_id = place.id.clone();

                async move {
                    if let Some(detail_query_result) = places_api
                        .place_search()
                        .place_details()
                        .with_place_id(&place_id)
                        .with_fields(HashSet::from([
                            PlaceDetailsPlaceFields::Website,
                            PlaceDetailsPlaceFields::PlaceId,
                        ]))
                        .execute()
                        .await
                    {
                        let details = detail_query_result.get_details();
                        println!("Processed: {:?}", details.place.website);
                    } else {
                        println!("Failed");
                    }
                }
            })
            .collect();

        join_all(tasks).await;
    }

    #[tokio::test]
    async fn test_details_interface() {
        let report = String::new();
        let start = std::time::Instant::now();

        let places_api = GooglePlacesAPI::from_env();

        let tasks: Vec<_> = places_api
            .place_search()
            .nearby_search()
            .with_location(Location::new(46.7749, 23.62))
            .with_radius(10000.0)
            .with_type(PlaceTypes::Accounting)
            .execute(3)
            .await
            .expect("Failed test_text_search_with_language")
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|place| {
                let places_api = &places_api;
                let place_id = place.id.clone();

                async move {
                    if let Some(detail_query_result) = places_api
                        .place_search()
                        .place_details()
                        .with_place_id(&place_id)
                        .with_fields(HashSet::from([
                            PlaceDetailsPlaceFields::Website,
                            PlaceDetailsPlaceFields::PlaceId,
                        ]))
                        .execute()
                        .await
                    {
                        let details = detail_query_result.get_details();
                        println!("Processed: {:?}", details.place.website);
                    } else {
                        println!("Failed");
                    }
                }
            })
            .collect();

        join_all(tasks).await;
    }


    #[tokio::test]
    async fn test_find_place() {
        let mut report = String::new();
        let start = std::time::Instant::now();

        let places_api = GooglePlacesAPI::from_env();
        let result = places_api
            .place_search()
            .find_place()
            .with_input("coffee")
            .with_input_type("textquery")
            .execute()
            .await
            .expect("Failed test_find_place")
            .get_result();

        report.push_str(&format!(
            "Test: test_find_place\nStatus: {:?}\nPlaces Count: {:?}\nError: {:?}\nInfo: {:?}\nDuration: {:?}\n\n",
            result.status,
            result.places.len(),
            result.error_message,
            result.info_messages,
            start.elapsed()
        ));

        assert_eq!(result.error_message, None);


    }


    #[tokio::test]
    async fn test_find_place_iterator() {
        let report = String::new();
        let start = std::time::Instant::now();

        let places_api = GooglePlacesAPI::from_env();
        let find_place = places_api
            .place_search()
            .find_place()
            .with_input("coffee")
            .with_input_type("textquery")
            .execute()
            .await
            .expect("Failed test_find_place_iterator")
            .iter().cloned().collect::<Vec<_>>();

        let mut count = 0;
        for place in find_place {
            count += 1;
            println!("Processed: {:?}", place.name);
        }


        assert!(count > 0);
    }


    #[tokio::test]
    async fn test_photo() {
        let places_api = GooglePlacesAPI::from_env();

        let photo = places_api
            .place_search()
            .place_photos()
            .with_photo_reference("ATJ83zhSSAtkh5LTozXMhBghqubeOxnZWUV2m7Hv2tQaIzKQJgvZk9yCaEjBW0r0Zx1oJ9RF1G7oeM34sQQMOv8s2zA0sgGBiyBgvdyMxeVByRgHUXmv")
            .execute()
            .await
            .unwrap()
            .get_photo();

        // Open the file and read its contents into a Vec<u8>
        let mut file = BufReader::new(File::open("src/tests/image.jpg").unwrap());
        let mut file_content = Vec::new();
        file.read_to_end(&mut file_content).unwrap();

        // Compare the two vectors
        assert!(photo.iter().zip(file_content.iter()).all(|(a, b)| a == b));
    }
}
