use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub overview: String,
}
