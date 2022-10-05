use std::fs::File;
use thiserror::Error;

use oxidized_dnd_lib::races::Race;

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
    // let race: Race = Default::default();
    // let json = File::create("race.json")?;
    // let yaml = File::create("race.yaml")?;
    // serde_yaml::to_writer(yaml, &race)?;
    // serde_json::to_writer(json, &race)?;
    let yaml = File::open("race1.yaml")?;
    let race: Race = serde_yaml::from_reader(yaml)?;
    println!("{:?}", race);
    Ok(())
}
