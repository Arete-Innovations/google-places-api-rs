use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, Display, EnumString)]
pub enum PlaceTypes {
    //Table 1 lists the types that are supported for place searches,
    //and can be returned with Place details results, and as part of autocomplete place predictions.
    #[strum(serialize = "accounting")]
    Accounting,
    #[strum(serialize = "airport")]
    Airport,
    #[strum(serialize = "amusement_park")]
    AmusementPark,
    #[strum(serialize = "aquarium")]
    Aquarium,
    #[strum(serialize = "art_gallery")]
    ArtGallery,
    #[strum(serialize = "atm")]
    Atm,
    #[strum(serialize = "bakery")]
    Bakery,
    #[strum(serialize = "bank")]
    Bank,
    #[strum(serialize = "bar")]
    Bar,
    #[strum(serialize = "beauty_salon")]
    BeautySalon,
    #[strum(serialize = "bicycle_store")]
    BicycleStore,
    #[strum(serialize = "book_store")]
    BookStore,
    #[strum(serialize = "bowling_alley")]
    BowlingAlley,
    #[strum(serialize = "bus_station")]
    BusStation,
    #[strum(serialize = "cafe")]
    Cafe,
    #[strum(serialize = "campground")]
    Campground,
    #[strum(serialize = "car_dealer")]
    CarDealer,
    #[strum(serialize = "car_rental")]
    CarRental,
    #[strum(serialize = "car_repair")]
    CarRepair,
    #[strum(serialize = "car_wash")]
    CarWash,
    #[strum(serialize = "casino")]
    Casino,
    #[strum(serialize = "cemetery")]
    Cemetery,
    #[strum(serialize = "church")]
    Church,
    #[strum(serialize = "city_hall")]
    CityHall,
    #[strum(serialize = "clothing_store")]
    ClothingStore,
    #[strum(serialize = "convenience_store")]
    ConvenienceStore,
    #[strum(serialize = "courthouse")]
    Courthouse,
    #[strum(serialize = "dentist")]
    Dentist,
    #[strum(serialize = "department_store")]
    DepartmentStore,
    #[strum(serialize = "doctor")]
    Doctor,
    #[strum(serialize = "drugstore")]
    Drugstore,
    #[strum(serialize = "electrician")]
    Electrician,
    #[strum(serialize = "electronics_store")]
    ElectronicsStore,
    #[strum(serialize = "embassy")]
    Embassy,
    #[strum(serialize = "fire_station")]
    FireStation,
    #[strum(serialize = "florist")]
    Florist,
    #[strum(serialize = "funeral_home")]
    FuneralHome,
    #[strum(serialize = "furniture_store")]
    FurnitureStore,
    #[strum(serialize = "gas_station")]
    GasStation,
    #[strum(serialize = "gym")]
    Gym,
    #[strum(serialize = "hair_care")]
    HairCare,
    #[strum(serialize = "hardware_store")]
    HardwareStore,
    #[strum(serialize = "hindu_temple")]
    HinduTemple,
    #[strum(serialize = "home_goods_store")]
    HomeGoodsStore,
    #[strum(serialize = "hospital")]
    Hospital,
    #[strum(serialize = "insurance_agency")]
    InsuranceAgency,
    #[strum(serialize = "jewelry_store")]
    JewelryStore,
    #[strum(serialize = "laundry")]
    Laundry,
    #[strum(serialize = "lawyer")]
    Lawyer,
    #[strum(serialize = "library")]
    Library,
    #[strum(serialize = "light_rail_station")]
    LightRailStation,
    #[strum(serialize = "liquor_store")]
    LiquorStore,
    #[strum(serialize = "local_government_office")]
    LocalGovernmentOffice,
    #[strum(serialize = "locksmith")]
    Locksmith,
    #[strum(serialize = "lodging")]
    Lodging,
    #[strum(serialize = "meal_delivery")]
    MealDelivery,
    #[strum(serialize = "meal_takeaway")]
    MealTakeaway,
    #[strum(serialize = "mosque")]
    Mosque,
    #[strum(serialize = "movie_rental")]
    MovieRental,
    #[strum(serialize = "movie_theater")]
    MovieTheater,
    #[strum(serialize = "moving_company")]
    MovingCompany,
    #[strum(serialize = "museum")]
    Museum,
    #[strum(serialize = "night_club")]
    NightClub,
    #[strum(serialize = "painter")]
    Painter,
    #[strum(serialize = "park")]
    Park,
    #[strum(serialize = "parking")]
    Parking,
    #[strum(serialize = "pet_store")]
    PetStore,
    #[strum(serialize = "pharmacy")]
    Pharmacy,
    #[strum(serialize = "physiotherapist")]
    Physiotherapist,
    #[strum(serialize = "plumber")]
    Plumber,
    #[strum(serialize = "police")]
    Police,
    #[strum(serialize = "post_office")]
    PostOffice,
    #[strum(serialize = "primary_school")]
    PrimarySchool,
    #[strum(serialize = "real_estate_agency")]
    RealEstateAgency,
    #[strum(serialize = "restaurant")]
    Restaurant,
    #[strum(serialize = "roofing_contractor")]
    RoofingContractor,
    #[strum(serialize = "rv_park")]
    RvPark,
    #[strum(serialize = "school")]
    School,
    #[strum(serialize = "secondary_school")]
    SecondarySchool,
    #[strum(serialize = "shoe_store")]
    ShoeStore,
    #[strum(serialize = "shopping_mall")]
    ShoppingMall,
    #[strum(serialize = "spa")]
    Spa,
    #[strum(serialize = "stadium")]
    Stadium,
    #[strum(serialize = "storage")]
    Storage,
    #[strum(serialize = "store")]
    Store,
    #[strum(serialize = "subway_station")]
    SubwayStation,
    #[strum(serialize = "supermarket")]
    Supermarket,
    #[strum(serialize = "synagogue")]
    Synagogue,
    #[strum(serialize = "taxi_stand")]
    TaxiStand,
    #[strum(serialize = "tourist_attraction")]
    TouristAttraction,
    #[strum(serialize = "train_station")]
    TrainStation,
    #[strum(serialize = "transit_station")]
    TransitStation,
    #[strum(serialize = "travel_agency")]
    TravelAgency,
    #[strum(serialize = "university")]
    University,
    #[strum(serialize = "veterinary_care")]
    VeterinaryCare,
    #[strum(serialize = "zoo")]
    Zoo,

    //Table 2 lists additional types that can be returned with Place details results,
    //and as part of autocomplete place predictions.
    #[strum(serialize = "administrative_area_level_1")]
    AdministrativeAreaLevel1,
    #[strum(serialize = "administrative_area_level_2")]
    AdministrativeAreaLevel2,
    #[strum(serialize = "administrative_area_level_3")]
    AdministrativeAreaLevel3,
    #[strum(serialize = "administrative_area_level_4")]
    AdministrativeAreaLevel4,
    #[strum(serialize = "administrative_area_level_5")]
    AdministrativeAreaLevel5,
    #[strum(serialize = "administrative_area_level_6")]
    AdministrativeAreaLevel6,
    #[strum(serialize = "administrative_area_level_7")]
    AdministrativeAreaLevel7,
    #[strum(serialize = "archipelago")]
    Archipelago,
    #[strum(serialize = "colloquial_area")]
    ColloquialArea,
    #[strum(serialize = "continent")]
    Continent,
    #[strum(serialize = "country")]
    Country,
    #[strum(serialize = "establishment")]
    Establishment,
    #[strum(serialize = "finance")]
    Finance,
    #[strum(serialize = "floor")]
    Floor,
    #[strum(serialize = "food")]
    Food,
    #[strum(serialize = "general_contractor")]
    GeneralContractor,
    #[strum(serialize = "geocode")]
    Geocode,
    #[strum(serialize = "health")]
    Health,
    #[strum(serialize = "intersection")]
    Intersection,
    #[strum(serialize = "landmark")]
    Landmark,
    #[strum(serialize = "locality")]
    Locality,
    #[strum(serialize = "natural_feature")]
    NaturalFeature,
    #[strum(serialize = "neighborhood")]
    Neighborhood,
    #[strum(serialize = "place_of_worship")]
    PlaceOfWorship,
    #[strum(serialize = "plus_code")]
    PlusCode,
    #[strum(serialize = "point_of_interest")]
    PointOfInterest,
    #[strum(serialize = "political")]
    Political,
    #[strum(serialize = "post_box")]
    PostBox,
    #[strum(serialize = "postal_code")]
    PostalCode,
    #[strum(serialize = "postal_code_prefix")]
    PostalCodePrefix,
    #[strum(serialize = "postal_code_suffix")]
    PostalCodeSuffix,
    #[strum(serialize = "postal_town")]
    PostalTown,
    #[strum(serialize = "premise")]
    Premise,
    #[strum(serialize = "room")]
    Room,
    #[strum(serialize = "route")]
    Route,
    #[strum(serialize = "street_address")]
    StreetAddress,
    #[strum(serialize = "street_number")]
    StreetNumber,
    #[strum(serialize = "sublocality")]
    Sublocality,
    #[strum(serialize = "sublocality_level_1")]
    SublocalityLevel1,
    #[strum(serialize = "sublocality_level_2")]
    SublocalityLevel2,
    #[strum(serialize = "sublocality_level_3")]
    SublocalityLevel3,
    #[strum(serialize = "sublocality_level_4")]
    SublocalityLevel4,
    #[strum(serialize = "sublocality_level_5")]
    SublocalityLevel5,
    #[strum(serialize = "subpremise")]
    Subpremise,
    #[strum(serialize = "town_square")]
    TownSquare,

    //Table 3 lists types you can use in place autocomplete requests.
    #[strum(serialize = "address")]
    Address,
    #[strum(serialize = "(regions)")]
    Regions,
    #[strum(serialize = "(cities)")]
    Cities,
}

mod tests {
    use super::PlaceTypes;

    #[test]
    fn test_language_as_str() {
        assert_eq!(PlaceTypes::Accounting.to_string(), "accounting");
        assert_eq!(PlaceTypes::Airport.to_string(), "airport");
        assert_eq!(PlaceTypes::AmusementPark.to_string(), "amusement_park");
    }

    #[test]
    fn test_language_parse() {
        let parsed_result: PlaceTypes = "accounting".parse().unwrap();
        assert_eq!(parsed_result, PlaceTypes::Accounting);
        let parsed_result: PlaceTypes = "airport".parse().unwrap();
        assert_eq!(parsed_result, PlaceTypes::Airport);
        let parsed_result: PlaceTypes = "amusement_park".parse().unwrap();
        assert_eq!(parsed_result, PlaceTypes::AmusementPark);
    }
}
