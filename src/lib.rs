use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::error::Error;
use std::fs;
use std::path::Path;

// Define the HousePrice structure
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)] // Added Clone
pub struct HousePrice {
    // Made public
    pub id: Option<i32>,
    pub MedInc: Option<f32>,
    pub HouseAge: Option<f32>,
    pub AveRooms: Option<f32>,
    pub AveBedrms: Option<f32>,
    pub Population: Option<f32>,
    pub AveOccup: Option<f32>,
    pub Latitude: Option<f32>,
    pub Longitude: Option<f32>,
    pub MedHouseVal: Option<f32>,
}

// Create or connect to the database
pub async fn create_database_pool(db_name: &str) -> Result<Pool<Sqlite>, Box<dyn Error>> {
    let database_url = format!("sqlite://./database/{}", db_name);
    let database_path = format!("{}{}", "./database/", db_name);
    let _conn = Connection::open(&database_path)?;
    let pool = SqlitePool::connect(&database_url).await?;

    println!("Creating Database Pool");

    // Create the table if it doesn't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tbl_house_prices (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            MedInc REAL,
            HouseAge REAL,
            AveRooms REAL,
            AveBedrms REAL,
            Population REAL,
            AveOccup REAL,
            Latitude REAL,
            Longitude REAL,
            MedHouseVal REAL
        )",
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

// Load a CSV file into a vector of HousePrice structs
pub async fn load_csv_to_struct(file_path: &str) -> Result<Vec<HousePrice>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: HousePrice = result?;
        records.push(record);
    }
    Ok(records)
}

// Insert records into the database
pub async fn insert_house_prices(
    pool: &Pool<Sqlite>,
    records: Vec<HousePrice>,
) -> Result<(), Box<dyn Error>> {
    for record in records {
        sqlx::query(
            "INSERT INTO tbl_house_prices (
                MedInc, HouseAge, AveRooms, AveBedrms, Population, AveOccup, Latitude, Longitude, MedHouseVal
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(record.MedInc)
        .bind(record.HouseAge)
        .bind(record.AveRooms)
        .bind(record.AveBedrms)
        .bind(record.Population)
        .bind(record.AveOccup)
        .bind(record.Latitude)
        .bind(record.Longitude)
        .bind(record.MedHouseVal)
        .execute(pool)
        .await?;
    }
    Ok(())
}

// Query all records from the database
pub async fn select_all_records(pool: &Pool<Sqlite>) -> Result<Vec<HousePrice>, Box<dyn Error>> {
    let records = sqlx::query_as::<_, HousePrice>("SELECT * FROM tbl_house_prices")
        .fetch_all(pool)
        .await?;
    Ok(records)
}

// Query a specific record by ID
pub async fn select_record_by_id(
    pool: &Pool<Sqlite>,
    id: i32,
) -> Result<HousePrice, Box<dyn Error>> {
    let record = sqlx::query_as::<_, HousePrice>("SELECT * FROM tbl_house_prices WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(record)
}

// Update a record by ID
pub async fn update_house_price(
    pool: &Pool<Sqlite>,
    id: i32,
    medinc: Option<f32>,
    houseage: Option<f32>,
    averooms: Option<f32>,
    avebedrms: Option<f32>,
    population: Option<f32>,
    aveoccup: Option<f32>,
    latitude: Option<f32>,
    longitude: Option<f32>,
    medhouseval: Option<f32>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut query = String::from("UPDATE tbl_house_prices SET ");
    let mut bindings: Vec<f32> = Vec::new(); // Explicitly set to `Vec<f32>`

    if let Some(value) = medinc {
        query.push_str("MedInc = ?, ");
        bindings.push(value);
    }
    if let Some(value) = houseage {
        query.push_str("HouseAge = ?, ");
        bindings.push(value);
    }
    if let Some(value) = averooms {
        query.push_str("AveRooms = ?, ");
        bindings.push(value);
    }
    if let Some(value) = avebedrms {
        query.push_str("AveBedrms = ?, ");
        bindings.push(value);
    }
    if let Some(value) = population {
        query.push_str("Population = ?, ");
        bindings.push(value as f32); // Convert to `f32` to match the bindings type
    }
    if let Some(value) = aveoccup {
        query.push_str("AveOccup = ?, ");
        bindings.push(value);
    }
    if let Some(value) = latitude {
        query.push_str("Latitude = ?, ");
        bindings.push(value);
    }
    if let Some(value) = longitude {
        query.push_str("Longitude = ?, ");
        bindings.push(value);
    }
    if let Some(value) = medhouseval {
        query.push_str("MedHouseVal = ?, ");
        bindings.push(value);
    }

    // Remove trailing comma and space
    if query.ends_with(", ") {
        query.truncate(query.len() - 2);
    }

    query.push_str(" WHERE id = ?");

    // Prepare the query
    let mut sql_query = sqlx::query(&query);

    // Bind all parameters dynamically
    for binding in bindings {
        sql_query = sql_query.bind(binding);
    }

    // Bind the ID (last parameter)
    sql_query = sql_query.bind(id);

    // Execute the query
    let result = sql_query.execute(pool).await?;
    Ok(result.rows_affected() > 0)
}

// Delete a record by ID
pub async fn delete_record(pool: &Pool<Sqlite>, id: i32) -> Result<bool, Box<dyn Error>> {
    let result = sqlx::query("DELETE FROM tbl_house_prices WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}
