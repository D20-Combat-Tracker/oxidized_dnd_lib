use serde::{Serialize, Deserialize};

use crate::{skills::Skills, languages::Languages};

#[derive(Debug, Serialize, Deserialize)]
pub struct Background {
    skill_proficiencies: Vec<Skills>,
    languages: Vec<Languages>,
    equipment: Vec<String>,
    suggested_personalities: Vec<String>,
    suggested_ideals: Vec<String>,
    suggested_bonds: Vec<String>,
    suggested_flaws: Vec<String>,
}
