use peter_data_eng::{extract, query, transform_load};
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [action]", args[0]);
        return;
    }

    let action = &args[1];
    let now = Instant::now();
    match action.as_str() {
        "extract" => {
            extract(
                "https://raw.githubusercontent.com/cpyang123/DE-W5/refs/heads/main/train.csv",
                "data/housing_data.csv",
                "data",
            );
        }
        "transform_load" => match transform_load("data/housing_data.csv") {
            Ok(_) => println!("Data loaded successfully!"),
            Err(err) => eprintln!("Error: {:?}", err),
        },
        "query" => {
            if let Some(q) = args.get(2) {
                if let Err(err) = query(q) {
                    eprintln!("Error: {:?}", err);
                } else {
                    println!("Query executed successfully!");
                }
            } else {
                println!("Usage: {} query [SQL query]", args[0]);
            }
        }
        _ => {
            println!("Invalid action. Use 'extract', 'transform_load', or 'query'.");
        }
    }
    println!("Time Taken: {:.2?}", now.elapsed());
}