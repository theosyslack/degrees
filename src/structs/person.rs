use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::api::{get_movie, get_person_credits};

use super::{credits::Credits, movie::Movie};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub profile_path: Option<String>,
}

impl Person {
    pub async fn get_credits(&self) -> Option<Credits> {
        let id_str = format!("{}", self.id);
        get_person_credits(&id_str).await
    }

    pub async fn in_same_movie(&self, other_person: &Person) -> bool {
        let credits = self.get_credits().await;
        let other_person_credits = other_person.get_credits().await;

        if let Some(credits) = credits {
            if let Some(other_person_credits) = other_person_credits {
                let movie_ids = credits.get_movies_ids();
                let other_movie_ids = other_person_credits.get_movies_ids();

                for id in movie_ids {
                    if other_movie_ids.contains(&id) {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub async fn get_same_movie_ids(&self, other_person: &Person) -> Vec<String> {
        let mut same_movie_ids: Vec<String> = vec![];

        let credits = self.get_credits().await;
        let other_person_credits = other_person.get_credits().await;

        if let Some(credits) = credits {
            if let Some(other_person_credits) = other_person_credits {
                let movie_ids = credits.get_movies_ids();
                let other_movie_ids = other_person_credits.get_movies_ids();

                for id in movie_ids {
                    if other_movie_ids.contains(&id) {
                        same_movie_ids.push(id)
                    }
                }
            }
        }

        same_movie_ids
    }

    pub async fn get_shared_movies(&self, other_person: &Person) -> Vec<Movie> {
        let mut movies: Vec<Movie> = vec![];
        let credits = self.get_credits().await;
        let other_person_credits = other_person.get_credits().await;

        if let Some(credits) = credits {
            if let Some(other_person_credits) = other_person_credits {
                let movie_ids = credits.get_movies_ids();
                let other_movie_ids = other_person_credits.get_movies_ids();

                for id in movie_ids {
                    if other_movie_ids.contains(&id) {
                        let movie = get_movie(&id).await.expect("Could not get movie");
                        movies.push(movie)
                    }
                }
            }
        }

        movies
    }
}


impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string_pretty(&self).expect("Could not Display Peron");
        write!(f, "{}", json)
    }
}