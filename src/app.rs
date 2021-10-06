use std::vec::Vec;
use crate::models::structs::Submodule;
use crate::search::perform_search;

pub struct App {
    pub submodules: Vec<Submodule>,
    pub filtered_submodules: Vec<Submodule>,
    pub last_query: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        App {
            filtered_submodules: vec![],
            submodules: vec![],

            last_query: None,
        }
    }
}

impl App {
    pub fn new(submodules: Vec<Submodule>) -> Self {
        App {
            submodules,
            ..App::default()
        }
    }

    // Performs search on submodules and updates app's internal
    // cached search. Only performs the search if it's needed.
    pub fn search(&mut self, query: &str) {
        // TODO: Process query to extract tags and name searches
        if self.last_query.is_none() || query != self.last_query.as_deref().unwrap() {

            self.filtered_submodules = perform_search(&self.submodules, query).expect("can search through submodules"); 

            self.last_query = Some(query.to_string());
        }
    }
}