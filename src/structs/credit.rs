use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Credit {
    #[serde(rename(deserialize = "credit_id"))]
    pub id: String,
    #[serde(rename(deserialize = "id"))]
    pub movie_id: i32,
    pub title: String,
    pub overview: String,
}
