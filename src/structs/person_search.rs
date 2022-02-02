use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    api::get_person,
    errors::{Error, Kind, Result},
};

use super::person::Person;

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonSearch {
    pub total_pages: i32,
    pub page: i32,
    pub results: Vec<PersonSearchResult>,
}

impl PersonSearch {
    pub fn get_first_result(&self) -> Result<PersonSearchResult> {
        if self.results.len() == 0 {
            Err(Error::from_kind(Kind::PersonSearchNoResults))
        } else {
            Ok(self.results[0].clone())
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PersonSearchResult {
    pub id: i32,
    pub name: String,
    pub known_for: Vec<MovieSearchResult>,
}

impl PersonSearchResult {
    pub async fn get_details(&self) -> Result<Person> {
        let id = format!("{}", &self.id);
        get_person(&id).await
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MovieSearchResult {
    pub id: i32,
    pub title: String,
}

impl Display for MovieSearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}
