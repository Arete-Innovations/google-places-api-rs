pub mod types;
pub mod endpoints;

mod tests;

/**
The library provides a comprehensive Rust interface for interacting with the Google Places API.
It supports various functionalities such as text search, nearby search, place details retrieval,
and find place operations. Users can query places based on different criteria like location, types,
and input types, while obtaining detailed information such as business status, address,
and user ratings. The API is designed with asynchronous support to handle network operations
efficiently, and includes serialization and deserialization capabilities for seamless data
exchange. This library is ideal for developers looking to integrate Google Places services
into their Rust applications.

*/

pub use endpoints::api::GooglePlacesAPI;
