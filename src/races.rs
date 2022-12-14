use serde::{Serialize, Deserialize};
use crate::{AbilitySheet, Size, languages::Languages};

/// Represents the information for a given race.
#[derive(Serialize, Deserialize, Debug)]
pub struct Race {
    name: String,
    ability_bonuses: AbilitySheet,
    age_of_maturity: u16,
    expected_lifespan: u16,
    speed: u8,
    size: Size,
    languages: Vec<Languages>,
    features: Vec<String>,
    description: String,
    subrace: Option<SubRace>,
}

impl Default for Race {
    fn default() -> Self {
        Self {
            name: String::from("Milkshake"),
            ability_bonuses: Default::default(),
            age_of_maturity: 30,
            expected_lifespan: 90,
            speed: 30,
            size: Size::Small,
            languages: vec![Languages::Common, Languages::Infernal],
            features: vec![String::from("stoneform"), String::from("whistling")],
            description: String::from("What a race this is. It is truly a fantasy race of fantastical proportions."),
            subrace: Some(Default::default())
        }
    }
}

/// This struct describes the additional bonuses and traits conferred from a subrace
#[derive(Serialize, Deserialize, Debug)]
pub struct SubRace {
    name: String,
    ability_bonuses: AbilitySheet,
    traits: Vec<String>,
    description: String,
}
impl Default for SubRace {
    fn default() -> Self {
        Self {
            name: String::from("Vanilla"),
            ability_bonuses: Default::default(),
            traits: vec![String::from("why though")],
            description: String::from("a little guy")
        }
    }
}

