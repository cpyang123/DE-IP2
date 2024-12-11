[![Python Application Test with Github Actions by Peter](https://github.com/cpyang123/DE-W5/actions/workflows/test.yml/badge.svg)](https://github.com/cpyang123/DE-W5/actions/workflows/test.yml)


# IP2 - Mini8 - Rust-version Command Line Tool for Managing House Price Records

## Demo Video (click on picture)
[![Quick Demo Video](https://img.youtube.com/vi/0EVFW7Jm_ko/hqdefault.jpg)](https://www.youtube.com/watch?v=0EVFW7Jm_ko)

## Overview

This project provides a command-line interface (CLI) that's implemented with both Rust and Python for managing house price records stored in a SQLite database. The tool allows users to create, read, update, and delete records in a database, as well as initialize the database with a predefined dataset.

We provide both the python and rust-based implementation here.

## Features
0. **Rust vs Python**:
   To run the commands, prefix the commands with the appropriate calls to the code:
   ```{bash}
   # With Rust:
   cargo run <command>

   # Rust Binary
   ./target/release/peter_data_eng

   # Python
   python main.ppy
   ```
   
2. **Initialize the Database**:
   - Loads a CSV file (`train.csv`) containing house price data and stores it in a SQLite database.
   - Command: `init`
   
3. **Create a New Record**:
   - Allows users to insert a new house price record into the database with specified details such as median income, house age, and median house value.
   - Command: `create_record`
   - Required Arguments:
     - `MedInc` (Median Income)
     - `HouseAge`
     - `AveRooms` (Average Rooms)
     - `AveBedrms` (Average Bedrooms)
     - `Population`
     - `AveOccup` (Average Occupants)
     - `Latitude`
     - `Longitude`
     - `MedHouseVal` (Median House Value)

4. **Update an Existing Record**:
   - Updates an existing house price record identified by an ID. All the attributes of the record can be modified.
   - Command: `update_record`
   - Required Arguments:
     - `id` (ID of the record)
     - `MedInc` (Median Income)
     - `HouseAge`
     - `AveRooms` (Average Rooms)
     - `AveBedrms` (Average Bedrooms)
     - `Population`
     - `AveOccup` (Average Occupants)
     - `Latitude`
     - `Longitude`
     - `MedHouseVal` (Median House Value)

5. **Read Records**:
   - Retrieves house price records from the database. Users can either retrieve all records or specify an ID to fetch a single record.
   - Command: `read`
   - Optional Argument:
     - `--id` (ID of the record to read; if not provided, all records are displayed)

6. **Delete a Record**:
   - Deletes a house price record from the database based on a provided ID.
   - Command: `delete_record`
   - Required Argument:
     - `--id` (ID of the record to delete)

## Getting Started

### Prerequisites


#### For python
- Python 3.x
- SQLite3
- A CSV file (`train.csv`) with the following structure:
  - `MedInc`: Median income in the area
  - `HouseAge`: Age of the house
  - `AveRooms`: Average number of rooms in houses
  - `AveBedrms`: Average number of bedrooms in houses
  - `Population`: Population of the area
  - `AveOccup`: Average occupancy
  - `Latitude`: Latitude coordinate
  - `Longitude`: Longitude coordinate
  - `MedHouseVal`: Median house value

#### For Rust:
- Rust 2021
- Everything in Cargo.toml

### Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd <repository-directory>
    ```

2. Run make all:
    This will install all requirements and make sure everything is running right
    For the Rust implementation:
    ```bash
    make all
    ```
    For the python version:
    ```bash
    make pyall
    ```

### Usage

0. **Compile (Rust only)**:
   ```bash
   make release
   ```

1. **Initialize the Database**:
   With Rust:
   ```bash
   # With binary
   ./target/release/peter_data_eng init

   # With Cargo
   cargo run init
   ```

   Python
   ```bash
   python main.py init
   ``` 

2. **Create a New Record**:
   Rust:
   ```bash
   ./target/release/peter_data_eng create_record <MedInc> <HouseAge> <AveRooms> <AveBedrms> <Population> <AveOccup> <Latitude> <Longitude> <MedHouseVal>

   # Or
   cargo run create_record <MedInc> <HouseAge> <AveRooms> <AveBedrms> <Population> <AveOccup> <Latitude> <Longitude> <MedHouseVal>
   ```

   Python
    ```bash
    python main.py create_record <MedInc> <HouseAge> <AveRooms> <AveBedrms> <Population> <AveOccup> <Latitude> <Longitude> <MedHouseVal>
    ```

3. **Update Existing Record**:
   Rust:
   ```bash
   # Binary
   ./target/release/peter_data_eng update_record <id> <MedInc> <HouseAge> <AveRooms> <AveBedrms> <Population> <AveOccup> <Latitude> <Longitude> <MedHouseVal>
   
   cargo run update_record <id> <MedInc> <HouseAge> <AveRooms> <AveBedrms> <Population> <AveOccup> <Latitude> <Longitude> <MedHouseVal>
   ```

   With Python
   ```bash
   python main.py update_record <id> <MedInc> <HouseAge> <AveRooms> <AveBedrms> <Population> <AveOccup> <Latitude> <Longitude> <MedHouseVal>
   ```

4. **Read Records**:
    Read all records:
    ```bash
    # Rust binary
    ./target/release/peter_data_eng read

    # Rust cargo
    cargo run read

    # Python
    python main.py read
    ```

    To read a specific record by ID:
    ```bash
    # Rust binary
    ./target/release/peter_data_eng read --id <id>
    
    # Rust cargo
    cargo run read --id <id>

    # Python
    python main.py read <id>
    ```

5. **Delete a Record**:
    ```bash
    # Rust binary
    ./target/release/peter_data_eng delete_record --id <id>

    # Rust cargo
    cargo run delete_record --id <id>

    # Python
    python main.py delete_record --id <id>
    ```

### Python vs Rust Processing Time
One of the main objective of this project is to compare the performmance for data processing between rust and python. 
We have tested the performance via initialization of the database and a read all. Here's the result:

#### Python Times:
```bash
python main.py read 
```
Time taken: 0.013905048370361328

```bash
python main.py init 
```
Time taken: 0.07440614700317383 
This is largely because the python implementation doesn't recreate the database

#### Rust with Cargo
```bash
cargo run read
```
Time taken: 6.330228ms

```bash
cargo run init
```
Time taken: 321.055324ms

#### Rust Binary
```bash
./target/release/peter_data_eng read 
```
Time taken: 4.361656ms

```bash
./target/release/peter_data_eng init train.csv 
```
Time taken: 309.027212ms


### LLM Contribution

I've used LLM to double check and generate test cases and double check syntaxes for Rust. I also consulted ChatGPT on ideas to create the functions.
