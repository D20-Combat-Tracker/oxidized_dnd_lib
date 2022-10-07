use serde::{Serialize, Deserialize};
use crate::dice::Dice;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassStats {
    hit_dice: (Dice, u8),
    starting_hit_points: u16,
    hit_points_per_level_set: u16,
    hit_points_per_level_roll: (Dice, u8),
    proficiencies: Vec<String>,
}
