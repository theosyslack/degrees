use serde::{Deserialize, Serialize};

use super::credit::Credit;

#[derive(Debug, Serialize, Deserialize)]
pub struct Credits {
    pub id: i32,
    pub cast: Vec<Credit>,
    pub crew: Vec<Credit>,
}

impl Credits {
    pub fn get_movies_ids(&self) -> Vec<String> {
        // TODO: It seems like it's possible to get duplicates for movies in the Credits. You should de-dupe here.
        let credits = &self.cast;
        let movie_ids = credits
            .iter()
            .map(|credit| format!("{}", credit.movie_id))
            .collect();

        movie_ids
    }
}
