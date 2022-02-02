mod api;
mod args;
mod errors;
mod structs;

use std::process::exit;

use api::search_person;
use args::ArgType;
use errors::{Error, Kind, Result};

#[tokio::main]
async fn main() {
    let args: ArgType = ArgType::from_env();

    let result = match args {
        ArgType::PersonSearch(person_name) => search_subcommand(&person_name).await,
        ArgType::PersonCompare((first, second)) => compare_subcommand(&first, &second).await,
        ArgType::PersonChain(_) => todo!(),
        ArgType::NoneProvided => Err(Kind::NoArgs.as_error()),
    };

    if let Err(err) = result {
        handle_error(err)
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
            let slice = get_data_parsing_error_slice(&body, col);
            eprintln!("Error parsing data:");
            eprintln!("{}, {}", row, col);
            eprintln!("Error at {}: {}", col, slice);
        }
        Kind::PersonSearchNoResults => {
            eprintln!("No Person found for query.");
        }
        _ => {
            eprintln!("{:?}", err);
            todo!()
        }
    }

    exit(1);
}

async fn search_subcommand(person_name: &str) -> Result<()> {
    let person_search = search_person(person_name).await?;
    let person = person_search.get_first_result()?;

    let person_with_details = person.get_details().await?;

    println!(
        "[{}]({})",
        person_with_details.name,
        person_with_details.imdb_url()
    );

    if person.known_for.is_empty() {
        print!("Known for: ");
        let movie_titles: Vec<String> = person
            .known_for
            .into_iter()
            .map(|m| {
                // Add some quotes around the title
                format!("\"{}\"", m)
            })
            .collect();
        let movie_string = movie_titles.join(", ");
        // for movie in person.known_for {
        println!(" {} ", movie_string);
        // }
        // print!("\n")
    }

    println!("---");

    // Print the bio as a markdown quote.
    pretty_print_description(&person_with_details.biography);
    // for line in .split_at(mid) {
    //     println!("> {}", line);    
    // }
    // println!("{}", person_with_details.biography);
    // println!(">>>");


    Ok(())
}

async fn compare_subcommand(first: &str, second: &str) -> Result<()> {
    let first_person_search = search_person(first).await?;
    let second_person_search = search_person(second).await?;

    let first_person = first_person_search
        .get_first_result()?
        .get_details()
        .await?;
    let second_person = second_person_search
        .get_first_result()?
        .get_details()
        .await?;

    let in_same_movie = first_person.in_same_movie(&second_person).await?;

    if in_same_movie {
        let shared_movies = first_person.get_shared_movies(&second_person).await?;

        println!(
            "{} and {} starred in {} movie(s) together.",
            &first_person.name,
            &second_person.name,
            &shared_movies.len()
        );

        for (index, movie) in shared_movies.into_iter().enumerate() {
            println!("  {}. [{}]({})", index + 1, &movie.title, &movie.imdb_url());
        }
    } else {
        println!(
            "{} and {} didn't star in anything together.",
            &first_person.name, &second_person.name
        );
    }

    Ok(())
}
// async fn search_for_person(person_name: &str) {
//     println!("Searching for {}", person_name);
//     let results = search_person(person_name).await.expect("Could not search");

//     if let Some(first_result) = results.get_first_result() {
//         println!("{:?}", first_result)
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


fn get_data_parsing_error_slice(body: &str, col: usize) -> &str {
    let before = 20;
    let after = 20;
    let start = col - before;
    let end = col - after;

    // Ensure there's enough room for start and end.
    let slice = body.get(start..end);

    if let Some(slice) = slice {
        slice
    } else {
        body
    }
}

fn pretty_print_description(description: &str) -> () {
    for line in description.lines() {
        println!("> {}", line);
    }
}