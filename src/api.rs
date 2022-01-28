use std::env;

use crate::structs::{person::Person, credits::Credits, movie::Movie};


fn get_api_key() -> String {
    env::var("TMDB_API_KEY").expect("$TMDB_API_KEY is undefined.")
}

fn uri(path: &str) -> String {
    let api_key = get_api_key();
    format!("https://api.themoviedb.org/3/{}?api_key={}", path, api_key)
}

pub async fn get_person(id: &str) -> Option<Person> {
    let path = format!("person/{}", id);
    let uri = uri(&path);
    let response = reqwest::get(uri).await.expect("Could not reach site.");
    let body = response.text().await.expect("Could not parse body into text.");


    let person: Person = serde_json::from_str(&body).expect("Could not parse response into Person.");

    Some(person)
}

pub async fn get_person_credits(id: &str) -> Option<Credits> {
    let path = format!("person/{}/movie_credits", id);
    let uri = uri(&path);
    let response = reqwest::get(uri).await.expect("Could not reach site.");
    let body = response.text().await.expect("Could not parse body into text.");


    let person_credits: Credits = serde_json::from_str(&body).expect("Could not parse response into Credits.");

    Some(person_credits)
}


pub async fn get_movie(id: &str) -> Option<Movie> {
    let path = format!("movie/{}", id);
    let uri = uri(&path);
    let response = reqwest::get(uri).await.expect("Could not reach site.");
    let body = response.text().await.expect("Could not parse body into text.");


    let person_credits: Movie = serde_json::from_str(&body).expect("Could not parse response into Movie.");

    Some(person_credits)
}

pub async fn get_movie_credits(id: &str) -> Option<Movie> {
    let path = format!("movie/{}/credits", id);
    let uri = uri(&path);
    let response = reqwest::get(uri).await.expect("Could not reach site.");
    let body = response.text().await.expect("Could not parse body into text.");


    let person_credits: Movie = serde_json::from_str(&body).expect("Could not parse response into Movie.");

    Some(person_credits)
}