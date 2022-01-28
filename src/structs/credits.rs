use serde::{Deserialize, Serialize};

use crate::api::get_movie;

use super::{credit::Credit, movie::Movie};

#[derive(Debug, Serialize, Deserialize)]
pub struct Credits {
    pub id: i32,
    pub cast: Vec<Credit>,
    pub crew: Vec<Credit>
}

impl Credits {
    pub fn get_movies_ids (&self) -> Vec<String> {
        let credits = &self.cast;
        let movie_ids = credits.into_iter().map(|credit| format!("{}", credit.movie_id) ).collect();

        movie_ids
    }

    pub async fn get_movies (&self) -> Vec<Movie> {
        let mut movies: Vec<Movie> = vec![];
        let movie_ids = self.get_movies_ids();

        for id in &movie_ids {
            let movie = get_movie(id).await;

            if let Some(movie) = movie {
                movies.push(movie)
            }
        }
        
        movies
    }
}