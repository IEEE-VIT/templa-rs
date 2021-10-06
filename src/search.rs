use crate::models::enums;
use crate::models::structs;

pub fn perform_search(
    submodules: &[structs::Submodule],
    name_query: &str,
    tags: &[&str]
) -> Result<Vec<structs::Submodule>, enums::Error> {
    let mut filtered_sm = vec![];
    let lowercase_query = name_query.to_lowercase();

    for submodule in submodules.iter() {
        if (tags.is_empty() || submodule.has_one_of_tags(tags)) && 
            (name_query.is_empty() || submodule.name.to_lowercase().contains(&lowercase_query)) {
            filtered_sm.push(submodule.clone());
        }
    }
    Ok(filtered_sm)
}
