use clap::{App, Arg, SubCommand};
use sqlx::sqlite::SqlitePool; // Only keep this if you're using it
use std::error::Error;

mod lib;
use crate::lib::{
    create_database_pool, delete_record, insert_house_prices, load_csv_to_struct,
    select_all_records, select_record_by_id, update_house_price, HousePrice,
};

use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the argument parser
    let matches =
        App::new("Record Management CLI")
            .version("1.0")
            .about("Command line tool for managing records")
            .subcommand(
                SubCommand::with_name("init")
                    .about("Create Database and load data")
                    .arg(
                        Arg::with_name("csv_path")
                            .help("Path to CSV file")
                            .required(true),
                    ),
            )
            .subcommand(
                SubCommand::with_name("create_record")
                    .about("Create a new record")
                    .arg(
                        Arg::with_name("MedInc")
                            .help("MedInc of the record")
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("HouseAge")
                            .help("HouseAge of the record")
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("AveRooms")
                            .help("AveRooms of the record")
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("AveBedrms")
                            .help("AveBedrms of the record")
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("Population")
                            .help("Population of the record")
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("AveOccup")
                            .help("AveOccup of the record")
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("Latitude")
                            .help("Latitude of the record")
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("Longitude")
                            .help("Longitude of the record")
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("MedHouseVal")
                            .help("MedHouseVal of the record")
                            .required(true),
                    ),
            )
            .subcommand(
                SubCommand::with_name("update_record")
                    .about("Update an existing record")
                    .arg(
                        Arg::with_name("id")
                            .help("ID of the record to update")
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("MedInc")
                            .help("MedInc of the record")
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("HouseAge")
                            .help("HouseAge of the record")
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("AveRooms")
                            .help("AveRooms of the record")
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("AveBedrms")
                            .help("AveBedrms of the record")
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("Population")
                            .help("Population of the record")
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("AveOccup")
                            .help("AveOccup of the record")
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("Latitude")
                            .help("Latitude of the record")
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("Longitude")
                            .help("Longitude of the record")
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("MedHouseVal")
                            .help("MedHouseVal of the record")
                            .required(true),
                    ),
            )
            .subcommand(SubCommand::with_name("read").about("Read a record").arg(
                Arg::with_name("id").help("ID of the record to read (leave empty to read all)"),
            ))
            .subcommand(
                SubCommand::with_name("delete_record")
                    .about("Delete a record")
                    .arg(
                        Arg::with_name("id")
                            .help("ID of the record to delete")
                            .required(true),
                    ),
            )
            .get_matches();

    // Create a connection pool
    let db_pool = create_database_pool("demo.db").await?;
    let start = Instant::now();
    println!("Handling Command Logic");
    // Command handling logic
    if let Some(matches) = matches.subcommand_matches("init") {
        println!("Initializing...");
        let database_url = format!("sqlite://./database/{}", "demo.db");
        let pool = SqlitePool::connect(&database_url).await?;

        sqlx::query("DROP TABLE IF EXISTS tbl_house_prices")
            .execute(&pool)
            .await?;

        let db_pool = create_database_pool("demo.db").await?;
        let csv_path = matches.value_of("csv_path").unwrap();
        let records = load_csv_to_struct(csv_path).await?;
        insert_house_prices(&db_pool, records).await?;
        println!("Database initialized and data loaded from {}", csv_path);
    } else if let Some(matches) = matches.subcommand_matches("create_record") {
        println!("Creating Records...");
        let record = HousePrice {
            id: None,
            MedInc: matches.value_of("MedInc").map(|v| v.parse()).transpose()?,
            HouseAge: matches
                .value_of("HouseAge")
                .map(|v| v.parse())
                .transpose()?,
            AveRooms: matches
                .value_of("AveRooms")
                .map(|v| v.parse())
                .transpose()?,
            AveBedrms: matches
                .value_of("AveBedrms")
                .map(|v| v.parse())
                .transpose()?,
            Population: matches
                .value_of("Population")
                .map(|v| v.parse())
                .transpose()?,
            AveOccup: matches
                .value_of("AveOccup")
                .map(|v| v.parse())
                .transpose()?,
            Latitude: matches
                .value_of("Latitude")
                .map(|v| v.parse())
                .transpose()?,
            Longitude: matches
                .value_of("Longitude")
                .map(|v| v.parse())
                .transpose()?,
            MedHouseVal: matches
                .value_of("MedHouseVal")
                .map(|v| v.parse())
                .transpose()?,
        };
        insert_house_prices(&db_pool, vec![record]).await?;
        println!("Record created successfully.");
    } else if let Some(matches) = matches.subcommand_matches("update_record") {
        println!("Updating Record(s)...");
        let id = matches.value_of("id").unwrap().parse()?;
        update_house_price(
            &db_pool,
            id,
            Some(matches.value_of("MedInc").unwrap().parse()?),
            Some(matches.value_of("HouseAge").unwrap().parse()?),
            Some(matches.value_of("AveRooms").unwrap().parse()?),
            Some(matches.value_of("AveBedrms").unwrap().parse()?),
            Some(matches.value_of("Population").unwrap().parse()?),
            Some(matches.value_of("AveOccup").unwrap().parse()?),
            Some(matches.value_of("Latitude").unwrap().parse()?),
            Some(matches.value_of("Longitude").unwrap().parse()?),
            Some(matches.value_of("MedHouseVal").unwrap().parse()?),
        )
        .await?;
        println!("Record with ID {} updated successfully.", id);
    } else if let Some(matches) = matches.subcommand_matches("read") {
        if let Some(id) = matches.value_of("id") {
            println!("Retriving Record...");
            let id = id.parse()?;
            let record = select_record_by_id(&db_pool, id).await?;
            println!("Record with ID {}: {:?}", id, record);
        } else {
            println!("Retriving All Record(s)...");
            let records = select_all_records(&db_pool).await?;
            for record in records {
                println!("{:?}", record);
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("delete_record") {
        let id = matches.value_of("id").unwrap().parse()?;
        let success = delete_record(&db_pool, id).await?;
        if success {
            println!("Record with ID {} deleted successfully.", id);
        } else {
            println!("No record found with ID {}.", id);
        }
    } else {
        println!("No valid command provided. Use --help for more details.");
    }

    println!("Time taken: {:?}", start.elapsed());
    Ok(())
}
