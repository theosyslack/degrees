use api::get_person;
use structs::person::Person;

mod api;
mod structs;

const KEVIN_BACON: &str = "4724";
const JOHN_LITHGOW: &str = "12074";
const DANNY_MCBRIDE: &str = "62862";
const SETH_ROGAN: &str = "19274";

#[tokio::main]
async fn main() {
    let first_person = get_person(DANNY_MCBRIDE).await.expect("Failed for unknown reason.");
    let second_person = get_person(SETH_ROGAN).await.expect("Failed for unknown reason.");

    let in_same_movie = first_person.in_same_movie(&second_person).await;

    if in_same_movie.is_some() {
        let shared_movies = first_person.get_shared_movies(&second_person).await;

        println!("{} and {} starred in {} movie(s) together.", &first_person.name, &second_person.name, &shared_movies.len());

        for (index, movie) in shared_movies.into_iter().enumerate() {
            println!("  {}. {}", index + 1, &movie.title);
        }
    } else {
        println!("{} and {} didn't star in anything together.", &first_person.name, &second_person.name);
    }

}


async fn print_credits(person: &Person) -> () {
    let credits = person.get_credits().await.expect("Could not get credits");
      
    if let Ok(json_string) = serde_json::to_string_pretty(&credits) {
        println!("{}", json_string);
    } 

    ()
}