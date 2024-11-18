use sqlx::{sqlite::SqlitePool, Pool};
use std::error::Error;
use std::fs;
use tokio;

use peter_data_eng::{
    create_database_pool, delete_record, insert_house_prices, load_csv_to_struct,
    select_all_records, select_record_by_id, update_house_price, HousePrice,
};

// Helper function to create a temporary database for testing
async fn setup_test_db() -> Result<Pool<sqlx::Sqlite>, Box<dyn Error>> {
    let db_path = "/database/test_database.db";
    if fs::metadata(db_path).is_ok() {
        fs::remove_file(db_path)?; // Remove old test database if it exists
    }

    let pool = create_database_pool("test_database.db").await?;

    sqlx::query("DROP TABLE IF EXISTS tbl_house_prices")
        .execute(&pool)
        .await?;
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

#[tokio::test]
async fn test_create_database_pool() -> Result<(), Box<dyn Error>> {
    let pool = setup_test_db().await?;
    assert!(pool.acquire().await.is_ok()); // Ensure connection can be acquired
    Ok(())
}

#[tokio::test]
async fn test_insert_and_select_record() -> Result<(), Box<dyn Error>> {
    let pool = setup_test_db().await?;
    sqlx::query("DROP TABLE IF EXISTS tbl_house_prices")
        .execute(&pool)
        .await?;
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

    // Insert a test record
    let test_records = vec![HousePrice {
        id: None,
        MedInc: Some(3.5),
        HouseAge: Some(20.0),
        AveRooms: Some(5.0),
        AveBedrms: Some(1.0),
        Population: Some(500.0),
        AveOccup: Some(3.2),
        Latitude: Some(37.77),
        Longitude: Some(-122.42),
        MedHouseVal: Some(450000.0),
    }];
    insert_house_prices(&pool, test_records.clone()).await?;

    // Select all records and verify
    let records = select_all_records(&pool).await?;
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].MedInc, test_records[0].MedInc);

    Ok(())
}

#[tokio::test]
async fn test_select_record_by_id() -> Result<(), Box<dyn Error>> {
    let pool = setup_test_db().await?;

    sqlx::query("DROP TABLE IF EXISTS tbl_house_prices")
        .execute(&pool)
        .await?;
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

    // Insert a test record
    let test_record = HousePrice {
        id: None,
        MedInc: Some(3.5),
        HouseAge: Some(20.0),
        AveRooms: Some(5.0),
        AveBedrms: Some(1.0),
        Population: Some(500.0),
        AveOccup: Some(3.2),
        Latitude: Some(37.77),
        Longitude: Some(-122.42),
        MedHouseVal: Some(450000.0),
    };
    insert_house_prices(&pool, vec![test_record.clone()]).await?;

    // Select the record by ID
    let record = select_record_by_id(&pool, 1).await?;
    assert_eq!(record.MedInc, test_record.MedInc);

    Ok(())
}

#[tokio::test]
async fn test_update_house_price() -> Result<(), Box<dyn Error>> {
    let pool = setup_test_db().await?;
    sqlx::query("DROP TABLE IF EXISTS tbl_house_prices")
        .execute(&pool)
        .await?;
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

    // Insert a test record
    let test_record = HousePrice {
        id: None,
        MedInc: Some(3.5),
        HouseAge: Some(20.0),
        AveRooms: Some(5.0),
        AveBedrms: Some(1.0),
        Population: Some(500.0),
        AveOccup: Some(3.2),
        Latitude: Some(37.77),
        Longitude: Some(-122.42),
        MedHouseVal: Some(450000.0),
    };
    insert_house_prices(&pool, vec![test_record]).await?;

    // Update the record
    let success = update_house_price(
        &pool,
        1,
        Some(4.5), // New value for `MedInc`
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await?;
    assert!(success);

    // Verify the update
    let record = select_record_by_id(&pool, 1).await?;
    assert_eq!(record.MedInc, Some(4.5));

    Ok(())
}

#[tokio::test]
async fn test_delete_record() -> Result<(), Box<dyn Error>> {
    let pool = setup_test_db().await?;
    sqlx::query("DROP TABLE IF EXISTS tbl_house_prices")
        .execute(&pool)
        .await?;
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

    // Insert a test record
    let test_record = HousePrice {
        id: None,
        MedInc: Some(3.5),
        HouseAge: Some(20.0),
        AveRooms: Some(5.0),
        AveBedrms: Some(1.0),
        Population: Some(500.0),
        AveOccup: Some(3.2),
        Latitude: Some(37.77),
        Longitude: Some(-122.42),
        MedHouseVal: Some(450000.0),
    };
    insert_house_prices(&pool, vec![test_record]).await?;

    // Delete the record
    let success = delete_record(&pool, 1).await?;
    assert!(success);

    // Verify the record was deleted
    let records = select_all_records(&pool).await?;
    assert!(records.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_load_csv_to_struct() -> Result<(), Box<dyn Error>> {
    // Create a temporary CSV file
    let csv_content =
        "MedInc,HouseAge,AveRooms,AveBedrms,Population,AveOccup,Latitude,Longitude,MedHouseVal\n\
        3.5,20.0,5.0,1.0,500,3.2,37.77,-122.42,450000.0";
    let csv_path = "./test_data.csv";
    fs::write(csv_path, csv_content)?;

    // Load the CSV into a struct
    let records = load_csv_to_struct(csv_path).await?;
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].MedInc, Some(3.5));

    // Clean up the temporary CSV file
    fs::remove_file(csv_path)?;

    Ok(())
}
