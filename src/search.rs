use crate::models::enums;
use crate::models::structs;

pub fn perform_search(
    submodules: &[structs::Submodule],
    key: String,
) -> Result<Vec<structs::Submodule>, enums::Error> {
    let mut filtered_sm = vec![];
    for submodule in submodules.iter() {
        if submodule.tags.contains(&key) {
            filtered_sm.push(submodule.clone());
        }
    }
    Ok(filtered_sm)
}
