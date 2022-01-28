use serde::{Deserialize, Serialize};

use super::person::Person;

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonSearch {
    pub total_pages: i32,
    pub page: i32,
    pub results: Vec<Person>,
}

impl PersonSearch {
    pub fn get_first_result(&self) -> Option<Person> {
        let first_result = self.results.get(0);

        if let Some(first_result) = first_result {
            Some(first_result.clone())
        } else {
            None
        }
    }
}
