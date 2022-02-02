use std::{env, os::unix::process, process::exit};

use urlencoding::encode;

use crate::{
    errors::{Error, Kind, Result},
    structs::{credits::Credits, movie::Movie, person::Person, person_search::PersonSearch},
};

fn has_defined_api_key() -> Result<String> {
    let api_key = env::var("TMDB_API_KEY")?;
    Ok(api_key)
}

fn get_api_key() -> String {
    let api_key = has_defined_api_key();

    if let Ok(api_key) = api_key {
        api_key
    } else {
        eprintln!("$TMDB_API_KEY is undefined. Define it before running again.");
        exit(1);
    }
}

fn uri(path: &str) -> String {
    let api_key = get_api_key();
    format!("https://api.themoviedb.org/3/{}?api_key={}", path, api_key)
}

pub async fn search_person(person_name: &str) -> Result<PersonSearch> {
    let uri_with_params = format!(
        "{}&language=en-US&page=1&include_adult=false&query={}",
        uri("search/person"),
        encode(person_name)
    );

    let response = reqwest::get(uri_with_params).await?;
    let body = response.text().await?;

    let person_search: PersonSearch = serde_json::from_str(&body)?;

    Ok(person_search)
}

pub async fn get_person(id: &str) -> Result<Person> {
    let path = format!("person/{}", id);
    let uri = uri(&path);
    let response = reqwest::get(uri).await?;
    let body = response.text().await?;

    let person: serde_json::Result<Person> = serde_json::from_str(&body);

    if let Err(err) = person {
        return Err(Kind::DataParsing((err.line(), err.column(), body)).as_error());
    }

    Ok(person.unwrap())
}

pub async fn get_person_credits(id: &str) -> Result<Credits> {
    let path = format!("person/{}/movie_credits", id);
    let uri = uri(&path);
    let response = reqwest::get(uri).await?;
    let body = response.text().await?;

    let person_credits: Credits = serde_json::from_str(&body)?;

    Ok(person_credits)
}

pub async fn get_movie(id: &str) -> Result<Movie> {
    let path = format!("movie/{}", id);
    let uri = uri(&path);
    let response = reqwest::get(uri).await?;
    let body = response.text().await?;

    let person_credits: Movie = serde_json::from_str(&body)?;

    Ok(person_credits)
}

pub async fn get_movie_credits(id: &str) -> Result<Movie> {
    let path = format!("movie/{}/credits", id);
    let uri = uri(&path);
    let response = reqwest::get(uri).await?;
    let body = response.text().await?;

    let person_credits: Movie = serde_json::from_str(&body)?;

    Ok(person_credits)
}
