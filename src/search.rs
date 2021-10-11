use crate::models::enums;
use crate::models::structs;

pub fn perform_search(
    submodules: &[structs::Submodule],
    name_query: &str,
    tags: &[&str]
) -> Result<Vec<structs::Submodule>, enums::Error> {
    let mut filtered_sm = vec![];
    let lowercase_query = name_query.to_lowercase();

    let x = submodules.len();

    let mut i = 0;

    while i < x {
        if (tags.is_empty() || submodules[i].has_one_of_tags(tags)) && 
            (name_query.is_empty() || submodules[i].name.to_lowercase().contains(&lowercase_query)) {
                filtered_sm.push(submodules[i].clone());
        } 

        i+=1;
    }

    Ok(filtered_sm)
}
