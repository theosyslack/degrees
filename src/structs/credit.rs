use serde::{Deserialize, Serialize};

use crate::api::get_movie;

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
    pub async fn as_movie(&self) -> Movie {
        Movie {
            id: self.movie_id,
            title: self.title.clone(),
            overview: self.overview.clone(),
        }
    }
}
