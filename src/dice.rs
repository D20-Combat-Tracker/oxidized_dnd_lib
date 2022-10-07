use serde::{Serialize, Deserialize};

/// Enum representing the different kinds of dice
#[derive(Debug, Serialize, Deserialize)]
pub enum Dice {
    D6,
    D8,
    D12,
    D20,
}
