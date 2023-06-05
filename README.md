# Google Places API

# Working Examples
* `cargo run --example nearby_search`
* `cargo run --example place_details`
* `cargo run --example find_place`
* `cargo run --example place_photos`

# Contributing Instructions
* Add `"rust-analyzer.diagnostics.disabled": ["unresolved-import"]` to your settings.json file to disable proc macro warnings for serde.

# Helpful Commands


# Repo Structure

    ├── src
    │   ├── lib.rs
    │   ├── error.rs
    │   ├── client.rs
    │   ├── models
    │   │   ├── place_details.rs
    │   │   └── ...
    │   ├── services
    │   │   ├── places.rs
    │   │   └── ...
    │   └── utils
    │       ├── request.rs
    │       └── ...
    ├── tests
    │   ├── integration.rs
    │   └── unit
    │       ├── client.rs
    │       ├── services
    │       │   ├── places.rs
    │       │   └── ...
    │       └── ...
    ├── examples
    │   ├── main.rs
    │   ├── basic_search.rs
    │   └── ...
    ├── Cargo.toml
    └── README.md
