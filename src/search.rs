use crate::models::enums;
use crate::models::structs;

pub fn perform_search(
    submodules: &[structs::Submodule],
    key: &str
) -> Result<Vec<structs::Submodule>, enums::Error> {
    let mut filtered_sm = vec![];

    for submodule in submodules.iter() {
        if key.is_empty() || submodule.has_tag(key) {
            filtered_sm.push(submodule.clone());
        }
    }
    Ok(filtered_sm)
}
