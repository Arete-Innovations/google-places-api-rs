# Google Places API Rust Wrapper

A comprehensive Rust library to interact with the Google Places API, providing modules for Place Search, Place Details, Photos, and more. This library is designed to make working with the Google Places API intuitive and efficient for developers.

---

## Warning

**This library is a work in progress**. Some features may not be fully implemented or could be subject to breaking changes. Use it at your own risk.

---

## Features

- Place Details
- Place Photos [TODO]
- Nearby Search
- Find Place Search
- Text Search
- Configurable parameters: language, input types, location bias, and more

---

## Installation

Add the following to your project

```bash
cargo add google_places_api
```

---

## Usage

### Initialization

To start using the library, initialize the `GooglePlacesAPI` instance:

```rust
use google_places_api::GooglePlacesAPI;

let api = GooglePlacesAPI::new(); // API key is loaded from the `GOOGLE_PLACES_API_KEY` environment variable.
```

### Example: Nearby Search

```rust
use google_places_api::models::Location;
use google_places_api::GooglePlacesAPI;
use google_places_api::place_search::NearbySearchResult;

#[tokio::main]
async fn main() {
    let api = GooglePlacesAPI::new();
    let mut place_search = api.place_search();

    let result: NearbySearchResult = place_search
        .nearby_search() // San Francisco coordinates with a 1.5 km radius
        .with_location(Location::new(37.7749, -122.4194))
        .with_radius(1500)
        .execute()
        .await
        .unwrap();

    println!("Nearby Places: {:?}", result.display());
}
```

### Example: Place Details

```rust
use google_places_api::GooglePlacesAPI;
use google_places_api::models::PlaceDetailsResult;

#[tokio::main]
async fn main() {
    let api = GooglePlacesAPI::new();
    let place_search = api.place_search();

    let result: PlaceDetailsResult = place_search
        .place_details() // Replace with a valid Place ID
        .with_place_id("ChIJVXealLU_xkcRja_At0z9AGY")
        .execute()
        .await
        .unwrap();

    println!("Place Details: {:?}", result.display());
}
```

### Example: Find Place

```rust
use google_places_api::GooglePlacesAPI;
use google_places_api::place_search::FindPlaceSearchResult;

#[tokio::main]
async fn main() {
    let api = GooglePlacesAPI::new();
    let mut place_search = api.place_search();

    let result: FindPlaceSearchResult = place_search
        .find_place()
        .with_input("Googleplex")
        .with_input_type("textquery")
        .execute()
        .await
        .unwrap();

    println!("Find Place Result: {:?}", result.display());
}
```



---

## Environment Variables

This library relies on the `GOOGLE_PLACES_API_KEY` environment variable. Ensure it is set before running your application.

```bash
export GOOGLE_PLACES_API_KEY=your_api_key_here
```

---

## Modules

### Place Search
- **Find Place Search**: Search for a place by text query or phone number.
- **Nearby Search**: Search for places near a specific location.
- **Text Search**: Search for places using a free-text query.

### Place Details
- Fetch detailed information about a specific place using its Place ID.

### Place Photos
- Retrieve photos of a place using a photo reference.

---

## Contributing

Contributions are welcome! Please follow the guidelines below:

1. Fork the repository.
2. Create a feature branch.
3. Commit your changes.
4. Open a pull request.

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---

## Support

For support or questions, please open an issue in the GitHub repository.

---

## Disclaimer

This library is not an official Google product and is not affiliated with or endorsed by Google.

