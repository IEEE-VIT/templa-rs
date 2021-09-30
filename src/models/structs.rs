use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Submodule {
    pub name: String,
    pub url: String,
    pub tags: Vec<String>,
}
