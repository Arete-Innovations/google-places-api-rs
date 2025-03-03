use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Display, EnumString)]
pub enum PlaceDetailsPlaceFields {
    // Basic Data
    #[strum(serialize = "address_components")]
    AddressComponents,
    #[strum(serialize = "adr_address")]
    AdrAddress,
    #[strum(serialize = "business_status")]
    BusinessStatus,
    #[strum(serialize = "formatted_address")]
    FormattedAddress,
    #[strum(serialize = "geometry/viewport")]
    Viewport,
    #[strum(serialize = "geometry/location")]
    Location,
    #[strum(serialize = "icon")]
    Icon,
    #[strum(serialize = "icon_mask_base_uri")]
    IconMaskBaseUri,
    #[strum(serialize = "icon_background_color")]
    IconBackgroundColor,
    #[strum(serialize = "name")]
    Name,
    // #[strum(serialize = "permanently_closed")]
    // PermanentlyClosed, // deprecated
    #[strum(serialize = "photos")]
    Photo,
    #[strum(serialize = "place_id")]
    PlaceId,
    #[strum(serialize = "plus_code")]
    PlusCode,
    #[strum(serialize = "type")]
    Type,
    #[strum(serialize = "url")]
    Url,
    #[strum(serialize = "utc_offset")]
    UtcOffset,
    #[strum(serialize = "vicinity")]
    Vicinity,
    #[strum(serialize = "wheelchair_accessible_entrance")]
    WheelchairAccessibleEntrance,

    // Contact Data
    #[strum(serialize = "formatted_phone_number")]
    FormattedPhoneNumber,
    #[strum(serialize = "international_phone_number")]
    InternationalPhoneNumber,
    #[strum(serialize = "opening_hours")]
    OpeningHours,
    #[strum(serialize = "current_opening_hours")]
    CurrentOpeningHours,
    #[strum(serialize = "secondary_opening_hours")]
    SecondaryOpeningHours,
    #[strum(serialize = "website")]
    Website,

    // Atmosphere Data
    #[strum(serialize = "curbside_pickup")]
    CurbsidePickup,
    #[strum(serialize = "delivery")]
    Delivery,
    #[strum(serialize = "dine_in")]
    DineIn,
    #[strum(serialize = "editorial_summary")]
    EditorialSummary,
    #[strum(serialize = "price_level")]
    PriceLevel,
    #[strum(serialize = "rating")]
    Rating,
    #[strum(serialize = "reservable")]
    Reservable,
    #[strum(serialize = "reviews")]
    Reviews,
    #[strum(serialize = "serves_beer")]
    ServesBeer,
    #[strum(serialize = "serves_breakfast")]
    ServesBreakfast,
    #[strum(serialize = "serves_brunch")]
    ServesBrunch,
    #[strum(serialize = "serves_dinner")]
    ServesDinner,
    #[strum(serialize = "serves_lunch")]
    ServesLunch,
    #[strum(serialize = "serves_vegetarian_food")]
    ServesVegetarianFood,
    #[strum(serialize = "serves_wine")]
    ServesWine,
    #[strum(serialize = "takeout")]
    Takeout,
    #[strum(serialize = "user_ratings_total")]
    UserRatingsTotal,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Display, EnumString)]
pub enum PlaceSearchPlaceFields {
    // Basic Data
    #[strum(serialize = "business_status")]
    BusinessStatus,
    #[strum(serialize = "formatted_address")]
    FormattedAddress,
    #[strum(serialize = "geometry/viewport")]
    Viewport,
    #[strum(serialize = "geometry/location")]
    Location,
    #[strum(serialize = "icon")]
    Icon,
    #[strum(serialize = "icon_mask_base_uri")]
    IconMaskBaseUri,
    #[strum(serialize = "icon_background_color")]
    IconBackgroundColor,
    #[strum(serialize = "name")]
    Name,
    #[strum(serialize = "photos")]
    Photo,
    #[strum(serialize = "place_id")]
    PlaceId,
    #[strum(serialize = "plus_code")]
    PlusCode,
    #[strum(serialize = "type")]
    Type,
    #[strum(serialize = "vicinity")]
    Vicinity,

    // Contact
    #[strum(serialize = "opening_hours")]
    OpeningHours,

    // Atmosphere Data
    #[strum(serialize = "price_level")]
    PriceLevel,
    #[strum(serialize = "rating")]
    Rating,
    #[strum(serialize = "user_ratings_total")]
    UserRatingsTotal,
}

mod tests {
    use crate::types::constants::{PlaceDetailsPlaceFields, PlaceSearchPlaceFields};

    #[test]
    fn test_place_details_fields_as_str() {
        assert_eq!(
            PlaceDetailsPlaceFields::AddressComponents.to_string(),
            "address_components"
        );
        assert_eq!(
            PlaceDetailsPlaceFields::BusinessStatus.to_string(),
            "business_status"
        );
        assert_eq!(
            PlaceDetailsPlaceFields::FormattedAddress.to_string(),
            "formatted_address"
        );
    }

    #[test]
    fn test_place_details_fields_parse() {
        let parsed_result: PlaceDetailsPlaceFields = "address_components".parse().unwrap();
        assert_eq!(parsed_result, PlaceDetailsPlaceFields::AddressComponents);
        let parsed_result: PlaceDetailsPlaceFields = "business_status".parse().unwrap();
        assert_eq!(parsed_result, PlaceDetailsPlaceFields::BusinessStatus);
        let parsed_result: PlaceDetailsPlaceFields = "formatted_address".parse().unwrap();
        assert_eq!(parsed_result, PlaceDetailsPlaceFields::FormattedAddress);
    }

    #[test]
    fn test_place_search_fields_as_str() {
        assert_eq!(
            PlaceSearchPlaceFields::FormattedAddress.to_string(),
            "formatted_address"
        );
        assert_eq!(
            PlaceSearchPlaceFields::BusinessStatus.to_string(),
            "business_status"
        );
        assert_eq!(
            PlaceSearchPlaceFields::FormattedAddress.to_string(),
            "formatted_address"
        );
    }

    #[test]
    fn test_place_search_fields_parse() {
        let parsed_result: PlaceSearchPlaceFields = "formatted_address".parse().unwrap();
        assert_eq!(parsed_result, PlaceSearchPlaceFields::FormattedAddress);
        let parsed_result: PlaceSearchPlaceFields = "business_status".parse().unwrap();
        assert_eq!(parsed_result, PlaceSearchPlaceFields::BusinessStatus);
        let parsed_result: PlaceSearchPlaceFields = "formatted_address".parse().unwrap();
        assert_eq!(parsed_result, PlaceSearchPlaceFields::FormattedAddress);
    }
}
