use std::env;

use api::{get_person, search_person};
use structs::person::Person;

use crate::structs::person;

mod api;
mod structs;

const KEVIN_BACON: &str = "4724";
const JOHN_LITHGOW: &str = "12074";
const DANNY_MCBRIDE: &str = "62862";
const SETH_ROGAN: &str = "19274";

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let first_person_name = args.get(1).expect("ERROR: Missing First Person");
    let second_person_name = args.get(2);

    if second_person_name.is_none() {
        let first_person_search = search_person(first_person_name)
            .await
            .expect(&format!("Could not search for '{}'", first_person_name));

        let first_person = first_person_search.get_first_result().expect("Couldn't find any results for first person");

        // Since this comes from a search result, 
        // some of the fields might be missing.
        // lets hit the endpoint one more time to
        // get all the details
        let first_person = first_person.get_details().await.expect("Couldn't get details");

        println!("{}", first_person);
    } else if let Some(second_person_name) = second_person_name {
        let first_person_search = search_person(first_person_name)
            .await
            .expect(&format!("Could not search for '{}'", first_person_name));
        let second_person_search = search_person(second_person_name)
            .await
            .expect(&format!("Could not search for '{}'", second_person_name));

        let first_person = first_person_search
            .get_first_result()
            .expect("No results in first person");
        let second_person = second_person_search
            .get_first_result()
            .expect("No results in second person");

        compare_people(first_person, second_person).await;
    }
}

async fn search_for_person(person_name: &str) {
    println!("Searching for {}", person_name);
    let results = search_person(person_name).await.expect("Could not search");

    if let Some(first_result) = results.get_first_result() {
        println!("{:?}", first_result)
    }
}

async fn compare_people(first_person: Person, second_person: Person) {
    let in_same_movie = first_person.in_same_movie(&second_person).await;

    if in_same_movie {
        let shared_movies = first_person.get_shared_movies(&second_person).await;

        println!(
            "{} and {} starred in {} movie(s) together.",
            &first_person.name,
            &second_person.name,
            &shared_movies.len()
        );

        for (index, movie) in shared_movies.into_iter().enumerate() {
            println!("  {}. {}", index + 1, &movie.title);
        }
    } else {
        println!(
            "{} and {} didn't star in anything together.",
            &first_person.name, &second_person.name
        );
    }
}

async fn print_credits(person: &Person) -> () {
    let credits = person.get_credits().await.expect("Could not get credits");

    if let Ok(json_string) = serde_json::to_string_pretty(&credits) {
        println!("{}", json_string);
    }

    ()
}
