use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Display, EnumString)]
pub enum RankBy {
    #[strum(serialize = "prominence")]
    Prominence,
    #[strum(serialize = "distance")]
    Distance,
}

mod tests {
    use crate::types::constants::RankBy;

    #[test]
    fn test_rankby_as_str() {
        assert_eq!(RankBy::Prominence.to_string(), "prominence");
        assert_eq!(RankBy::Distance.to_string(), "distance");
    }

    #[test]
    fn test_rankby_parse() {
        let parsed_result: RankBy = "prominence".parse().unwrap();
        assert_eq!(parsed_result, RankBy::Prominence);
        let parsed_result: RankBy = "distance".parse().unwrap();
        assert_eq!(parsed_result, RankBy::Distance);
    }
}
