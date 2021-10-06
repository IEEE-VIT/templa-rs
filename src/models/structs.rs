use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Submodule {
    pub name: String,
    pub url: String,
    pub tags: Vec<String>,
}

impl Submodule {
    pub fn has_one_of_tags(&self, tags: &[&str]) -> bool {
        for own_t in self.tags.iter() {
            for t in tags.iter() {
                if own_t == t {
                    return true;
                }
            }
        }

        false
    }
}