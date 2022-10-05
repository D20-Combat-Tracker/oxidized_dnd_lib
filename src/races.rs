use serde::{Serialize, Deserialize};
use crate::AbilitySheet;

/// Represents the information for a given race.
#[derive(Serialize, Deserialize)]
pub struct Race {
    ability_bonuses: AbilitySheet,
    age_of_maturity: u16,
    expected_lifespan: u16,
    speed: u8,
    languages: Vec<String>,
    traits: Vec<String>,
    description: String,
    subrace: Option<SubRace>,
}

/// This struct describes the additional bonuses and traits conferred from a subrace
#[derive(Serialize, Deserialize)]
pub struct SubRace {
    ability_bonuses: AbilitySheet,
    traits: Vec<String>,
    description: String,
}

