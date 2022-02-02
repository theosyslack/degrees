use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    pub id: i32,
    pub imdb_id: String,
    pub title: String,
    pub overview: String,
}

impl Movie {
    pub fn imdb_url(&self) -> String {
        format!("https://www.imdb.com/title/{}", &self.imdb_id)
    }
}
