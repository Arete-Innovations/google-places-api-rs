use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum PlaceDataField {
    // Basic Data
    AddressComponents,
    BusinessStatus,
    FormattedAddress,
    Viewport,
    Location,
    Icon,
    IconMaskBaseUri,
    IconBackgroundColor,
    Name,
    PermanentlyClosed,
    Photo,
    PlaceId,
    PlusCode,
    Type,
    Url,
    UtcOffset,
    Vicinity,
    WheelchairAccessibleEntrance,

    // Contact Data
    PhoneNumber,
    InternationalPhoneNumber,
    OpeningHours,
    CurrentOpeningHours,
    SecondaryOpeningHours,
    Website,

    // Atmosphere Data
    CurbsidePickup,
    Delivery,
    DineIn,
    EditorialSummary,
    PriceLevel,
    Rating,
    Reservable,
    Reviews,
    ServesBeer,
    ServesBreakfast,
    ServesBrunch,
    ServesDinner,
    ServesLunch,
    ServesVegetarianFood,
    ServesWine,
    Takeout,
    UserRatingsTotal,
}

impl PlaceDataField {
    pub fn as_str(&self) -> &'static str {
        match self {
            PlaceDataField::AddressComponents => "address_components",
            PlaceDataField::BusinessStatus => "business_status",
            PlaceDataField::FormattedAddress => "formatted_address",
            PlaceDataField::Viewport => "geometry/viewport",
            PlaceDataField::Location => "geometry/location",
            PlaceDataField::Icon => "icon",
            PlaceDataField::IconMaskBaseUri => "icon_mask_base_uri",
            PlaceDataField::IconBackgroundColor => "icon_background_color",
            PlaceDataField::Name => "name",
            PlaceDataField::PermanentlyClosed => "permanently_closed",
            PlaceDataField::Photo => "photos",
            PlaceDataField::PlaceId => "place_id",
            PlaceDataField::PlusCode => "plus_code",
            PlaceDataField::Type => "type",
            PlaceDataField::Url => "url",
            PlaceDataField::UtcOffset => "utc_offset",
            PlaceDataField::Vicinity => "vicinity",
            PlaceDataField::WheelchairAccessibleEntrance => "wheelchair_accessible_entrance",
            PlaceDataField::PhoneNumber => "formatted_phone_number",
            PlaceDataField::InternationalPhoneNumber => "international_phone_number",
            PlaceDataField::OpeningHours => "opening_hours",
            PlaceDataField::CurrentOpeningHours => "current_opening_hours",
            PlaceDataField::SecondaryOpeningHours => "secondary_opening_hours",
            PlaceDataField::Website => "website",
            PlaceDataField::CurbsidePickup => "curbside_pickup",
            PlaceDataField::Delivery => "delivery",
            PlaceDataField::DineIn => "dine_in",
            PlaceDataField::EditorialSummary => "editorial_summary",
            PlaceDataField::PriceLevel => "price_level",
            PlaceDataField::Rating => "rating",
            PlaceDataField::Reservable => "reservable",
            PlaceDataField::Reviews => "reviews",
            PlaceDataField::ServesBeer => "serves_beer",
            PlaceDataField::ServesBreakfast => "serves_breakfast",
            PlaceDataField::ServesBrunch => "serves_brunch",
            PlaceDataField::ServesDinner => "serves_dinner",
            PlaceDataField::ServesLunch => "serves_lunch",
            PlaceDataField::ServesVegetarianFood => "serves_vegetarian_food",
            PlaceDataField::ServesWine => "serves_wine",
            PlaceDataField::Takeout => "takeout",
            PlaceDataField::UserRatingsTotal => "user_ratings_total",
        }
    }
}

