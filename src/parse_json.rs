use std::fs;
use crate::models::enums;
use crate::models::structs;

const JSON_PATH: &str = "./submodules.json";

pub fn read_json() -> Result<Vec<structs::Submodule>, enums::Error> {
    let jsonfile = fs::read_to_string(JSON_PATH)?;
    let parsed: Vec<structs::Submodule> = serde_json::from_str(&jsonfile)?;
    Ok(parsed)
}