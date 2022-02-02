use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    api::{get_movie, get_person_credits},
    errors::Result,
};

use super::{credits::Credits, movie::Movie};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub profile_path: String,
    pub imdb_id: String,
    pub homepage: Option<String>,
    pub biography: String,
}

impl Person {
    pub async fn get_credits(&self) -> Result<Credits> {
        let id_str = format!("{}", self.id);
        get_person_credits(&id_str).await
    }

    pub async fn in_same_movie(&self, other_person: &Person) -> Result<bool> {
        let credits = self.get_credits().await?;
        let other_person_credits = other_person.get_credits().await?;

        let movie_ids = credits.get_movies_ids();
        let other_movie_ids = other_person_credits.get_movies_ids();

        for id in movie_ids {
            if other_movie_ids.contains(&id) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub async fn get_shared_movies(&self, other_person: &Person) -> Result<Vec<Movie>> {
        let mut movies: Vec<Movie> = vec![];
        let credits = self.get_credits().await?;
        let other_person_credits = other_person.get_credits().await?;

        let movie_ids = credits.get_movies_ids();
        let other_movie_ids = other_person_credits.get_movies_ids();

        for id in movie_ids {
            if other_movie_ids.contains(&id) {
                let movie = get_movie(&id).await?;
                movies.push(movie)
            }
        }

        Ok(movies)
    }

    pub fn imdb_url(&self) -> String {
        format!("https://www.imdb.com/name/{}", &self.imdb_id)
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string_pretty(&self).expect("Could not Display Person");
        write!(f, "{}", json)
    }
}
