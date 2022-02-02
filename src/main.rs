mod api;
mod args;
mod errors;
mod structs;

use api::search_person;
use args::ArgType;
use errors::{Error, Kind, Result};
use structs::person::Person;

const KEVIN_BACON: &str = "4724";
const JOHN_LITHGOW: &str = "12074";
const DANNY_MCBRIDE: &str = "62862";
const SETH_ROGAN: &str = "19274";

#[tokio::main]
async fn main() {
    let args: ArgType = ArgType::from_env();

    let result = match args {
        ArgType::PersonSearch(person_name) => search_subcommand(&person_name).await,
        ArgType::PersonCompare((first, second)) => todo!(),
        ArgType::PersonChain(_) => todo!(),
        ArgType::Invalid => Err(Kind::InvalidArgs.to_error()),
    };

    if let Ok(_) = result {
        ()
    } else {
        handle_error(result.unwrap_err())
    }
}

fn handle_error(err: Error) {
    match err.kind {
        Kind::NoArgs => {
            eprintln!("No args provided. Try sending a series of double quoted names.");
            eprintln!("Example:");
            eprintln!("degrees \"Kevin Bacon\" \"John Lithgow\"");
        }
        Kind::DataParsing((row, col, body)) => {
            let slice = &body[col - 20 .. col + 20];
            eprintln!("Error parsing data:");
            eprintln!("{}, {}", row, col);
            eprintln!("Error at {}: {}", col, slice);
        }
        _ => todo!(),
    }
}

async fn search_subcommand(person_name: &str) -> Result<Person> {
    let person_search = search_person(person_name).await?;
    let person = person_search.get_first_result()?;

    let person_with_details = person.get_details().await?;

    println!(
        "{} ( {} )",
        person_with_details.name,
        person_with_details.imdb_url()
    );

    if person.known_for.len() > 0 {
        print!("Known for: ");
        let movie_titles: Vec<String> = person
            .known_for
            .into_iter()
            .map(|m| {
                // Add some quotes around the title
                format!("\"{}\"", m.title)
            })
            .collect();
        let movie_string = movie_titles.join(", ");
        // for movie in person.known_for {
        println!(" {} ", movie_string);
        // }
        // print!("\n")
    }

    println!("----------");
    println!("{}", person_with_details.biography);

    Ok(person_with_details)
}

// async fn search_for_person(person_name: &str) {
//     println!("Searching for {}", person_name);
//     let results = search_person(person_name).await.expect("Could not search");

//     if let Some(first_result) = results.get_first_result() {
//         println!("{:?}", first_result)
//     }
// }

// async fn compare_people(first_person: Person, second_person: Person) {
//     let in_same_movie = first_person.in_same_movie(&second_person).await;

//     if in_same_movie {
//         let shared_movies = first_person.get_shared_movies(&second_person).await;

//         println!(
//             "{} and {} starred in {} movie(s) together.",
//             &first_person.name,
//             &second_person.name,
//             &shared_movies.len()
//         );

//         for (index, movie) in shared_movies.into_iter().enumerate() {
//             println!("  {}. {}", index + 1, &movie.title);
//         }
//     } else {
//         println!(
//             "{} and {} didn't star in anything together.",
//             &first_person.name, &second_person.name
//         );
//     }
// }

// async fn print_credits(person: &Person) -> () {
//     let credits = person.get_credits().await.expect("Could not get credits");

//     if let Ok(json_string) = serde_json::to_string_pretty(&credits) {
//         println!("{}", json_string);
//     }

//     ()
// }

// fn parse_args (args: &Vec<String>) -> Result<Vec<String>> {
//     let args: Vec<String> = env::args().collect();

//     if args.len() == 0 {
//         return Err( NoArgsError )
//     }

//     Ok(args)
// }
