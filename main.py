import argparse
from mylib.lib import (
    create_connect_database,
    delete_record,
    load_dataframe,
    load_to_table,
    select_all,
    insert_house_price_record,
    select_id,
)
from mylib.lib import update_house_price_record
import time


# Initialize the argument parser
parser = argparse.ArgumentParser(description="Command line tool for managing records")

# Create subparsers for the different commands
subparsers = parser.add_subparsers(dest="command", help="Available commands")

init_parser = subparsers.add_parser("init", help="Create Database and load data")

# 1. Create 'create_record' command
create_parser = subparsers.add_parser("create_record", help="Create a new record")
create_parser.add_argument("MedInc", type=float, help="MedInc of the record")
create_parser.add_argument("HouseAge", type=float, help="HouseAge of the record")
create_parser.add_argument("AveRooms", type=float, help="AveRooms of the record")
create_parser.add_argument("AveBedrms", type=float, help="AveBedrms of the record")
create_parser.add_argument("Population", type=float, help="Population of the record")
create_parser.add_argument("AveOccup", type=float, help="AveOccup of the record")
create_parser.add_argument("Latitude", type=float, help="Latitude of the record")
create_parser.add_argument("Longitude", type=float, help="Longitude of the record")
create_parser.add_argument("MedHouseVal", type=float, help="MedHouseVal of the record")

# 2. Create 'update_record' command
update_parser = subparsers.add_parser("update_record", help="Update an existing record")
update_parser.add_argument("id", type=int, help="ID of the record to update")
update_parser.add_argument("MedInc", type=float, help="MedInc of the record")
update_parser.add_argument("HouseAge", type=float, help="HouseAge of the record")
update_parser.add_argument("AveRooms", type=float, help="AveRooms of the record")
update_parser.add_argument("AveBedrms", type=float, help="AveBedrms of the record")
update_parser.add_argument("Population", type=float, help="Population of the record")
update_parser.add_argument("AveOccup", type=float, help="AveOccup of the record")
update_parser.add_argument("Latitude", type=float, help="Latitude of the record")
update_parser.add_argument("Longitude", type=float, help="Longitude of the record")
update_parser.add_argument("MedHouseVal", type=float, help="MedHouseVal of the record")

# 3. Create 'read' command
read_parser = subparsers.add_parser("read", help="Read a record")
read_parser.add_argument(
    "--id", type=int, help="ID of the record to read (leave empty to read all)"
)

# 4. Create 'delete_record' command
delete_parser = subparsers.add_parser("delete_record", help="Delete a record")
delete_parser.add_argument(
    "--id", type=int, required=True, help="ID of the record to delete"
)

# Parse the arguments
args = parser.parse_args()
start_time = time.time()

# Command handling logic
if args.command == "init":
    df = load_dataframe("train.csv")
    load_to_table(df, "tbl_house_prices", create_connect_database("demo.db"))

elif args.command == "create_record":
    # Handle record creation.
    engine = create_connect_database("demo.db")
    insert_house_price_record(
        args.MedInc,
        args.HouseAge,
        args.AveRooms,
        args.AveBedrms,
        args.Population,
        args.AveOccup,
        args.Latitude,
        args.Longitude,
        args.MedHouseVal,
    )
    print(f"Creating record: Medinc = {args.MedInc}, HouseAge = {args.HouseAge}")

elif args.command == "update_record":
    # Handle record update
    engine = create_connect_database("demo.db")
    update_house_price_record(
        args.id,
        args.MedInc,
        args.HouseAge,
        args.AveRooms,
        args.AveBedrms,
        args.Population,
        args.AveOccup,
        args.Latitude,
        args.Longitude,
        args.MedHouseVal,
    )
    print(f"Updated record ID {args.id}")

elif args.command == "read":
    # Handle reading records
    engine = create_connect_database("demo.db")
    with engine.connect() as connection:
        if args.id:
            print(f"Reading record with ID: {args.id}")
            rows = select_id("tbl_house_prices", args.id, connection)
            for row in rows:
                print(" | ".join(map(str, row)))
        else:
            print("Reading all records")
            rows = select_all("tbl_house_prices", connection)
            for row in rows:
                print(" | ".join(map(str, row)))

elif args.command == "delete_record":
    # Handle record deletion
    print(f"Deleting record with ID: {args.id}")
    engine = create_connect_database("demo.db")
    with engine.connect() as connection:
        delete_record("tbl_house_prices", args.id, connection)

else:
    print("No valid command provided. Use --help for more details.")

print("Time taken: {}".format(time.time() - start_time))
