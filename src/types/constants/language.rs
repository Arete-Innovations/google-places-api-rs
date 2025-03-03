use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Display, EnumString)]
pub enum Language {
    #[strum(serialize = "af")]
    Af,
    #[strum(serialize = "am")]
    Am,
    #[strum(serialize = "ar")]
    Ar,
    #[strum(serialize = "hy")]
    Hy,
    #[strum(serialize = "az")]
    Az,
    #[strum(serialize = "eu")]
    Eu,
    #[strum(serialize = "be")]
    Be,
    #[strum(serialize = "bn")]
    Bn,
    #[strum(serialize = "bs")]
    Bs,
    #[strum(serialize = "bg")]
    Bg,
    #[strum(serialize = "my")]
    My,
    #[strum(serialize = "ca")]
    Ca,
    #[strum(serialize = "zh")]
    Zh,
    #[strum(serialize = "zh-CN")]
    ZhCN,
    #[strum(serialize = "zh-HK")]
    ZhHK,
    #[strum(serialize = "zh-TW")]
    ZhTW,
    #[strum(serialize = "hr")]
    Hr,
    #[strum(serialize = "cs")]
    Cs,
    #[strum(serialize = "da")]
    Da,
    #[strum(serialize = "nl")]
    Nl,
    #[strum(serialize = "en")]
    En,
    #[strum(serialize = "en-AU")]
    EnAU,
    #[strum(serialize = "en-GB")]
    EnGB,
    #[strum(serialize = "et")]
    Et,
    #[strum(serialize = "fa")]
    Fa,
    #[strum(serialize = "fi")]
    Fi,
    #[strum(serialize = "fil")]
    Fil,
    #[strum(serialize = "fr")]
    Fr,
    #[strum(serialize = "fr-CA")]
    FrCA,
    #[strum(serialize = "gl")]
    Gl,
    #[strum(serialize = "ka")]
    Ka,
    #[strum(serialize = "de")]
    De,
    #[strum(serialize = "el")]
    El,
    #[strum(serialize = "gu")]
    Gu,
    #[strum(serialize = "iw")]
    Iw,
    #[strum(serialize = "hi")]
    Hi,
    #[strum(serialize = "hu")]
    Hu,
    #[strum(serialize = "is")]
    Is,
    #[strum(serialize = "id")]
    Id,
    #[strum(serialize = "it")]
    It,
    #[strum(serialize = "ja")]
    Ja,
    #[strum(serialize = "kn")]
    Kn,
    #[strum(serialize = "kk")]
    Kk,
    #[strum(serialize = "km")]
    Km,
    #[strum(serialize = "ko")]
    Ko,
    #[strum(serialize = "ky")]
    Ky,
    #[strum(serialize = "lo")]
    Lo,
    #[strum(serialize = "lv")]
    Lv,
    #[strum(serialize = "lt")]
    Lt,
    #[strum(serialize = "mk")]
    Mk,
    #[strum(serialize = "ms")]
    Ms,
    #[strum(serialize = "ml")]
    Ml,
    #[strum(serialize = "mr")]
    Mr,
    #[strum(serialize = "mn")]
    Mn,
    #[strum(serialize = "ne")]
    Ne,
    #[strum(serialize = "no")]
    No,
    #[strum(serialize = "pl")]
    Pl,
    #[strum(serialize = "pt")]
    Pt,
    #[strum(serialize = "pt-BR")]
    PtBR,
    #[strum(serialize = "pt-PT")]
    PtPT,
    #[strum(serialize = "pa")]
    Pa,
    #[strum(serialize = "ro")]
    Ro,
    #[strum(serialize = "ru")]
    Ru,
    #[strum(serialize = "sr")]
    Sr,
    #[strum(serialize = "si")]
    Si,
    #[strum(serialize = "sk")]
    Sk,
    #[strum(serialize = "sl")]
    Sl,
    #[strum(serialize = "es")]
    Es,
    #[strum(serialize = "es-419")]
    Es419,
    #[strum(serialize = "sw")]
    Sw,
    #[strum(serialize = "sv")]
    Sv,
    #[strum(serialize = "ta")]
    Ta,
    #[strum(serialize = "te")]
    Te,
    #[strum(serialize = "th")]
    Th,
    #[strum(serialize = "tr")]
    Tr,
    #[strum(serialize = "uk")]
    Uk,
    #[strum(serialize = "ur")]
    Ur,
    #[strum(serialize = "uz")]
    Uz,
    #[strum(serialize = "vi")]
    Vi,
    #[strum(serialize = "zu")]
    Zu,
}

#[cfg(test)]
mod tests {
    use super::Language;

    #[test]
    fn test_language_as_str() {
        assert_eq!(Language::En.to_string(), "en");
        assert_eq!(Language::FrCA.to_string(), "fr-CA");
        assert_eq!(Language::ZhHK.to_string(), "zh-HK");
    }

    #[test]
    fn test_language_parse() {
        let parsed_result: Language = "en".parse().unwrap();
        assert_eq!(parsed_result, Language::En);
        let parsed_result: Language = "fr-CA".parse().unwrap();
        assert_eq!(parsed_result, Language::FrCA);
        let parsed_result: Language = "zh-HK".parse().unwrap();
        assert_eq!(parsed_result, Language::ZhHK);
    }
}
