use serde::{Deserialize, Serialize};

use crate::api::{get_person_credits, get_movie};

use super::{credits::Credits, credit::Credit, movie::Movie};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub profile_path: String
}

impl Person {
    pub async fn get_credits (&self) -> Option<Credits> {
        let id_str = format!("{}", self.id);
        get_person_credits(&id_str).await
    }

    pub async fn in_same_movie (&self, other_person: &Person) -> Option<Movie> {
        let credits = self.get_credits().await;
        let other_person_credits = other_person.get_credits().await;

        if let Some(credits) = credits {
            if let Some(other_person_credits) = other_person_credits {


                let movie_ids = credits.get_movies_ids();
                let other_movie_ids = other_person_credits.get_movies_ids();

                for id in movie_ids {
                    if other_movie_ids.contains(&id) {
                        let movie = get_movie(&id).await;
                        return movie
                    }
                }
            }
        }
        
        None
    }

    pub async fn get_shared_movies (&self, other_person: &Person) -> Vec<Movie> {
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