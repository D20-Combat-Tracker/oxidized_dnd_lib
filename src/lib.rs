use std::ops::{Add, AddAssign};

use serde::{Serialize, Deserialize};
mod error;
pub mod races;

/// This trait defines some common behaviour for creatures
/// (creatures include players and npcs and anything that can fight)
pub trait Creature {
    /// Gets the creature's ability score for a given ability type
    fn get_ability_score(&self, ability_type: Ability) -> u8;
    /// Gets the size of the creature
    fn get_size(&self) -> Size;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Size {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AbilitySheet {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
}
impl Add for AbilitySheet {
    type Output = AbilitySheet;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            strength: self.strength + rhs.strength,
            dexterity: self.dexterity + rhs.dexterity,
            constitution: self.constitution + rhs.constitution,
            intelligence: self.intelligence + rhs.intelligence,
            wisdom: self.wisdom + rhs.wisdom,
            charisma: self.charisma + rhs.charisma
        }
    }
}
impl AddAssign for AbilitySheet {
    fn add_assign(&mut self, rhs: Self) {
        self.strength = self.strength + rhs.strength;
        self.dexterity = self.dexterity + rhs.dexterity;
        self.constitution = self.constitution + rhs.constitution;
        self.intelligence = self.intelligence + rhs.intelligence;
        self.wisdom = self.wisdom + rhs.wisdom;
        self.charisma = self.charisma + rhs.charisma;
    }
}

impl AbilitySheet {
    pub fn new(strength: u8, dexterity: u8, constitution: u8, intelligence: u8, wisdom: u8, charisma: u8) -> Self { 
        Self { strength, dexterity, constitution, intelligence, wisdom, charisma } 
    }
}

pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma
}

/// Gets the modifier from an ability score.
/// The returned modifier will be between -5 and 10.
pub fn get_modifier(ability_score: u8) -> i8 {
    // can always cast ability_score as an i8 since a u8 divided by 2 will fit into i8
    (ability_score / 2) as i8 - 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_modifier() {
        assert_eq!(get_modifier(1), -5);
        assert_eq!(get_modifier(20), 5);
        assert_eq!(get_modifier(13), 1);
    }
}
