use serde::{Serialize, Deserialize};

/// An enum for DnD languages
#[derive(Debug, Serialize, Deserialize)]
pub enum Languages {
    Common,
    Dwarvish,
    Elvish,
    Giant,
    Gnomish,
    Goblin,
    Halfling,
    Orc,
    Abyssal,
    Celestial,
    Draconic,
    DeepSpeech,
    Infernal,
    Primordial,
    Sylvan,
    Undercommon,
}
impl Languages {
    pub fn get_script(&self) -> Script {
        match self {
            Languages::Common => Script::Common,
            Languages::Dwarvish => Script::Dwarvish,
            Languages::Elvish => Script::Elvish,
            Languages::Giant => Script::Dwarvish,
            Languages::Gnomish => Script::Dwarvish,
            Languages::Goblin => Script::Dwarvish,
            Languages::Halfling => Script::Common,
            Languages::Orc => Script::Dwarvish,
            Languages::Abyssal => Script::Infernal,
            Languages::Celestial => Script::Celestial,
            Languages::Draconic => Script::Draconic,
            Languages::DeepSpeech => Script::None,
            Languages::Infernal => Script::Infernal,
            Languages::Primordial => Script::Dwarvish,
            Languages::Sylvan => Script::Elvish,
            Languages::Undercommon => Script::Elvish,
        }
    }
}
pub enum Script {
    Common,
    Dwarvish,
    Elvish,
    Infernal,
    Celestial,
    Draconic,
    None
}
