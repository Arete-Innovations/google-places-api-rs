use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Display, EnumString)]
pub enum ReviewSort {
    #[strum(serialize = "most_relevant")]
    MostRelevant,
    #[strum(serialize = "newest")]
    Newest,
}

mod tests {
    use super::ReviewSort;

    #[test]
    fn test_rankby_as_str() {
        assert_eq!(ReviewSort::MostRelevant.to_string(), "most_relevant");
        assert_eq!(ReviewSort::Newest.to_string(), "newest");
    }

    #[test]
    fn test_rankby_parse() {
        let parsed_result: ReviewSort = "most_relevant".parse().unwrap();
        assert_eq!(parsed_result, ReviewSort::MostRelevant);
        let parsed_result: ReviewSort = "newest".parse().unwrap();
        assert_eq!(parsed_result, ReviewSort::Newest);
    }
}
