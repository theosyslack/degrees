use std::{env, fmt::format};

use serde_json::Result;
use urlencoding::encode;

use crate::structs::{
    credits::Credits,
    movie::Movie,
    person::Person,
    person_search::{self, PersonSearch},
};

fn get_api_key() -> String {
    env::var("TMDB_API_KEY").expect("$TMDB_API_KEY is undefined.")
}

fn uri(path: &str) -> String {
    let api_key = get_api_key();
    format!("https://api.themoviedb.org/3/{}?api_key={}", path, api_key)
}

pub async fn search_person(person_name: &str) -> Option<PersonSearch> {
    let uri_with_params = format!(
        "{}&language=en-US&page=1&include_adult=false&query={}",
        uri("search/person"),
        encode(person_name)
    );

    println!("{}", &uri_with_params);
    let response = reqwest::get(uri_with_params)
        .await
        .expect("Could not reach site.");
    let body = response
        .text()
        .await
        .expect("Could not parse body into text.");

    let maybe_person_search: Result<PersonSearch> = serde_json::from_str(&body);

    if let Ok(person_search) = maybe_person_search {
        Some(person_search)
    } else {
        println!("Couldn't parse");
        println!("{}", &body);
        println!("{}", maybe_person_search.unwrap_err());

        None
    }
    // let person_search: PersonSearch = serde_json::from_str(&body).expect("Could not parse response into PersonSearch.");
}

pub async fn get_person(id: &str) -> Option<Person> {
    let path = format!("person/{}", id);
    let uri = uri(&path);
    let response = reqwest::get(uri).await.expect("Could not reach site.");
    let body = response
        .text()
        .await
        .expect("Could not parse body into text.");

    let person: Person =
        serde_json::from_str(&body).expect("Could not parse response into Person.");

    Some(person)
}

pub async fn get_person_credits(id: &str) -> Option<Credits> {
    let path = format!("person/{}/movie_credits", id);
    let uri = uri(&path);
    let response = reqwest::get(uri).await.expect("Could not reach site.");
    let body = response
        .text()
        .await
        .expect("Could not parse body into text.");

    let person_credits: Credits =
        serde_json::from_str(&body).expect("Could not parse response into Credits.");

    Some(person_credits)
}

pub async fn get_movie(id: &str) -> Option<Movie> {
    let path = format!("movie/{}", id);
    let uri = uri(&path);
    let response = reqwest::get(uri).await.expect("Could not reach site.");
    let body = response
        .text()
        .await
        .expect("Could not parse body into text.");

    let person_credits: Movie =
        serde_json::from_str(&body).expect("Could not parse response into Movie.");

    Some(person_credits)
}

pub async fn get_movie_credits(id: &str) -> Option<Movie> {
    let path = format!("movie/{}/credits", id);
    let uri = uri(&path);
    let response = reqwest::get(uri).await.expect("Could not reach site.");
    let body = response
        .text()
        .await
        .expect("Could not parse body into text.");

    let person_credits: Movie =
        serde_json::from_str(&body).expect("Could not parse response into Movie.");

    Some(person_credits)
}
