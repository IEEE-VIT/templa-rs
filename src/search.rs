use crate::models::structs;
use crate::models::enums;

pub fn perform_search(submodules: Vec<structs::Submodule>, key: String) -> Result<Vec<structs::Submodule>, enums::Error> {
    let mut filtered_sm = vec![];
    for submodule in submodules.iter() {
        if submodule.tags.contains(&key) {
            filtered_sm.push(submodule.clone());
        }
    }
    Ok(filtered_sm)
}