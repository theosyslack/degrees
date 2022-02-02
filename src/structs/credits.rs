use serde::{Deserialize, Serialize};

use crate::{api::get_movie, errors::Result};

use super::{credit::Credit, movie::Movie};

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
            .into_iter()
            .map(|credit| format!("{}", credit.movie_id))
            .collect();

        movie_ids
    }

    pub async fn get_movies(&self) -> Result<Vec<Movie>> {
        let mut movies: Vec<Movie> = vec![];
        let movie_ids = self.get_movies_ids();

        for id in &movie_ids {
            let movie = get_movie(id).await?;
            movies.push(movie)
        }

        Ok(movies)
    }
}
