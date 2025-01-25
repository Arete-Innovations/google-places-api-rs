use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Display, EnumString)]
pub enum InputType {
    #[strum(serialize = "textquery")]
    TextQuery,
    #[strum(serialize = "phonenumber")]
    PhoneNumber,
}

mod tests {
    use crate::models::constants::InputType;

    #[test]
    fn test_rankby_as_str() {
        assert_eq!(InputType::TextQuery.to_string(), "textquery");
        assert_eq!(InputType::PhoneNumber.to_string(), "phonenumber");
    }

    #[test]
    fn test_rankby_parse() {
        let parsed_result: InputType = "textquery".parse().unwrap();
        assert_eq!(parsed_result, InputType::TextQuery);
        let parsed_result: InputType = "phonenumber".parse().unwrap();
        assert_eq!(parsed_result, InputType::PhoneNumber);
    }
}
