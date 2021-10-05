use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Submodule {
    pub name: String,
    pub url: String,
    pub tags: Vec<String>,
}

impl Submodule {
    pub fn has_tag(&self, tag: &str) -> bool {
        for t in self.tags.iter() {
            if t == tag {
                return true;
            }
        }

        false
    }
}