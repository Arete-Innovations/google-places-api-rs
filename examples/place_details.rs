use dotenv::dotenv;
use std::env;
use std::collections::HashSet;
use google_places_api::client::GooglePlacesClient;
use google_places_api::services::PlaceDetailsService;
use google_places_api::models::constants::*;

#[tokio::main]
async fn main() {
    // Load environment variables from the .env file
    dotenv().ok();

    // Retrieve the API key from the environment variable
    let api_key = env::var("GOOGLE_PLACES_API_KEY")
        .expect("Please set the GOOGLE_PLACES_API_KEY environment variable");

    // Create a Google Places client
    let client = GooglePlacesClient::new(&api_key);

    // Create a PlaceSearchService instance
    let place_details_service = PlaceDetailsService::new(client);

    // Define place_id
    let place_id = "ChIJN1t_tDeuEmsRUsoyG83frY4";
    
    // Define the fields as a HashSet
    let fields: HashSet<PlaceDataField> = vec![
        PlaceDataField::Name,
        PlaceDataField::Rating,
        PlaceDataField::PhoneNumber,
    ]

    let language: Language ;
    .into_iter()
    .collect();


    // Perform the place details request
    match place_details_service.get_place_details(place_id, Some(fields), None, None, None, None, None).await {
        Ok(place_details) => {
            // Display the place details
            println!("{}", place_details.display());
        }
        Err(error) => {
            // Handle the error
            eprintln!("Error: {:?}", error);
        }
    }
}
