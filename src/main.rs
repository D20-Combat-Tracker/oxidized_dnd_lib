use std::fs::File;
use thiserror::Error;

use oxidized_dnd_lib::races::{Race, SubRace};

#[derive(Error, Debug)]
enum Errors {
    #[error("Error serializing to yaml")]
    Yaml(#[from] serde_yaml::Error),
    #[error("Error creating file")]
    File(#[from] std::io::Error),
    #[error("Error writing to json")]
    Json(#[from] serde_json::Error)
}

fn main() -> Result<(), Errors> {
    let race: Race = Default::default();
    let file = File::create("race.json")?;
    serde_json::to_writer(file, &race)?;
    Ok(())
}
