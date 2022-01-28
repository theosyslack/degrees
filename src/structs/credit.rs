use serde::{Serialize, Deserialize};

use crate::api::{get_movie};

use super::movie::Movie;

#[derive(Debug, Serialize, Deserialize)]
pub struct Credit {
    #[serde(rename(deserialize = "credit_id"))]
    pub id: String,
    #[serde(rename(deserialize = "id"))]
    pub movie_id: i32,
    pub title: String,
    pub overview: String,
}

impl Credit {
    pub async fn get_movie (&self) -> Option<Movie> {
        let id_str = format!("{}", self.id);
        get_movie(&id_str).await
    }
}